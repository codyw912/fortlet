# Experiment 0052: CA-corrected public Nix-provider rehearsal

Status: declared
Design: FIP-0012

## Baseline / Control

Experiment 0051 cleared Docker Hub DNS and reached Fortlet's common-base
script, then exited 1 because package extraction could not produce the required
generated CA bundle. Its capsule and exact root were removed. The operator
authorized the deterministic correction and one fresh validation successor.

Checkpoint `dd72b652` constructs the bundle from enabled trusted Debian
certificate entries, rejects traversal, labels base-script failures, sanitizes
their diagnostics, and bumps the immutable layer identity. Its complete
`aarch64-darwin` runbook gate passed: 102 unit tests and every enabled
integration test, formatting, strict Clippy, conformance, Nix packaging, and
shim activation were green. The two stock-Codex tests remain deliberately
ignored, and only the expected incompatible x86_64-linux check was omitted.

## Hypothesis and Production Mechanism

The corrected common base will publish with a nonempty CA bundle. Fortlet will
then provision Tact, bootstrap Nix in a separate credential-free capsule,
resolve the locked reflow dev shell, publish its bounded record, and remove all
provisioning capsules. An identical second prepare will verify the cache without
VM or network contact.

## Declared Scope

Use only `/private/tmp/f52`, with absolute checkout `r`, home `h`, Fortlet state
`s` and data `d`, MicroSandbox home `m`, and copied runtime files under
`runtime/`. The runtime and firmware MUST match SHA-256
`799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`
and `f6080a4c487aad2fc5e07f09987f337589e9f0d5cf255855078457d97e97f480`.

Clone `https://github.com/muesli/reflow` directly to the absolute checkout,
verify parent `83f63799117108248750298720ed34dd55d5201a`, and add only the frozen
manifest, flake, and lock using nixpkgs
`f13ff45afd1bb73e640eaa08a7066dbed07e3238`.

Run initial plan, first network-approved `prepare tact`, identical second
prepare, final plan, raw isolated inventory, and bounded state inspection in
order. No model, credential, harness launch, managed project capsule,
project-source edit, checkpoint, remote write, or publication is permitted.

## Alternatives and Risks

Resuming a terminal unit is forbidden. A host-only provider simulation would
not validate guest construction. The fresh isolated unit is the smallest test
of the corrected production path. Any failure, hash mismatch, undeclared path,
credential access, host repository execution, remaining capsule, or remote
mutation rejects the unit. No correction, retry, or further successor is
authorized.

## Acceptance Criteria

1. Runtime hashes, exact checkout parent/diff, and redacted unprepared plan
   match the prior frozen units.
2. First prepare prints exactly `tact<TAB>ready`, produces verified base,
   harness, provider store, and resolved record, and leaves isolated inventory
   empty.
3. Second prepare prints the same line without first-use output, VM/network
   contact, or marker or record rewrite.
4. Final plan reports prepared state and activation names only.
5. Exact bounded inspection precedes cleanup; isolated inventory is empty,
   `/private/tmp/f52` is absent, and global inventory remains unchanged.

Any failed criterion rejects this fresh unit terminally.

## Budget and Plan

One provider unit, at most 45 minutes, zero model calls, and $0. Stop on the
first failure, hard-invariant anomaly, remote mutation, or FIP change.

## Results

Pending.

## Terminal Closure

Pending.
