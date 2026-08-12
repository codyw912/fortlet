# Session Handoff — Project resolution and broad-root safety

Audience: a fresh agent session. `GOAL.md` is normative and active. The current
mission is repository-local and governed by FIP-0001. It requires no live
experiment or harness launch.

## Verified baseline

1. The standard Rust, formatting, strict Clippy, conformance, Nix flake, and
   doctor gates passed at closure checkpoint `45e4ea51`.
2. The working copy was clean after that checkpoint.
3. Experiments 0001 through 0009 are terminally closed. Codex-specific
   automated exit work is exhausted and must not be resumed under this goal.
4. `src/project.rs` currently resolves explicit roots, then searches ancestor
   markers in Jujutsu, Git, devenv, and flake order. It has only one direct
   unit test, for identity stability.
5. Current broad-root protection checks only whether the original current
   directory equals home or `/`, and explicit project selection returns before
   that check. The accepted hard invariant applies to the selected mount root,
   so this is an evidence and likely behavior gap.

## Mission guidance

Read FIP-0001 and
`docs/plans/2026-08-12-project-resolution-and-broad-root-design.md` in full
before implementation. Keep the public production resolver, but parameterize
its policy inputs so tests never change global `HOME` or the process working
directory.

Use real temporary directory trees. Apply broad-root protection after project
selection and before identity derivation. An explicit broad project is not the
override; only `--allow-broad-mount` is. Keep session-stage errors and all
unrelated runtime behavior stable unless a contract test proves a change is
required.

## What not to do

1. Do not resume Experiments 0003 through 0009 or launch a real harness.
2. Do not add a filesystem trait, management command, lease model, environment
   mechanism, package path, or new FIP.
3. Do not mutate the real home directory, filesystem root, process environment,
   or process working directory in tests.
4. Do not claim capsule topology, concurrency, or terminal conformance from
   project-resolution tests.

## Completion boundary

Update code, tests, user-facing broad-root documentation, and conformance in
the same reviewable checkpoint. Run the full gate in `docs/RUNBOOK.md`, rewrite
this handoff for the successor, mark `GOAL.md` complete, inspect `main..@`, then
stop. The mission ceiling is two engineering hours with no external budget.
