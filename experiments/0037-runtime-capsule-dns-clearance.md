# Experiment 0037: Runtime capsule DNS clearance

Status: declared
Design: FIP-0001, FIP-0002, FIP-0003, FIP-0004, FIP-0005, FIP-0006,
FIP-0007, FIP-0008, FIP-0009, FIP-0010, and FIP-0011
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0036

## Baseline / Control

Experiment 0036's firewall-approved cold preparation and immediate cache hit
succeeded. Its sole Codex process authenticated, streamed, reported exact
`CARGO_TARGET_DIR=target/fortlet-guest`, and produced only the accepted test
diff. The exact guest Cargo command then failed before test execution when the
MicroSandbox build dependency could not resolve GitHub. The same focused host
test, all 14 tests in its integration file, formatting, the complete local
gate, and hosted Rust verification pass on signed revision
`cf2762c4849baa0eb4ef59c58d0f03340ae14541`.

The operator reports that local firewall approval may explain the transient
and requests a rerun before it is treated as public product evidence. The
working copy is clean, and Experiment 0036 ended with packaged Codex absent,
`no capsules`, and raw MicroSandbox inventory `[]`.

## Hypothesis and Production Mechanism

With the host firewall decision settled, a fresh ordinary Fortlet-owned Codex
capsule can resolve GitHub and run the exact retained focused Cargo test under
the packaged project environment. If it does, the earlier DNS failure is a
cleared operator-environment transient, not a current Fortlet limitation.

## Declared Scope

1. Use the immutable Experiment 0036 package
   `/nix/store/is3sfw30rzr58w29q8rnsdcdidwxqgzn-fortlet-0.1.0` from a clean,
   marker-free Herdr pane in the real project.
2. Require packaged Codex status `absent`, packaged global list output
   `no capsules`, and raw packaged MicroSandbox inventory `[]`.
3. Invoke the packaged shim once with `--version`. This credential-free Codex
   process may create and start the ordinary owned capsule but must not make a
   model or provider request.
4. Identify exactly that owned running capsule from raw labeled MicroSandbox
   inventory. Run exactly once inside it, from `/Users/cody/dev/fortlet`:

   ```text
   /bin/bash -lc 'printf "cargo_target=%s\n" "$CARGO_TARGET_DIR"; cargo test --test pre_runtime_failures codex_shim_invalid_project_environment_fails_before_credentials_or_runtime_artifacts'
   ```

   Do not override the capsule environment, Cargo target, DNS, network route,
   image, mounts, package, manifest, or recipe. Ordinary unauthenticated public
   Cargo/GitHub traffic made by the exact command is allowed.
5. Require exact `cargo_target=target/fortlet-guest`, one passing focused test,
   and command exit zero. A DNS or dependency-fetch failure is terminal; do not
   retry or substitute another probe.
6. Inspect repository state, then use only packaged public stop/reset. Require
   final status `absent`, `no capsules`, and raw inventory `[]`.

## Alternatives

1. Run another model prompt. Rejected because the unresolved fact is capsule
   network/test execution, not provider or model behavior.
2. Delete Experiments 0035 and 0036 if this passes. Rejected because terminal
   experiment history remains honest even when a successor clears a transient.
3. Add a Fortlet DNS workaround. Rejected because no repeatable product defect
   is established.

## Risks

1. The firewall or DNS issue may recur. Preserve the first result and stop.
2. A stale or unowned capsule would invalidate attribution. Stop without
   mutation if the empty preflight or unique ownership check fails.
3. Cargo may expose a different project-relevant dependency failure after DNS
   clears. Record it exactly rather than attributing it to the firewall.

## Acceptance Criteria

1. Clean preflight proves a marker-free shell, public absence, and empty raw
   and managed inventories.
2. One packaged `--version` launch returns zero without provider traffic and
   creates exactly one owned running Codex capsule.
3. The sole exact diagnostic reports the corrected Cargo target and passes the
   focused test inside that capsule.
4. Public cleanup restores absence and empty inventories without deleting
   immutable layers or persistent harness state.
5. On acceptance, the PR no longer presents the cleared firewall/DNS event as
   a current product limitation. On rejection, the PR retains it as relevant
   project evidence.

## Budget and Plan

At most 20 minutes, one immutable package, one credential-free Codex version
process, one exact in-capsule Cargo command, one owned capsule, zero retries,
zero model prompts, and zero external money. Preflight, launch, diagnose,
inspect, clean up publicly, and close terminally in that order.

## Rehearsal

The exact focused test and the complete local and hosted gates already pass on
the retained diff. The packaged project environment is an immutable verified
cache hit. This declaration adds no product code and spends no provider call.
