# Session Handoff — Workspace-backed Codex test execution complete

Audience: a fresh agent session. `GOAL.md` is normative and complete pending
operator merge. Verify `main`, the Jujutsu stack, conformance, and the complete
`docs/RUNBOOK.md` gate before relying on this summary. Read every FIP named by
a successor GOAL in full before implementation or experiment dispatch.

## Baseline and publication

Merged `main` is `be43d5909b6a65e9ca6984859b85b6d27c2601dc` (PR #11), whose
tree was proved byte-identical to reviewed signed tip
`d8fa6c027d10f5622fd7e2eb6282b3a1bd8a424b`. This GOAL uses bookmark
`codex-firewall-recovery` and PR #12; the historical bookmark name does not
represent a current Fortlet limitation. The operator remains the sole merge
authority.

The complete standard gate passes on aarch64-darwin: all 88 unit tests and
every enabled integration test, formatting, strict all-target/all-feature
Clippy, conformance, and `nix flake check`. Nix emits only the expected
incompatible `x86_64-linux` omission warning. Hosted Rust verification also
passes. FIP-0001 and FIP-0002 remain partial with explicit gaps; FIP-0003
through FIP-0011 remain conformant.

## Verified result

The immutable treatment package is
`/nix/store/is3sfw30rzr58w29q8rnsdcdidwxqgzn-fortlet-0.1.0`. Public cold
`prepare codex` succeeded, and the immediate identical invocation verified the
published project layer as a cache hit.

One packaged Codex 0.147.0 process ran from marker-free Herdr pane `wA:p9`. It
authenticated, streamed without an Apps warning, reported
`cargo_target=target/fortlet-guest`, and changed only
`tests/pre_runtime_failures.rs`. The retained diff extracts two file-local
fixture helpers and adds
`codex_shim_invalid_project_environment_fails_before_credentials_or_runtime_artifacts`.
The test exercises the compiled binary through a `codex` symlink, asserts the
exact invalid normalized project-environment failure, and proves project,
tool, and environment roots remain absent.

Experiment 0037 independently created an ordinary owned Codex capsule with the
same immutable package and no provider request. Inside that capsule, the exact
focused Cargo command reported `target/fortlet-guest`, compiled the previously
cold dependency set, passed 1/1, and exited zero. This controlled successor
cleared the earlier environment-specific DNS observation. It is not an open
product issue and should not direct successor work.

The exact host test and all 14 `pre_runtime_failures` tests pass. Public
inventory identified exactly one owned capsule; packaged stop/reset returned
`stopped` then `reset`, and final status was absent with `no capsules` and raw
inventory `[]`. Immutable layers and persistent harness state remain.

## Next action

Update and sign the final branch tip, keep PR #12's framing focused on the
workspace-backed guest test and retained regression coverage, require hosted
Rust verification, and leave squash merge to the operator. Do not add product
DNS/firewall work, retry a terminal experiment, dispatch another model process,
mutate `main`, or merge.

After operator squash merge, fetch `main`, prove its tree equals the exact
reviewed tip, and remove only this GOAL's local and remote bookmark. A successor
GOAL can return to product-focused daily-use work rather than environment
diagnosis.
