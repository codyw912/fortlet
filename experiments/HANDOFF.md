# Session Handoff — Project capsule reset is conformant

Audience: a fresh agent session. `GOAL.md` is normative and complete. No
experiment is active; Experiments 0001 through 0012 are terminally closed.

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

## Product state

The public alpha repository remains at `https://github.com/codyw912/fortlet`
with SSH origin `git@github.com:codyw912/fortlet.git`. The reset mission made
no remote mutation, so the new local stack is not published.

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

Use Jujutsu for all local history. Inspect the unpublished stack with
`jj log -r 'main..@'`. Do not push, create a PR, or mutate GitHub without a new
operator authorization.

## Successor boundary

The reset/recovery mission is complete. The next GOAL should be selected with
the operator rather than inferred. Product-focused candidates are the first
observable workload-lease contract, a similarly narrow restart/log surface,
or deeper capsule topology and terminal evidence. Standalone installation was
explicitly deferred until later. Any new public CLI or lifecycle contract
requires an accepted FIP before implementation.
