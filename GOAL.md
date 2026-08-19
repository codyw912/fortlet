# GOAL: Recover firewall-blocked Codex readiness evidence

Status: active — operator accepted 2026-08-19

## Outcome

Correct the attribution error in the prior daily-work closure by proving the
same corrected Codex path after the operator is present to approve the local
MicroSandbox firewall request. First warm the immutable project environment
through packaged, credential-free `fortlet prepare codex`; only after that
public command succeeds may one packaged Codex shim process perform one useful
repository edit and focused guest test.

This GOAL changes no product or architecture contract. The prior DNS failure is
operator-environment evidence, not a Fortlet defect. Do not modify production
code to accommodate it. Read FIP-0001 through FIP-0011 in full before dispatch.

## Deliverable 0 — Freeze the merged recovery baseline

1. Verify fetched `main`, PR #11 landing tree equality, bookmark cleanup, a
   clean Jujutsu working copy, conformance, and the complete `docs/RUNBOOK.md`
   gate through `nix develop`.
2. Archive the completed prior GOAL and replace the handoff with this recovery
   mission without rewriting Experiments 0034 or 0035.
3. Use one `codex-firewall-recovery` bookmark and one draft pull request
   targeting `main` under FIP-0005. The operator remains the sole merge
   authority.

## Deliverable 1 — Establish firewall-ready cold preparation

1. Predeclare one successor experiment that identifies Experiment 0035's DNS
   result as invalid daily-readiness evidence because the operator was absent
   for the host firewall approval.
2. Freeze one immutable aarch64-darwin package from the declaration checkpoint
   and record its exact revision and store path.
3. Require a clean repository, packaged Codex status `absent`, packaged
   `fortlet list` output `no capsules`, and raw packaged MicroSandbox inventory
   `[]` before preparation.
4. With the operator present, invoke that package's public
   `fortlet prepare codex --project /Users/cody/dev/fortlet` exactly once. The
   operator may approve the expected local MicroSandbox firewall prompt, but no
   shell, network, manifest, recipe, or package substitution is allowed.
5. Require exact stdout `codex<TAB>ready` and verify the resulting immutable
   project layer through an immediate second public `prepare` cache hit that
   performs no provisioning or network contact. The cache-hit check is
   verification, not a failed-unit retry.
6. If the first preparation fails after the firewall decision, stop the GOAL.
   Do not retry, remove a layer manually, or spend a provider request.

## Deliverable 2 — Run one corrected ordinary Codex unit

1. After preparation succeeds, record a fresh clean working copy, a marker-free
   outer shell, public Codex absence, `no capsules`, and raw inventory `[]`.
2. Invoke the immutable package-owned `codex` shim once in non-interactive
   `exec --ephemeral --skip-git-repo-check` mode from the real Fortlet project.
3. Send one frozen prompt requiring Codex to report exactly
   `CARGO_TARGET_DIR=target/fortlet-guest`, add only the previously declared
   invalid-project-environment shim regression test to
   `tests/pre_runtime_failures.rs`, run its exact focused Cargo test without an
   environment override, summarize, and exit.
4. Require visible streaming, no Apps or authentication failure, only the
   declared test edit, the exact guest Cargo test passing, and host exit status
   zero. No follow-up, repair, retry, native fallback, second model process, or
   alternate test command is authorized.

## Deliverable 3 — Verify, clean up, and publish

1. Preserve and inspect the exact model diff. Reject any production, VCS, or
   undeclared path change.
2. Run the same focused test independently through `nix develop`, plus any
   deterministic check required to validate the retained test.
3. Observe the owned capsule through packaged list and status, then use only
   packaged public stop/reset. Require final status `absent`, `no capsules`, and
   raw inventory `[]`; preserve immutable layers and persistent harness state.
4. Update experiment index, GOAL, handoff, conformance, and public docs only
   where the recovered evidence changes a claim. Run the complete standard
   gate through `nix develop`.
5. Sign and publish the final `codex-firewall-recovery` tip, require hosted
   `Rust verification`, keep the single PR accurate, and mark it ready. Leave
   squash merge to the operator. After merge, prove exact tree equality and
   remove only this GOAL's bookmark.

## Definition of Done

1. Packaged cold preparation succeeds with the operator-approved firewall path
   and an immediate public cache hit verifies the published project layer.
2. One immutable packaged shim completes the corrected useful Codex edit/test
   unit with streaming output and exact zero exit status.
3. Independent host verification accepts the exact retained test, and no Apps,
   authentication, EOF, PATH, Cargo-target, terminal, or reconciliation failure
   is observed.
4. Public cleanup ends at absence and empty inventories without deleting
   durable inputs.
5. Conformance remains honest, local and hosted gates pass, and one PR is ready
   for operator squash merge.

## Excluded scope

No product code, FIP, public contract, additional harness, Apps authorization,
credential mechanism, firewall configuration change, network workaround,
layer purge, workload lease, standalone installation, release, or unrelated
work is in scope.

## Budget and escalation

Engineering ceiling: 45 minutes. External money is zero. Runtime activity is
limited to one cold public prepare, one immediate cache-hit verification, one
Codex model process and prompt, one owned capsule, and public cleanup. The
operator may approve only the expected local MicroSandbox firewall request.

Acceptance of this GOAL authorizes the `codex-firewall-recovery` bookmark and
one draft PR under FIP-0005. Stop for a preparation failure after the firewall
decision, an unexpected firewall target, a provider/authentication anomaly,
any undeclared mutation, destructive or unowned cleanup, a second model
process, changed scope, external spend, or merge.

## Verification

Run focused checks while working. Run the complete standard verification set
from `docs/RUNBOOK.md` through `nix develop` before dispatch and publication
readiness.
