# Fortlet

Fortlet runs coding-agent CLIs inside project-scoped MicroSandbox capsules
while preserving their ordinary terminal experience. The host workspace stays
live, provider credentials are brokered rather than mounted, and failure to
establish the isolation boundary prevents the agent from starting.

Fortlet is an early Rust prototype. It currently supports explicit Codex and
Tact launches; transparent command shims and the broader management surface are
the next product milestone.

## Quick start

The reproducible Nix package includes the matching MicroSandbox host runtime:

```sh
nix run . -- doctor
nix run . -- run codex --
nix run . -- run tact --
```

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

Normal use should remain the native harness commands:

```text
codex
claude
tact
```

Fortlet will provide transparent fail-closed shims, one capsule per project
and harness, declarative tool environments, concurrent same-harness sessions,
and an explicit host-side publication boundary. Local execution comes first;
the architecture leaves room for a self-hosted remote execution host without
pretending that transport already exists.

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
