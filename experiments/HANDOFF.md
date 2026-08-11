# Session Handoff — Standalone foundation complete

Audience: a fresh agent session. `GOAL.md` is normative and currently closed;
verify this briefing, then obtain a new goal before writing product code.

## Program context: why the next mission exists

1. Fortlet aims to make isolated agent use feel native, not to expose a second
   command surface during ordinary work.
2. MicroSandbox is the selected local runtime and is used through its Rust SDK.
3. The current implementation proves the capsule and terminal path but exposes
   only explicit `doctor` and `run` commands.
4. Product work should now move toward the smallest usable vertical slice,
   beginning with transparent shims and lifecycle observability.

## Current state (verify before relying)

1. `cargo test` verifies project resolution, argument handling, authentication
   helpers, and the conformance map.
2. `cargo clippy --all-targets --all-features -- -D warnings` is the static gate.
3. `nix flake check` builds and tests the reproducible package on the current
   system.
4. `nix run . -- doctor` verifies the local MicroSandbox and credential boundary
   without printing secret values.
5. FIP-0001 is the accepted product architecture; conformance is intentionally
   partial and lists the unimplemented surface.

## Governance conventions that keep working

1. Use Jujutsu checkpoints with intent-based descriptions.
2. Accept architecture-track FIPs before implementation and update conformance
   with the behavior that changes it.
3. Run the complete verification set before real capsule or external actions.
4. Keep successful startup silent and errors stage-specific and actionable.

## Design guidance (advisory)

1. Keep capsule-session orchestration as the product seam; do not add a generic
   runtime trait for a hypothetical backend.
2. Let each harness adapter own its version, provisioning, executable, state,
   credentials, and launch environment.
3. Centralize the product name and state namespace so future naming changes do
   not leak across the implementation.
4. Treat remote execution as a future transport and workspace design, not as a
   flag on the local bind-mount implementation.
5. Keep Nix as a reproducibility and advanced-environment capability rather than
   an end-user prerequisite. A general release needs a supported standalone
   installation path.

## Known local quirks

1. Nix-managed Rust development on macOS needs `SDKROOT` and `LIBRARY_PATH`;
   `nix develop` supplies both through `xcrun`.
2. The package bundles MicroSandbox's matching guest agent and runtime. Updating
   MicroSandbox requires updating fixed-output artifact hashes and the SDK patch.
3. `x86_64-linux` artifacts are declared but have not yet passed native CI.
4. Fortlet uses a new state and capsule namespace; old prototype capsules are
   intentionally not adopted.
5. Direct Cargo builds may download MicroSandbox's pinned guest agent for each
   new build profile. The Nix package replaces that download with fixed-output
   inputs and builds offline.

## What NOT to do (paid-for lessons)

1. Do not restart the backend comparison. Fast cached MicroSandbox launches and
   SDK integration are established; another abstraction layer would not improve
   the current product boundary.
2. Do not mount host credential files, SSH keys, or the 1Password agent to make
   authentication easier.
3. Do not add an in-TUI isolation indicator. Silent success and clear pre-launch
   failure are the chosen UX.
4. Do not preserve opaque configured capsule roots. Promote reusable setup into
   declarative layers or project environment files.
