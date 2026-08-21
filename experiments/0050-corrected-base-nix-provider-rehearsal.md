# Experiment 0050: corrected-base public Nix-provider rehearsal

Status: completed — rejected before provider preparation
Design: FIP-0012

## Baseline / Control

Experiment 0049 reached the credential-free base provisioning script with
byte-pinned MicroSandbox runtime and firmware, then exited 127 because Fortlet
validated Debian tar at `/out/usr/bin/tar` rather than `/out/bin/tar`. Its
isolated inventory and root were exactly cleaned up before Nix ran.

Revision `95a87bec` corrects only that path and adds a matching deterministic
assertion. The full post-fix `aarch64-darwin` runbook gate passed: 101 unit
tests and every enabled integration test, formatting, strict Clippy,
conformance, Nix packaging, and shim activation were green. Public source,
schema-2 setup, runtime hashes, Nix 2.35.2 distribution hashes, nixpkgs lock,
target, and harness remain unchanged from Experiment 0049.

## Hypothesis and Production Mechanism

Validating `/out/bin/tar` will allow the otherwise unchanged common-base layer
to publish. Fortlet will then provision Tact, bootstrap Nix in a separate
credential-free capsule, archive and resolve the locked reflow dev shell,
publish its bounded record, and remove every provisioning capsule. A second
prepare will verify the unchanged cache without VM or network contact.

## Declared Scope

Use only `/private/tmp/f50`, with the same exact layout and selectors as
Experiment 0049 under the new root: absolute checkout `r`, home `h`, Fortlet
state `s` and data `d`, MicroSandbox home `m`, and copied runtime files beneath
`runtime/`.

The runtime and firmware MUST match SHA-256
`799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`
and `f6080a4c487aad2fc5e07f09987f337589e9f0d5cf255855078457d97e97f480`.
Clone `https://github.com/muesli/reflow` directly to the absolute checkout and
verify parent `83f63799117108248750298720ed34dd55d5201a`. Add only the frozen
manifest, flake, and lock using nixpkgs
`f13ff45afd1bb73e640eaa08a7066dbed07e3238`.

Run initial plan, first `prepare tact`, identical second prepare, final plan,
raw isolated inventory, and state inspection in order. No model, credential,
harness launch, managed project capsule, project-source edit, checkpoint,
remote write, or publication is permitted. The sole treatment is the reviewed
Fortlet tar-path correction.

## Alternatives

Resuming Experiment 0049 was rejected by experiment policy. Changing package
selection or provider code was rejected because the observed defect was one
exact validation path. The active GOAL permits at most this one fresh successor
after deterministic correction.

## Risks

A later common-tool path or the actual Nix provider may still fail. Any failure,
hash mismatch, undeclared path, credential access, host repository execution,
remaining capsule, or remote mutation rejects the unit. No retry or further
successor is authorized.

## Acceptance Criteria

1. Runtime hashes, exact checkout parent/diff, and redacted unprepared plan
   match Experiment 0049.
2. First prepare prints exactly `tact<TAB>ready`, produces verified base,
   harness, provider store, and resolved record, and leaves isolated inventory
   empty.
3. Second prepare prints the same line without first-use output, VM/network
   contact, or marker rewrite.
4. Final plan reports prepared state and activation names only.
5. Exact inspection precedes cleanup; isolated inventory is empty,
   `/private/tmp/f50` is absent, and global inventory remains unchanged.

Any failed criterion rejects this sole validation unit terminally.

## Budget and Plan

One provider unit, at most 45 minutes, zero model calls, and $0. Stop on the
first failure, hard-invariant anomaly, remote mutation, or FIP change.

## Rehearsal

The focused base-script test and strict Clippy passed after correction, followed
by the complete runbook gate. Experiment 0049 already proved the identical
runtime-copy, absolute checkout, initial-plan, capsule-execution, and cleanup
controls through the exact failing instruction.

## Results

Both runtime hashes, the absolute checkout, exact public parent, and three-file
diff matched their declarations. Initial plan was redacted and unprepared
without creating Fortlet state, data, or MicroSandbox roots.

The sole first prepare stopped while MicroSandbox tried to fetch the common
`node:24-bookworm` base image. DNS lookup for `index.docker.io` failed, so no
base provisioning script, harness provisioning, or schema-2 Nix command ran.
The corrected `/out/bin/tar` validation path was therefore not exercised by
this live unit; its evidence remains the focused test and complete green
runbook gate.

Fortlet did not create a capsule: isolated raw inventory was `[]`. Inspection
found only the failed-pull cache/database roots and empty Fortlet lock/tool
parents. The exact temporary root was removed and proved absent. Global
inventory retained only the unrelated pre-existing stopped Codex capsule. No
model, credential read, managed project capsule, project-source edit,
checkpoint, or remote mutation occurred.

## Terminal Closure

1. Outcome: rejected because first-use base image resolution lost DNS before
   any provider operation.
2. Root cause: the execution environment could not resolve Docker Hub during
   the one permitted preparation attempt. This neither confirms nor falsifies
   the corrected common-base path or FIP-0012 provider implementation.
3. Actual cost: zero capsules, zero Nix commands, zero model calls, and $0.
   Elapsed engineering time was not instrumented, within the 45-minute cap.
4. Cleanup: isolated inventory was empty, the exact temporary root was removed,
   and global inventory was unchanged.
5. Next action: none under this GOAL. Its demonstrated-defect successor budget
   is exhausted; do not resume, retry, or declare another provider/model unit
   without new operator direction.
