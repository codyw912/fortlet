# Session Handoff — Publish opt-in project environments

Audience: a fresh agent session. `GOAL.md` is normative and locally complete;
only FIP-0005 publication, operator merge, and landing verification remain.
Read FIP-0001, FIP-0005, and FIP-0006 in full before continuing. Experiments
0001 through 0020 are terminally closed; no experiment is active.

## Verified product state

Fortlet now supports opt-in project environments in addition to explicit
`run`, optional shims, `native`, `doctor`, project-scoped `status`, bounded
`stop`, and terminal `reset`. A project without `.fortlet/environment.json`
retains the existing immutable base-plus-harness behavior.

An opted-in project supplies the fixed manifest and adjacent POSIX recipe.
Fortlet snapshots and hashes both files, runs the recipe only in a
credential-free provisioning capsule with an empty `/out`, validates and
content-digests the result, publishes it atomically, and mounts it read-only at
`/opt/fortlet/project`. Project paths and literal variables reach both Codex and
Tact; protected variables remain adapter-owned. Environment identity participates
in launch configuration, while public stop/reset retain stable management
identity for recovery.

## Acceptance evidence

Experiment 0020 accepted the real immutable public path on `aarch64-darwin`
with MicroSandbox 0.6.8 and Codex 0.147.0. Package
`/nix/store/ki9llb8dy1zrwxbypmpk4lbzy3p7ax6n-fortlet-0.1.0` provisioned
environment identity `a0e90418dca950e944e6243c446e93eb977061cd73333da4996a8d560d021252`.
The owned Codex capsule mounted that sole layer read-only and exposed:

- Cargo 1.97.1 and rustc 1.97.1 from `/opt/fortlet/project/bin`;
- Jujutsu 0.43.0 from the same layer;
- the declared persistent Cargo home and Linux target directory.

The focused in-capsule filter passed 10 tests. Public stop and reset returned
the project to absent status; no capsule remains. The project, persistent Codex
state, base layer, harness layer, Cargo cache, and verified project layer are
preserved. FIP-0006 conformance is `conformant` with no gaps.

## Paid-for implementation lessons

1. Nix's filtered package source must include every conformance-referenced file,
   including `.fortlet`, `package.nix`, `README.md`, and `OVERVIEW.md`.
2. Large recipe downloads and extraction belong under the host-backed output
   staging directory, not the provisioning capsule root disk.
3. Debian `libcap-ng-dev` supplies absolute `/lib/<triplet>/...` development
   links on both arm64 and amd64. The pinned recipe checks their exact targets
   and rewrites only those links layer-relatively. Do not weaken Fortlet's
   general rejection of absolute or escaping links.
4. The first focused Linux Cargo build downloads public crates and populates
   `/home/agent/.cargo/fortlet-target`; subsequent capsules reuse that explicit
   persistent cache.
5. Experiments 0017 through 0019 retain the honest baseline, capacity, and
   absolute-link failures that led to accepted experiment 0020. Never rewrite
   or resume those terminal units.

## Publication state

The local stack starts at fetched public `main`
`633ea638013ba675e82ea7b06b1d3a287d7aa003` and contains accepted FIP-0006,
implementation, tests, documentation, four terminal experiment records, and
the local-completion records. Before requesting the one publication approval:

1. Run the complete verification set after these final record changes.
2. Inspect and shape `main..@` into reviewable semantic checkpoints; preserve
   the honest rejected experiments and accepted experiment 0020.
3. Review the exact diff, sign every outgoing revision, choose one descriptive
   bookmark, and prepare the complete FIP-0005 packet: bookmark, origin target,
   draft PR title/body, expected `Rust verification`, readiness criteria, known
   warnings/failures, and named post-merge bookmark cleanup.
4. STOP for one operator approval. That approval permits one bookmark push, one
   draft PR, initial hosted-run observation, evidence-only body refresh,
   readiness after success, and named bookmark cleanup after operator merge and
   exact tree-equality verification. The operator merges manually.

Do not add installation, services, private overlays, Nix activation, releases,
packages, registry authentication, remote execution, or another product goal
to this stack. Do not push, open a PR, retry a failed remote transaction, change
repository settings, or merge without the exact authority FIP-0005 requires.
