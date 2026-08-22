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
nix run . -- list
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

For a bounded opt-in command where `codex` or `tact` uses Fortlet, execute it
through the named shell explicitly:

```sh
nix develop .#agents -c codex
nix develop .#agents -c tact
```

The shell is minimal: it adds `fortlet` and the package-owned shim directory,
but no project toolchain. Plain `nix develop` remains Fortlet's Rust development
shell and does not activate the shims.

Do not wrap every daily command in `nix develop path:/path/to/fortlet#agents`.
An unlocked local path input is snapshotted into the Nix store on each fresh
evaluation and is too slow for that loop. For daily project use, compose the
two outputs into devenv as shown below so direnv evaluates once and reuses its
cached environment. Explicit `fortlet run codex --` and
`fortlet run tact --` remain available without shim activation.

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

When stdin or stdout is not a terminal, Fortlet closes harness stdin, streams
stdout and stderr without allocating a PTY, and returns the exact guest exit
status.
Managed Codex commands have a ten-minute inactivity ceiling that renews on
output; expiry kills only that command and preserves the reusable capsule.
Use interactive managed Codex for work that may legitimately remain silent
longer. Tact, interactive sessions, and `fortlet native` have no such ceiling.

`fortlet prepare <harness>` is an optional eager warm-up. It loads the packaged
Nix-built OCI image locally, seeds one labeled ext4 Nix-store volume per
project, imports only the selected harness closure, and prepares optional
project tools without reading provider
credentials, creating a reusable project capsule or persistent harness home,
or attaching a terminal. If it is omitted, `fortlet run` and the optional
shims perform the same preparation automatically on first launch. A successful
preparation prints `<harness><TAB>ready`; a verified cache hit performs no
provisioning, capsule launch, store mutation, registry access, or network
contact. Live harness capsules mount the project store read-only and use the
same image regardless of harness; Node and npm are not part of the runtime
contract. The immutable Nix closure supplies the initial CA roots, while the
image materializes the active bundle at `/etc/ssl/certs/ca-certificates.crt`
so MicroSandbox can extend it only in each capsule's disposable root.

## Project tool environments

A repository opts into project tools with two fixed files at its resolved root:
`.fortlet/environment.json` and `.fortlet/environment.sh`. The manifest
declares layer-relative `PATH` entries and literal environment variables. The
adjacent POSIX recipe installs tools beneath `$FORTLET_OUTPUT`.

Fortlet reads and hashes both exact files on the host but never executes the
recipe there. On first use, it runs the snapshotted recipe in a dedicated
MicroSandbox capsule with public network access, no live project mount, no
persistent home, and no provider, SSH, signing, publication, or registry
credentials. The validated output is content-digested, sealed against writes,
fully reverified, and published atomically. Cache hits perform bounded
verification of only the real canonical root and its versioned marker, then
mount the layer read-only at `/opt/fortlet/project`. Provisioning uses a strict
mirrored output bind so a package-owned guest finalizer can preserve executable
intent. After that capsule is retired, Fortlet validates the resulting
`0755`/`0644` host tree and seals it to canonical `0555`/`0444` modes. Runtime
consumption uses an explicit relaxed, private, read-only, `nosuid`, `nodev`
mount; this remains usable when MicroSandbox stat-override metadata is absent.
A repository without the manifest uses only the common runtime and selected
harness closures.

Changing either file selects a new immutable layer. If an old capsule exists,
run `fortlet stop <harness>` and `fortlet reset <harness>` before launching
the new environment. A failed build leaves the prior layer and capsule intact;
the old environment can be selected again by restoring its two files. Public
downloads can drift unless the recipe pins and verifies them.
The internal publication contract is versioned independently of schema 1, so
layers made under an older permission contract are not cache hits and malformed
legacy `0500`/`0400` trees fail closed.
The published cache and invoking host user are one trusted principal. Complete
tree validation protects that principal from provisioning-guest output, and
the runtime mount prevents guest writes; bounded reuse does not claim to detect
arbitrary cache changes made directly by that same host principal.

This repository's recipe pins Rust, Cargo, Zig, Jujutsu, and the Linux
development library needed to build Fortlet. It downloads exact public
artifacts and does not invoke apt, dpkg, npm, or another package resolver. It
serves as the first project-environment fixture; Nix remains the host
development and package-reproduction system.
Fortlet also restores the managed tool PATH for non-interactive Bash login
shells through a package-owned process environment hook. It does not write a
user `.profile`, `.bashrc`, fish configuration, or other startup file.

## Product direction

Users may explicitly activate Fortlet's package-owned shim directory so normal
use remains the native harness commands:

```text
codex
tact
```

The shims are optional; `fortlet run` remains fully usable without them.
Fortlet provides one capsule per project and harness and can list every owned
capsule across projects. Its repository uses an explicit host-side publication
boundary; an automated publication command, private project overlays, explicit
workload leases, and standalone distribution remain future work. Local
execution comes first; remote execution requires its own architecture.

Projects that already use devenv can opt in without a Fortlet shell hook. Add
Fortlet as a pinned flake input (following the project's `nixpkgs` when
appropriate), then include both outputs in `devenv.nix`:

```nix
# flake.nix
fortlet = {
  url = "github:codyw912/fortlet";
  inputs.nixpkgs.follows = "nixpkgs";
};

# devenv.nix
packages = [
  inputs.fortlet.packages.${pkgs.system}.fortlet
  inputs.fortlet.packages.${pkgs.system}.shim-activation
];
```

The separate activation output contains no second launcher implementation: its
`bin` resolves to the immutable shim directory in the Fortlet package. Remove
that one output from the project environment to return `codex` and `tact` to
their prior resolution. On a normal fresh project entry, direnv applies the
cached environment after fish startup; manual PATH mutation in a long-lived
shell can desynchronize shell-manager state and should be cleared with a fresh
terminal before evaluating activation behavior.

The first managed command for an absent project capsule must start that capsule
and can take several seconds. Experiment 0042 observed approximately nine
seconds for `codex --version` from absence on Apple silicon. That experiment did
not measure attachment to an already-running capsule.

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
