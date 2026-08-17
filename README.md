# Fortlet

Fortlet runs coding-agent CLIs inside project-scoped MicroSandbox capsules
while preserving their ordinary terminal experience. The host workspace stays
live, provider credentials are brokered rather than mounted, and failure to
establish the isolation boundary prevents the agent from starting.

Fortlet is an early Rust prototype. It supports explicit Codex and Tact
launches, optional transparent command shims, and project-scoped capsule
status, stop, and reset commands.

## Quick start

The reproducible Nix package includes the matching MicroSandbox host runtime:

```sh
nix run . -- doctor
nix run . -- run codex --
nix run . -- run tact --
nix run . -- status
nix run . -- stop codex
nix run . -- reset codex
```

Fortlet resolves the nearest Jujutsu, Git, or recognized development-
environment root and preserves a launch from its subdirectories. A selected
home directory or filesystem root uses Fortlet's persistent scratch workspace
by default, including when selected with `--project`. Exposing either broad
root requires the separate, deliberate `--allow-broad-mount` flag.

Nix is the current reproducible alpha installation path, not an intended
general-user requirement. Fortlet's release distribution should also provide
standalone platform artifacts; Nix remains an optional first-class path for
users who prefer it.

For local Rust development:

```sh
nix develop
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -- doctor
```

MicroSandbox must be usable on the execution host. Codex authentication is
read from the host and exposed through MicroSandbox's credential broker; the
credential file itself must remain outside all guest mounts.

## Product direction

Users may explicitly activate Fortlet's package-owned shim directory so normal
use remains the native harness commands:

```text
codex
tact
```

The shims are optional; `fortlet run` remains fully usable without them.
Fortlet provides one capsule per project and harness. Its repository uses an
explicit host-side publication boundary; an automated publication command,
declarative project tool environments, explicit workload leases, and
standalone distribution remain future work. Local execution comes first;
remote execution requires its own architecture.

See [OVERVIEW.md](OVERVIEW.md) for durable product scope and
[GOAL.md](GOAL.md) for the current mission.

## Workflow

This repository uses the [SPAWN.md](https://github.com/0xprincess/SPAWN.md)
workflow. Fresh agent sessions start with `GOAL.md`, then
`experiments/HANDOFF.md`, then `governance/CHARTER.md`. Significant design
changes require an accepted Fortlet Improvement Proposal (FIP) before
implementation.

## License

Fortlet is licensed under either Apache-2.0 or MIT, at your option.
