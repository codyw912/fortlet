# GOAL: Optional transparent harness shims

Status: completed 2026-08-11. The package now preserves the fixed-output
MicroSandbox runtime byte-for-byte, the complete verification set is green,
and Experiment 0002 proved both packaged shims return their pinned Linux guest
versions without native fallback. Interactive shim terminal semantics remain
an explicit conformance gap rather than an exercised claim.

Deliver the smallest daily-use vertical slice of FIP-0001: optional,
package-owned `codex` and `tact` shims that transparently enter Fortlet's
fail-closed capsule-session path while preserving the explicit Fortlet CLI and
an intentional native escape hatch.

Before designing or implementing, read FIP-0001 in full and the validated
design at `docs/plans/2026-08-11-transparent-shim-slice-design.md`.

## Deliverable 1 — Accept the shim contract

Completed 2026-08-11: accepted FIP-0002 and added its initial conformance entry
before implementation.

Record and accept the next numbered FIP extending FIP-0001 with:

1. An immutable, package-owned shim directory separate from the ordinary
   binary directory.
2. Optional activation through explicit `PATH` ordering, including declarative
   Nix and Home Manager use.
3. Invocation-name dispatch for both registered harnesses.
4. `fortlet native <harness> -- <arguments>` as the explicit, recursion-safe
   host escape hatch, with no automatic native fallback.
5. No shell startup-file writes, aliases, functions, `eval`, or interactive
   shell dependency.

Add its initial conformance entry in the same checkpoint. The proposal MUST be
Accepted before implementation begins.

## Deliverable 2 — Implement the optional transparent slice

Completed 2026-08-11: implemented optional packaged `codex` and `tact` shims,
explicit recursion-safe native execution, staged launch failures, focused
coverage, and partial conformance evidence. Live packaged smoke remains in
Deliverable 3.

1. Package `codex` and `tact` launchers in the dedicated shim directory and
   keep harness registration as their source of truth.
2. Route shim invocation through the existing capsule-session transaction,
   preserving arguments, current directory, terminal behavior, signals, and
   exit status.
3. Keep `fortlet doctor` and `fortlet run <harness> -- <arguments>` fully usable
   without shim activation.
4. Implement the explicit native escape hatch without a shell and without
   recursion into Fortlet's own shims.
5. Make failures on the affected launch path identify their stage and one
   actionable next step. Successful wrapper startup remains silent.
6. Add focused automated coverage and update conformance with the code and
   tests that change it.

## Deliverable 3 — Prove the packaged workflow

Completed 2026-08-11: the package integrity check prevents runtime stripping,
all standard gates passed, and accepted Experiment 0002 exercised both shims
through VM creation and guest harness execution.

1. Verify both launchers exist in the packaged immutable shim directory and
   work without shell initialization.
2. Run the complete standard verification set.
3. After the gates are green, predeclare and run one bounded local smoke
   experiment for `codex --version` and `tact --version` through the activated
   packaged shims.
4. Record actual evidence and effort, update the runbook and conformance
   honestly, rewrite the handoff for the successor, then STOP and report.

## Definition of Done

1. With the shim directory first on `PATH`, ordinary `codex` and `tact`
   invocations enter Fortlet and never silently fall back to host execution.
2. Without shim activation, the explicit Fortlet CLI remains fully supported.
3. `fortlet native` deliberately reaches the next matching host executable or
   fails with one actionable correction; it cannot recurse into a Fortlet shim.
4. Shim activation requires no mutation of user shell files and works from a
   minimal non-interactive environment.
5. Automated and local smoke evidence cover both harnesses, argument flow,
   project selection, isolation, and exit status. Claims not exercised remain
   labeled as claims or explicit conformance gaps.
6. `arch/conformance.json` passes its checker and records the pre-existing
   adapter-ownership, failure-UX, and insufficient-coverage gaps that remain.
7. The complete verification set is green; then STOP and report. Lifecycle
   commands, lease implementation, general installation management, standalone
   distribution, remote execution, publication, and new harnesses are not
   authorized by this goal.

## Binding rules

1. Preserve every hard invariant and direction constraint in the operator
   charter and FIP-0001.
2. Do not write or propose edits to shell startup files; activation guidance is
   declarative and user-controlled.
3. Never fall back from an isolated launch to a host harness.
4. Do not mount host credentials, SSH keys, signing agents, or publication
   authority into a capsule.
5. Do not introduce a generic runtime abstraction or broaden this into
   lifecycle, distribution, remote, or publication work.
6. Architecture-track implementation starts only after its proposal is
   Accepted; conformance changes stay with the behavior and tests they describe.
7. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Up to 8 hours of actual engineering effort; this is a ceiling, not a target.
   Stop as soon as the Definition of Done is met and report actual effort.
2. Zero external spend, paid quota, remote mutation, or repository publication.
3. Local Fortlet-owned test capsules and state are allowed under the charter.
4. Stop on any need to weaken an invariant, change FIP-0001, mutate user-owned
   shell configuration, or expand the product boundary.
5. Two consecutive terminal failures sharing an assumption trigger escalation.

## Verification

Run from `nix develop`:

- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- The packaged shim smoke commands declared in the experiment record
