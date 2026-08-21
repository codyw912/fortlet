# Experiment 0054: Nix runtime substrate feasibility

Status: declared
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0012, FIP-0013

## Baseline / Control

The control is accepted public revision
`d496628fd15c811657b539748daf946c45981371`, packaged Fortlet 0.1.0,
MicroSandbox 0.6.8, Tact 0.3.7, common-base identity `bookworm-5`, and
`node:24-bookworm` at locally inspected arm64 manifest digest
`sha256:7e4b2953088599075c288871d109e23bc7a33384b96ca443a7cfb7b5c318b099`.
Experiment 0053 accepted this production package-lifecycle mechanism but did
not measure its cold or lifecycle durations.

The candidate is public revision
`894d306b9f24b342372873d333e6f108709c463b`, packaged Fortlet 0.1.0,
MicroSandbox 0.6.8, Tact 0.3.7, and runtime contract `fip0013-1` for
`aarch64-linux`. Its local OCI manifest digest is
`sha256:86f93316b38332d732e9092562319eb8af2eb4fb6ad23bf78db4ca357a37e9b0`.
The OCI archive, runtime seed, and Tact closure SHA-256 identities are,
respectively, `8acd80db302c20c3cfe626b0cdebf8899ead35ae1e7d482723ba8efdbd101697`,
`508e9c861808dd63ee6967b71ed015e9409004e15b88b4477ef08cdc593ec04d`,
and `7fb40fe4eaad08a5d4ce4dd780c1caccf0624b3ca31e6a0587cf61fd3e3fa3e3`.

Both arms use the same Apple-silicon macOS reference host, the same exact
candidate revision of Fortlet as the public schema-1 project input, fresh
isolated Fortlet and MicroSandbox roots, public unauthenticated network during
cold preparation, Tact `--version`, and a public non-secret synthetic auth and
identity fixture. No real provider or model credential is read.

## Hypothesis and Production Mechanism

Replacing the registry image, Debian package lifecycle, and separately
downloaded mutable tool output with a locally loaded Nix OCI archive, a seeded
per-project ext4 store, and a selected prebuilt Tact closure will make repeated
preparation a marker-only offline operation without moving cold work into
launch. The candidate may pay a larger cold materialization and disk cost, but
one cold preparation will finish within ten minutes; five unchanged prepares
will have a sub-second median; and prepared absent, stopped, and running
launches will retain sub-second medians with every running sample below one
second.

## Declared Scope

Build the exact control and candidate packages. Use one clean public Fortlet
project at candidate revision and one empty no-provider staging project. Each
arm receives new isolated home, state, data, and MicroSandbox roots. Run the
control first and candidate second. Never have more than one owned capsule.

The uninstrumented control's cold phases are isolated through production
preparation calls: preseed the exact Tact marker so the empty project measures
only the common base; remove only that synthetic marker so the next prepare
measures only Tact; then prepare the Fortlet project so the third call measures
only its schema-1 layer. The sum is the control cold total. The candidate runs
one cold Fortlet-project prepare with `FORTLET_PREPARE_TIMINGS=1`, whose fixed
events separately report image, runtime-store, harness-closure,
schema-1-layer, and final-verification durations.

For each completed arm, run five consecutive unchanged prepares without
network approval and compare all prepared marker identities, byte sizes, and
modification times before and after. Then run three fixed cycles: launch Tact
`--version` from absent, launch it while running, stop, launch from stopped,
launch it while running, stop, and reset. Record independent wall time and the
bounded startup events for all twelve launches per arm.

Record archive or image bytes available before preparation, bytes reported by
public downloads where available, materialized base/tool/project/store disk
use, sparse-store apparent and allocated size, capsule inventory, project
status, and exact cleanup. HTTP transfer byte accounting absent from the old
implementation is recorded as unavailable, not inferred. The fixed ordering,
host filesystem cache, and one cold sample per arm are limitations, so cold
comparisons are feasibility evidence rather than a general performance
distribution.

