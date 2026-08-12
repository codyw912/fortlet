# GOAL: Prove project resolution and broad-root safety

Status: completed on 2026-08-12.

Checkpoint `e764dfc1` deterministically proves project selection,
canonicalization, identity, broad-root scratch substitution, explicit
overrides, and fail-closed errors. It also fixes explicit and marker-selected
home roots that previously bypassed scratch protection.

Close the project-resolution and broad-root portion of FIP-0001's conformance
gap with deterministic automated evidence. Fix only behavior that the tests
show conflicts with the accepted design. Do not add commands, change capsule
topology, alter harness behavior, or perform a live experiment.

Before implementation, read FIP-0001 in full and the validated design at
`docs/plans/2026-08-12-project-resolution-and-broad-root-design.md`. Preserve
every terminal result from Experiments 0001 through 0009.

## Deliverable 1 — Make resolution policy deterministic to test

Completed at checkpoint `e764dfc1`: `project::resolve` remains the production
boundary and delegates to explicit policy inputs without process-global test
mutation.

1. Preserve `project::resolve` as the production boundary that reads the real
   current directory and `HOME`.
2. Extract the smallest parameterized resolver that accepts the current
   directory and home directory explicitly.
3. Use real temporary directory trees in tests. Do not mutate process-global
   environment, call `chdir`, or introduce a filesystem abstraction.
4. Keep project-stage error wrapping at the session boundary unchanged unless
   a failing contract test proves it insufficient.

## Deliverable 2 — Prove project selection and canonicalization

Completed at checkpoint `e764dfc1`: focused tests prove explicit selection,
working-directory preservation, every marker variant and class priority,
ordinary fallback, symlink canonicalization, and canonical identity.

Use test-first vertical slices to prove:

1. an explicit project root wins;
2. a current directory inside an explicit root is preserved, while an
   unrelated current directory starts at that root;
3. discovery selects the nearest root within each marker class and applies
   class priority in this order: Jujutsu, Git, devenv, flake;
4. an unmarked directory resolves to itself;
5. selected roots and working directories are canonicalized; and
6. identity is stable and derived from the final canonical root.

## Deliverable 3 — Enforce and prove broad-root safety

Completed at checkpoint `e764dfc1`: protection now applies after project-root
selection. Tests cover direct, explicit, and marker-selected home roots,
filesystem root, explicit overrides, scratch identity, and fail-closed path
errors.

1. Apply broad-root protection to the selected project root, not only to the
   original current directory.
2. Replace a selected home directory or filesystem root with the persistent
   Fortlet scratch workspace unless `allow_broad_mount` is true.
3. Apply the same rule when the broad root came from `--project` or marker
   discovery. `--project $HOME` and `--project /` are not implicit overrides.
4. When scratch is selected, use it as both root and working directory and
   derive identity from its canonical path.
5. Prove explicit broad-mount authorization retains the selected broad root
   and corresponding working directory.
6. Prove scratch creation and canonicalization failures are fail-closed and
   include useful path context.

## Deliverable 4 — Document, conform, and close

Completed at checkpoints `e764dfc1` and the terminal closure checkpoint:
README and runbook guidance distinguish `--project` from
`--allow-broad-mount`; conformance removes only the proved gap; the complete
gate passed.

1. Document that explicit broad projects still require
   `--allow-broad-mount`.
2. Update `arch/conformance.json` in the same checkpoint as the code and tests.
   Remove only the project-resolution and broad-root coverage gap; retain
   capsule topology, concurrency, and terminal-semantic gaps.
3. Run the complete verification set, rewrite `experiments/HANDOFF.md`, mark
   this GOAL complete, inspect `main..@`, then STOP and report.

## Definition of Done

1. Deterministic tests cover explicit selection, marker precedence, ordinary
   fallback, nested working directories, canonicalization, and stable identity.
2. Deterministic tests cover home and filesystem-root protection for direct,
   explicit, and marker-selected broad roots, plus the explicit override.
3. Broad-root scratch failure is fail-closed with actionable project-stage
   context.
4. Documentation and conformance describe exactly the behavior proved.
5. The complete verification set is green; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001 constraint.
2. Do not change FIP-0001, add a new architecture mechanism, or expand into
   capsule topology, leases, management commands, environments, or packaging.
3. Do not mutate the operator's actual home, root directory, environment, or
   process working directory in tests.
4. Do not launch Codex, Tact, MicroSandbox capsules, paid services, or external
   experiments.
5. Prefer the smallest production refactor that permits deterministic evidence.
6. Keep conformance changes in the same checkpoint as their code and tests.
7. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: two hours from implementation start.
2. External budget: zero money, zero paid quota, zero model prompts, and no
   live experiment.
3. Stop on any need to change an accepted FIP, weaken broad-root protection,
   mutate outside the project, or expand beyond this mission.

## Verification

Run before claiming completion:

- focused `project` module tests during development;
- `cargo test`;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --test conformance`;
- `nix flake check`.
