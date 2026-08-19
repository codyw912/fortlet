# Experiment 0011: Public alpha repository

Status: accepted — terminal
Design: existing accepted FIPs; no product mechanism changes
Charter scope: `local-foundation/v1`

## Baseline / Control

Fortlet's verified history ends at checkpoint `a01c4ead` (`close project
capsule management mission`) with a clean empty working copy above it. Local
bookmark `main` remains at earlier checkpoint `20a050fa`; no Git remote is
configured. GitHub identity `codyw912` is authenticated over SSH and read-only
lookup confirms `codyw912/fortlet` does not exist.

The tree contains dual MIT/Apache-2.0 licenses and describes itself as an early
Rust prototype. Current-tree and full-history pattern scans found no private
key, GitHub token, bearer token, API-key, or credential-value match. The
operator explicitly accepted the retained `/Users/cody` paths in experiment
and design evidence.

## Hypothesis and Production Mechanism

Creating one public GitHub repository and pushing the complete Jujutsu `main`
history over authenticated SSH will establish a durable, reviewable remote
without altering product behavior or hiding negative evidence. A second push
will publish this experiment's honest terminal record so the remote is
self-describing at completion.

## Declared Scope

Destination is exactly `github.com/codyw912/fortlet`, visibility public,
description `Project-scoped isolation for coding-agent CLIs`, default branch
`main`, and remote name `origin`.

Scope permits exactly one repository creation and two Jujutsu pushes of only
bookmark `main`: initial full-history publication, then the terminal closure
checkpoint. No other bookmark, tag, release, issue, package, workflow, deploy
key, secret, webhook, discussion, project, or repository setting may be
created or changed. Local history stays complete and is not rewritten.

## Alternatives

1. Keep building locally. Rejected because it leaves the only copy and review
   surface on one workstation.
2. Publish a squashed alpha snapshot. Rejected because the decision and failed
   experiment history are part of the repository's integrity.
3. Create a private repository first. Rejected because the public-readiness
   audit passed and the operator chose public publication.
4. Add CI in the same dispatch. Rejected because workflows introduce a new
   remote execution and permission surface; this experiment proves publication
   only.

## Risks

1. A secret or unwanted personal detail could become public. Pattern scans,
   license review, exact file review, and operator acceptance of personal paths
   are mandatory before dispatch; any new anomaly terminates publication.
2. The wrong owner, name, visibility, or branch could be created. Exact
   preflight and post-creation API queries must match; mismatch stops before
   push where possible.
3. A push could publish an unintended revision or extra ref. Only an inspected
   Jujutsu `main` bookmark may be pushed, followed by exact remote-tip checks.
4. Authentication or 1Password signing could block. Stop for operator action;
   do not change protocols, signing policy, or credentials as a workaround.
5. Initial publication could pass while the terminal record remains local.
   The declared second push and final equality check are part of acceptance.

## Acceptance Criteria

1. The complete standard verification set passes against the initial outgoing
   tree.
2. Preflight shows the exact authenticated owner, absent destination, clean
   tree, fast-forward `main`, and only the intended complete history.
3. GitHub reports `codyw912/fortlet`, public visibility, default branch `main`,
   and the expected description after creation and initial push.
4. Initial remote `main` equals the inspected local publication revision.
5. After terminal closure, the second push advances only `main` to the closure
   revision and GitHub reports exact local/remote equality.
6. No unplanned remote resource or ref is created.
7. Any verification, destination, visibility, history, or credential anomaly
   rejects the experiment and stops further mutation.

## Budget and Plan

Budget: zero money, one public repository creation, two pushes of `main`, zero
other remote mutations, and at most one hour. Run full gates, checkpoint this
declaration, fast-forward local `main`, inspect outgoing history, create the
repository, push, verify, close the record, checkpoint closure, push once more,
and verify equality. Do not retry a failed mutation under changed scope.

## Rehearsal

Completed on 2026-08-12 without remote mutation:

1. Checkpoint `5028d8cf` records the authorized mission and this experiment
   declaration before repository creation or push.
2. The exact outgoing tree passed 34 unit tests, one conformance test, two
   management-failure integration tests, two native integration tests, ten
   pre-runtime integration tests, formatting, strict all-target/all-feature
   Clippy, and `nix flake check`. Nix emitted the known missing app metadata
   warning and omitted incompatible `x86_64-linux`.
3. GitHub reported authenticated account `codyw912`, SSH Git protocol, and no
   existing `codyw912/fortlet` repository. No credential value was persisted.
4. The clean local stack from existing `main` through declaration was
   inspected in full: 51 linear checkpoints, all authored with the operator's
   GitHub no-reply address. The declaration's qualification checkpoint adds
   only this rehearsal record before `main` is fast-forwarded.
5. Current-tree and full-history secret-pattern scans were clean. The complete
   tracked file list, licenses, Cargo metadata, public README/overview, and all
   accepted `/Users/cody` occurrences were inspected.
6. Local `main` is an ancestor of the qualified publication tip, so moving it
   is a fast-forward. The exact outgoing stack will be inspected once more
   after the qualification checkpoint and bookmark move.

## Results

The initial publication completed on 2026-08-12.

1. GitHub created `https://github.com/codyw912/fortlet` with public visibility,
   the declared description, and SSH origin
   `git@github.com:codyw912/fortlet.git`. Before the first push its default
   branch was correctly empty.
