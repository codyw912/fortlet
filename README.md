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
nix run . -- prepare codex
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
read from a file-backed host ChatGPT login; keyring-backed login, API keys, and
other authentication modes are not yet supported. Fortlet renews a near-expiry
Codex token with the host refresh token, atomically updates the host login, and
rotates the access token through MicroSandbox's credential broker. The guest
sees only stable placeholders and an empty refresh field. The credential file
itself must remain outside all guest mounts; run `codex login` on the host when
Fortlet reports a permanent refresh failure.

Fortlet-managed Codex 0.147.0 launches disable the built-in Apps connector
client because its separate authorization contract is not available inside the
capsule. This does not disable user-configured MCP servers or local plugin
skills. Connector-backed live Excel control and Sites hosting are unavailable
through Fortlet; use native Codex explicitly if either capability is needed.

`fortlet prepare <harness>` is an optional eager warm-up. It ensures the base,
selected harness, and optional project-tool layers without reading provider
credentials, creating a reusable project capsule or persistent harness home,
or attaching a terminal. If it is omitted, `fortlet run` and the optional
shims perform the same preparation automatically on first launch. A successful
preparation prints `<harness><TAB>ready`; a verified cache hit performs no
provisioning or network contact.

## Project tool environments

A repository opts into project tools with two fixed files at its resolved root:
`.fortlet/environment.json` and `.fortlet/environment.sh`. The manifest
declares layer-relative `PATH` entries and literal environment variables. The
adjacent POSIX recipe installs tools beneath `$FORTLET_OUTPUT`.

Fortlet reads and hashes both exact files on the host but never executes the
recipe there. On first use, it runs the snapshotted recipe in a dedicated
MicroSandbox capsule with public network access, no live project mount, no
persistent home, and no provider, SSH, signing, publication, or registry
credentials. The validated output is content-digested, published atomically,
and mounted read-only at `/opt/fortlet/project`. A repository without the
manifest keeps the existing base-plus-harness environment.

Changing either file selects a new immutable layer. If an old capsule exists,
run `fortlet stop <harness>` and `fortlet reset <harness>` before launching
the new environment. A failed build leaves the prior layer and capsule intact;
the old environment can be selected again by restoring its two files. Public
downloads can drift unless the recipe pins and verifies them.

This repository's recipe pins Rust, Cargo, Jujutsu, and the Linux development
library needed to build Fortlet. It serves as the first project-environment
fixture; Nix remains the host development and package-reproduction system.

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
private project overlays, explicit workload leases, and standalone distribution
remain future work. Local execution comes first; remote execution requires its
own architecture.

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
