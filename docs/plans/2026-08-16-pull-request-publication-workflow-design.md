# Pull request publication workflow design

Status: validated by the operator on 2026-08-16.
Governing design: FIP-0001 and FIP-0005.

## Purpose

Keep Fortlet's public `main` history concise without giving up useful local
Jujutsu checkpoints or the repository's explicit publication boundary. Each
authorized GOAL should be reviewed through one GitHub pull request and land as
one squash commit. The operator remains the default merge authority.

## Goal and branch lifecycle

An authorized GOAL begins from current `main`. Work uses semantic Jujutsu
checkpoints and one descriptive bookmark such as `goal/workload-leases`. The
exact bookmark is pushed only after the operator reviews and approves the
outgoing revisions, diff, destination, and PR metadata. A draft PR opens after
goal authorization and remains draft while implementation or an experiment is
active.

Each later bookmark push is a new publication transaction with its own exact
review and operator approval. The mutable goal bookmark may be force-updated
when Jujutsu stack shaping requires it; `main` may never be force-pushed. A PR
becomes ready only after goal closure, complete local verification, recorded
Nix evidence, and passing hosted Rust verification.

The first PR is a bootstrap exception. It contains the unpublished project
capsule reset stack plus the workflow infrastructure that governs its review.
Its final tree retains the completed reset GOAL and Experiment 0012 records.

## Review and merge contract

The PR title is the eventual squash-commit subject and states completed intent.
The description records the GOAL and FIPs, included and excluded scope,
experiments, exact reviewed revision, verification evidence, and known gaps.
Intermediate local checkpoints remain available for review on the branch, but
GitHub squash-merges the PR into one goal-level commit on `main`.

The operator merges manually unless they explicitly authorize the agent to
merge one specific PR. Approval to push, open, update, or mark a PR ready does
not authorize merge or any other remote mutation.

After merge, Fortlet fetches the new `main` and proves its tree matches the
reviewed branch tip. Commit IDs are expected to differ after a squash. Only
after tree equality is established may the landed bookmark be removed and a
successor GOAL begin from the new `main`.

## Repository artifacts and checks

`.github/pull_request_template.md` requests evidence for scope, governing
documents, experiments, verification, exclusions, and review revision without
adding generic ceremony. `.github/workflows/verify.yml` runs the Rust runbook
gates for pull requests targeting `main`: tests, formatting, strict Clippy, and
conformance. Hosted verification receives no repository secrets and does not
start MicroSandbox, a VM, or a harness.

`nix flake check` remains a mandatory local gate initially. The PR records its
host system and terminal result. Hosted Nix or cross-platform verification is
a later goal rather than a prerequisite for establishing reviewable history.

After the bootstrap PR lands, GitHub permits squash merging only. `main`
requires a pull request and the named Rust verification check; force-push and
deletion are blocked. Protection is applied after the workflow exists on
`main`, so it never requires an unknown check.

## Publication boundary and failures

Every remote mutation is explicit and bounded. Before it occurs, the agent
presents its exact revisions and diff, source bookmark and origin destination,
complete PR metadata change, verification state, and known failures. The
operator approves that transaction before trusted host tools sign and push the
bookmark or change GitHub state.

Publication uses Jujutsu for the bookmark and GitHub's authenticated CLI or API
for PR and repository metadata. It must not execute repository-controlled
hooks, aliases, pagers, credential helpers, or generated shell text. It grants
no authority over releases, tags, packages, secrets, deploy keys, unrelated
branches, repositories, or settings.

A push failure, CI failure, stale-base conflict, signature failure, unexpected
remote change, or landing tree mismatch stops publication. Failures are
recorded rather than silently retried, bypassed, force-pushed to `main`, or
hidden by changing evidence.

## Verification and rollout

Deterministic evidence validates the checked-in workflow structure and pull
request template. The complete local runbook gate must pass before the first
remote mutation. A predeclared publication experiment records the exact
bootstrap branch, PR metadata, remote state before and after, CI result, manual
squash merge, landing tree equality, repository merge settings, branch
protection, and absence of unrelated remote changes.

The workflow is conformant only after the bootstrap PR and post-merge
protection are verified. Until then FIP-0005 remains unimplemented or partial.

