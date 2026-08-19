# GOAL: Prove one ordinary managed Codex work loop

Status: active — operator-accepted 2026-08-19. PR #8 merged the FIP-0010
non-interactive correction to `main` at
`53378399bfa581d5a7b3db8538b234dc3733bb26`; its tree matched the reviewed
signed tip, the goal bookmark was removed locally and remotely, and the working
copy began clean directly above merged `main`.

Prove that the corrected packaged Fortlet path is suitable for ordinary daily
work: establish guest DNS/TLS readiness without provider credentials, then run
one bounded non-interactive Codex task that makes one small real repository
change, runs its declared check, streams visible progress, and exits cleanly.
Separate host firewall readiness from Fortlet behavior and retain a failed unit
honestly rather than iterating until it appears successful.

Before implementation or experiment dispatch, read FIP-0001, FIP-0002,
FIP-0005, FIP-0006, FIP-0007, FIP-0008, FIP-0009, and FIP-0010 in full.

## Deliverable 0 — Verify and freeze the daily-use unit

1. Reverify the merged `main` baseline, conformance, clean working copy, public
   Codex absence, and empty MicroSandbox inventory before relying on the prior
   handoff.
2. Select one useful, low-risk repository maintenance task with a narrow file
   boundary and an existing or predeclared deterministic check. The task must
   be representative of an edit/test loop, not a synthetic response-only
   prompt or an architecture change.
3. Freeze the exact immutable Fortlet package, project path, prompt, permitted
   files, expected check, lifecycle cleanup, and acceptance criteria in one
   experiment record before any external request.
4. Do not create a new public command, configuration key, timeout, credential
   policy, or workload-lease semantic. Any such need requires a new proposal
   and operator acceptance before implementation.

## Deliverable 1 — Establish credential-free network readiness

1. Before the model task, use one disposable synthetic-auth Codex capsule and
   the packaged MicroSandbox runtime to test the same guest network path's DNS
   resolution and TLS reachability without sending a bearer token, model
   request, prompt, repository content, or paid traffic.
2. Keep the probe bounded, record its exact destination and output, and clean
   up through packaged public stop/reset. The final public status must be
   absent and MicroSandbox inventory empty.
3. Fortlet must not read or change macOS firewall configuration. The operator
   may approve one ordinary OS firewall prompt manually while the already-
   declared probe is running; record whether a prompt appeared and whether the
   same process then completed.
4. If DNS/TLS remains unavailable, terminate the experiment with one manual
   host correction and do not spend the model unit. Do not add firewall
   automation or general network diagnostics under this GOAL.

## Deliverable 2 — Run one real managed work loop

1. Only after the credential-free preflight and complete standard gate pass,
   launch the frozen immutable package once through
   `fortlet run codex --project /Users/cody/dev/fortlet -- exec ...`.
2. Send exactly one prompt for the predeclared repository task. Require Codex
   to stay within the permitted files, use no unrelated tools or network
   resources, run the declared deterministic check, report its result, and
   terminate normally.
3. Require streamed progress before completion, no Apps warning, exact host
   status zero, the intended diff only, and an independently repeated host
   check after Codex exits.
4. Regardless of outcome, record the first result without retry. Verify the
   reusable capsule remains publicly manageable, then stop/reset it and require
   final absence and empty MicroSandbox inventory.

## Deliverable 3 — Close and publish once

1. Attribute any failure to host network readiness, Fortlet, MicroSandbox, or
   Codex using the preflight, streamed output, exit status, and pinned source;
   do not infer from silence.
2. Fix only a demonstrated Fortlet-owned defect already governed by accepted
   FIPs. Do not issue a second model prompt to validate a correction.
3. Update conformance, runbook, GOAL, experiment index, and handoff with the
   exact supported surface and remaining limitation. Preserve rejected units.
4. Run the complete standard verification set through `nix develop`, including
   any pinned compatibility fixture whose boundary changed.
5. Use one descriptive bookmark and one draft PR under FIP-0005. Sign the final
   tip, require hosted `Rust verification`, mark the PR ready, and leave the
   squash merge to the operator.

## Definition of Done

1. A credential-free control proves guest DNS/TLS readiness without credential
   or model traffic, or terminates with one verified manual host correction
   before model spend.
2. The sole model-backed unit either completes the declared repository
   edit/test loop with visible progress and exact status zero, or fails within
   the accepted FIP-0010 bound with attributable evidence and one correction.
3. No unexpected repository file, credential content, host configuration,
   Apps authorization, browser state, or external resource is read or changed.
4. Public lifecycle cleanup restores Codex absence and an empty MicroSandbox
   inventory while preserving intended durable Fortlet layers and cache.
5. Experiment records are terminal, conformance is honest, local and hosted
   gates pass, and the single PR is ready for operator merge.

## Excluded scope

Do not add firewall automation, a general network diagnostic command, logs,
restart, global inventory, cross-project cleanup, standalone installation, a
Codex fork, an alternative runtime, remote execution, a proxy, a new secret
class, hosted CI changes, repository settings, release, tag, package
publication, or an unrelated product feature.

## Budget and escalation

Engineering ceiling: 90 minutes. External money and paid quota remain zero
beyond one short operator-authorized model-backed Codex work unit after the
credential-free preflight and complete gate pass. This GOAL, once accepted,
authorizes one descriptive bookmark and one draft PR targeting `main` under
FIP-0005.

Stop for material scope expansion, a new architecture contract, a second model
prompt, automated firewall mutation, destructive or unrelated mutation, merge,
any credential or data-boundary anomaly, or two failures sharing an unresolved
assumption.

## Verification

Run focused deterministic checks while selecting and rehearsing the unit.
Before live dispatch and readiness, run the complete standard verification set
from `docs/RUNBOOK.md` inside `nix develop`, plus any pinned compatibility
command required by a touched boundary.
