# GOAL: Simplify goal publication authority

Status: completed on 2026-08-17.

Archived after PR #3 squash-merged as
`633ea638013ba675e82ea7b06b1d3a287d7aa003`; its tree is byte-identical to
reviewed tip `063e82455b8a9046afa6a8b2a236a6856235e8ab`.

Correct FIP-0005's over-granular approval model before more product work.
Preserve exact review, protected `main`, hosted verification, squash history,
and operator-owned merge while reducing the normal publication path to one
operator approval of a bounded draft-PR transaction and one manual merge.

## Deliverables

1. Amend FIP-0005 so one exact publication packet authorizes the named branch
   push, one draft PR, hosted-check observation, evidence-only PR-body refresh,
   readiness after success, and verified landed-bookmark cleanup.
2. Require new approval if the reviewed tree, scope, branch, base, title,
   substantive body, run count, or declared resources change, or if any push,
   check, signature, remote-state, merge, or landing comparison fails.
3. Keep repository-setting changes, additional pull requests, retries,
   releases, tags, packages, secrets, deployments, unrelated refs, and merge
   outside that authority.
4. Treat the approved routine publication packet as governed rollout rather
   than creating an experiment solely because it contacts the remote.
5. Update durable workflow and runbook instructions plus deterministic tests;
   rewrite the handoff and keep FIP-0005 conformant.

## Definition of Done

1. Normal goals require at most one publication approval and the operator's
   manual squash merge.
2. The approved transaction cannot change reviewed code or expand remote scope.
3. Hosted Rust verification and local Nix verification remain mandatory; a
   failure stops without retry or readiness.
4. The complete runbook verification set passes, the outgoing stack is signed
   and reviewable, and the handoff is current.

## Binding rules

1. This operator-accepted FIP-0005 correction governs its own publication.
2. Use Jujutsu and never push or force-push `main`.
3. The operator remains the sole merge authority unless they explicitly
   authorize the agent to merge one named pull request.
4. Make no product, workflow-job, dependency, repository-setting, protection,
   runtime, release, or deployment change.

## Verification

Run the complete standard verification set in `docs/RUNBOOK.md` before the one
publication approval request.

## Completion

FIP-0005, `WORKFLOW.md`, `AGENTS.md`, and the runbook now define one bounded
publication approval per ordinary goal while preserving exact-tree review,
hosted verification, protected `main`, and operator-owned merge. The focused
four-test publication contract and complete local verification set passed.
The first bounded packet produced PR #3; hosted Rust run `32065763746` passed,
the operator squash-merged it, and read-only landing verification established
exact tree equality before both declared goal bookmarks were removed.
