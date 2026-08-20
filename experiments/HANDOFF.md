# Session Handoff — Project-local shim activation active

Audience: a fresh agent session. `GOAL.md` is normative. Verify `main`, the
Jujutsu stack, conformance, and the complete `docs/RUNBOOK.md` gate before
relying on this summary. Read FIP-0001, FIP-0002, and FIP-0005 in full before
implementation or experiment dispatch.

## Verified baseline

PR #12 was squash-merged as
`6336815763aa64222ad43342f8b6e1e9fca9e953`. Its tree matches the reviewed
result. The working copy began as a clean empty child of `main`, and
`main..@` contained no work.

The complete baseline gate passes on aarch64-darwin through `nix develop`: all
88 unit tests and every enabled integration test, formatting, strict
all-target/all-feature Clippy, conformance, and `nix flake check`. Nix emits
only the expected incompatible `x86_64-linux` omission warning. FIP-0001 and
FIP-0002 remain partial with explicit gaps; FIP-0003 through FIP-0011 remain
conformant.

## Current mission

Add one explicit activation output whose `bin` resolves to the existing
package-owned `libexec/fortlet/shims` directory, then use it in a minimal named
`agents` dev shell. The directory-level link is important: native resolution
canonicalizes the `PATH` directory to the existing `FORTLET_SHIM_DIR`, so
`fortlet native` skips it without a second shim identity or resolver change.

The default development shell must remain unchanged. The named shell must not
depend on Bash or fish initialization and must compose with an existing project
environment. Documentation should show both the immediate fish command and the
two outputs a devenv project can add declaratively.

## Next action

Implement the flake outputs and deterministic check, update README, runbook,
and conformance evidence in the same checkpoint, and predeclare Experiment
0038 before any real packaged shim launch. Use bookmark
`project-local-shim-activation` and one draft PR. Do not modify AGD, launch a
model, edit global shell configuration, merge, or expand runtime behavior.

