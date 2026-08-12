# Experiment 0012: Project capsule reset

Status: declared
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

Pending. Before dispatch, record the exact treatment identity and package,
prove the temporary project absent, verify immutable layers complete, exercise
all reset decisions through deterministic tests, and pass the complete
standard verification set. Rehearsal MUST NOT create a capsule, read a real
credential, launch a harness, or contact the network.

## Results

Pending declared dispatch.

## Terminal Closure

Pending.
