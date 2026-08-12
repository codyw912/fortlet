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
nix run . -- run codex --
nix run . -- run tact --
nix run . -- status
nix run . -- stop codex
```

For a development binary inside `nix develop`:

```bash
cargo run -- doctor
cargo run -- run codex --
cargo run -- status codex
cargo run -- stop codex
```

Project resolution prefers the nearest Jujutsu root, then Git root, then a
recognized devenv or flake root. It canonicalizes the selected root and
preserves a launch from a subdirectory. Home-directory and filesystem-root
projects use Fortlet's persistent scratch workspace unless the user passes
`--allow-broad-mount`; `--project "$HOME"` and `--project /` do not imply that
override.

`doctor` reads authentication metadata but never prints token values. Real
launches require a healthy MicroSandbox host and a valid ChatGPT credential.

## Project capsule management

`fortlet status [harness]` reports `codex` and `tact` in registry order, or one
selected harness. Each output row is the harness, a tab, and `absent` or its
lowercase MicroSandbox lifecycle state. `fortlet stop <harness>` stops only the
owned capsule for the resolved project and reports `absent`, `already-stopped`,
or `stopped`.

Both commands accept `--project <path>` and `--allow-broad-mount` with the same
project rules as `run`. They do not read provider credentials, provision tool
layers, launch a harness, or expose internal capsule names. Stop preserves the
capsule record and persistent harness state for a later run.

The deterministic management gate uses fake lifecycle observations and
isolated real-CLI failures without starting a VM:

```bash
cargo test --bin fortlet management
cargo test --test management_failures
```

## Deterministic launch-failure evidence

The pre-runtime integration gate invokes the compiled CLI with a cleared
subprocess environment and temporary home, project, state, data, and fake-auth
paths:

```bash
cargo test --test pre_runtime_failures
```

It proves actionable, fail-closed errors for unsupported harnesses, invalid
projects, credential loading and mount boundaries, local capsule-state
preparation, guest auth projection, and incomplete base or harness layers. The
environment-layer fixtures are rejected before provisioning begins. This gate
does not exercise live MicroSandbox reconciliation or terminal attachment;
those failure paths remain outstanding.

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

## Experiments

Create a numbered record from `experiments/0000-template.md` before a benchmark,
user test, paid run, remote mutation, or other result learned from reality
rather than the test suite. Declare an effort budget and rehearse the complete
path without external effects before dispatch. Close the record terminally and
add its one-line outcome to `experiments/README.md`.

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
