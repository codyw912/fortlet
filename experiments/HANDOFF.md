# Session Handoff — First daily Fortlet session

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001, FIP-0002, FIP-0005, FIP-0006, and accepted FIP-0007 in full before
implementation. Experiments 0001 through 0020 are terminally closed; no
experiment is active.

## Verified landed baseline

PR #4 landed opt-in project environments on protected `main` as squash commit
`ad199140ac8acf20cc73b1b5ea5ad6b2f1ce2d05`. Its tree is byte-identical to
reviewed signed tip `3bda446d7d51eeeddcbfde781fbdde62d7d11994`.
Hosted Rust verification passed, the declared branch bookmarks were removed,
and the working copy began this mission as an empty change directly on fetched
`main`.

The last exact outgoing verification passed 53 unit tests, 22 integration
tests, formatting, strict all-target/all-feature Clippy, conformance,
`nix flake check`, and `nix build .#fortlet` on `aarch64-darwin`. Nix emitted
only the known missing app-metadata warning and omitted incompatible
`x86_64-linux` on that host.

## Current product surface

Fortlet provides explicit `run`, optional transparent Codex and Tact shims,
`native`, `doctor`, project-scoped `status`, bounded `stop`, terminal `reset`,
and opt-in immutable project-tool layers. Authentication remains host-owned and
brokered. Project recipes execute only in credential-free provisioning
capsules, and successful layers are content-verified and mounted read-only.

Experiment 0020 proved the real Fortlet repository path: Cargo and rustc 1.97.1
plus Jujutsu 0.43.0 reached Codex from the project layer, 10 focused Linux tests
passed, and public stop/reset preserved persistent state plus every immutable
layer. FIP-0006 is conformant.

## Recorded daily-use friction

First use can spend minutes provisioning immutable inputs while the user is
trying to launch an agent. Today that work is reachable only from the launch
pipeline after credential validation and capsule-specific preparation. The
failed acceptance units also showed that provisioning errors are easier to
understand as environment preparation than as interactive agent-startup
failures.

The existing `EnvironmentStore::ensure` seam already prepares the exact base,
selected harness, and optional project layers. It does not intrinsically need a
provider credential or reusable project+harness capsule. This is the narrow
opportunity; do not generalize it into tasks, background work, inventory, or a
new runtime abstraction.

## Accepted contract

FIP-0007 adds:

```text
fortlet prepare <harness> [--project <path>] [--allow-broad-mount]
```

The command uses launch-equivalent harness and project selection, validates the
optional project environment, ensures immutable layers synchronously, and
prints `<harness><TAB>ready`. It never reads provider credentials, creates a
guest auth projection, prepares persistent harness state, constructs the
managed reusable capsule, or attaches a terminal. Cache hits verify required
markers and content without provisioning contact.

## What to do next

1. Implement only the accepted FIP-0007 contract. Keep conformance
   `unimplemented` until code plus tests establish observable behavior.
2. Rehearse and predeclare one bounded daily-session experiment only after the
   complete deterministic and package gates pass. Any model prompt or
   operator-run interactive observation needs the exact experiment budget
   accepted before dispatch.
3. Publish through one FIP-0005 packet; the operator merges manually.

Do not start another experiment, execute a model prompt, remove the verified
project layer, add global inventory/pruning, implement background work, or begin
standalone distribution before FIP-0007 and the new GOAL establish authority.
