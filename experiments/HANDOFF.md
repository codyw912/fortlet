# Session Handoff — First daily Fortlet session

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001, FIP-0002, FIP-0005, FIP-0006, accepted FIP-0007, and accepted FIP-0008
in full before implementation. Experiments 0001 through 0021 are terminally
closed; no experiment is active.

## Verified landed baseline

PR #4 landed opt-in project environments on protected `main` as squash commit
`ad199140ac8acf20cc73b1b5ea5ad6b2f1ce2d05`. Its tree is byte-identical to
reviewed signed tip `3bda446d7d51eeeddcbfde781fbdde62d7d11994`.
Hosted Rust verification passed, the declared branch bookmarks were removed,
and the working copy began this mission as an empty change directly on fetched
`main`.

The current FIP-0007 implementation gate passed 57 unit tests, 28 integration
tests, formatting, strict all-target/all-feature Clippy, conformance,
`nix flake check`, and `nix build .#fortlet` on `aarch64-darwin`. Nix emitted
only the known missing app-metadata warning and omitted incompatible
`x86_64-linux` on that host.

## Current product surface

Fortlet provides explicit `run`, optional transparent Codex and Tact shims,
`native`, `doctor`, project-scoped `status`, bounded `stop`, terminal `reset`,
explicit credential-free `prepare`, and opt-in immutable project-tool layers.
Project recipes execute only in credential-free provisioning
capsules, and successful layers are content-verified and mounted read-only.

Experiment 0020 proved the real Fortlet repository path: Cargo and rustc 1.97.1
plus Jujutsu 0.43.0 reached Codex from the project layer, 10 focused Linux tests
passed, and public stop/reset preserved persistent state plus every immutable
layer. FIP-0006 is conformant.

## Recorded daily-use blocker

Experiment 0021 proved the immutable packaged `prepare` cache hit in 2.46
seconds with exact `codex<TAB>ready`, no credential read, no reusable capsule,
and no provisioning capsule. A non-prompt Codex version launch then created one
owned capsule, and the packaged shim attached to its interactive UI.

The first prompt failed before model work because Codex 0.147.0 attempted to
refresh its access token. Fortlet's persistent projection deliberately contains
a fake refresh token and relies on static request-time broker substitution, so
it cannot satisfy this lifecycle. The `codex_apps` MCP also returned HTTP 451
`no_biscuit_no_service`; treat that as a separate missing credential class.
Public stop/reset restored absence, all immutable layers and Cargo cache remain,
the repository was unchanged, and the operator removed the possible guest auth
residue without exposing it.

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

FIP-0008 adds a Codex-specific, host-owned renewable credential lease. It
keeps the refresh token outside the guest, uses Codex's external-token
projection, and rotates only the access token and account identifier through
MicroSandbox's existing live broker. Infisical Agent Proxy was reviewed before
acceptance: it validates the broker pattern and renews supported dynamic-secret
leases, but it cannot renew an existing Codex ChatGPT OAuth login and would
make an Infisical control plane plus an Enterprise feature required.

## What to do next

1. Implement accepted FIP-0008. It selects a Codex-specific host renewal lease
   over MicroSandbox's existing live secret rotation and keeps Infisical Agent
   Proxy and iron-proxy deferred.
2. Preserve the accepted FIP-0007 implementation; its conformance remains
   partial only because the daily edit/test unit could not begin.
3. Require that no refresh token or durable provider credential becomes guest
   readable and that login cannot silently persist a real guest credential.
   Treat MCP biscuit authorization separately.
4. Implement deterministic source locking, atomic host refresh, external-token
   projection compatibility, live broker rotation, and a process-bounded
   renewal lease before any new live prompt.
5. Predeclare a new experiment rather than retrying 0021. Publication remains
   one later FIP-0005 packet after the daily work unit succeeds.

Do not retry the prompt, add a refresh token to the guest projection, start a
new experiment, remove verified layers, add global inventory/pruning, or begin
standalone distribution before FIP-0008 is implemented and verified.