2. Jujutsu pushed only bookmark `main`. Publication signed 55 previously
   unsigned commits, preserving their stable Jujutsu change IDs and content
   while changing their Git object IDs, then advanced the public tip to
   `2742eff2fd238114bde553c29f286fbb2a2e01b6` (`qualify public alpha repository
   publication`).
3. Local `main`, `main@origin`, and GitHub `refs/heads/main` all resolved to
   that exact object. GitHub reported public visibility and default branch
   `main`.
4. The complete remote ref query returned only `refs/heads/main`. GitHub
   reported zero releases, Actions workflows, webhooks, and deployments.
5. No destination, visibility, authentication, history, or credential anomaly
   occurred.

### Historical checkpoint citation map

Earlier records intentionally cite the short object IDs observed when their
experiments ran. First publication added SSH signatures and therefore changed
those Git object IDs without changing their stable Jujutsu change identities.
This map keeps every cited local checkpoint resolvable in the public history:

| Recorded ID | Signed public ID | Description |
| --- | --- | --- |
| `20a050fa` | `6cb246d3` | define optional Nix product boundary |
| `29d42a82` | `c6ccd07a` | implement optional transparent harness shims |
| `98b4b639` | `7493b2aa` | declare packaged transparent shim smoke |
| `85d4766b` | `9f58c75f` | preserve packaged MicroSandbox runtime integrity |
| `ad7fdd0e` | `e79fd005` | add bounded interactive PTY observer |
| `147e956d` | `a2c28b61` | add exact exit command PTY observation |
| `31b4f073` | `c49b2eca` | add human-paced typed exit observation |
| `cc52a2e9` | `df5f85a6` | add idle Ctrl-C key observation |
| `21ca0022` | `290df260` | accept project capsule status and stop contract |
| `bfafd305` | `7f662b8e` | add project capsule status and stop |
| `a01c4ead` | `73f9774f` | close project capsule management mission |
| `5028d8cf` | `07fc2a96` | authorize public alpha repository publication |

## Terminal Closure

1. Outcome: accepted — the complete verified history is public at the exact
   declared repository, and initial local/remote `main` equality was proved.
2. Root cause: an exact public-readiness audit, one Jujutsu bookmark, and
   explicit destination checks kept publication bounded to the intended
   complete history and repository.
3. Actual total cost: zero money, one of one public repository creation, one of
   two declared `main` pushes before this closure checkpoint, zero other remote
   mutations, and within the one-hour ceiling. Numeric elapsed time was not
   captured and is not backfilled.
4. Next action: push this terminal closure as the declared second and final
   `main` update, verify exact equality, then design reset/recovery with the
   operator under a new product GOAL.
