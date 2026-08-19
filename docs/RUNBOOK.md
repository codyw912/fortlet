# Fortlet Runbook

Operational truth for verifying and running Fortlet. A behavior claim not
backed by a command here remains an agent claim rather than evidence.

## Standard verification set

Run from `nix develop` before claiming work complete:

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --test conformance
nix flake check
```

`nix flake check` builds the package for the current system and may take longer
than the Rust-only gates. Direct Cargo builds may download MicroSandbox's pinned
guest agent once per build profile; the Nix package uses fixed-output inputs and
builds offline. Native Linux verification is still outstanding.

## Running the system

```bash
nix run . -- doctor
nix run . -- prepare codex
nix run . -- run codex --
nix run . -- run tact --
nix run . -- list
nix run . -- status
nix run . -- stop codex
nix run . -- reset codex
```

For a development binary inside `nix develop`:

```bash
cargo run -- doctor
cargo run -- prepare codex
cargo run -- run codex --
cargo run -- list
cargo run -- status codex
cargo run -- stop codex
cargo run -- reset codex
```

Project resolution prefers the nearest Jujutsu root, then Git root, then a
recognized devenv or flake root. It canonicalizes the selected root and
preserves a launch from a subdirectory. Home-directory and filesystem-root
projects use Fortlet's persistent scratch workspace unless the user passes
`--allow-broad-mount`; `--project "$HOME"` and `--project /` do not imply that
override.

`doctor` reads authentication metadata but never prints token values. Real
launches require a healthy MicroSandbox host and a valid file-backed ChatGPT
credential. Codex launch and shim invocations acquire a source-scoped host
lease. A token with less than one hour of usable life is refreshed once with
bounded timeouts; successful refresh atomically replaces the host `auth.json`
at mode 0600 and live-rotates the existing MicroSandbox secrets without
restarting the capsule. The guest projection uses `chatgptAuthTokens`, stable
broker placeholders, an empty refresh field, and the current non-secret
`last_refresh` metadata required by Codex to emit its bearer header. Run
`codex login` on the host for missing, keyring-backed, account-changing, or
permanently rejected login state. `prepare`, `doctor`, `native`, `status`,
`stop`, and `reset` never renew.

Managed Codex 0.147.0 launches prepend `--disable apps`. This suppresses the
unsupported reserved `codex_apps` client without rewriting `config.toml` or
disabling user-configured MCP servers. It also makes connector-backed live
Excel control and Sites hosting unavailable inside Fortlet; local spreadsheet
files, local site development, and skill-only plugins remain available. Use
`fortlet native codex -- <arguments>` when Apps are deliberately required.

When host stdin or stdout is not a terminal, Fortlet uses a non-PTY streaming
exec session and immediately closes an empty stdin pipe. This explicit EOF is
the pinned MicroSandbox 0.6.8 workaround for its streaming null-stdin mode,
which does not close guest stdin. Stdout and stderr are forwarded and flushed
as events arrive, and the guest exit code becomes the Fortlet exit code. Managed
Codex has a ten-minute inactivity ceiling renewed by each stdout or stderr
event. On expiry Fortlet kills only the guest command, waits up to five seconds
for its terminal event, cancels the credential-renewal task, and leaves the
capsule running. Retry interactively from a supported terminal when more than
ten silent minutes are expected. Tact and native execution have no
Fortlet-owned inactivity ceiling.

The credential-free deterministic gate for this boundary is:

```bash
cargo test --bin fortlet runtime::tests::non_interactive
cargo test --bin fortlet runtime::tests::output_activity
cargo test --bin fortlet session::tests::attachment
```

## Project tool environments

`fortlet prepare <harness> [--project <path>] [--allow-broad-mount]` is an
optional synchronous warm-up for the immutable base, selected harness, and
optional project layers. It uses the same project selection as `run` and
prints `<harness><TAB>ready` only after every selected layer verifies. It does
not read provider credentials, create a reusable project capsule or persistent
harness home, project guest authentication, or attach a terminal. A cache hit
does not contact MicroSandbox or the network and does not rewrite a layer.

Skipping `prepare` is supported: explicit `run` and the transparent shims
perform the same layer preparation automatically on first launch. Preparation
does not validate login, runtime health, or terminal readiness and does not
change an existing capsule after project environment inputs change.

Only `.fortlet/environment.json` at the resolved project root opts a repository
into a project environment. It requires the fixed adjacent
`.fortlet/environment.sh`. The schema-1 manifest contains exactly `schema`,
`path`, and `environment`; path entries are relative to the immutable layer
and environment values are literal:

```json
{
  "schema": 1,
  "path": ["bin"],
  "environment": {
    "CARGO_HOME": "/home/agent/.cargo"
  }
}
```

On first use Fortlet snapshots and hashes both files, then executes the recipe
with `/bin/sh -eu` inside a dedicated provisioning capsule. The recipe sees
only `$FORTLET_OUTPUT` (`/out`), `$FORTLET_TARGET`, the image's base
environment, unauthenticated public network access, and an empty output mount.
It does not receive the manifest, live project, persistent home, host
environment, provider broker, SSH or signing material, publication authority,
or registry credentials.

Successful output is validated, content-digested, atomically published, and
re-verified before every read-only mount at `/opt/fortlet/project`. Manifest
paths precede the base and harness paths, while the harness executable remains
an absolute Fortlet-owned path. `HOME`, `PATH`, terminal, credential, Fortlet,
MicroSandbox, and harness-owned variables cannot be overridden. `BASH_ENV` is
also reserved: a read-only package-owned hook restores Fortlet's managed PATH
after Debian login profiles replace it, so agent commands launched through
non-interactive `bash -lc` keep project tools. Fortlet does not edit host or
persistent user startup files.

Changing either file changes capsule configuration. Stop and reset each
affected harness before launching the new identity:

```bash
fortlet stop codex
fortlet reset codex
fortlet run codex --
```

A failed update preserves previous layers and the existing capsule. Restore the
old manifest and recipe to select that layer again. There is no project
environment update, rollback, purge, private-input, service, or Nix activation
command in this slice. Recipes must pin and verify public downloads when
cross-machine reproducibility matters.

The deterministic gate does not start a VM or execute the checked-in recipe on
the host:

```bash
cargo test --bin fortlet project_environment
cargo test --bin fortlet environment
cargo test --test prepare
cargo test --test pre_runtime_failures invalid_project_environment
```

## Global capsule inventory

`fortlet list` reports every Fortlet-owned capsule across projects without
resolving the current project. Each row contains the safely rendered project
path, harness, and lowercase lifecycle state separated by tabs. Rows are sorted
by project then harness; an empty inventory prints `no capsules`.

Inventory consumes every MicroSandbox page and validates the stored Fortlet
schema, same-path project mount, project identity, harness, effective-user
capsule name, and handle/configuration agreement before printing any row.
Malformed or ambiguous managed state fails the complete command. Unrelated
MicroSandbox capsules are excluded by the managed label and never interpreted
as Fortlet state.

List is read-only: it does not require `HOME`, resolve or open a project, read
provider credentials, prepare layers, create Fortlet state, or mutate a
capsule. For a listed project that still exists, cleanup remains explicit:

```bash
fortlet status codex --project /path/to/project
fortlet stop codex --project /path/to/project
fortlet reset codex --project /path/to/project
```

The displayed path is escaped for one-line terminal safety and is not generated
shell text. Pass the actual project path as an ordinary argument. A moved or
deleted project remains visible but cannot yet be removed through a global
Fortlet selector; bulk cleanup, automatic expiry, and workload leases are not
part of this surface.

The deterministic inventory gate does not start a VM:

```bash
cargo test --bin fortlet management::tests::inventory
cargo test --test management_failures list_is_empty_without_home_project_credentials_or_fortlet_state
```

Experiment 0033 records the bounded real-runtime screen: immutable packaged
inventory reported one owned running Tact capsule from outside its project,
and the listed path selected packaged public status/stop/reset until final
absence, `no capsules`, and raw MicroSandbox inventory `[]`.

## Project capsule management

`fortlet status [harness]` reports `codex` and `tact` in registry order, or one
selected harness. Each output row is the harness, a tab, and `absent` or its
lowercase MicroSandbox lifecycle state. `fortlet stop <harness>` stops only the
owned capsule for the resolved project and reports `absent`, `already-stopped`,
or `stopped`. `fortlet reset <harness>` removes only an owned stopped or
crashed capsule's disposable runtime state and reports `absent` or `reset`.
Reset refuses every active or transitional state and directs the user to stop
the capsule first.

All three commands accept `--project <path>` and `--allow-broad-mount` with the
same project rules as `run`. Repeat an explicit `--project <path>` on stop and
reset so they select the same launch target. They do not read provider
credentials, provision tool layers, launch a harness, or expose internal
capsule names. Stop preserves the capsule record; reset preserves the project,
persistent harness state and placeholder-only credential projection, and every
immutable tool layer for a later run. Fortlet does not persist a token
fingerprint.

The deterministic management gate uses fake lifecycle observations and
isolated real-CLI failures without starting a VM:

```bash
cargo test --bin fortlet management
cargo test --test management_failures
```

Experiment 0010 records the bounded real-runtime screen: one uniquely scoped
owned Codex capsule was observed absent, running, stopped, and absent through
the immutable public CLI, with exact ownership verification before cleanup.
Experiment 0012 records the bounded real-runtime screen: the immutable public
CLI refused active reset, removed the same owned capsule after explicit stop,
preserved project and harness state, and reported subsequent absence
idempotently before exact cleanup.

## Deterministic launch-failure evidence

The pre-runtime integration gate invokes the compiled CLI with a cleared
subprocess environment and temporary home, project, state, data, and fake-auth
paths:

```bash
cargo test --test pre_runtime_failures
```

It proves actionable, fail-closed errors for unsupported harnesses, invalid
projects and project-environment manifests, credential loading and mount
boundaries, local capsule-state preparation, guest auth projection, and
incomplete base or harness layers. The environment-layer fixtures are rejected
before provisioning begins. This gate does not exercise live MicroSandbox
reconciliation or terminal attachment; those failure paths remain outstanding.

The credential unit gate covers source validation, expiry scheduling, bounded
refresh, atomic replacement, account binding, concurrent lock convergence,
single-request behavior, redacted failures, live-disposition enforcement, and
lease cancellation:

```bash
cargo test --bin fortlet auth::
cargo test --bin fortlet runtime::tests::credential_rotation
cargo test --bin fortlet session::tests::renewal_cancellation
cargo test --bin fortlet session::tests::attachment
```

The pinned stock-Codex compatibility fixture is separate because it requires a
native Codex 0.147.0 executable. It proves the fixed Apps disable wins over
configuration and command-line re-enables, preserves another configured MCP
server, and keeps the existing external-token behavior. It sends
placeholder-only authentication only to local fake model and refresh servers;
it makes no provider or paid request:

```bash
FORTLET_CODEX_COMPAT_BINARY=/path/to/codex-0.147.0 \
  cargo test --test codex_compatibility -- --ignored
