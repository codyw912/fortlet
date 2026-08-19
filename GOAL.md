# GOAL: Delegate goal-scoped publication iteration

Status: active — operator-accepted 2026-08-19. PR #5 proved that exact
per-push packets turn an ordinary hosted test failure into repeated approval
ceremony without changing the protected `main` or operator merge boundary.

Replace that approval model with standing authority derived from an accepted
GOAL. Keep one branch, one pull request, required checks, protected `main`, and
manual operator merge; remove approval stops for normal branch iteration.

Before implementation, read FIP-0001 and FIP-0005 in full.

## Deliverable 0 — Accept delegated goal publication

1. Amend FIP-0001 so exact review occurs in the pull request before merge,
   rather than requiring approval before the first branch push.
2. Amend FIP-0005 so accepting a GOAL authorizes one goal bookmark and pull
   request through ordinary pushes, CI diagnosis and fixes, body updates,
   readiness, landing verification, and cleanup.
3. Require only the publishable branch tip to be signed; its commit identity
   transitively fixes its parent history.
4. Preserve operator-only merge, protected `main`, required hosted checks,
   trusted host tooling, and every secret and publication boundary.

## Deliverable 1 — Remove duplicated packet ceremony

1. Replace exact-packet and per-retry approval language in `AGENTS.md`,
   `WORKFLOW.md`, the runbook, and the active charter.
2. Let the agent create and maintain one draft PR after goal acceptance,
   including repeated in-scope pushes and diagnosed CI repairs.
3. Mark the PR ready only after locally knowable work, experiments, local
   verification, and hosted verification are complete.
4. Report the final reviewed tip, scope, verification, and limitations for
   operator review, but do not turn that report into another approval gate.

## Deliverable 2 — Verify and publish once

1. Update publication-workflow evidence to require standing goal authority,
   signed-tip integrity, iteration after diagnosed failures, and operator merge.
2. Keep FIP-0005 conformant in the same checkpoint as the rules and tests.
3. Run the complete standard verification set.
4. Publish this GOAL through one branch and draft PR under its own accepted
   amendment. The operator performs the squash merge.

## Definition of Done

1. Accepting a GOAL is the only routine publication authorization.
2. One goal branch and PR may iterate without additional operator approval.
3. CI failures are diagnosed and fixed normally; repeated failures sharing one
   assumption still trigger the charter stop.
4. The final branch tip is signed, local and hosted gates pass, and the PR is
   ready for operator merge.
5. Direct or force pushes to `main`, agent-owned merge, settings changes,
   releases, tags, packages, secrets, and unrelated remote resources remain
   unauthorized.

## Excluded scope

Do not change hosted CI jobs, branch protection, merge settings, product code,
runtime behavior, experiment mechanics, release policy, or repository secrets.

## Budget and escalation

Engineering ceiling: one hour. External spend and paid quota remain zero. This
accepted GOAL authorizes one descriptive bookmark, one draft PR targeting
`main`, ordinary in-scope pushes and hosted runs, PR maintenance, readiness,
and exact landed-bookmark cleanup. Stop on material scope expansion, a new
architecture decision, secrets or credential handling, destructive remote
mutation, repository settings, release resources, merge, or two failures that
share an unresolved cause.

## Verification

Run `cargo test --test publication_workflow` while editing, then the complete
standard verification set from `docs/RUNBOOK.md` before readiness.
