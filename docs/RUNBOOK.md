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
nix run . -- status
nix run . -- stop codex
nix run . -- reset codex
```

For a development binary inside `nix develop`:

```bash
cargo run -- doctor
cargo run -- prepare codex
cargo run -- run codex --
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
launches require a healthy MicroSandbox host and a valid ChatGPT credential.

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
MicroSandbox, and harness-owned variables cannot be overridden.

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
persistent harness state, credential projection and fingerprint, and immutable
tool layers for a later run.

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

After all locally knowable work is complete, inspect and sign the outgoing
stack, then present one exact publication packet containing its revisions and
diff, bookmark and `origin` destination, full draft-PR metadata, expected
initial hosted check, readiness criteria, verification state, known failures,
and any exact landed bookmarks proposed for cleanup:

```bash
jj status
jj log -r 'main..@'
jj diff -r 'main..@' --stat
jj diff -r 'main..@'
```

One explicit approval authorizes only the packet's single signed-bookmark push,
one draft PR, observation of its one initial hosted run, declared evidence-only
body refresh, readiness after that run succeeds, and cleanup of named landed
bookmarks after operator merge plus exact tree equality. It does not authorize
changed code or scope, another push or PR, a retry, substantive metadata change,
repository settings, unexpected remote changes, or unrelated resources. Any
such change or any failed step stops for a new exact review and approval. The
operator merges unless they explicitly authorize the agent to merge one
specific PR. Never push or force-push `main`, enable auto-merge, or use mutating
Git commands.

Before publication, run the complete standard verification set and record the
local `nix flake check` host and result in the PR. Mark the PR ready only after
the expected `Rust verification` job passes. The GOAL may remain conditionally
active only for hosted verification, operator merge, and read-only landing
observation. A failure stops publication rather than being bypassed or silently
retried.

After an operator squash merge, verify the PR targeted `main`, fetch `origin`,
and compare the reviewed branch-tip tree with fetched `main`:

```bash
gh pr view <number> --json baseRefName,mergeCommit,state
jj git fetch --remote origin
jj diff --from <reviewed-tip> --to main
```

The final diff must be empty. Squash commit identity is expected to differ from
the reviewed Jujutsu commits. Delete only landed bookmarks named in the
approved publication packet after base, merge state, and tree equality are
established; otherwise defer cleanup.

## Experiments

Create a numbered record from `experiments/0000-template.md` before a benchmark,
user test, paid run, novel remote mutation, settings change, retry, or other
uncertain result learned from reality rather than the test suite. Declare an
effort budget and rehearse the complete path without external effects before
dispatch. Close the record terminally and add its one-line outcome to
`experiments/README.md`.

Routine publication through an approved FIP-0005 packet is governed rollout,
not a new experiment. Its exact packet, pull request, hosted check, operator
merge, and landing comparison are the evidence. A failure stops the packet;
any novel recovery or retry returns to the experiment rule above.

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
