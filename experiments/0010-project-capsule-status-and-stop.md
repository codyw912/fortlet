# Experiment 0010: Project capsule status and stop

Status: declared
Design: FIP-0003
Charter scope: `local-foundation/v1`

## Baseline / Control

Checkpoint `21ca0022` accepts the project-scoped status-and-stop contract. At
that baseline Fortlet can create and reuse deterministic project-and-harness
capsules, but exposes no product command that observes or stops them.

The treatment will be the exact implementation checkpoint produced by the
current GOAL. Its revision and immutable package output MUST be recorded here
before dispatch. The host is `aarch64-darwin`, MicroSandbox is SDK 0.6.8, and
Codex is pinned to 0.147.0.

## Hypothesis and Production Mechanism

Because launch, status, and stop derive one capsule descriptor, a capsule
created by `fortlet run codex -- --version` for a unique project should be the
same capsule that `fortlet status codex` reports and `fortlet stop codex`
targets. Ownership-label validation should authorize that exact capsule; the
SDK's bounded stop should move it to stopped state without deleting it.

## Declared Scope

Use one newly created temporary project directory whose exact path is recorded
before dispatch. Use the treatment's public explicit CLI only, in this fixed
order:

1. `status codex` reports `absent` for the temporary project.
2. `run codex -- --version` creates the owned capsule and reports pinned Codex
   version 0.147.0 without a prompt.
3. `status codex` reports `running`.
4. `stop codex` reports `stopped`.
5. `status codex` reports `stopped`.
6. Read-only MicroSandbox metadata verifies the exact capsule ownership before
   its stopped runtime record and owned temporary Fortlet state are removed.
7. `status codex` again reports `absent`.

The treatment revision, package, host, runtime, harness version, temporary
project, command order, and arguments remain frozen. Reuse only already
complete immutable environment layers; if they are absent, do not dispatch.
The commands may read the normal host credential through Fortlet's existing
broker but MUST NOT print, copy, or inspect its value.

No model prompt, repository mutation, publication, remote execution, shell
mutation, native harness, package provisioning, unowned capsule, second
harness, retry, or network-backed layer creation is in scope.

## Alternatives

1. Stop an existing repository capsule. Rejected because an existing capsule
   may carry an operator workload and would not provide an absent baseline or
   disposable cleanup target.
2. Prove only deterministic mocks. Rejected because they cannot establish that
   the public CLI agrees with MicroSandbox's real lifecycle transitions.
3. Add a test-only fixture command that creates a capsule. Rejected because the
   existing no-prompt Codex version path exercises the actual launch identity
   with less product-only apparatus.
4. Run both harnesses. Rejected because one project+harness unit falsifies the
   shared lifecycle mechanism and stays within the smallest live scope.

## Risks

1. The capsule may fail to start or stop. The failed unit is recorded and the
   experiment closes without retry.
2. A stale or colliding capsule could target the generated identity. The
   absent precheck and exact ownership inspection prevent dispatch or cleanup
   in that case.
3. Missing layers could trigger provisioning or network use. A read-only layer
   precheck is mandatory; absence prevents dispatch.
4. Credential, mount, unexpected output, unowned process, or outside-project
   mutation anomalies trigger immediate terminal closure and charter
   escalation.
5. Cleanup may fail after the behavior passes. That rejects the complete unit
   and leaves the exact owned artifact reported for manual recovery.

## Acceptance Criteria

1. Deterministic management tests and the complete standard verification set
   pass before dispatch.
2. Initial and final public status are exactly `codex<TAB>absent`.
3. The launch exits zero and reports pinned guest Codex version `0.147.0`
   without a prompt or credential value.
4. The intermediate public observations are exactly
   `codex<TAB>running`, `codex<TAB>stopped`, and the stop result
   `codex<TAB>stopped`.
5. Read-only metadata proves the target carries the expected Fortlet managed,
   schema, project, and tool ownership labels before cleanup.
6. Cleanup removes only the exact stopped owned capsule and owned temporary
   state, and the final status proves the capsule absent.
7. Every criterion must pass to accept the experiment. Any failure rejects the
   one-unit local screen; no retry is permitted.
8. The experiment terminates on one failure, a hard-invariant anomaly, complete
   cleanup, or 15 elapsed minutes after dispatch begins.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, zero remote mutation,
one Codex `--version` launch, one local Fortlet-owned capsule, zero retries,
and at most 15 elapsed minutes after dispatch begins. Record exact commands,
revision, package output, temporary path, statuses, sanitized output, ownership
metadata, cleanup result, and elapsed time.

## Rehearsal

Pending. Before dispatch, record the exact treatment identity and package,
prove the generated target absent, verify immutable layers already complete,
exercise all command decisions through deterministic tests, and pass the
complete standard verification set. Rehearsal MUST NOT create a capsule, read
a real credential, launch a harness, or contact the network.

## Results

Pending declared dispatch.

## Terminal Closure

Pending.
