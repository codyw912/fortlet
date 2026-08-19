# GOAL: Make every owned capsule visible

Status: active — operator-accepted 2026-08-19.

The deterministic implementation is complete and Experiment 0033 is accepted.
An immutable package listed one real owned running Tact capsule from outside
its project; the listed path then selected unchanged public status, stop, and
reset through final absence and empty MicroSandbox inventory. Goal closure,
the final complete gate, and PR #10 readiness remain.

## Outcome

Make Fortlet practical to operate across ordinary projects by adding one
read-only global capsule inventory. From any directory, the operator can see
each Fortlet-owned project+harness capsule, its project path, harness, and
lifecycle state, then use the existing explicit project-scoped `status`,
`stop`, and `reset` commands for cleanup.

This is the lifecycle slice immediately before one final Codex work-readiness
goal and subsequent harness expansion. It deliberately gathers real lifecycle
visibility before defining automatic expiry, background-workload leases, or a
bulk removal operation.

Before implementation or experiment dispatch, read FIP-0001, FIP-0003,
FIP-0004, FIP-0005, and FIP-0011 in full.

## Deliverable 0 — Freeze the verified baseline and contract

1. Reverify merged `main`, the clean Jujutsu working copy, conformance, and the
   complete standard verification set through `nix develop` before relying on
   the prior handoff.
2. Accept FIP-0011 before implementation. Do not change its public command,
   ownership, path-presentation, pagination, or failure contract in code first.
3. Use one descriptive bookmark and one draft pull request under FIP-0005.
4. Do not alter project identity, capsule topology, credential policy,
   persistent state, or existing project-scoped management semantics.

## Deliverable 1 — Add global owned-capsule inventory

1. Implement `fortlet list` as the read-only FIP-0011 inventory over every
   page of MicroSandbox capsules carrying Fortlet's managed label.
2. Validate complete ownership and recover the stored project path before
   rendering any row. Ignore unrelated MicroSandbox capsules and fail the
   complete inventory without partial output on malformed or ambiguous managed
   state.
3. Render deterministic, single-line-safe project, harness, and lifecycle
   fields without exposing internal capsule names, project identity hashes,
   credentials, configuration bodies, or host state paths.
4. Keep inventory independent of the current project, provider credentials,
   immutable-layer preparation, persistent harness-state creation, capsule
   start, and terminal attachment.

## Deliverable 2 — Prove the cleanup path

1. Add deterministic evidence for complete pagination, ordering, ownership
   acceptance and rejection, project-path recovery, every lifecycle state,
   safe rendering, empty inventory, unrelated-capsule exclusion, and absence
   of project, credential, layer, and mutation side effects.
2. Predeclare one bounded credential-free local experiment against an immutable
   package. Start at an empty MicroSandbox inventory, create at most one owned
   capsule through a public non-model harness command, observe it globally,
   and clean it up only through explicit packaged `stop` and `reset` commands
   using the listed project path.
3. Require final public absence, empty MicroSandbox inventory, and no repository
   diff from the live unit. One failed unit is terminal and must not be retried
   under the same experiment identity.

## Deliverable 3 — Close and publish once

1. Update conformance, README, overview, runbook, experiment index, GOAL, and
   handoff with the exact implemented surface and honest limitations.
2. Run the complete standard verification set through `nix develop` and record
   the local Nix host and result.
3. Sign the final publishable tip, require hosted `Rust verification`, mark the
   single pull request ready, and leave squash merge to the operator.
4. After operator merge, fetch `main`, prove exact reviewed-tree equality, and
   remove only this goal's local and remote bookmark.

## Definition of Done

1. `fortlet list` reports every valid owned capsule across projects in stable
   order and from outside any project, or reports an empty inventory exactly.
2. Malformed, ambiguous, or spoofed managed records fail closed before any row
   is printed; unrelated MicroSandbox capsules are ignored.
3. Inventory reads no provider credential, resolves no current project,
   prepares no layer, creates no state directory, and performs no lifecycle
   mutation.
4. One packaged runtime unit proves a listed capsule can be managed and removed
   through the existing explicit project-scoped public commands, ending absent
   with empty MicroSandbox inventory.
5. FIP-0011 is conformant, all local and hosted gates pass, and one PR is ready
   for operator squash merge.

## Excluded scope

Do not add global or bulk removal, automatic expiry, workload leases, grace
periods, restart, logs, persistent-state purge, tool-layer cleanup, structured
output, a daemon, another harness, standalone installation, native Linux CI,
remote execution, provider traffic, model prompts, or unrelated product work.
Projects whose stored root no longer exists remain visible but are not made
globally removable by this slice.

## Budget and escalation

Engineering ceiling: two hours. External money and paid quota are zero. The
accepted GOAL authorizes one descriptive bookmark and one draft pull request
targeting `main` under FIP-0005.

Stop for material scope expansion, a changed public contract, destructive or
bulk cleanup, mutation of an unowned capsule, a credential or data-boundary
anomaly, merge, or two failures sharing an unresolved assumption.

## Verification

Run focused deterministic tests while implementing. Before live dispatch and
publication readiness, run the complete standard verification set from
`docs/RUNBOOK.md` inside `nix develop`.
