# Session Handoff — Project resolution and broad-root safety is complete

Audience: a fresh agent session. `GOAL.md` is normative and complete. No
experiment is active, and Experiments 0001 through 0009 remain terminally
closed.

## Verified result

1. Checkpoint `e764dfc1` keeps `project::resolve` as the production boundary
   while delegating explicit current-directory and home inputs to deterministic
   policy code.
2. Fifteen project tests prove explicit selection, nested working directories,
   all marker variants, class priority, ordinary fallback, symlink
   canonicalization, canonical identity, broad-root substitution and override,
   and fail-closed path errors.
3. Broad-root protection now evaluates the final selected project root. An
   explicit home root or a home root selected by a marker can no longer bypass
   persistent scratch substitution.
4. `--project "$HOME"` and `--project /` remain selection requests, not safety
   overrides. Only `--allow-broad-mount` authorizes the broad root.
5. README and runbook guidance state that contract. FIP-0001 conformance now
   removes only the project-resolution and broad-root coverage gap.

## Verification

The focused project suite passed with fifteen tests. The full suite passed with
25 unit tests, one conformance test, and two native integration tests. Formatting,
strict all-target/all-feature Clippy, the standalone conformance gate, and
`nix flake check` also passed. Nix emitted the existing missing app metadata
warning and omitted incompatible `x86_64-linux`; native Linux verification
remains outstanding.

No harness, MicroSandbox capsule, external experiment, model prompt, paid quota,
or external money was used. Engineering remained within the two-hour ceiling;
numeric elapsed time was not captured and is not backfilled.

## Successor boundary

Do not reopen project-resolution or Codex-specific exit work without new
evidence. The next goal should select another existing FIP-0001 gap. The most
natural bounded candidate is failure-stage evidence: exercise fail-closed
launch failures and their single actionable correction without launching a
real harness. Capsule topology, lifecycle leases, management commands,
standalone installation, and native Linux package verification remain larger
alternatives.

Preserve optional shim activation, selected-root broad-mount protection,
canonical identity, credential isolation, package runtime integrity, and
Jujutsu discipline.
