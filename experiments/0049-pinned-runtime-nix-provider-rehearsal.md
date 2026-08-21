# Experiment 0049: pinned-runtime public Nix-provider rehearsal

Status: declared
Design: FIP-0012

## Baseline / Control

The operator authorized a successor after Experiment 0048 proved that its
absolute checkout and redacted plan controls worked but its empty isolated
`MSB_HOME` hid MicroSandbox's separately installed runtime. No guest or Nix
command ran. Exact capsule and root cleanup completed.

The trusted SDK installation contains an arm64 `msb` runtime with SHA-256
`799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`
and firmware with SHA-256
`f6080a4c487aad2fc5e07f09987f337589e9f0d5cf255855078457d97e97f480`.
The complete code gate remains green at `062a68c69026`. Public source remains
`https://github.com/muesli/reflow` at
`83f63799117108248750298720ed34dd55d5201a`, using nixpkgs
`f13ff45afd1bb73e640eaa08a7066dbed07e3238`.

## Hypothesis and Production Mechanism

Copying the exact verified runtime and firmware into the isolated short root
and selecting them explicitly will preserve Experiment 0048's state isolation
while allowing MicroSandbox capsule creation. Fortlet will then bootstrap
digest-pinned Nix 2.35.2 in a credential-free guest, archive and resolve the
locked public dev shell, publish its bounded record, and remove provisioning
capsules. The identical second prepare will be a verified no-contact cache hit.

## Declared Scope

Use only `/private/tmp/f49` with:

- checkout `/private/tmp/f49/r`;
- home `/private/tmp/f49/h`;
- Fortlet state `/private/tmp/f49/s` and data `/private/tmp/f49/d`;
- MicroSandbox home `/private/tmp/f49/m`;
- verified runtime `/private/tmp/f49/runtime/msb`;
- verified firmware `/private/tmp/f49/runtime/libkrunfw.5.dylib`.

The setup MUST verify the root absent, copy the two trusted installation files,
verify both declared hashes, clone directly to the absolute checkout, verify
the exact public parent, and add only the three frozen schema-2 setup files.
Every Fortlet call uses literal `MSB_PATH` and `MSB_LIBKRUNFW_PATH` selectors.

Run initial plan, first `prepare tact`, identical second prepare, final plan,
raw isolated inventory, and isolated-state inspection in order. No model,
credential, harness launch, managed project capsule, project-source edit,
checkpoint, remote write, or publication is permitted.

## Alternatives

Reusing the personal MicroSandbox state root was rejected because experiment
ownership would become ambiguous. Recording its host path was rejected because
public artifacts exclude personal host-state paths. A verified runtime copy
gives the isolated root the same bytes without publishing their source path.

## Risks

The copied runtime may require additional installation context, or the provider
may expose a guest-user, mount, disk, network, archive, structured-output, or
activation defect. Any failure, hash mismatch, undeclared path, credential
access, host repository execution, remaining experiment capsule, or remote
mutation rejects the unit. No retry is authorized.

## Acceptance Criteria

1. Both runtime hashes, the absolute checkout, exact parent, three-file diff,
   and side-effect-free unprepared plan match declarations.
2. First prepare prints exactly `tact<TAB>ready`, produces a verified isolated
   provider store/record, and leaves no provisioning or managed capsule.
3. Second prepare prints the same line without first-use output, VM/network
   contact, or record rewrite.
4. Final plan reports prepared state and activation names only.
5. Exact inventory and state inspection precede cleanup; isolated inventory is
   empty and `/private/tmp/f49` absent afterward. Global inventory is unchanged.

Any failed criterion rejects the experiment terminally.

## Budget and Plan

One provider unit, at most 45 minutes, zero model calls, and $0. Stop on the
first failure, hard-invariant anomaly, remote mutation, or FIP change.

## Rehearsal

Experiment 0048 proved the absolute-root clone, exact revision/diff check,
redacted initial plan, and capsule cleanup path. This successor adds only the
two byte-identified runtime selectors authorized by the operator.

## Results

Pending.

## Terminal Closure

Pending.
