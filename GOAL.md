# GOAL: Prove fail-closed pre-runtime launch errors

Status: completed on 2026-08-12.

Checkpoint `f5f1d341` proves ten real-CLI failure cases through local
environment-layer validation. It also moves known guest-mount credential
checks before capsule-state creation, so an exposed project credential now
fails without creating later transaction artifacts.

Prove Fortlet's real CLI failure behavior from process entry through the last
deterministic pre-runtime boundary. Fix only diagnostic inconsistencies that
the process-level tests expose. Do not start MicroSandbox, provision packages,
contact a network service, or execute Codex or Tact.

Before implementation, read FIP-0001 in full and the validated design at
`docs/plans/2026-08-12-pre-runtime-failure-evidence-design.md`. Preserve every
terminal result from Experiments 0001 through 0009 and the project-resolution
contract established at checkpoint `e764dfc1`.

## Deliverable 1 — Build an isolated real-CLI failure harness

Completed at checkpoint `f5f1d341`: the Unix integration fixture clears each
subprocess environment, supplies only temporary paths and fake auth, and
asserts nonzero status, exact stage/action context, stable cause, and artifact
boundaries.

1. Add one Unix integration-test module that invokes the compiled `fortlet`
   binary through its public `run` interface.
2. Give each subprocess temporary `HOME`, `XDG_STATE_HOME`, `XDG_DATA_HOME`,
   `FORTLET_AUTH_FILE`, project, state, and data paths as needed.
3. Clear inherited environment before adding only the test's required values.
   Do not inherit real credentials, runner markers, shell initialization, or
   operator paths.
4. Use only structurally valid but unusable fake credentials. Never print or
   assert credential values.
5. Provide shared assertions for nonzero exit, outer stage, one primary
   correction, representative cause, and absence of later-stage artifacts.

## Deliverable 2 — Prove early transaction failures

Completed at checkpoint `f5f1d341`: six cases cover unsupported harness,
missing project, missing/malformed/expired auth, and project-mounted auth. The
last case exposed and fixed the credential-validation ordering defect.

Use test-first vertical slices to prove:

1. an unknown harness fails at the `harness` stage before project, credential,
   environment, or capsule work;
2. a missing explicit project fails at the `project` stage before credentials
   or runtime state;
3. missing auth, malformed auth, an expired fake token, and an auth file inside
   the selected project mount fail at the `credentials` stage; and
4. each failure reports its existing actionable correction plus a useful cause
   without creating later-stage artifacts.

## Deliverable 3 — Prove the deepest deterministic failures

Completed at checkpoint `f5f1d341`: four cases cover invalid local capsule
state, invalid guest projection, incomplete base layer, and incomplete harness
layer without entering provisioning or runtime reconciliation.

1. Precreate an invalid project/harness state location and prove local capsule
   preparation fails at the `capsule` stage.
2. Precreate a symlinked or otherwise invalid guest auth projection and prove
   it fails at the `credentials` stage after local capsule-state creation.
3. Seed incomplete base and registered-harness layers separately and prove both
   fail at the `environment` stage before provisioning begins.
4. Prove no test reaches MicroSandbox reconciliation, terminal attachment,
   harness execution, or network-backed provisioning.

## Deliverable 4 — Document, conform, and close

Completed at checkpoint `f5f1d341` and the terminal closure checkpoint: the
runbook documents the deterministic gate, conformance retains the two live
runtime failure gaps, and the complete standard verification set passed.

1. Document the deterministic pre-runtime failure gate and its exact boundary
   in `docs/RUNBOOK.md`.
2. Update `arch/conformance.json` in the same checkpoint as the code and tests.
   Replace only the broad failure-stage gap. Retain an exact gap for live
   capsule reconciliation and terminal attachment failures.
3. Run the complete verification set, rewrite `experiments/HANDOFF.md`, mark
   this GOAL complete, inspect `main..@`, then STOP and report.

## Definition of Done

1. Real subprocess tests cover harness, project, credential, local capsule,
   projection, and local environment validation failures.
2. Every case proves nonzero exit, stage, one primary correction, useful cause,
   and absence of later-stage artifacts.
3. Tests use no real credential, inherited operator environment, VM, harness,
   network service, or external experiment.
4. Documentation and conformance state only the deterministic boundary proved.
5. The complete verification set is green; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001 constraint.
2. Do not change FIP-0001 or expand into runtime injection, live fault
   campaigns, capsule topology, terminal semantics, lifecycle, management
   commands, environments, or packaging.
3. Do not introduce a generic runtime abstraction, injectable orchestrator,
   test-only product flag, or network-backed fixture.
4. Tests may mutate only their owned temporary directories and subprocess
   environments.
5. Do not launch Codex, Tact, MicroSandbox capsules, paid services, or external
   experiments.
6. Prefer public CLI assertions over private implementation assertions.
7. Keep conformance changes in the same checkpoint as their code and tests.
8. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: two hours from implementation start.
2. External budget: zero money, zero paid quota, zero model prompts, zero
   harness launches, zero MicroSandbox capsules, and no live experiment.
3. Stop on any network attempt, credential anomaly, outside-project mutation,
   need to change an accepted FIP, or need to exercise a live runtime failure.

## Verification

Run before claiming completion:

- focused pre-runtime failure integration tests during development;
- `cargo test`;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --test conformance`;
- `nix flake check`.
