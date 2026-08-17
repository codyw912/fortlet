# Session Handoff — Bootstrap merge exception is authorized

Audience: a fresh agent session. `GOAL.md` is normative and active. Experiments
0013 and 0014 are terminally rejected after their single declared hosted units.
Experiments 0001 through 0015 are terminally closed. Experiment 0015 accepted
its one hosted unit, so the primary PR's required Rust verification is green.
PR #1's exact reviewed tree landed through a two-parent merge commit. The
operator authorized FIP-0005's one-time primary-bootstrap exception and
Experiment 0016; no remote setting, protection, bookmark, closure PR, branch,
or other mutation is authorized without its separate exact approval.

## Verified result

1. FIP-0004 is implemented by project-scoped `fortlet reset <harness>` with
   the same explicit project and broad-mount selection as run, status, and
   stop.
2. Reset returns absence before creating Fortlet state or a capsule lock. A
   present target is locked, fetched again, parsed, and checked for exact name
   plus managed, schema, project, and tool ownership. Version skew remains
   acceptable for management.
3. Reset removes only owned stopped or crashed capsules through the
   MicroSandbox SDK handle. Created, starting, running, draining, and paused
   capsules fail with the direct `fortlet stop <harness>` correction; reset
   never stops or kills work implicitly.
4. Launch's stale-configuration guidance now uses public Fortlet stop and
   reset commands without exposing the internal capsule name or project
   identity.
5. Experiment 0012 accepted one immutable public-CLI unit. It proved initial
   side-effect-free absence, running refusal, explicit stop, exact four-label
   ownership, terminal reset, runtime-record absence, project and persistent
   state preservation, idempotent second reset, and exact cleanup.
6. The complete standard verification set passed immediately before the live
   unit. FIP-0003 and FIP-0004 are conformant; FIP-0001 and FIP-0002 remain
   partial for their explicitly listed gaps.
7. The primary publication bookmark is public at signed tip
   `0b75f02dd5422362dd49acff776aecff1e94ad7b`, and draft pull request
   `https://github.com/codyw912/fortlet/pull/1` stores the exact reviewed
   metadata. All fourteen outgoing commits have valid GitHub SSH signatures.
8. Hosted run `32039976577` rejected at its first `cargo test`: Linux linking
   required `-lcap-ng`, but the clean `ubuntu-24.04` runner lacked the
   development linker file. That run was not rerun; its separately authorized
   successor correction is recorded below.
9. The separately approved Experiment 0014 correction installed only
   `libcap-ng-dev`. Replacement run `32042472155`, job `95424137789`, checked
   out the exact corrected tip; package installation, Rust installation,
   `cargo test`, and formatting passed. Strict Clippy then rejected
   `examples/pty_observer.rs:233` because Linux `openpty` does not need a
   mutable winsize reference, and conformance was skipped. No rerun or later
   remote mutation occurred.
10. Experiment 0015 replaces the syntactic mutable reference with a named raw
    pointer derived from the same mutable winsize. Its focused 15-test observer
    suite and complete standard local verification set pass on
    `aarch64-darwin`.
11. The separately approved correction pushed only the existing goal branch to
    signed tip `e2e4a6d8b24e974add01a720ddaf73c71de7963a`. All 18 PR commits have
    valid GitHub SSH signatures. Exactly one new hosted run, `32052509798`, job
    `95455045523`, passed package and Rust installation, tests, formatting,
    strict Linux Clippy, conformance, and cleanup in 3 minutes 19 seconds.
12. A separately approved PR-body-only update first received GitHub GraphQL
    HTTP 503. REST read-back proved that attempt made no change, so it was not
    silently retried. The operator approved one unchanged retry; it succeeded,
    and REST read-back matched the reviewed body while title, open draft state,
    `main` base, goal-branch head, and signed head SHA remained unchanged.
13. The operator separately approved the exact draft-to-ready mutation. REST
    read-back reports PR #1 open, non-draft, cleanly mergeable, targeting
    unchanged `main` from unchanged signed head
    `e2e4a6d8b24e974add01a720ddaf73c71de7963a`. The successful hosted run
    remained current, and no new run or ref appeared.
14. PR #1 reports merged at `2026-08-17T18:58:59Z` into `main` as validly
    signed commit `e0f919f80ed90589735f15ff7779ed229122ab1f`. A Jujutsu diff from
    reviewed tip `e2e4a6d8b24e974add01a720ddaf73c71de7963a` to fetched `main` is
    empty, so destination and exact tree equality pass. The landed commit has
    two parents—the previous `main` and reviewed branch tip—and GitHub stores
    the merge-commit subject. The required squash-history criterion therefore
    fails despite the correct tree.
15. The operator chose to preserve `main` and authorized a dated FIP-0005
    amendment accepting only that exact tree-equal primary landing. It forbids
    rewrite or revert repair and retains squash-only landing for the closure PR
    and every future goal. Experiment 0016 is the declared closure successor.

## Product state

The public repository remains at `https://github.com/codyw912/fortlet` with SSH
origin `git@github.com:codyw912/fortlet.git`. Remote `main` is signed merge
commit `e0f919f80ed90589735f15ff7779ed229122ab1f`; the primary goal branch remains
at `e2e4a6d8b24e974add01a720ddaf73c71de7963a`, and PR #1 is closed and merged.
Merge commits, rebase merges, and squash merges all remain enabled. No branch
protection, bookmark cleanup, closure PR, revert, or history rewrite occurred.

Daily-use surfaces are `doctor`, explicit `run`, optional package-owned
`codex` and `tact` shims, explicit `native`, project-scoped `status`, bounded
`stop`, and terminal `reset`. Reusable project+harness capsules, immutable base
and harness layers, brokered ChatGPT credentials, safe project resolution,
broad-root protection, deterministic pre-runtime diagnostics, and explicit
disposable-root recovery are established.

## Remaining gaps

FIP-0001 remains partial. Exact gaps are harness-adapter ownership of
persistent paths and credential policy; live capsule-reconciliation and
terminal-attachment failure evidence; capsule topology, concurrency, and
terminal coverage; restart, logs, and tool-update commands; explicit
interactive/background leases; declarative project environments and private
overlays; host-side publication; standalone non-Nix installation; and native
`x86_64-linux` package verification. FIP-0002 retains the documented automated
Codex exit-status evidence gap.

## Operational notes

Run Cargo verification inside `nix develop`; a vanilla host shell may not
provide macOS `libiconv`. Commands that access `~/.microsandbox` also require
the assistant execution environment's explicit filesystem escalation even
when the operator has already authorized the experiment. Experiment 0012
records one denied preflight before runtime contact so this is not rediscovered
or silently hidden.

Use Jujutsu for all local history. Do not push, rerun, or mutate the existing
PR or GitHub without a new exact operator authorization. Experiments 0014 and
0015 have exhausted their correction pushes and hosted runs.

## Current mission boundary

Experiment 0016's amendment, remote baseline, repository-settings payload,
`main` protection payload, and complete local gates are qualified. Present the
repository PATCH for separate exact approval and verify its read-back before
presenting the protection PUT as a second transaction. Never rewrite or revert
`main`, permit a second merge-method exception, remove a bookmark before tree
equality, or infer authority for settings, protection, a closure branch or PR,
readiness, merge, or cleanup.
