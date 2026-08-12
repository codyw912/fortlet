# Experiment 0012: Project capsule reset

Status: completed — accepted
Design: FIP-0004
Charter scope: `local-foundation/v1`

## Baseline / Control

Checkpoint `50637a5c` accepts the project-scoped reset contract. FIP-0003 and
Experiment 0010 establish shared capsule identity, status, bounded stop,
ownership validation, and exact owned cleanup. At this baseline, users still
need raw `msb` commands to remove a terminal capsule or recover from stale
configuration.

The treatment will be the exact implementation checkpoint produced by the
current GOAL. Its revision and immutable package output MUST be recorded before
dispatch. The host is `aarch64-darwin`, MicroSandbox is SDK 0.6.8, and Codex is
pinned to 0.147.0.

## Hypothesis and Production Mechanism

Because reset shares launch's descriptor and the existing management lock, it
should refuse the exact owned capsule while running, then remove its disposable
runtime record after explicit stop. Since the project and harness state are
bind mounts outside the disposable VM root, a sentinel and public credential
projection in that persistent directory should survive removal.

## Declared Scope

Use one newly created temporary project at
`/private/tmp/fortlet-reset-0012`. Use the treatment's immutable explicit CLI
in this fixed order:

1. `reset codex` reports `absent`; no project+harness state or capsule lock is
   created.
2. `run codex -- --version` creates one owned capsule and reports pinned Codex
   version 0.147.0 without a prompt.
3. `reset codex` fails with the stop-first correction, and `status codex`
   remains `running`.
4. `stop codex` reports `stopped`.
5. Create one harmless sentinel file in the exact owned persistent
   project+harness state directory and record only its fixed non-secret
   contents.
6. Verify exact capsule ownership and terminal state, then `reset codex`
   reports `reset`.
7. `status codex` reports `absent`; the project, sentinel, public credential
   projection, and existing immutable base and Codex layers remain, while the
   MicroSandbox record is absent.
8. A second `reset codex` reports `absent` without changing preserved state.
9. Remove only the enumerated owned Fortlet state, lock, and empty temporary
   project, then verify their absence.

The treatment revision, package, host, runtime, harness version, temporary
project, command order, and arguments remain frozen. Reuse only already
complete immutable environment layers; if absent, do not dispatch. The launch
may use the normal host credential through Fortlet's broker but no command may
print, copy, inspect, or record its value or the fingerprint contents.

No model prompt, repository mutation, publication, remote execution, shell
mutation, native harness, package provisioning, unowned capsule, second
harness, retry, or network-backed layer creation is in scope.

## Alternatives

1. Reset an existing repository capsule. Rejected because it may carry an
   operator workload and cannot prove side-effect-free initial absence.
2. Prove only deterministic fake lifecycle observations. Rejected because they
   cannot establish real SDK removal or bind-mounted state preservation.
3. Automatically stop inside reset. Rejected by FIP-0004 because it hides
   termination of live work.
4. Launch again after reset. Rejected because final absence plus preserved
   state directly proves reset's boundary; another launch adds VM work without
   testing a distinct reset decision.

## Risks

1. The capsule may fail to launch, stop, or reset. The failed unit is recorded
   and the experiment closes without retry.
2. A stale or colliding capsule could target the generated identity. Initial
   absence and exact ownership inspection prevent dispatch or removal in that
   case.
3. Missing layers could trigger provisioning or network use. A read-only layer
   marker precheck is mandatory; absence prevents dispatch.
4. Reset could delete or alter persistent state. The sentinel, project, public
   projection, and layer checks make that failure visible before cleanup.
5. Credential, mount, unexpected output, unowned process, or outside-scope
   mutation anomalies trigger immediate terminal closure and charter
   escalation.
6. Cleanup may fail after behavior passes. That rejects the complete unit and
   leaves the exact owned artifact reported for manual recovery.

## Acceptance Criteria

1. Deterministic reset tests and the complete standard verification set pass
   before dispatch.
2. Initial and second public reset outputs are exactly
   `codex<TAB>absent`, and initial absence creates no capsule state or lock.
3. The launch exits zero and reports pinned guest Codex 0.147.0 without a
   prompt, provisioning message, native output, or credential value.
4. Reset while running exits nonzero with the stop-first correction, does not
   remove or stop the capsule, and status remains exactly
   `codex<TAB>running`.
5. Public stop and terminal reset outputs are exactly
   `codex<TAB>stopped` and `codex<TAB>reset`.
6. Four ownership labels and terminal state match the exact generated target
   before reset.
7. After reset, public status and a four-label MicroSandbox query prove the
   runtime record absent while the project, fixed sentinel, public credential
   projection, and immutable layer markers remain.
