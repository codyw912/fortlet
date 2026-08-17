# Session Handoff — Portable `openpty` successor is locally qualified

Audience: a fresh agent session. `GOAL.md` is normative and active. Experiments
0013 and 0014 are terminally rejected after their single declared hosted units.
Experiments 0001 through 0014 are terminally closed, and Experiment 0015 is the
one active successor. Its local implementation is qualified; no remote
correction, push, or run is authorized without a new exact transaction
approval.

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
    suite and the complete standard local verification set pass on
    `aarch64-darwin`; no remote mutation has tested Linux yet.

## Product state

The public repository remains at `https://github.com/codyw912/fortlet` with SSH
origin `git@github.com:codyw912/fortlet.git`. Remote `main` remains at
`e95b0cdb1308f732d3f45db7a85027d45bcd4048`; the primary goal bookmark is at
`0b75f02dd5422362dd49acff776aecff1e94ad7b`, and PR #1 remains open and draft.
No readiness, merge, protection, or settings mutation has occurred.

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
PR or GitHub without a new exact operator authorization. Experiment 0014's one
correction push and one replacement hosted run are exhausted. Experiment 0015
authorizes local implementation and verification only until its exact remote
transaction is separately presented and approved.

## Current mission boundary

The Experiment 0015 correction is locally qualified. Refresh and present the
exact stack, diff, destination, remote PR/head/run baseline, signatures,
verification state, rejected control, and Jujutsu dry-run before requesting
approval for its one correction push. The operator still owns readiness and
both bootstrap merges. Do not push, rerun, update PR metadata, mark readiness,
merge, or change repository settings on inferred authority.
