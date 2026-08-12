# Session Handoff — Project capsule status and stop are complete

Audience: a fresh agent session. `GOAL.md` is normative and complete. No
experiment is active; Experiments 0001 through 0010 are terminally closed.

## Verified result

1. FIP-0003 is accepted and conformant. Checkpoint `bfafd305` adds
   `fortlet status [harness]` and `fortlet stop <harness>` with the same
   `--project` and `--allow-broad-mount` semantics as launch.
2. Status reports `harness<TAB>state`, selects all registered harnesses in
   registry order by default, and performs no credential read, provisioning,
   capsule-state preparation, harness launch, or runtime mutation.
3. Stop is project+harness scoped and idempotent. It shares the capsule lock
   with launch, rechecks stored configuration and ownership before mutation,
   invokes MicroSandbox's bounded stop, and leaves the stopped capsule and
   persistent harness state reusable.
4. Launch, status, and stop share a pure descriptor. Management authorization
   requires managed, schema, project, and tool labels. Fortlet version is
   diagnostic for management, so an older owned capsule remains stoppable;
   launch still rejects stale runtime configuration.
5. Malformed configuration, deterministic-name collisions, ownership
   mismatches, runtime failures, unknown harnesses, and project failures are
   fail-closed with stage and correction context.
6. README, overview, and runbook now describe optional transparent shims,
   explicit launch, native escape, and project-scoped status and stop as the
   current product surface.

## Live evidence and cleanup

Experiment 0010 used immutable package
`/nix/store/5w8rn5bhck9k063k3qdjmd1mvbqq03ki-fortlet-0.1.0` and one temporary
project. The public CLI observed Codex absent, launched only `codex --version`
at pinned 0.147.0, observed running, stopped it, observed stopped, and observed
absent after cleanup. A four-label AND query proved exact Fortlet ownership
before removal.

The experiment used zero money, paid quota, model prompts, remote mutation, or
retries. Its one capsule, Fortlet project state, lock, and empty temporary
project were removed and their absence verified.

## Verification

After terminal closure, the full suite passed with 34 unit tests, one
conformance test, two management-failure integration tests, two native
integration tests, and ten pre-runtime integration tests. Formatting, strict
all-target/all-feature Clippy, the standalone conformance gate, and
`nix flake check` passed. Nix emitted the existing missing app metadata warning
and omitted incompatible `x86_64-linux`; native Linux verification remains
outstanding.

## Current product boundary

Implemented daily-use surfaces are `doctor`, explicit `run`, optional
package-owned `codex` and `tact` shims, explicit `native`, project-scoped
`status`, and project-scoped `stop`. Reusable project+harness capsules,
immutable base and harness layers, brokered ChatGPT credentials, safe project
resolution, broad-root protection, and deterministic pre-runtime diagnostics
are established.

FIP-0001 remains partial. Exact gaps are harness-owned persistent paths and
credential policy; live capsule-reconciliation and terminal-attachment failure
evidence; capsule topology, concurrency, and terminal coverage; restart, logs,
and tool-update commands; explicit interactive/background leases; declarative
project environments and private overlays; host-side publication; standalone
non-Nix installation; and native `x86_64-linux` package verification. FIP-0002
also retains the documented automated Codex exit-status evidence gap.

## Successor boundary

Do not broaden FIP-0003 into global inventory or destructive removal without a
new accepted design. Do not reopen deterministic pre-runtime failures,
project resolution, transparent shim activation, or Codex PTY exit attempts
without new evidence. The next GOAL should select one bounded product gap with
the operator; no gap above is automatically authorized.
