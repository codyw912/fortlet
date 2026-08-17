# FIP-0005: Goal-scoped pull request publication

Status: Accepted
Recorded: 2026-08-16 from the operator-validated pull request workflow design
Requires: FIP-0001

## Summary

Fortlet publishes each authorized GOAL through one operator-approved GitHub
pull request and lands it as one squash commit. Semantic Jujutsu checkpoints
remain available during development, while protected `main` contains a concise
goal-level history.

## Motivation

Fortlet now has a public remote, but completed work after initial publication
exists only as a local Jujutsu stack. Continuing to accumulate or push that
stack directly would make review boundaries and public history increasingly
unclear. Pull requests should provide review, hosted verification, and a durable
record without granting an agent implicit merge or publication authority.

## Decision

Use one mutable Jujutsu bookmark and one draft pull request per authorized
GOAL. Every push or GitHub mutation remains a separately approved host
transaction. Completed pull requests are squash-merged manually by the
operator by default. Hosted Rust verification and protected-branch rules guard
`main`; Nix verification remains a recorded local gate initially.

## Specification

### Goal and review scope

1. Each authorized GOAL MUST use one descriptive Jujutsu bookmark and one pull
   request targeting `main`.
2. The pull request SHOULD open as a draft after goal authorization and MUST
   remain draft while implementation or an experiment is active.
3. Local work MAY retain multiple semantic Jujutsu checkpoints for review.
4. The pull request title MUST state completed intent and MUST be suitable as
   the eventual squash-commit subject.
5. The pull request description MUST identify the GOAL and governing FIPs,
   included and excluded scope, experiments, exact reviewed revision,
   verification evidence, and known limitations.
6. Bootstrap MUST use one primary pull request combining the unpublished
   project capsule reset stack with the repository artifacts that establish
   this workflow, followed by one closure pull request recording evidence that
   can exist only after the primary merge and protection changes.
7. The two-pull-request bootstrap MUST be a one-time exception. Later GOALs
   MUST use exactly one pull request each.

### Publication authority

1. Before every branch push, pull request creation or update, readiness change,
   merge, or repository-setting mutation, the agent MUST present the exact
   outgoing revisions and diff, destination, complete metadata change,
   verification state, and known failures.
2. Each remote mutation MUST receive explicit operator approval before it
   occurs. Approval MUST NOT carry over to a later mutation.
3. The operator MUST be the default merge authority. The agent MUST NOT merge
   unless the operator explicitly authorizes that specific pull request merge.
4. Publication MUST use trusted host-side Jujutsu and GitHub interfaces and
   MUST NOT execute repository-controlled hooks, aliases, pagers, credential
   helpers, or generated shell text.
5. Publication MUST NOT expose credentials or add agent provenance or local
   setup identifiers to commits, pull requests, or repository metadata.
6. Goal authority MUST NOT imply authority over releases, tags, packages,
   secrets, deploy keys, unrelated branches, repositories, or settings.

### Verification and readiness

1. A checked-in pull request template MUST request concrete scope, governing
   design, experiment, verification, exclusion, and revision evidence.
2. Pull requests targeting `main` MUST run `cargo test`, formatting check,
   strict all-target/all-feature Clippy, and the conformance test in hosted CI.
3. Hosted CI MUST NOT receive repository secrets or start MicroSandbox, a VM,
   or a harness.
4. `nix flake check` MUST remain a local readiness gate and its host and result
   MUST be recorded in the pull request until a later FIP moves it into hosted
   verification.
5. A pull request MUST NOT become ready while its GOAL or experiment is active,
   a required local gate is failing, or hosted Rust verification is failing.

### Merge and protected main

1. GitHub MUST allow squash merging and MUST disable merge commits and rebase
   merging for this repository.
2. `main` MUST require a pull request and the named hosted Rust verification
   check before merge.
3. `main` MUST reject force-push and deletion.
4. Protection MUST be installed after the primary bootstrap pull request puts
   the required workflow on `main` and before the bootstrap closure pull
   request is merged.
5. After squash merge, the fetched `main` tree MUST equal the exact reviewed
   branch-tip tree before the goal bookmark is removed or successor work begins.
6. The squash commit ID MAY differ from every reviewed Jujutsu revision and
   MUST be recorded during landing verification.

### Failure behavior and evidence

1. Push, CI, signature, stale-base, remote-state, or landing-tree failures MUST
   stop publication and MUST NOT be bypassed or silently retried.
2. `main` MUST NOT be force-pushed to repair a publication failure.
3. Automated evidence MUST validate the workflow and template structure without
   credentials, a VM, a harness, or remote mutation.
4. A predeclared experiment MUST verify the primary bootstrap pull request,
   hosted Rust result, operator squash merge, landing tree equality, merge
   settings, protected branch, and absence of unrelated remote mutation.
5. The bootstrap closure pull request MUST carry that terminal experiment
   record and MUST pass hosted Rust verification. After its operator squash
   merge, a read-only landing gate MUST verify its `main` destination and tree
   equality and report the result without requiring another closure commit.

## Consequences

Public history becomes one commit per goal while Jujutsu remains flexible
locally. Draft PRs improve visibility, but exact approval before every update
adds deliberate operator interaction. Initial CI is fast and deterministic but
does not independently verify Nix packaging or native Darwin behavior.

Squash merging changes commit identity, so landing is complete only after tree
equality rather than commit ancestry is verified. Repository protection cannot
be fully established until the primary bootstrap workflow has landed on
`main`. One small closure PR is then necessary to commit evidence of events the
primary PR could not observe; its own landing is the irreducible final external
fact and is verified read-only rather than causing an infinite closure chain.
This is deliberately not a reusable exception to one GOAL per PR.

## Alternatives Considered

1. Preserving all semantic commits on `main` retains checkpoint identity but
   produces noisier public history and makes later stack shaping harder.
2. Opening pull requests only after goal completion reduces publication
   transactions but loses early review visibility.
3. Goal-scoped standing authority for repeated draft updates is smoother but
   weakens FIP-0001's exact per-transaction approval boundary.
4. Full hosted Nix and cross-platform CI would improve independent evidence but
   expands setup, runtime, and supply-chain scope before the basic PR workflow
   is proven.
5. Convention-only branch discipline avoids repository settings but cannot
   prevent accidental direct pushes or unsafe merges.

## Open Questions

1. When hosted Nix verification is worth its setup and execution cost.
2. Whether additional collaborators justify required approving reviews or a
   CODEOWNERS policy.
3. Whether a future trusted publication command should automate the exact
   presentation, approval, push, PR, and landing transaction.
