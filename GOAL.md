# GOAL: Establish goal-scoped pull request publication

Status: authorized on 2026-08-16; hosted-CI portability successor authorized on
2026-08-17.

Implement accepted FIP-0005 and publish the already-completed project capsule
reset stack through the one-time two-PR bootstrap. Establish reviewable draft
pull requests, hosted Rust verification, squash-only history, and protected
`main` without weakening FIP-0001's exact per-transaction approval boundary.

Before implementation, read FIP-0001 and FIP-0005 in full and the validated
design at
`docs/plans/2026-08-16-pull-request-publication-workflow-design.md`. Preserve
every terminal experiment record and the public repository established by
Experiment 0011.

## Deliverable 1 — Add deterministic PR infrastructure

1. Add a concise pull request template covering scope, GOAL/FIPs, exclusions,
   experiments, reviewed revision, local Nix evidence, hosted checks, and known
   limitations.
2. Add one pull-request workflow targeting `main` that runs `cargo test`,
   formatting check, strict all-target/all-feature Clippy, and conformance on
   Linux.
3. Pin trusted workflow actions, grant only read access to repository contents,
   and provide no secrets, MicroSandbox runtime, VM, or harness execution.
4. Add deterministic evidence for the workflow triggers, permissions, commands,
   and template contract without contacting GitHub.

## Deliverable 2 — Bind the Jujutsu publication lifecycle

1. Update durable workflow, agent, and runbook instructions for one bookmark
   and one draft PR per future GOAL, semantic local checkpoints, exact approval
   before every remote mutation, operator-owned merge, and squash landing.
2. Record the one-time primary-plus-closure bootstrap and archive the completed
   reset GOAL so the primary squash retains it.
3. Document readiness, failure, signature, post-squash tree-equality, bookmark
   cleanup, and local `nix flake check` evidence.
4. Keep FIP-0005 partial until the primary PR, remote protection, and closure
   evidence establish the contract.

## Deliverable 3 — Publish and land the primary bootstrap PR

1. Rehearse Experiment 0013 completely without remote mutation, then run the
   complete standard verification set and repository credential scan.
2. Present the exact primary bookmark, outgoing stack and diff, origin
   destination, title, complete draft body, signatures, and remote baseline for
   operator approval before pushing or creating the PR.
3. Open exactly one draft PR containing the unpublished reset stack and workflow
   infrastructure, observe the hosted Rust check, then separately present any
   readiness mutation for operator approval.
4. Stop for the operator's manual squash merge. After notification, fetch and
   prove `main` has the reviewed primary tree before changing settings.

### Authorized hosted-CI successor

Experiment 0013's sole initial hosted unit rejected because the clean
`ubuntu-24.04` runner lacked the `libcap-ng` development linker file required
by the Linux MicroSandbox dependency. The operator authorized one successor on
2026-08-17: declare Experiment 0014, install only `libcap-ng-dev` without
recommended packages before the existing Rust gates, extend deterministic
workflow evidence, run the complete local verification set, and—after a new
exact approval—push one correction to the existing primary bookmark and
observe one replacement hosted unit. This authority creates no new bookmark,
pull request, readiness, merge, settings, or retry permission.

Experiment 0014 exhausted that authority on 2026-08-17. Its package step
closed the linker defect and hosted tests passed, but the one replacement unit
rejected on a Linux-only strict Clippy error in the PTY observer's `openpty`
winsize argument. No further source correction, push, rerun, or remote mutation
is authorized by this section.

The operator separately authorized Experiment 0015 on 2026-08-17. It may make
only the minimum platform-correct `openpty` winsize-pointer change, exercise the
existing PTY observer fixtures, and run the complete local verification set.
After a new exact approval, it may push one correction to the existing primary
bookmark and observe one automatically created replacement hosted unit. This
authority creates no new workflow, dependency, bookmark, pull request,
readiness, merge, settings, or retry permission.

## Deliverable 4 — Protect main and close bootstrap publication

1. Present exact repository-setting and branch-protection payloads for operator
   approval. Enable squash merges only; require PRs and the named Rust check on
   `main`; block force-push and deletion.
2. Close Experiment 0013 terminally from observed primary-PR and remote-setting
   evidence, mark FIP-0005 conformant, conditionally complete this GOAL, and
   rewrite the handoff.
3. Present the exact closure bookmark, diff, destination, title, and body for a
   new approval; create the small closure PR and wait for hosted Rust CI.
4. Stop for the operator's manual closure squash merge. After notification,
   fetch and verify its `main` destination and tree equality read-only, remove
   only landed goal bookmarks, inspect the final state, then STOP and report.

## Definition of Done

1. Future authorized GOALs have a documented one-bookmark, one-draft-PR,
   operator-merge, squash-history workflow.
2. Hosted Rust verification is deterministic, credential-free, and green;
   local Nix evidence remains mandatory.
3. Every remote mutation receives exact operator review and explicit approval.
4. The primary bootstrap PR lands the reviewed reset and workflow tree on
   `main`; the closure PR lands terminal publication evidence.
5. GitHub permits only squash merging and protects `main` with required PR and
   Rust-check rules while blocking force-push and deletion.
6. Both squash landings pass destination and tree-equality verification; no
   unrelated remote resource changes occur.
7. FIP-0005 is conformant, Experiment 0013 is terminal, the handoff is current,
   and the complete standard verification set is green; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001 through FIP-0005.
2. Use Jujutsu for local history and bookmark publication. Do not use mutating
   Git commands.
3. Each push, PR mutation, merge, or settings mutation is a separate trusted
   host transaction. Present its exact inputs and wait for explicit operator
   approval; approval never carries forward.
4. The operator merges unless they explicitly authorize the agent to merge one
   named PR. Do not enable auto-merge.
5. Do not execute repository-controlled hooks, aliases, pagers, credential
   helpers, or generated shell text during publication.
6. Do not add releases, tags, packages, secrets, deploy keys, collaborators,
   webhooks, deployments, unrelated branches, or other repository settings.
7. Do not add hosted Nix, macOS, VM, harness, runtime, release, or deployment
   jobs.
8. The two-PR bootstrap is a one-time exception. Do not generalize it to future
   GOALs or create a third closure PR.
9. Keep conformance changes with their establishing evidence. Record remote
   results honestly; do not silently retry a failed experiment unit.

## Budget and escalation

1. Engineering ceiling: two hours, excluding waits for operator merge and
   hosted CI.
2. External budget: zero money, zero paid quota, two named bookmarks, two pull
   requests, two operator squash merges, the one separately approved
   Experiment 0014 correction push and replacement hosted unit, and only the
   other explicitly approved remote transactions declared by this GOAL.
3. Stop on any credential anomaly, unsigned outgoing revision, unexpected
   remote change, successor CI failure, stale base, branch or tree mismatch,
   unavailable required protection, need for a push beyond Experiment 0014's
   one correction or any extra PR, need to change an accepted FIP, or request
   for any undeclared GitHub resource.

## Verification

Run before the primary publication transaction and again before the closure
publication transaction:

- focused workflow-contract tests;
- `cargo test`;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --test conformance`;
- `nix flake check`.
