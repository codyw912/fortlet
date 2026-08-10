# Session Handoff — Bootstrap standalone Fortlet

Audience: the fresh session executing `GOAL.md`. This briefing is descriptive;
the goal and charter win on conflict.

## Program context: why this mission exists

1. A Rust implementation using the MicroSandbox SDK has passed its architecture
   and packaging spike in the operator's personal Nix configuration.
2. Fortlet now needs a portable repository and durable decision workflow before
   product behavior expands.
3. The prototype history informed the decision but is not product history and
   must not be imported as Fortlet experiments.

## Current state (verify before relying)

1. The new repository contains only the instantiated workflow and accepted
   architecture; product conformance is `unimplemented` here.
2. The source baseline is `tools/agent-env-rs` in the operator's `nix-config`
   working copy.
3. The source package previously passed three Rust tests and a reproducible
   `aarch64-darwin` Nix build; re-verify after import rather than trusting this
   claim.

## Governance conventions that keep working

1. Use Jujutsu checkpoints with intent-based descriptions.
2. Keep accepted design separate from mutable implementation conformance.
3. Run the complete verification set before external or capsule actions.

## Design guidance (advisory)

1. Preserve the existing module boundaries during import; rearchitecture is not
   part of bootstrap.
2. Centralize the Fortlet name and state namespace.
3. Keep the standalone flake independent of the personal configuration flake.

## Known local quirks

Nix-managed Rust on macOS needs Apple SDK paths. The standalone development
shell should supply them automatically.

## What NOT to do (paid-for lessons)

1. Do not import the Python prototype or backend bake-off records.
2. Do not add transparent shims or management commands during extraction.
3. Do not publish a GitHub repository under this goal.
