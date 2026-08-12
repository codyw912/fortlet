# GOAL: Add project capsule status and stop

Status: completed on 2026-08-12.

Checkpoint `bfafd305` implements the accepted FIP-0003 management slice.
Experiment 0010 then accepted the immutable public-CLI path against one exact
owned local capsule and removed all of its disposable state.

## Completed deliverables

1. Launch, status, and stop share one pure capsule descriptor for deterministic
   runtime names and labels.
2. Management validates stored configuration and the managed, schema, project,
   and tool ownership labels before reporting or mutation. Diagnostic Fortlet
   version skew remains manageable; launch retains exact-version
   reconciliation.
3. `fortlet status [harness]` reports all registered harnesses in registry
   order or one selected harness as `harness<TAB>state`.
4. `fortlet stop <harness>` shares Fortlet's capsule lock with launch, performs
   a final ownership check, uses MicroSandbox's bounded stop, preserves reusable
   state, and reports `absent`, `already-stopped`, or `stopped`.
5. Both commands share `run`'s project selection and broad-root rules without
   reading credentials, provisioning layers, preparing capsule state, or
   launching a harness.
6. Deterministic unit and real-CLI failure tests cover the complete command
   decision surface without a VM or credential.
7. Experiment 0010 proved `absent -> running -> stopped -> absent` through the
   immutable public CLI and verified all four ownership labels before exact
   cleanup.
8. README, overview, runbook, and conformance describe the implemented surface;
   FIP-0003 is conformant.

## Verification

The complete standard set passed after terminal experiment closure: 34 unit
tests, one conformance test, two management-failure integration tests, two
native integration tests, ten pre-runtime integration tests, formatting,
strict all-target/all-feature Clippy, and `nix flake check`. Nix emitted the
existing missing app metadata warning and omitted incompatible
`x86_64-linux`; native Linux verification remains outstanding.

## Terminal boundary

No experiment is active. Do not add global inventory, removal, restart, logs,
tool updates, explicit leases, environments, distribution, or another product
surface under this completed mission. Choose and authorize a new GOAL with the
operator before further implementation.
