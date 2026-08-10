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
than the Rust-only gates. Native Linux verification is still outstanding.

## Running the system

```bash
nix run . -- doctor
nix run . -- run codex --
nix run . -- run tact --
```

For a development binary inside `nix develop`:

```bash
cargo run -- doctor
cargo run -- run codex --
```

`doctor` reads authentication metadata but never prints token values. Real
launches require a healthy MicroSandbox host and a valid ChatGPT credential.

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
