# Experiment 0011: Public alpha repository

Status: declared
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

Pending. Record the declaration checkpoint, run all gates, advance local
`main`, and inspect the exact outgoing history and destination without network
mutation.

## Results

Pending declared dispatch.

## Terminal Closure

Pending.
