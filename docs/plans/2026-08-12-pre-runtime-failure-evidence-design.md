# Pre-runtime failure evidence design

Status: validated by the operator on 2026-08-12.
Governing design: FIP-0001.

## Purpose

Prove Fortlet's real CLI failure behavior from process entry through the last
deterministic pre-runtime boundary. The slice does not start MicroSandbox,
provision packages, contact a network service, or execute Codex or Tact.

## Observable contract

Each integration case invokes the compiled `fortlet run` command in an
isolated temporary environment and proves:

1. the command exits unsuccessfully;
2. standard error begins with `fortlet: <stage> stage failed;`;
3. the stage context gives one primary corrective action before its underlying
   cause; and
4. artifacts from later transaction stages are absent.

The deterministic matrix covers an unsupported harness, a missing explicit
project, missing and invalid credentials, local capsule-state preparation,
guest credential projection, and incomplete environment layers. Live capsule
reconciliation and terminal attachment remain outside this slice.

## Test harness

Add one Unix integration-test module for the real compiled binary. A fixture
builder creates an isolated home, project, state root, data root, and fake auth
document, then invokes an explicit project launch such as:

```text
fortlet run codex --project <temporary-project> -- --version
```

The subprocess environment is cleared and rebuilt with only required values,
including temporary `HOME`, `XDG_STATE_HOME`, `XDG_DATA_HOME`, and
`FORTLET_AUTH_FILE`. It cannot inherit real authentication, runner markers,
shell configuration, or credential paths. Fake access tokens are structurally
valid and long-lived but unusable outside the test. Assertions never print or
depend on their value.

Tests advance only as far as necessary. Harness and project failures need no
auth fixture. Credential cases use absent or deliberately invalid auth.
Capsule-state and projection cases use valid fake auth plus precreated invalid
state. Environment cases seed incomplete layers so `EnvironmentStore` rejects
them synchronously before provisioning can begin.

A shared assertion helper checks the outer stage, action, representative cause,
nonzero status, and absence of later artifacts. One primary correction means
the stage context contains one imperative action between its semicolon and the
first cause separator; nested filesystem causes may retain their own details.
Tests do not assert platform-specific operating-system wording.

## Test matrix

Build the evidence test-first, one stage at a time:

1. `harness`: unknown harness name;
2. `project`: missing explicit project;
3. `credentials`: missing auth, malformed auth, expired fake token, and an auth
   file inside the selected project mount;
4. `capsule`: an invalid project/harness state location blocks local capsule
   preparation;
5. `credentials`: a symlinked or invalid guest auth projection fails after
   local capsule-state creation; and
6. `environment`: incomplete base and registered-harness layers.

Every case verifies that no later environment layer or capsule artifact was
created. Environment cases additionally establish that no provisioning capsule
was attempted by exercising only the local incomplete-layer rejection path.

## Implementation constraints

Production changes are limited to diagnostic inconsistencies exposed by the
process-level tests. If repeated stage names or actions need consolidation, use
small constants or a small stage representation in `session.rs`. Do not add an
injectable launch orchestrator, generic runtime abstraction, test-only product
flags, or a live fault mechanism.

## Documentation and conformance

The runbook will document the deterministic pre-runtime failure gate and its
boundary. Conformance will replace the broad failure-stage gap with the exact
remaining gap: deterministic evidence covers failures through local environment
validation, while live capsule reconciliation and terminal attachment failure
evidence remains outstanding.

No experiment record is needed because all evidence is deterministic and
repository-local.

## Completion boundary

Run the focused integration suite, all tests, formatting, strict Clippy,
conformance, and `nix flake check`. Keep code, tests, documentation, and
conformance in one checkpoint. Close the GOAL and rewrite the handoff in a
second checkpoint, rerun conformance from the clean successor, inspect
`main..@`, then stop.

The engineering ceiling is two hours. External spend, model prompts, harness
launches, MicroSandbox capsules, networking, and live experiments are all zero.
