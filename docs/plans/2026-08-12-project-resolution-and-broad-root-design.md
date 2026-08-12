# Project resolution and broad-root safety design

Status: validated by the operator on 2026-08-12.
Governing design: FIP-0001.

## Purpose

Close the project-resolution and broad-root portion of FIP-0001's conformance
gap with deterministic automated evidence. The slice may fix behavior that the
tests disprove, but it does not add commands, change capsule topology, alter
harness behavior, or require a live MicroSandbox experiment.

## Behavior contract

1. An explicit `--project` directory wins. A current directory inside that
   root is preserved; an unrelated current directory becomes the explicit
   root.
2. Without an explicit root, ancestor discovery prefers the nearest Jujutsu
   root, then the nearest Git root, then the nearest recognized devenv root,
   then the nearest flake root.
3. If no marker exists, the current directory is the project root.
4. Selected host paths are canonicalized before identity derivation or mount
   use.
5. A selected home-directory or filesystem root is replaced by Fortlet's
   persistent scratch workspace unless `--allow-broad-mount` is present. This
   protection also applies when `--project` or a discovered marker selects the
   broad root; selecting a broad project is not itself the safety override.
6. Scratch becomes both project root and working directory. Failure to create
   or canonicalize it is fail-closed.
7. Project identity is derived only from the final canonical root.

## Implementation shape

Keep `project::resolve` as the production entry point that reads the process
current directory and `HOME`. Delegate to a parameterized resolver that
receives those values explicitly. Tests use real temporary filesystem trees
and markers without mutating process-global environment or working directory.

The parameterized resolver canonicalizes its inputs, selects a candidate root,
applies broad-root protection to that selected root, establishes the final
working directory, then derives the identity. Existing session-stage error
wrapping remains responsible for naming the project stage and its single
actionable correction.

This is preferred over a filesystem trait because the real filesystem is
cheap and deterministic inside temporary directories. It is preferred over
tests that mutate `HOME` or call `chdir` because those tests would interfere
with parallel execution.

## Test design

Build the evidence in test-first vertical slices:

1. ordinary directory fallback and explicit-root selection;
2. nearest-root selection within each marker class and priority across marker
   classes;
3. nested working-directory preservation and unrelated explicit-root fallback;
4. symlink canonicalization and stable identity;
5. home and filesystem-root scratch substitution, including broad roots chosen
   explicitly or through markers;
6. explicit broad-mount authorization; and
7. fail-closed scratch-creation errors with useful path context.

No live experiment is necessary. The behavior is fully observable through
repository-local filesystem tests and the standard verification gate.

## Documentation and conformance

User-facing documentation will state that `--project $HOME` and `--project /`
still require `--allow-broad-mount`. The conformance map will remove only the
project-resolution and broad-root coverage gap in the same checkpoint as the
code and tests. Capsule topology, concurrency, and terminal semantics remain
explicit gaps.

## Completion boundary

Run focused project tests, the full Rust suite, formatting, strict Clippy,
conformance, and `nix flake check`. Rewrite the handoff, mark the mission
complete, inspect `main..@`, then stop. The engineering ceiling is two hours;
there is no external spend or experiment budget.