8. Cleanup removes only exact enumerated owned state and the empty temporary
   project; final absence is verified.
9. Every criterion must pass to accept this one-unit local screen. Any failure
   rejects it; no retry is permitted.
10. The experiment terminates on one failure, a hard-invariant anomaly,
    complete cleanup, or 15 elapsed minutes after dispatch begins.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, zero remote mutation,
one Codex `--version` launch, one local Fortlet-owned capsule, zero retries,
and at most 15 elapsed minutes after dispatch begins. Record exact commands,
treatment, package, temporary path, statuses, sanitized output, ownership
metadata, preservation checks, cleanup result, and elapsed time.

## Rehearsal

Completed on 2026-08-12 without creating a capsule, reading a real credential,
launching a harness, or contacting the network:

1. The treatment was checkpoint `3eada810cedd` (`implement safe project
   capsule reset`) and immutable package output
   `/nix/store/7iqradjm2f8pwwc04zzvja2vrikjwnx4-fortlet-0.1.0`.
2. The complete standard gate passed: 39 unit tests, one conformance test, four
   management-failure integration tests, two native integration tests, ten
   pre-runtime integration tests, formatting, strict all-target/all-feature
   Clippy, and `nix flake check`. Nix emitted only the known missing app
   metadata and incompatible `x86_64-linux` warnings.
3. The existing immutable-layer markers for `_base/bookworm-1` and
   `codex/0.147.0` were present under Fortlet's data root, so dispatch required
   no provisioning.
4. The exact temporary project was absent. The immutable CLI rejected that
   missing explicit project at the project stage before runtime access.
5. Deterministic tests exercised every SDK state, side-effect-free absence,
   lock/refetch ordering, ownership, version skew, malformed configuration,
   output, parsing, and lookup, lock, and removal failures.

## Results

Dispatch ran from 2026-08-12 17:35:54 EDT through 17:40:18 EDT against the
treatment and package above.

The execution harness initially denied the immutable CLI access to
`~/.microsandbox` while it opened the migration lock. This preflight invocation
did not reach MicroSandbox and created neither the generated Fortlet harness
state nor its capsule lock. The required elevated invocation then began the
declared live unit. This harness-level correction is retained here rather than
silently omitted; it consumed no capsule, launch, credential read, or runtime
retry.

The canonical temporary project produced identity `463f816302bcded3` and
exact capsule `fortlet-501-codex-463f816302bcded3`. Initial public reset exited
zero with `codex<TAB>absent`; both the exact harness-state path and capsule lock
remained absent. The single no-prompt launch exited zero with:

```text
codex-cli 0.147.0
```

Reset while running exited nonzero with exactly:

```text
fortlet: capsule stage failed; run `fortlet stop codex` and retry reset: capsule is running
```

Public status remained `codex<TAB>running`, proving refusal did not stop or
remove the capsule. Public stop then returned `codex<TAB>stopped`.

An empty non-secret `experiment-0012-sentinel` file was created in the exact
owned persistent harness-state directory. Presence-only checks found the
project, sentinel, public credential projection, credential fingerprint, and
both immutable layer markers. A stopped MicroSandbox query AND-matched
`fortlet.managed=true`, `fortlet.schema=1`,
`fortlet.project=463f816302bcded3`, and `fortlet.tool=codex`, returning exactly
`fortlet-501-codex-463f816302bcded3` before removal.

Terminal public reset exited zero with `codex<TAB>reset`. Public status then
returned `codex<TAB>absent`, and the repeated four-label query returned no
runtime record. Before any cleanup, all six preservation checks still passed.
A second reset exited zero with `codex<TAB>absent`.

Final names-only inspection confirmed the exact empty temporary project,
generated capsule lock, and owned harness-state tree. Cleanup removed only
those enumerated artifacts. Their final absence was verified at 17:40:18 EDT.

## Terminal Closure

1. Outcome: accepted — the immutable public CLI refused active reset, removed
   the exact owned terminal capsule, preserved every declared durable surface,
   and reported subsequent absence idempotently.
2. Root cause: reset shares launch's descriptor and lock, reauthorizes fresh
   stored configuration under that lock, permits version skew, and delegates
   only stopped or crashed removal to the SDK handle without touching
   project, harness-state, credential, or layer paths.
3. Actual total cost: zero money, zero paid quota, zero model prompts, zero
   remote mutation, one of one Codex `--version` launch, one of one owned local
   capsule, zero live-runtime retries, and 4 minutes 24 seconds versus the
   15-minute ceiling. One execution-harness permission preflight was denied
   before runtime contact and recorded above.
4. Next action: mark FIP-0004 conformant, close the reset mission, rewrite the
   handoff, and stop for operator selection of the next product goal.
