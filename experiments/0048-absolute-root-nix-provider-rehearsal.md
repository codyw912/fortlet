# Experiment 0048: absolute-root public Nix-provider rehearsal

Status: declared
Design: FIP-0012

## Baseline / Control

The operator authorized one successor on 2026-08-21 after Experiments 0046
and 0047 stopped before provider dispatch. Experiment 0046 used an overlong
isolated home; Experiment 0047 used a relative clone destination from the wrong
directory. Both were exactly cleaned up and neither created a VM or ran Nix.

The implementation remains `062a68c69026`, whose complete `aarch64-darwin`
runbook gate is green. The sole public source remains
`https://github.com/muesli/reflow` at
`83f63799117108248750298720ed34dd55d5201a`, with nixpkgs
`f13ff45afd1bb73e640eaa08a7066dbed07e3238`.

## Hypothesis and Production Mechanism

One canonical absolute short root removes both failed setup mechanisms before
Fortlet runs. Fortlet will bootstrap digest-pinned Nix 2.35.2 inside one
credential-free capsule, archive the locked public checkout into its isolated
project store, resolve the scalar-only dev shell, publish its bounded record,
and clean up provisioning capsules. An identical second prepare will verify
the cached record without VM or network contact.

## Declared Scope

Use only these exact absolute paths:

- root `/private/tmp/f48`;
- checkout `/private/tmp/f48/r`;
- `HOME=/private/tmp/f48/h`;
- `XDG_STATE_HOME=/private/tmp/f48/s`;
- `XDG_DATA_HOME=/private/tmp/f48/d`;
- `MSB_HOME=/private/tmp/f48/m`.

The setup command MUST verify the root is absent, create only the home, clone
directly to the absolute checkout, and verify the exact parent revision before
adding the three Experiment 0046 schema-2 setup files. Run initial plan,
`prepare tact`, identical second prepare, final plan, raw isolated inventory,
and isolated-state inspection in that order.

No model, credential, harness launch, managed project capsule, project-source
edit, local checkpoint, remote write, or publication is permitted. Provider,
target, harness, source, setup files, lock, commands, and network inputs remain
frozen.

## Alternatives

Another relative or variable-derived setup was rejected by the two terminal
records. Reusing either prior checkout was rejected by experiment policy.
Changing provider code was rejected because no provider operation has failed.

## Risks

The isolated `MSB_HOME` may lack required runtime state, or the provider may
expose installer, mount, guest-user, disk, network, archive, structured-output,
or activation defects. Any failure, undeclared path, credential access, host
repository execution, remaining experiment capsule, or remote mutation rejects
the sole unit. No retry or successor is authorized by this record.

## Acceptance Criteria

1. Absolute setup and exact public parent verification pass before Fortlet.
2. Initial plan is unprepared, redacted, and creates no state or data.
3. First prepare prints exactly `tact<TAB>ready`, produces a verified isolated
   store and record, and leaves no provisioning or managed project capsule.
4. Second prepare prints the same line without first-use output, VM or network
   contact, or record rewrite.
5. Final plan reports prepared state and activation names only.
6. Exact inspection precedes cleanup; `/private/tmp/f48` is absent and raw
   isolated inventory is empty after settlement.

Any failed criterion rejects the experiment terminally.

## Budget and Plan

One provider unit, at most 45 minutes, zero model calls, and $0. Stop on the
first failure, hard-invariant anomaly, remote mutation, or need to change
FIP-0012.

## Rehearsal

The unchanged implementation has the complete green gate recorded in
Experiment 0046. The operator explicitly authorized this one successor after
reviewing both pre-provider setup failures. Absolute literal paths replace all
relative checkout placement and long-root choreography.

## Results

Pending.

## Terminal Closure

Pending.