```

## Optional transparent shims

The Nix package exposes `codex` and `tact` in a dedicated directory without
activating them implicitly:

```bash
nix build .#fortlet
env PATH="$PWD/result/libexec/fortlet/shims:$PATH" codex --version
env PATH="$PWD/result/libexec/fortlet/shims:$PATH" tact --version
```

Declarative users may prepend the same package path through Nix or Home
Manager. Fortlet does not edit shell startup files, and omitting the shim path
leaves the explicit CLI fully usable. `fortlet native <harness> -- <arguments>`
is the deliberate host escape hatch; isolated launch failures never select it.

Package install checks compare the bundled MicroSandbox runtime byte-for-byte
with its fixed-output release archive. This preserves the macOS Hypervisor
entitlement that Nix's generic stripping phase would otherwise remove.

## Interactive acceptance observer

The repository-local PTY observer is test tooling, not an installed Fortlet
command. Verify its bounded dimensions, resize, signal, timeout, status, and
owned-cleanup behavior before using it against a packaged shim:

```bash
cargo test --example pty_observer
cargo run --quiet --example pty_observer -- fixture
cargo run --quiet --example pty_observer -- fixture-exit
cargo run --quiet --example pty_observer -- fixture-typed-exit
cargo run --quiet --example pty_observer -- fixture-ctrl-c
```

The signal fixture proves foreground-group signal status. The exit fixture
proves exact `/exit\r` input, structural action ordering, and distinctive exit
code 23. The typed-exit fixture additionally proves separate `/`, `e`, `x`,
`i`, `t`, and Enter writes with a fixed 20-millisecond delay after each
character. The Ctrl-C fixture proves one `0x03` PTY write, a distinct structural
action, and code 23 preservation. The observer emits structured event lines and
a numeric summary; it never persists raw PTY screen content. Live `observe`,
`observe-exit`, `observe-typed-exit`, and `observe-ctrl-c` invocation parameters
and timeouts belong in a predeclared experiment record before dispatch.

## Pull request publication

Each authorized GOAL uses one descriptive Jujutsu bookmark and one draft pull
request targeting `main`. Local checkpoints may remain semantic and reviewable;
the operator squash-merges the completed goal into one public commit. FIP-0005
records one two-PR bootstrap exception, after which every GOAL uses one PR.

Accepting a GOAL grants standing authority for exactly that bookmark and pull
request. Create the draft PR when useful, then maintain it through in-scope
pushes, diagnosed CI fixes and replacement runs, description updates, and
readiness without another publication approval. Inspect the outgoing stack
throughout development:

```bash
jj status
jj log -r 'main..@'
jj diff --from main --to @ --stat
jj diff --from main --to @
```

Before readiness, run the complete standard verification set, record the local
`nix flake check` host and result in the PR, and sign the final publishable tip:

```bash
jj sign -r <publishable-tip>
```

Intermediate checkpoints may remain unsigned. The PR must state the exact
final tip, scope, experiments, verification, and limitations. Report that final
state to the operator, but do not request a separate publication approval. Mark
the PR ready only after the required `Rust verification` job passes. Diagnose
and report a failure before fixing it or replacing the run; an in-scope repair
does not require another approval or experiment. Two failures with one
unresolved cause stop under the charter.

The operator merges unless they explicitly authorize the agent to merge one
specific PR. Never push or force-push `main`, enable auto-merge, use mutating
Git commands, or extend standing authority to a changed GOAL, another PR,
different base or repository, settings, releases, tags, packages, secrets,
external spend, destructive actions, or unrelated resources.

After an operator squash merge, verify the PR targeted `main`, fetch `origin`,
and compare the reviewed branch-tip tree with fetched `main`:

```bash
gh pr view <number> --json baseRefName,mergeCommit,state
jj git fetch --remote origin
jj diff --from <reviewed-tip> --to main
```

The final diff must be empty. Squash commit identity is expected to differ from
the reviewed Jujutsu commits. Delete only the landed goal's local and remote
bookmarks after base, merge state, and tree equality are established.

## Experiments

Create a numbered record from `experiments/0000-template.md` before a benchmark,
user test, paid run, novel remote mutation, settings change, or other uncertain
result learned from reality rather than the test suite. Declare an effort
budget and rehearse the complete path without external effects before dispatch.
Close the record terminally and add its one-line outcome to
`experiments/README.md`.

Routine goal-branch publication, hosted CI, and diagnosed in-scope repair are
governed development, not experiments. The pull request, checks, operator
merge, and landing comparison are the evidence. A novel publication mechanism
or action outside standing goal authority remains experiment-track work.

## Generator-owned artifacts — never hand-edit

None currently.

## Environment

- Supported package systems: `aarch64-darwin` and `x86_64-linux`.
- The execution host must support MicroSandbox and its virtualization runtime.
- `nix develop` provides the Rust toolchain and macOS SDK environment.
- Provider credentials remain host-owned and are delegated through the
  MicroSandbox broker. Never place credentials in repository files, logs, test
  fixtures, experiment records, or prompts.
- Updating MicroSandbox requires updating `Cargo.lock`, fixed-output hashes in
  `package.nix`, and `nix/microsandbox-reproducible-build.patch` as needed.
