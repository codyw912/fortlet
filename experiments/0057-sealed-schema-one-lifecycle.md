# Experiment 0057: Sealed schema-1 prepared lifecycle

Status: terminal — superseded without product lifecycle evidence
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0013

## Baseline / Control

Experiment 0054 accepted the Nix runtime candidate's 267.29-second cold
preparation, below its 600-second ceiling, and five unchanged offline prepares
with a 0.01-second median. Its first prepared absent launch failed before Tact
because the schema-1 layer had owner-only literal modes. Experiments 0055 and
0056 isolated that failure and accepted strict/mirrored publication, trusted
host canonical `0555`/`0444` sealing, and relaxed/private read-only
consumption.

The production correction is signed public implementation tip
`dd75b81cf8c7736ad57864c0af792ce636e17765`, Fortlet 0.1.0,
MicroSandbox 0.6.8, Tact 0.3.7, and runtime contract `fip0013-1` for
`aarch64-linux`. Its OCI archive, runtime seed, and Tact closure retain the
frozen SHA-256 identities
`8acd80db302c20c3cfe626b0cdebf8899ead35ae1e7d482723ba8efdbd101697`,
`508e9c861808dd63ee6967b71ed015e9409004e15b88b4477ef08cdc593ec04d`,
and `7fb40fe4eaad08a5d4ce4dd780c1caccf0624b3ca31e6a0587cf61fd3e3fa3e3`.
Its local OCI manifest digest remains
`sha256:86f93316b38332d732e9092562319eb8af2eb4fb6ad23bf78db4ca357a37e9b0`.

The exact implementation tip passed the complete `aarch64-darwin` runbook
gate: 105 unit tests and every enabled integration test, formatting, strict
all-target/all-feature Clippy, conformance, the Nix package, and shim
activation were green. The two stock-Codex compatibility tests remained
explicitly ignored, and Nix reported only the expected incompatible
`x86_64-linux` omission. Draft PR #15's signed-tip hosted Rust verification
also passed.

## Hypothesis and Production Mechanism

The accepted publication correction should preserve Experiment 0054's cold and
warm preparation results while making the canonical sealed schema-1 layer
usable by a numeric non-root managed capsule. Because every prepared launch
uses the same locally loaded image, immutable project store, Tact closure, and
verified project layer, absent and stopped medians should remain below one
second and running attachment should remain below one second for every sample.

## Declared Scope

Use only exact owned root `/private/tmp/f57`, with workspace `p` at signed tip
`dd75b81cf8c7`, home `h`, Fortlet state `s`, Fortlet data `d`, isolated
MicroSandbox home `m`, synthetic auth file `auth.json`, and evidence beneath
`e`. Build one immutable Fortlet package from that exact workspace. The package
wrapper supplies its pinned MicroSandbox runtime, firmware, OCI archive,
runtime seed, and harness closures. Never have more than one owned capsule.

Preflight MUST prove the signed revision and clean workspace, exact artifact
and manifest identities, absent experiment root before setup, empty isolated
capsule and image inventories, and an unchanged global Fortlet inventory. The
synthetic auth document contains a far-future locally generated non-provider
JWT, fake refresh string, and zero account identifier at mode `0600`. It MUST
never be printed, refreshed, or sent to a provider.

Run one cold `fortlet prepare tact` with
`FORTLET_PREPARE_TIMINGS=1`, public unauthenticated network available only for
the frozen schema-1 recipe inputs, and a 600-second ceiling. It must print
exactly `tact<TAB>ready`, emit the bounded image, runtime-store,
harness-closure, schema-1-layer, and final-verification phases that actually
perform work, publish canonical modes, and leave no capsule.

Snapshot the runtime, Tact, and project-layer marker hashes, sizes, and mtimes,
the selected store and layer disk sizes, and isolated inventory. Then run five
consecutive unchanged prepares without network approval. Record independent
wall time for each. Every call must print the same ready line, emit no first-use
event, create no capsule, and leave every snapshot byte-for-byte unchanged.

Run three fixed lifecycle cycles. Each begins from confirmed absence and runs,
in order:

1. one timed `fortlet run tact -- --version` from absent;
2. one timed identical launch while running;
3. public stop;
4. one timed identical launch from stopped;
5. one second timed identical launch while running; and
6. public stop and reset, followed by absent status and empty isolated
   inventory.

Set `FORTLET_STARTUP_TIMINGS=1` on every launch. All twelve launches use the
same exact package, project, synthetic auth, terminal mode, command, and
ordering. Once during a running state, inspect the stored capsule configuration
and require the project store to be read-only and the schema-1 mount to retain
the accepted relaxed/private `ro,nosuid,nodev` policy with no root-symlink
following. Compare marker snapshots after every cycle.

No schema-2 preparation, provider request, model request, project edit,
checkpoint inside the testbed, real credential read, authenticated input,
public-project mutation, remote mutation, publication outside draft PR #15, or
architecture change is permitted.