No schema-2 preparation, model request, project edit, checkpoint inside the
testbed, provider credential, real model credential, remote mutation,
publication, or architecture change is permitted.

## Alternatives

Reusing Experiment 0053 was rejected because it lacks cold and lifecycle
timings. A direct MicroSandbox benchmark was rejected because it would omit
Fortlet's preparation, mounts, identity, and capsule reconciliation. Backporting
new instrumentation into the control was rejected because it would no longer
be the accepted existing package. More cold repetitions were rejected because
they would multiply public downloads and schema-1 compilation without being
needed for this bounded adoption screen.

## Risks

Registry drift, firewall handling, CDN cache state, host load, sparse-file
accounting, or fixed control-first ordering can affect the single cold samples.
The exact local control digest prevents tag drift after preflight. A missing
phase event, unexpected network use on a warm prepare, marker rewrite,
remaining preparation capsule, nonzero Tact exit, ownership anomaly, or cleanup
failure is a visible failed unit. A control failure attributable only to its
declared mechanism is retained and does not suppress the candidate; a shared
network or host failure stops the campaign rather than biasing one arm.

The synthetic auth fixture carries no upstream authority. Any attempt to
refresh it, contact a model endpoint, or read the operator's real credential
terminates the campaign immediately.

## Acceptance Criteria

1. The candidate's complete `aarch64-darwin` runbook gate remains green; both
   packages and all public inputs match the declared identities.
2. Each successful cold phase emits or receives one bounded elapsed duration,
   each arm ends with Tact 0.3.7 and the schema-1 project layer verified, and
   the candidate cold total is at most 600 seconds. One sample per arm is a
   feasibility result only.
3. All five candidate repeated prepares succeed offline, create no capsule,
   and preserve every selected marker and store identity, size, and mtime. The
   median wall time must be below one second.
4. All twelve candidate prepared launches return Tact 0.3.7. Absent, stopped,
   and running medians must each be below one second, and every running sample
   must be below one second. Slower samples are retained.
5. Candidate launch performs no image pull, artifact download, preparation
   capsule, Nix evaluation, closure import, provider realization, or project
   store write. The mounted project store is read-only and survives stop and
   reset.
6. Final isolated inventories are empty, the public project tree is unchanged,
   exact experiment roots are removed, and global inventory is unchanged.
7. Reject adoption if the candidate exceeds its cold ceiling or any candidate
   warm, launch, integrity, isolation, credential, or cleanup criterion. A
   failed unit is never resumed or silently replaced. Two failures sharing one
   unresolved assumption terminate the campaign.

## Budget and Plan

Budget: 60 minutes total, one cold control sequence, one cold candidate prepare,
five warm prepares per completed arm, three lifecycle cycles per completed arm,
zero provider calls, zero model calls, zero real credentials, and $0. Stop on
the first candidate rejection, a shared-condition failure, any hard-invariant
anomaly, or the time ceiling. Do not add samples or alter thresholds after the
control result.

## Rehearsal

On 2026-08-21 the candidate's complete `nix flake check` passed at revision
`894d306b9f24b342372873d333e6f108709c463b`: 102 unit tests and every enabled
integration test, formatting, strict all-target/all-feature Clippy,
conformance, the curated package, and shim activation were green. The two
stock-Codex compatibility tests remained explicitly ignored. Nix reported only
the expected incompatible `x86_64-linux` omission.

Read-only inspection verified the candidate's declared manifests and archive
identities and reported packaged byte sizes of 104,432,128 for the OCI archive,
98,195,343 for the runtime seed, and 33,334,073 for the Tact closure. Read-only
control-image inspection verified the exact digest, arm64/Linux platform, and
381.5 MiB reported size. No experiment root, capsule, project store, provider,
harness, or model process was created during rehearsal.

## Results

Pending.

## Terminal Closure

Pending.
