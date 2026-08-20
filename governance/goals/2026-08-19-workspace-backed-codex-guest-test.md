# GOAL: Validate workspace-backed Codex guest test execution

Status: complete — merged as PR #12

## Outcome

Verify the corrected workspace-owned Cargo target in Fortlet's ordinary Codex
capsule and retain one useful shim regression test produced through the
packaged Codex path. Keep environmental transients out of the product surface
unless they reproduce under a controlled successor check.

This GOAL changes no product or architecture contract. FIP-0001 through
FIP-0011 were read in full before dispatch. The work used one
`codex-firewall-recovery` bookmark and PR #12 under FIP-0005; the historical
bookmark name does not describe a current product limitation.

## Deliverables

1. Verify merged `main`, clean Jujutsu state, conformance, and the complete
   `docs/RUNBOOK.md` gate through `nix develop`.
2. Freeze and build one immutable aarch64-darwin package, prepare the project
   environment through the public packaged command, and verify an immediate
   immutable cache hit.
3. Run one packaged Codex process from a marker-free outer shell. Require
   authentication, visible streaming, exact
   `CARGO_TARGET_DIR=target/fortlet-guest`, and only the declared integration
   test edit.
4. Independently accept the retained test through `nix develop` and verify the
   exact focused Cargo command in an ordinary owned Codex capsule without a
   model or provider request.
5. Clean up only through packaged public stop/reset, require final absence and
   empty inventories, run the complete local and hosted gates, and leave the
   PR ready for the operator's squash merge.

## Terminal outcome

All deliverables passed. The frozen package prepared the immutable project
environment and verified its cache hit. Packaged Codex 0.147.0 authenticated,
streamed without an Apps warning, reported exact
`CARGO_TARGET_DIR=target/fortlet-guest`, and produced only the intended valid
test diff.

Experiment 0037 then created the ordinary owned Codex capsule through the
packaged `--version` path without provider traffic. The exact focused Cargo
command reported the same target, compiled its dependencies, passed the test
1/1, and exited zero. This controlled successor cleared the earlier
environmental observation; it is not a current Fortlet product limitation.

The exact host test, all 14 tests in its integration file, the complete local
gate, and hosted Rust verification passed. Packaged public stop/reset restored
absence, `no capsules`, and raw inventory `[]` while preserving immutable
layers and persistent harness state. The operator squash-merged PR #12 as
`6336815763aa64222ad43342f8b6e1e9fca9e953`; its tree was verified before
successor work began.

## Excluded scope

No product code, FIP, public contract, additional harness, Apps authorization,
credential mechanism, network or firewall workaround, layer purge, workload
lease, standalone installation, release, or unrelated work was in scope.

## Budget and authority

The completed work used one immutable package, one cold prepare and cache-hit
verification, one Codex model process and prompt, one credential-free Codex
version process, one exact diagnostic Cargo command, one owned capsule at a
time, zero retries, and zero external money.

