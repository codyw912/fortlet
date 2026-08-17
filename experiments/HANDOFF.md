# Session Handoff — Publication authority simplification

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001 and FIP-0005 in full before continuing. Experiments 0001 through 0016
are terminally closed; no experiment is active.

## Verified baseline

1. Public repository `codyw912/fortlet` has protected default branch `main`.
   GitHub permits squash merging only, requires pull requests and strict
   `Rust verification`, enforces the rules for administrators, requires linear
   history, and blocks force-push plus deletion.
2. Primary PR #1 landed the exact reviewed product and workflow tree as signed
   two-parent commit `e0f919f80ed90589735f15ff7779ed229122ab1f`.
   FIP-0005's dated amendment accepts only that historical bootstrap landing;
   it authorizes no later merge-method exception.
3. Closure PR #2 passed its sole hosted run `32062492778`, job `95486740889`,
   then the operator squash-merged it as validly signed one-parent commit
   `e9f2591f86ab9c12fff518c34deabb4abd89d6ab`.
4. Fetched `main` is byte-identical to reviewed closure tip
   `1a2c12a8bb425d82bc4f38519250d4552c08b24e`. The bootstrap publication GOAL
   is complete and archived under `governance/goals/`.
5. The primary and closure remote goal branches still exist. No cleanup,
   rewrite, revert, release, tag, package, secret, deployment, settings change,
   or unrelated remote mutation followed the closure merge.

## Active governance correction

The operator found FIP-0005's per-mutation approvals too ceremonial and
accepted a bounded transaction instead. The active GOAL records that decision.
The local amendment and its derived instructions preserve one exact review but
reduce the normal path to two operator interactions: approve one publication
packet, then manually squash-merge when the PR is ready.

The packet names one signed tree, bookmark, destination, draft title and body,
expected initial hosted run, readiness criteria, and optional landed bookmarks
for cleanup. Approval covers one push, one draft PR, observation of that initial
run, declared evidence-only body changes, readiness only after success, and
named cleanup only after operator merge plus exact tree equality. It never
covers merge, changed code or scope, another push or PR, a retry, substantive
metadata, settings, unrelated resources, unexpected remote state, or failure
recovery.

## Product state

Daily-use surfaces are `doctor`, explicit `run`, optional package-owned
`codex` and `tact` shims, explicit `native`, project-scoped `status`, bounded
`stop`, and terminal `reset`. Reusable project+harness capsules, immutable base
and harness layers, brokered ChatGPT credentials, safe project resolution,
broad-root protection, deterministic pre-runtime diagnostics, disposable-root
recovery, and the explicit host publication workflow are established.

FIP-0001 remains partial for adapter ownership of persistent paths and
credential policy; live reconciliation and terminal-attachment failures;
topology, concurrency, and terminal coverage; restart, logs, and tool updates;
explicit workload leases; declarative environments and private overlays;
standalone non-Nix installation; and native `x86_64-linux` package
verification. FIP-0002 retains its recorded automated Codex exit-status gap.

## Current mission boundary

The governance correction is locally complete and the GOAL is conditionally
complete. Its focused four-test publication contract and complete runbook set
pass: 39 unit tests, 21 integration tests, formatting, strict
all-target/all-feature Clippy, conformance, and `nix flake check` on
`aarch64-darwin`. Nix emitted only the known missing app metadata warning and
omitted incompatible `x86_64-linux`.

Checkpoint and sign the reviewable stack, then present one publication packet.
Do not mutate GitHub before that approval. This policy correction governs its
own publication. The operator remains the sole merge authority.
