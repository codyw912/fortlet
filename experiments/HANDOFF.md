# Session Handoff — First daily Fortlet session

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001, FIP-0002, FIP-0005, FIP-0006, accepted FIP-0007, and accepted FIP-0008
in full before implementation. Experiments 0001 through 0021 are terminally
closed, Experiment 0022 is terminally rejected, and no experiment is active.

## Verified landed baseline

PR #4 landed opt-in project environments on protected `main` as squash commit
`ad199140ac8acf20cc73b1b5ea5ad6b2f1ce2d05`. Its tree is byte-identical to
reviewed signed tip `3bda446d7d51eeeddcbfde781fbdde62d7d11994`.
Hosted Rust verification passed, the declared branch bookmarks were removed,
and the working copy began this mission as an empty change directly on fetched
`main`.

The FIP-0008 implementation checkpoint `e7b1d5a26693` passed 69 unit tests, 28
non-ignored integration tests, formatting, strict all-target/all-feature
Clippy, conformance, `nix flake check`, and `nix build .#fortlet` on
`aarch64-darwin`. The separate stock Codex 0.147.0 compatibility fixture also
passed against local fake model and OAuth-refresh servers. Nix emitted only the
known missing app-metadata warning and omitted incompatible `x86_64-linux`.

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

The accepted FIP-0008 implementation now loads only file-backed ChatGPT auth,
renews inside a one-hour safety window under a source-scoped lock, atomically
updates the host file, projects Codex `chatgptAuthTokens` with an empty guest
refresh field, and rotates both MicroSandbox broker secrets live. Tact keeps
its prior managed projection. Token-derived persistent fingerprints were
removed.

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

## Terminal renewable-credential experiment

Experiment 0022 pinned checkpoint
`e7b1d5a26693`, package
`/nix/store/kym4by5g33jw1z3wdg9p70manajzmnx0-fortlet-0.1.0`, one non-prompt
version launch, one shimmed interactive session, one exact prompt, at most one
host refresh, one owned capsule, one focused test, zero remote mutation, no
retry, and 15 elapsed minutes. The clean pre-dispatch baseline matched exactly.

The sole prompt caused Codex to fall back from WebSockets after a 401; its
HTTPS request also returned 401 with `Your ChatGPT login did not make it to
this service.` No model response, repository work, or test occurred. Unlike
Experiment 0021, Codex did not attempt guest-owned refresh, but the brokered
authentication still was not accepted. The observation does not yet separate
MicroSandbox substitution from host-token provider validity or request/account
shape. Public stop/reset restored absence, the working copy stayed empty, the
MicroSandbox inventory is empty, and immutable layers plus persistent Codex
and Cargo state remain.

## What to do next

1. Do not retry the terminal treatment. Obtain deterministic evidence that
   separates broker substitution on Codex's WebSocket and HTTPS requests from
   host-token validity and request/account shape before proposing a fix.
2. Preserve the accepted FIP-0007 implementation; its conformance remains
   partial only because the daily edit/test unit could not begin.
3. Require that no refresh token or durable provider credential becomes guest
   readable and that login cannot silently persist a real guest credential.
   Treat MCP biscuit authorization separately.
4. Record the sole session, inspect the exact diff, and close Experiment 0022
   terminally. Publication remains one later FIP-0005 packet after the daily
   work unit succeeds.

Do not resume Experiments 0021 or 0022, add a refresh token to the guest
projection, inspect credential values, remove verified layers, add global
inventory/pruning, or begin standalone distribution.
