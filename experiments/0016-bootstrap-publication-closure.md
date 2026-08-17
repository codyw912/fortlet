# Experiment 0016: Bootstrap publication closure

Status: accepted
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

Completed locally on 2026-08-17 without remote mutation:

1. Stable change `rkmwtvtxwltpvqulmwmkppxqowvnnmzl` (then commit
   `29045650bfb261efbbf489f21cc26b0480da9885`) appends the
   operator-authorized FIP-0005 amendment and declares this experiment without
   changing product code, the landed tree, or any remote resource.
2. Read-only GitHub inspection refreshed the control. Public repository
   `codyw912/fortlet` has default branch `main` at
   `e0f919f80ed90589735f15ff7779ed229122ab1f`; the primary goal branch remains
   at `e2e4a6d8b24e974add01a720ddaf73c71de7963a`. Squash, merge-commit, and
   rebase methods are enabled; auto-merge and merged-branch deletion are off;
   the squash defaults are `COMMIT_OR_PR_TITLE` and `COMMIT_MESSAGES`.
   `main` protection returns `404 Branch not protected`, and repository
   rulesets are empty.
3. The repository-settings payload changes only six fields: retain squash,
   disable merge commits and rebases, retain disabled auto-merge, and set
   squash title/body defaults to `PR_TITLE` and `PR_BODY`.
4. The `main` protection payload requires strict context `Rust verification`,
   applies requirements to administrators, requires PRs with zero approving
   reviews and no bypasses, requires linear history, blocks force-push and
   deletion, and leaves creation blocking, conversation resolution, branch
   locking, fork syncing, and push restrictions disabled. GitHub identifies
   the successful reviewed-tip check by that exact name from GitHub Actions app
   ID `15368`.
5. Both JSON payloads passed exact local `jq` assertions. GitHub's current
   official REST schemas confirm the merge-setting enums and every protection
   field, including zero required reviewers and personal-repository omission
   of dismissal restrictions. No mutation endpoint was called.
6. The recovery checkpoint passed 39 unit tests, 20 integration tests,
   formatting, strict all-target/all-feature Clippy, the dedicated conformance
   test, and `nix flake check` on `aarch64-darwin`. Nix emitted the known
   missing app metadata warning and omitted incompatible `x86_64-linux`.

This rehearsal authorizes no settings change. Present the exact repository
payload and endpoint for separate approval, read it back after mutation, then
present the exact `main` protection payload as its own transaction.

## Results

1. The operator approved the exact repository PATCH after reviewing its
   endpoint, six-field JSON body, control, local gates, and exclusions. GitHub
   accepted it on 2026-08-17.
2. Independent REST read-back reports squash merging enabled, merge commits
   disabled, rebase merging disabled, auto-merge disabled, and squash defaults
   `PR_TITLE` plus `PR_BODY`. Default branch, visibility, archive state,
   merged-branch deletion, and every unmentioned repository field remained
   outside the transaction.
3. Remote refs remained exactly `main` at
   `e0f919f80ed90589735f15ff7779ed229122ab1f` and the primary goal branch at
   `e2e4a6d8b24e974add01a720ddaf73c71de7963a`. `main` protection still returned
   `404 Branch not protected`, confirming that the second settings resource was
   not mutated implicitly.
4. The operator separately approved the exact `main` protection PUT after
   reviewing its endpoint, complete JSON body, verified check identity, local
   gates, and exclusions. GitHub accepted it on 2026-08-17.
5. Independent REST read-back reports strict required context
   `Rust verification`, automatically bound to GitHub Actions app ID `15368`;
   administrator enforcement; required PRs with zero approving reviews; linear
   history; and force-push plus deletion disabled. Review dismissal, code-owner
   review, last-push approval, signatures, push restrictions, creation
   blocking, conversation resolution, branch locking, and fork syncing remain
   disabled, with no bypass allowance.
6. Squash-only repository settings remained exact. Both branch refs were
   unchanged, and repository rulesets remained empty. No closure branch, PR,
   workflow run, bookmark cleanup, revert, rewrite, or unrelated remote
   mutation occurred.

7. The closure evidence was shaped on fetched `main` without product-code
   changes. The complete standard verification set passed on `aarch64-darwin`:
   39 unit tests, 20 integration tests, formatting, strict
   all-target/all-feature Clippy, conformance, and `nix flake check`. Nix
   emitted only the known missing app metadata warning and omitted the
   incompatible `x86_64-linux` package.

### Acceptance-boundary correction

The original Declared Scope items 5 and 6 and Acceptance Criteria items 4
through 6 incorrectly made this experiment depend on the closure pull
request's own hosted run and merge. No closure bookmark, push, pull request,
hosted run, readiness change, merge, or cleanup had occurred when this defect
was found. FIP-0005 requires the closure pull request to carry a terminal
experiment record and treats its own merge plus tree equality as a final
read-only landing gate. Therefore this experiment's terminal unit ends with
the verified primary landing, exact repository controls, unchanged remote
inventory, and locally qualified closure evidence. Closure publication remains
governed rollout under the GOAL and FIP-0005; it is not another experimental
dispatch and cannot reopen this record.

## Terminal Closure

Accepted on 2026-08-17. PR #1's exact reviewed tree is preserved under the
operator-authorized historical exception, while squash-only repository
settings and protected `main` prevent that merge method from recurring. The
two separately approved settings mutations matched their reviewed payloads,
changed no refs or unrelated resources, and the closure evidence passes the
complete local verification set. The remaining closure push, draft PR, hosted
check, readiness change, operator squash merge, landing comparison, and landed
bookmark cleanup each retain their separate FIP-0005 approval or read-only
boundary.
