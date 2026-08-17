# Experiment 0016: Bootstrap publication closure

Status: declared
Design: FIP-0001, FIP-0005 including its 2026-08-17 amendment
Charter scope: `local-foundation/v1` plus the active publication GOAL and
explicit operator primary-bootstrap exception

## Baseline / Control

PR #1 merged into `main` at `2026-08-17T18:58:59Z` as validly signed commit
`e0f919f80ed90589735f15ff7779ed229122ab1f`. It has two parents rather than the
required squash shape, but its tree is byte-identical to reviewed signed tip
`e2e4a6d8b24e974add01a720ddaf73c71de7963a`. FIP-0005's dated amendment accepts
only that exact primary-bootstrap landing and prohibits rewriting it.

GitHub currently permits merge commits, rebase merges, and squash merges.
`main` has no declared FIP-0005 protection. The primary goal branch remains at
the reviewed tip; no bookmark cleanup, repository-setting change, protection,
closure branch, closure PR, revert, or history rewrite has occurred.

## Hypothesis and Production Mechanism

Disabling merge and rebase methods while retaining squash, then requiring PRs
and the successful named Rust check on `main`, will make the amended exception
historical rather than repeatable. A small closure PR based on landed `main`
can then publish the exception, terminal successor evidence, settings and
protection reads, final FIP-0005 conformance, and completed GOAL as one squash
commit. Exact post-merge tree equality will close the bootstrap without a third
PR.

## Declared Scope

1. Preserve the exact landed `main`; record the amended exception and terminal
   Experiments 0014 and 0015 without rewriting or reverting history.
2. After separate exact approvals, enable squash merging only and install the
   reviewed FIP-0005 protection on `main`; read both resources back exactly.
3. Verify no unrelated repository resource or ref changed, then prepare the
   closure evidence on the one declared closure bookmark based on fetched
   `main`.
4. Run the complete local verification set, inspect the exact closure stack,
   diff, signatures, and remote baseline, and request separate approval before
   its push and draft PR creation.
5. Observe exactly one initial hosted closure run. After another exact approval
   mark the closure PR ready, then stop for the operator's manual squash merge.
6. After notification, verify the closure PR's `main` destination and exact
   tree equality, then remove only landed goal bookmarks through separately
   approved remote mutations and report the final inventory read-only.

No `main` rewrite, revert, direct push, agent merge, auto-merge, third PR,
additional merge-method exception, release, tag, package, secret, deploy key,
collaborator, webhook, deployment, runtime, VM, harness, hosted Nix, or unrelated
repository setting is in scope.

## Alternatives

1. Rewrite `main` to a synthetic squash. Rejected because FIP-0005 prohibits
   force-push repair and the exact reviewed tree has already landed publicly.
2. Revert and re-land. Rejected because a revert cannot remove the published
   merge ancestry and adds product churn without improving tree integrity.
3. Leave every merge method enabled. Rejected because it would make the
   bootstrap deviation repeatable instead of narrowly historical.
4. Skip the closure PR. Rejected because the amendment, terminal experiment
   evidence, protection state, conformance, and GOAL closure require a durable
   publication path already designed by FIP-0005.

## Risks

1. Repository or branch-protection APIs may reject or normalize the reviewed
   payload. Any mismatch stops before the closure PR; do not weaken the rule.
2. Local evidence descendants currently sit above the merged branch history.
   Stack shaping must base the closure only on fetched `main` and preserve a
   small, reviewable tree diff without losing records.
3. Closure CI, signing, squash landing, or tree equality may fail. Each failure
   settles its unit and stops without retry or bypass.
4. Bookmark cleanup could remove an unlanded ref. Verify PR destination and
   tree equality before proposing each deletion.

## Acceptance Criteria

1. `main` remains at the exact amended primary landing until changed only by
   the operator squash-merging the closure PR.
2. GitHub permits squash merging only; merge commits and rebase merges are
   disabled, and auto-merge remains disabled.
3. `main` requires a pull request and strict successful context
   `Rust verification`, applies to administrators, requires linear history,
   and rejects force-push and deletion with no undeclared restriction.
4. The closure branch contains only reviewed durable evidence and conformance
   changes based on landed `main`; the complete local verification set and its
   one hosted Rust run pass.
5. The operator squash-merges the closure PR, and fetched `main` has its exact
   reviewed tree. Only then may landed goal bookmarks be removed.
6. Final inventory shows no unrelated remote mutation. Any payload, check,
   signature, ref, run-count, landing-method, or tree mismatch rejects the
   affected unit and stops without bypass or silent retry.

## Budget and Plan

Budget: zero money, the two already declared settings resources, one closure
bookmark, one closure PR, one hosted closure run, one operator squash merge,
only separately approved landed-bookmark cleanup, and zero other remote
mutations. Rehearse every request and stack operation without remote effect;
present every transaction exactly; stop on the first mismatch.

## Rehearsal

Pending local recovery preparation.

## Results

Pending declared treatment.

## Terminal Closure

Pending.