## Alternatives

Repeating Experiment 0054's obsolete control was rejected because the control
already terminated on an incompatible common-tool contract and cannot answer
whether the corrected production candidate launches. Reusing Experiment 0054's
state was rejected because its failed unit and owner-only layer are terminal.
A direct MicroSandbox consumer was rejected because Experiment 0056 already
proved that mechanism; this experiment must include Fortlet preparation,
identity, reconciliation, mounts, lifecycle, and latency.

## Risks

Full project-layer verification, host filesystem cache state, package build
load, or MicroSandbox startup variance may exceed a latency ceiling. Every
sample is retained. A prompt, auth refresh, provider diagnostic, model contact,
network use after cold preparation, missing timing event, marker rewrite,
store mutation, preparation capsule, nonzero or unexpected Tact output,
ownership anomaly, mount-policy mismatch, remaining capsule, project change,
inventory mismatch, or cleanup failure rejects the candidate.

A failed unit is terminal and is never resumed or replaced. Two failures that
share one unresolved assumption stop the campaign. Cleanup may restore owner
access only on the exact terminal schema-1 layer after evidence is retained.

## Acceptance Criteria

1. Preflight proves the exact signed implementation, package, artifact
   identities, clean public workspace, isolated empty inventories, synthetic
   credential boundary, and unchanged global inventory.
2. Cold preparation succeeds within 600 seconds, emits bounded phase evidence,
   publishes a verified canonical schema-1 layer, and leaves no capsule.
3. All five unchanged prepares succeed without network approval, capsule
   creation, or marker/store mutation. Their median wall time is below one
   second.
4. All twelve launches return Tact 0.3.7 with status zero. Absent, stopped, and
   running medians are each below one second, and every running sample is below
   one second. Slower samples are retained.
5. Prepared launch performs no image pull, artifact download, preparation
   capsule, Nix evaluation, closure import, provider realization, or project
   store write. The project store and project layer remain read-only and
   survive stop and reset unchanged.
6. Every cycle ends absent with empty isolated capsule inventory. Final marker
   snapshots and the workspace tree equal baseline; the exact experiment root
   is removed and absent; global inventory equals preflight.
7. Reject the candidate on any cold, warm, lifecycle, integrity, isolation,
   credential, mount-policy, project, inventory, or cleanup failure. Do not add
   samples, change ordering, alter thresholds, or retry a failed unit.

## Budget and Plan

Budget: 30 minutes after live setup, one cold candidate prepare, five warm
prepares, three fixed lifecycle cycles, twelve Tact version launches, one owned
capsule at a time, zero provider calls, zero model calls, zero real credentials,
zero project edits, $0, and no retry. Stop on the first candidate rejection,
shared-condition failure, hard-invariant anomaly, or time ceiling. After
terminal evidence, use public stop/reset, forget only workspace `exp0057`,
restore cleanup-only owner access if necessary, remove only
`/private/tmp/f57`, and prove absence.

## Rehearsal

Read-only inspection confirmed that the packaged wrapper supplies the pinned
runtime, firmware, and runtime-artifact paths; project and harness cache markers
are explicit ordinary files; public status, stop, reset, list, timing events,
and exact workspace cleanup are already accepted mechanisms. No experiment
root, package, image, store, capsule, credential fixture, or live command was
created during rehearsal.

## Results

Preflight passed against signed implementation
`dd75b81cf8c7736ad57864c0af792ce636e17765` and exact immutable package
`/nix/store/l3a280n1rnsnazchhqhzc359xngl4v4r-fortlet-0.1.0`. The workspace
was clean, artifact identities matched, synthetic auth passed structural and
mode checks without disclosure, isolated inventories were empty, and global
inventory matched baseline.

The local run did not reach schema-1 publication and produced no warm or
prepared-lifecycle sample. It therefore establishes no product acceptance or
rejection for the portable sealing mechanism or FIP-0013 latency requirements.
No provider request, model request, schema-2 preparation, project edit, real
credential read, or remote mutation occurred.

Public stop and reset reported absence. Raw capsule inventory was empty, the
testbed workspace remained clean and was forgotten, the exact experiment root
was removed and proved absent, and final global inventory matched preflight.

## Terminal Closure

1. Outcome: superseded without product lifecycle evidence; warm preparation
   and lifecycle distributions are unavailable.
2. Attribution: the local run stopped before schema-1 publication, so it does
   not attribute behavior to the Nix-built image, sealed-layer permission
   correction, or runtime latency.
3. Actual cost: bounded local setup only, zero provider calls, zero model
   calls, zero real credentials, and $0.
4. Cleanup: public stop/reset reported absence, isolated capsule inventory was
   empty, the exact workspace was clean and forgotten, `/private/tmp/f57` was
   removed, and global inventory was unchanged.
5. Next action: Experiment 0058 is the operator-authorized fresh rerun.
   Schema-2 preparation and provider or model dispatch remain unauthorized.
