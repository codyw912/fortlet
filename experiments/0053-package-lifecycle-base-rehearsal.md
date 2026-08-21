# Experiment 0053: package-lifecycle common-base rehearsal

Status: completed — accepted
Design: FIP-0012

## Baseline / Control

Experiment 0052's exact diagnostic proved that raw `ca-certificates` extraction
does not create Debian's generated selection configuration or CA bundle. Its
capsule and root were removed before harness, provider, or model work. The
operator authorized one mechanism-level correction and one base-only rehearsal
while leaving the base-image choice open.

Checkpoint `d7edc93d` reinstalls `ca-certificates` inside the disposable,
credential-free capsule, explicitly invokes Debian's generator, copies the
resulting nonempty bundle into the common layer, retains command-level failure
diagnostics, and bumps the base identity. Its complete `aarch64-darwin` runbook
gate passed: 101 unit tests and every enabled integration test, formatting,
strict Clippy, conformance, Nix packaging, and shim activation were green.

## Hypothesis and Production Mechanism

Using the package lifecycle rather than archive contents will create the
selection configuration and bundle before Fortlet publishes the common layer.
An identical second preparation will verify the immutable layer without VM or
network contact.

## Declared Scope

Use only `/private/tmp/f53`, with empty project `p`, home `h`, Fortlet state `s`
and data `d`, MicroSandbox home `m`, and copied runtime files under `runtime/`.
The runtime and firmware MUST match SHA-256
`799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`
and `f6080a4c487aad2fc5e07f09987f337589e9f0d5cf255855078457d97e97f480`.

Before preparation, seed only Tact's exact immutable marker at
`d/fortlet/tools/tact/0.3.7/.fortlet-tool.json`, bound to Tact 0.3.7 and
`node:24-bookworm`. This MUST make harness preparation a cache hit. The empty
project has no provider declaration. Run initial plan, one network-approved
`prepare tact`, bounded base inspection, identical second prepare, marker
comparison, raw isolated inventory, and exact cleanup in order.

No harness package download, provider operation, credential or identity read,
harness launch, managed project capsule, project edit, remote mutation, or
publication is permitted.

## Alternatives and Risks

A provider rehearsal would conflate base and provider mechanisms and is not
authorized. A bespoke script invocation would bypass production orchestration.
The preseeded marker is the smallest way to exercise the production base path
without spending a harness or provider unit. Any unexpected harness first-use
output, provider work, failure, hash mismatch, credential access, remaining
capsule, or remote mutation rejects the unit. No retry is authorized.

## Acceptance Criteria

1. Runtime hashes, exact preseeded harness marker, and provider-free initial
   plan match their declarations.
2. First prepare prints exactly `tact<TAB>ready` after one base first-use event,
   publishes the `bookworm-5` marker, nonempty CA bundle, and declared tools,
   and leaves isolated inventory empty.
3. Second prepare prints exactly `tact<TAB>ready` without first-use output,
   VM/network contact, or base/harness marker rewrite.
4. Exact bounded inspection precedes cleanup; isolated inventory is empty,
   `/private/tmp/f53` is absent, and global inventory remains unchanged.

Any failed criterion rejects this unit terminally.

## Budget and Plan

One base-only unit, at most 20 minutes, zero provider or model calls, and $0.
Stop on the first failure, hard-invariant anomaly, remote mutation, or FIP
change.

## Results

Both runtime hashes and the exact preseeded Tact marker matched their
declarations. The initial plan reported no provider, no declarations, no
activation, and no unsupported requirements without creating Fortlet state or
MicroSandbox roots.

The first network-approved prepare printed one base first-use event followed by
exactly `tact<TAB>ready`. It emitted no Tact first-use event. The published
`bookworm-5` layer had the exact marker, a nonempty 224,449-byte generated CA
bundle, and nonempty bwrap, Git, curl, tar, and xz files. The production script
had executed every tool successfully inside the guest before publication.

MicroSandbox's macOS bind backing normalized extracted tool modes to 0600, so a
host `test -x` is not evidence of guest executability; the guest validations
are. A read-only inspection command also briefly used zsh's special `path`
variable and thereby hid its own `stat` and `env` commands. Repeating that
inspection with non-special names and absolute tools succeeded; neither
inspection mutated experiment state.

The second identical prepare ran without network approval and printed exactly
`tact<TAB>ready` with no first-use output. Base marker, CA bundle, and Tact
marker hashes, byte sizes, and mtimes were identical before and after. Raw
isolated inventory was `[]` after both preparations. Global inventory retained
only the unrelated pre-existing stopped Codex capsule. The exact temporary root
was removed and proved absent. No harness package download, provider operation,
credential or identity read, model process, project edit, checkpoint, or remote
mutation occurred.

## Terminal Closure

1. Outcome: accepted; the package-lifecycle correction produced and reused the
   complete common base through the production preparation path.
2. Root cause closed: Debian maintainer-generated CA state must be produced by
   the real package lifecycle rather than inferred from raw archive contents.
3. Actual cost: one credential-free base provisioning capsule, automatically
   removed; zero harness downloads, provider commands, model calls, and $0.
   Elapsed engineering time was not instrumented, within the 20-minute cap.
4. Cleanup: both isolated inventories were empty, the exact temporary root was
   removed, and global inventory was unchanged.
5. Next action: stop and revisit the base-image choice with the operator before
   any provider or model authority is amended.
