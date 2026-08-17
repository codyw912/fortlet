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

Completed on 2026-08-16 without remote mutation:

1. Stable change `kqyknyqqkltumxrkozkwsvortpqllvyw` (then commit
   `cb72898240cbe9421949374997f6b82b9ef76e68`) implements the local publication
   contract, pinned hosted workflow, template, conformance map, tests, and
   packaged source boundary. Signing on push is enabled with the SSH backend;
   the Git object IDs are expected to change when the reviewed bookmark is
   first published, while Jujutsu change IDs and trees remain stable.
2. The final exact tree passed 39 unit tests, 20 integration tests, formatting,
   strict all-target/all-feature Clippy, the dedicated conformance test, and
   `nix flake check` on `aarch64-darwin`. Nix emitted the known missing app
   metadata warning and omitted incompatible `x86_64-linux`.
3. Two earlier local flake attempts rejected the tree before dispatch. The
   first found that the package source omitted the two `.github` files compiled
   by the publication test; the second found that it omitted other paths named
   by conformance. The implementation checkpoint was amended to include the
   complete minimal conformance surface, and the full verification set then
   passed. No remote retry or mutation occurred.
4. Read-only GitHub inspection refreshed the baseline: authenticated account
   `codyw912` uses SSH; public repository `codyw912/fortlet` has default branch
   `main` at `e95b0cdb1308f732d3f45db7a85027d45bcd4048`, the expected description,
   and no other branch or tag. It has no pull request, ruleset, branch
   protection, Actions workflow, release, webhook, or deployment. Merge,
   rebase, and squash methods are all currently enabled.
5. The clean pre-qualification outgoing stack contains nine linear checkpoints
   and changes 25 paths by 1,909 insertions and 110 deletions. Current-tree and
   66-revision history scans found no credential material or suspicious
   credential filename. Their sole textual match is the Rust identifier
   conversion `access_token: access_token.into()` in `src/auth.rs`, not a
   credential value. Previously accepted `/Users/cody` evidence paths remain
   unchanged by this treatment.
6. The hosted workflow targets only pull requests to `main`, grants only
   `contents: read`, persists no checkout credential, pins checkout to full SHA
   `3d3c42e5aac5ba805825da76410c181273ba90b1` and Rust `1.97.1`, and contains
   no secret, MicroSandbox, VM, runtime, or harness use. The draft PR title is
   `Add safe capsule reset and establish PR workflow`; its body follows the
   checked-in template and records the exact signed tip after publication.
7. The post-merge repository-settings request is exactly a repository `PATCH`
   enabling only squash merge, disabling merge commits, rebase merge, and
   auto-merge, and using the PR title and body for the squash commit. The
   separately approved `main` protection `PUT` requires strict status context
   `Rust verification` and a pull request with zero required approvals, applies
   to administrators, requires linear history, and sets force-push and deletion
   allowances false. Restrictions, branch locking, creation blocking,
   conversation resolution, and fork syncing remain disabled.
8. Landing rehearsal uses the PR's reported `main` base and merge state, a
   Jujutsu fetch, and an empty diff from the exact reviewed signed tip to fetched
   `main`. The closure PR carries terminal results and final FIP-0005
   conformance; its landing is the finite final read-only observation.

## Results

Pending declared dispatch.

## Terminal Closure

Pending.
