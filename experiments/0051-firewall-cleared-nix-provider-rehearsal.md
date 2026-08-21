# Experiment 0051: firewall-cleared public Nix-provider rehearsal

Status: declared
Design: FIP-0012

## Baseline / Control

Experiment 0050 passed its exact runtime, checkout, diff, and pure-plan
preflight, then stopped before capsule creation when Docker Hub DNS failed.
The operator identified the execution firewall as the likely cause and
explicitly authorized one quick fresh retry with approval available. Experiment
0050 remains terminal and its root was removed.

Revision `95a87bec` contains the deterministic Debian tar-path correction. Its
focused test and complete `aarch64-darwin` runbook gate passed. All public
source, schema-2 setup, runtime hashes, Nix 2.35.2 distribution hashes, nixpkgs
lock, target, and harness inputs remain frozen from Experiment 0050.

## Hypothesis and Production Mechanism

Running the otherwise identical first preparation with network approval from
the outset will clear the infrastructure-only Docker Hub lookup failure.
Fortlet will publish the corrected common base, provision Tact, bootstrap Nix
inside a separate credential-free capsule, resolve the locked reflow dev shell,
publish its bounded record, and remove every provisioning capsule. An identical
second prepare will verify the cache without VM or network contact.

## Declared Scope

Use only `/private/tmp/f51`, with absolute checkout `r`, home `h`, Fortlet state
`s` and data `d`, MicroSandbox home `m`, and copied runtime files under
`runtime/`. The runtime and firmware MUST match SHA-256
`799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`
and `f6080a4c487aad2fc5e07f09987f337589e9f0d5cf255855078457d97e97f480`.

Clone `https://github.com/muesli/reflow` directly to the absolute checkout,
verify parent `83f63799117108248750298720ed34dd55d5201a`, and add only the frozen
manifest, flake, and lock using nixpkgs
`f13ff45afd1bb73e640eaa08a7066dbed07e3238`.

Run initial plan, first `prepare tact` with network approval, identical second
prepare, final plan, raw isolated inventory, and bounded state inspection in
order. No model, credential, harness launch, managed project capsule,
project-source edit, checkpoint, remote write, or publication is permitted.

## Risks and Stop Conditions

Any failure, hash mismatch, undeclared path, credential access, host repository
execution, remaining capsule, or remote mutation rejects the unit. This is the
only firewall-clearance successor authorized; no retry or further successor is
permitted. Use at most one owned capsule.

## Acceptance Criteria

1. Runtime hashes, exact checkout parent/diff, and redacted unprepared plan
   match Experiment 0050.
2. First prepare prints exactly `tact<TAB>ready`, produces verified base,
   harness, provider store, and resolved record, and leaves isolated inventory
   empty.
3. Second prepare prints the same line without first-use output, VM/network
   contact, or marker rewrite.
4. Final plan reports prepared state and activation names only.
5. Exact bounded inspection precedes cleanup; isolated inventory is empty,
   `/private/tmp/f51` is absent, and global inventory remains unchanged.

Any failed criterion rejects this fresh unit terminally.

## Budget and Plan

One provider unit, at most 45 minutes, zero model calls, and $0. Stop on the
first failure, hard-invariant anomaly, remote mutation, or FIP change.

## Results

Pending.

## Terminal Closure

Pending.
