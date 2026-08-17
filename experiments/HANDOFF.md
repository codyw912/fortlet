# Session Handoff — Opt-in project environments

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001, FIP-0005, and FIP-0006 in full before continuing. Experiments 0001
through 0016 are terminally closed; no experiment is active.

## Verified baseline

1. Public repository `codyw912/fortlet` has protected default branch `main`.
   GitHub permits squash merging only, requires pull requests and strict
   `Rust verification`, enforces rules for administrators, requires linear
   history, and blocks force-push plus deletion.
2. The publication-policy correction landed through PR #3 as signed squash
   commit `633ea638013ba675e82ea7b06b1d3a287d7aa003`. Its tree is byte-identical
   to reviewed tip `063e82455b8a9046afa6a8b2a236a6856235e8ab`; the hosted run
   passed, and the declared landed bookmarks were removed locally and remotely.
3. The working copy began this mission as an empty revision on that fetched
   `main`. The completed policy goal is archived under `governance/goals/`.
4. The last complete local gate recorded 39 unit tests, 21 integration tests,
   formatting, strict all-target/all-feature Clippy, conformance, and
   `nix flake check` on `aarch64-darwin`. Nix emitted only the known missing app
   metadata warning and omitted incompatible `x86_64-linux`.

## Product state and blocker

Daily-use surfaces are `doctor`, explicit `run`, optional package-owned
`codex` and `tact` shims, explicit `native`, project-scoped `status`, bounded
`stop`, and terminal `reset`. Fortlet creates immutable base and harness layers
and mounts them into one reusable capsule per project+harness with a persistent
harness home and brokered credentials.

No project tool environment is activated. The fixed `node:24-bookworm` capsule
and harness layer do not guarantee a repository's compiler, test runner, or VCS
client. In Fortlet's own repository an isolated agent therefore cannot assume
that `cargo`, `rustc`, or `jj` exists. This is the primary current obstacle to
ordinary project work.

## Accepted design

FIP-0006 is Accepted and conformance is `unimplemented`. It defines one
opt-in root manifest, `.fortlet/environment.json`, plus a fixed adjacent POSIX
recipe, `.fortlet/environment.sh`. The manifest contains only layer-relative
path entries and literal variables. The host snapshots and hashes both files
but does not execute them. The recipe snapshot runs only in a credential-free
provisioning capsule with an empty `/out`; successful output is validated,
content-digested, published atomically, and mounted read-only at
`/opt/fortlet/project`.

The proposal prefers this slice over a pinned OCI project image because an OCI
contract would require every project to build and publish an image and may add
registry lifecycle or login. It prefers it over Nix/devenv activation because
FIP-0001 keeps Nix optional for ordinary users and host-platform closures do
not directly supply a Linux guest. Public provisioning network is allowed
without authentication; recipes remain responsible for pinning downloads, so
the FIP deliberately does not claim cross-machine bit reproducibility.

## What to do next

Implement only the GOAL's manifest, isolated immutable layer, activation,
reconciliation, Fortlet toolchain fixture, deterministic evidence, and one
bounded experiment. Do not expand into services, private overlays,
general Nix integration, installation, logs, restart, leases, tool-update
commands, or extra harnesses. Publication uses one exact FIP-0005 packet; the
operator remains the sole merge authority.
