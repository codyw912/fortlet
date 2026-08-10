# GOAL: Standalone Fortlet foundation

Status: complete on 2026-08-10. This mission is closed. A fresh session MUST
verify the baseline, then stop and obtain a new `GOAL.md` from the operator
before beginning follow-on product work.

The Rust capsule prototype has been extracted from the personal Nix
configuration into a standalone Fortlet repository. The repository now carries
its own reproducible package, validated product architecture, SPAWN workflow,
and verification commands.

## Deliverable 1 — Instantiate project governance (offline)

Completed: bind SPAWN to Fortlet, record FIP-0001, seed conformance, and define
the standing operator charter without importing obsolete prototype history.

## Deliverable 2 — Import the standalone Rust baseline (offline)

Completed: rename the crate, binary, runtime namespace, package, and
documentation to Fortlet while preserving the validated Codex and Tact launch
behavior.

## Deliverable 3 — Establish reproducible verification (offline)

Completed: provide a standalone flake, Rust conformance test, standard test and
lint commands, and an operational runbook.

## Definition of Done

1. The repository builds and tests independently of `nix-config`.
2. No instantiated template placeholders remain.
3. `arch/conformance.json` passes its checker and honestly lists current gaps.
4. The standard verification set is green.
5. Then STOP and report. Transparent shims, new management commands, additional
   harnesses, publication, remote execution, and GitHub publication were not
   authorized by this goal.

## Binding rules (unchanged ceilings)

1. Do not weaken the credential boundary or mount host signing authority.
2. Do not silently launch a harness outside the expected isolation boundary.
3. Do not introduce a generic backend abstraction before a real second runtime
   provides evidence for the seam.
4. Record claims as claims until a runner-executed command verifies them.

## Budget and escalation

1. Zero external spend and zero remote mutation.
2. Report at deliverable completion.
3. Stop on any need to change the validated product boundary, publish remotely,
   weaken a hard invariant, or overwrite user-owned state.

## Verification

- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
