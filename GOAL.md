# GOAL: Bootstrap standalone Fortlet repository

You are a fresh session. Read this file, then `experiments/HANDOFF.md`, then
`governance/CHARTER.md`, before writing code. Verify claimed state before
relying on it.

The validated Rust capsule prototype currently lives in the operator's
`nix-config` repository. This mission establishes Fortlet's standalone history,
imports that baseline under the Fortlet name, and makes its package and workflow
self-contained.

## Deliverable 1 — Instantiate project governance (offline)

Bind SPAWN to Fortlet, record FIP-0001 honestly as an already-approved design,
seed conformance, and define the standing operator charter. Do not import old
Python or backend-evaluation experiment records.

## Deliverable 2 — Import the standalone Rust baseline (offline)

Import the Rust implementation and reproducible package. Rename the crate,
binary, runtime namespace, package, and documentation to Fortlet while
preserving validated Codex and Tact behavior.

## Deliverable 3 — Establish reproducible verification (offline)

Add a standalone flake, a Rust conformance checker, and the commands documented
in `docs/RUNBOOK.md`. Verify the package independently of `nix-config`.

## Definition of Done

1. The repository builds and tests independently of `nix-config`.
2. No instantiated template placeholders remain.
3. `arch/conformance.json` passes its checker and honestly lists current gaps.
4. The standard verification set is green.
5. Then STOP and report. Transparent shims, new management commands, additional
   harnesses, publication, remote execution, and GitHub publication are not
   authorized under this goal.

## Binding rules

1. Do not weaken the credential boundary or mount host signing authority.
2. Do not silently launch a harness outside the expected isolation boundary.
3. Do not introduce a generic backend abstraction before a real second runtime
   provides evidence for the seam.
4. Record claims as claims until a runner-executed command verifies them.

## Budget and escalation

1. Eight hours of engineering effort; zero external spend or remote mutation.
2. Report at every deliverable completion.
3. Stop on any need to change the validated product boundary, publish remotely,
   weaken a hard invariant, or overwrite user-owned state.

## Verification

- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
