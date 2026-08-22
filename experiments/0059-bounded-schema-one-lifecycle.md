# Experiment 0059: Bounded schema-1 reuse lifecycle

Status: declared — awaiting operator authorization
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0013

## Baseline / Control

Experiment 0058 used the accepted portable seal and exact immutable package
`/nix/store/l3a280n1rnsnazchhqhzc359xngl4v4r-fortlet-0.1.0`. Cold schema-1
preparation passed in 200.95 seconds, but five unchanged prepares had a
5.14-second median because ordinary reuse recursively validated modes and
rehashed the complete 1.2-GiB output. It stopped before lifecycle sampling and
completed exact cleanup with unchanged global inventory.

The accepted FIP-0006 bounded verified-reuse amendment moves complete tree
validation and digest enforcement to cold publication. Signed implementation
`95107fdbc686058ab4bb2fb11c9c3ffb9fc606a9` advances the internal publication
contract to `fortlet-project-layer-v3` and limits ordinary reuse to the real
canonical root and bounded marker. Its complete local gate passed.

## Hypothesis and Production Mechanism

The new contract should retain Experiment 0058's accepted cold preparation and
canonical publication while removing work proportional to the sealed layer's
entry count and byte size. Five unchanged prepares should therefore have a
sub-second median without capsule creation or mutation. If that hard gate
passes, the same package should retain sub-second prepared absent, stopped, and
running Tact lifecycle medians.

## Frozen Inputs

Use exact immutable package
`/nix/store/6ldx5571fpz2gah2c9c5lydi46w36br2-fortlet-0.1.0` without rebuild.
Its Fortlet executable SHA-256 is
`91a0a7a16438fcc68e0e515e05d0d7172ed717eec4d4f05afbe48a99e56eb1f7`.
It contains Fortlet 0.1.0, MicroSandbox 0.6.8, Tact 0.3.7, runtime contract
`fip0013-1`, and these unchanged runtime artifacts:

- OCI archive SHA-256
  `8acd80db302c20c3cfe626b0cdebf8899ead35ae1e7d482723ba8efdbd101697`;
- runtime seed SHA-256
  `508e9c861808dd63ee6967b71ed015e9409004e15b88b4477ef08cdc593ec04d`;
- Tact closure SHA-256
  `7fb40fe4eaad08a5d4ce4dd780c1caccf0624b3ca31e6a0587cf61fd3e3fa3e3`;
  and
- OCI manifest digest
  `sha256:86f93316b38332d732e9092562319eb8af2eb4fb6ad23bf78db4ca357a37e9b0`.

Use only exact owned root `/private/tmp/f59`, with Jujutsu workspace `p` named
`exp0059` at the signed implementation, home `h`, Fortlet state `s`, Fortlet
data `d`, isolated MicroSandbox home `m`, mode-`0600` synthetic non-provider
auth file `auth.json`, and evidence beneath `e`. Never have more than one owned
capsule.

## Method

1. Preflight MUST prove the implementation signature, clean exact workspace,
   package executable and artifact identities, absent experiment root before
   setup, empty isolated image and capsule inventories, structurally valid
   synthetic auth without disclosure, and a recorded global Fortlet inventory
   baseline.
2. Run one cold `fortlet prepare tact` with
   `FORTLET_PREPARE_TIMINGS=1`, public unauthenticated access only to the frozen
   schema-1 recipe inputs, and a 600-second ceiling. Require exact
   `tact<TAB>ready`, bounded cold phases, no capsule, the new publication
   contract, and canonical `0555`/`0444` output.
3. Snapshot runtime, Tact, and project-layer marker hashes, sizes, mtimes, and
   modes; selected store and layer disk sizes; project status; and isolated
   inventory. Complete cold validation MAY traverse the published tree for
   experiment evidence after preparation.
4. Run five consecutive unchanged prepares without network use. Record every
   wall time and bounded schema-1 phase. Every call MUST print the exact ready
   line, emit no image, runtime-store, or harness first-use event, create no
   capsule, and leave every snapshot unchanged. Their median MUST be below one
   second. Any failure or rejected median ends the campaign before lifecycle.
5. If the warm gate passes, run three fixed lifecycle cycles. Each starts
   absent and performs one timed `fortlet run tact -- --version` from absent,
   one identical timed launch while running, public stop, one timed launch from
   stopped, a second timed launch while running, then public stop/reset, absent
   status, and empty inventory. Set `FORTLET_STARTUP_TIMINGS=1` on all twelve
   launches. Every launch MUST return Tact 0.3.7 with status zero.
6. During one running state, inspect the stored capsule configuration and
   require the project store read-only mount plus schema-1 relaxed/private
   `ro,nosuid,nodev` policy with no root-symlink following. Compare every marker
   snapshot after every cycle.

Absent, stopped, and running launch medians MUST each be below one second, and
every running sample MUST be below one second. Prepared launch may perform no
image pull, artifact download, preparation capsule, Nix evaluation, closure
import, provider realization, project-store write, or schema-1 tree traversal.

No schema-2 preparation, provider or model request, project edit, testbed
checkpoint, real credential read, authenticated input, public-project mutation,
remote mutation, package rebuild, or publication outside draft PR #15 is
permitted.

## Risks and Stop Rules

The complete campaign ends after 30 minutes. A cold failure, warm failure,
launch failure, timing rejection, marker mutation, mount mismatch, credential
anomaly, project change, remaining capsule, inventory mismatch, or cleanup
failure is terminal product evidence. Do not change inputs, ordering,
thresholds, or successful sample counts. Do not retry a terminal product unit.

## Acceptance Criteria

1. Exact preflight passes against fresh isolated state.
2. Cold preparation succeeds within 600 seconds, publishes the new canonical
   schema-1 contract, and leaves no capsule.
3. Five unchanged offline prepares have a sub-second median without descendant
   traversal, mutation, or capsule creation.
4. All twelve launches return Tact 0.3.7; each lifecycle-state median is
   sub-second and every running sample is below one second.
5. Mount policy, no-prepared-work, marker integrity, project integrity,
   per-cycle absence, empty inventory, and exact cleanup all pass.

## Budget and Cleanup

Budget: 30 minutes of measured work, one cold unit, five warm prepares, twelve
fixed launches, one owned capsule at a time, zero provider calls, zero model
calls, zero real credentials, zero project edits, and $0. After terminal
evidence, use public stop/reset, forget only workspace `exp0059`, restore
cleanup-only owner access if necessary, remove only `/private/tmp/f59`, and
prove absence and unchanged global inventory.

## Rehearsal

Read-only inspection confirmed the exact signed implementation and immutable
package, executable, MicroSandbox runtime, and artifact paths are present;
`/private/tmp/f59` is absent; and no `exp0059` workspace exists. No workspace,
experiment state, image, capsule, credential fixture, network request,
provider, harness, model, project mutation, or remote mutation was created.
