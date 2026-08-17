# Session Handoff — Hosted Linux linker successor is authorized

Audience: a fresh agent session. `GOAL.md` is normative and active. Experiment
0013 is terminally rejected after its one declared hosted unit; Experiment 0014
is the authorized successor. Experiments 0001 through 0013 are terminally
closed.

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
   `650ead13e926086f10f4453da83a7704fdad6bad`, and draft pull request
   `https://github.com/codyw912/fortlet/pull/1` stores the exact reviewed
   metadata. All ten outgoing commits have valid GitHub SSH signatures.
8. Hosted run `32039976577` rejected at its first `cargo test`: Linux linking
   required `-lcap-ng`, but the clean `ubuntu-24.04` runner lacked the
   development linker file. It was not rerun or corrected remotely.

## Product state

The public repository remains at `https://github.com/codyw912/fortlet` with SSH
origin `git@github.com:codyw912/fortlet.git`. Remote `main` remains at
`e95b0cdb1308f732d3f45db7a85027d45bcd4048`; only the primary goal bookmark and
draft PR were added. No readiness, merge, protection, or settings mutation has
occurred.

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

Use Jujutsu for all local history. Inspect the unpublished successor above the
primary bookmark with `jj log -r 'goal/project-capsule-reset..@'`. Do not push
or mutate the existing PR or GitHub without a new exact operator authorization.

## Current mission boundary

Implement Experiment 0014's minimum `libcap-ng-dev` workflow dependency and
deterministic evidence, then run the complete local verification set. Present
the exact successor stack and diff, same bookmark destination, verification
state, rejected run, and expected PR-head update before requesting approval for
the one correction push. The operator still owns readiness and both bootstrap
merges. Do not push, update PR metadata, mark readiness, merge, or change
repository settings on inferred authority.
