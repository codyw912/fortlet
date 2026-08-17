# Experiment 0013: Pull request workflow bootstrap

Status: declared
Design: FIP-0001, FIP-0005
Charter scope: `local-foundation/v1` plus explicit operator publication
authorization on 2026-08-16

## Baseline / Control

Experiment 0011 established the public GitHub repository with only `main` and
no CI, protection, release, package, webhook, deployment, or policy resources.
The project capsule reset and pull-request publication design now exist as an
unpublished local Jujutsu stack. Before dispatch, read-only GitHub inspection
MUST refresh every baseline claim and record exact identifiers.

The treatment will be the exact primary implementation checkpoint produced by
the current GOAL, its `goal/project-capsule-reset` bookmark, and a draft pull
request targeting `main`. The later closure branch carries this experiment's
terminal results but is outside the treatment whose outcome it records.

## Hypothesis and Production Mechanism

Because one reviewed goal bookmark feeds one draft PR, hosted Rust verification,
and an operator squash merge, the reset and workflow tree should land on
`main` as one concise commit without losing reviewable local checkpoints. Once
that workflow exists on `main`, squash-only merge settings and a required PR
plus Rust-check rule should prevent future direct or unverified changes.

## Declared Scope

1. Rehearse the exact publication and settings requests without mutation where
   supported; verify local gates, signatures, remote baseline, outgoing diff,
   and absence of credential material.
2. After exact operator approval, push only bookmark
   `goal/project-capsule-reset` to `origin` and create one draft PR targeting
   `main` with the reviewed title and body.
3. Observe exactly one initial hosted Rust workflow run. A failure rejects the
   unit; do not rerun or push a correction without a successor authorization.
4. After a separate operator approval, update only the reviewed PR evidence and
   readiness state.
5. The operator manually squash-merges the primary PR. After notification,
   fetch and verify its base, squash revision, and byte-identical reviewed tree.
6. After exact operator approval, change only merge-method settings and the
   `main` protection rule declared by FIP-0005. Read back and verify them.
7. Verify no unrelated branches or repository resources changed, then close
   this experiment terminally. A separately approved closure PR publishes the
   terminal record and conformance state.

No agent merge, auto-merge, direct `main` push, third pull request, release,
tag, package, secret, deploy key, collaborator, webhook, deployment, issue,
project, unrelated branch, repository rename, visibility change, runtime
contact, harness launch, model prompt, or credential inspection is in scope.

## Alternatives

1. Push the entire stack directly to `main`. Rejected because it bypasses the
   review and concise-history mechanism being established.
2. Use one pull request and leave post-merge evidence outside the repository.
   Rejected because terminal experiment and conformance records need a durable
   publication path.
3. Add hosted Nix and cross-platform CI now. Rejected because Rust verification
   is enough to prove the initial PR mechanism without expanding build scope.
4. Grant standing permission for draft updates. Rejected because FIP-0001
   requires exact review and approval for each publication transaction.

## Risks

1. The outgoing stack, signatures, PR metadata, or remote baseline may differ
   from rehearsal. Any mismatch prevents mutation.
2. Hosted CI may fail despite local verification. The unit rejects without a
   silent rerun or correction push.
3. Squash merge changes ancestry. Landing therefore relies on exact tree
   equality, not commit reachability from the reviewed branch.
4. GitHub may not expose the required protection semantics for the repository.
   Missing or weaker enforcement rejects the settings unit.
5. A remote command could mutate more than declared. Exact payload review and
   complete before/after inventory make that failure visible.
6. The 1Password signing or SSH approval may block. Pause for operator approval
   and retry only the unchanged transaction; do not change credentials.

## Acceptance Criteria

1. Deterministic workflow tests and the complete standard verification set pass
   before remote mutation.
2. The primary bookmark contains exactly the reviewed local stack, every
   outgoing commit satisfies the repository signing policy, and the draft PR
   targets `main` with exact reviewed metadata.
3. Hosted Rust CI runs without secrets, MicroSandbox, a VM, or a harness and all
   four declared gates pass.
4. Only the operator marks merge authority through the GitHub squash merge; the
   resulting `main` tree equals the reviewed primary tip byte-for-byte.
5. Merge commits and rebase merges are disabled; squash merge remains enabled.
6. `main` requires a pull request and the named Rust check and rejects
   force-push and deletion.
7. Remote inventory shows no unrelated mutation, and the local closure record
   is sufficient to create the one declared closure PR.
8. Every criterion must pass. A failure rejects the affected unit and stops the
   experiment without a silent retry.

## Budget and Plan

Budget: zero money, zero paid quota, one primary bookmark, one primary pull
request, one initial hosted CI run, one operator squash merge, the minimum
reviewed merge-setting and protection mutations, zero runtime or harness
contact, and zero silent retries. Every remote mutation requires its own exact
operator approval. Record commands, payloads, revision IDs, PR URL, CI result,
merge ID, tree comparison, settings reads, inventory, elapsed time, and any
failed unit.

## Rehearsal

Pending. It MUST exercise the complete local and read-only remote path through
PR metadata, CI configuration, merge/settings payloads, landing comparison,
terminal record, and closure-PR preparation without remote mutation.

## Results

Pending declared dispatch.

## Terminal Closure

Pending.

