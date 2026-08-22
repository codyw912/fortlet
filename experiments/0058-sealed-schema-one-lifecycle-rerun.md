# Experiment 0058: Sealed schema-1 lifecycle rerun

Status: terminal — rejected on repeated-preparation latency
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0013

## Baseline / Control

Experiment 0057 used signed implementation
`dd75b81cf8c7736ad57864c0af792ce636e17765` and exact immutable package
`/nix/store/l3a280n1rnsnazchhqhzc359xngl4v4r-fortlet-0.1.0`, but produced no
schema-1 publication or prepared-lifecycle evidence. It left no capsule, and
exact cleanup restored the baseline. The operator authorized this fresh rerun
under the same frozen product conditions.

The package retains Fortlet 0.1.0, MicroSandbox 0.6.8, Tact 0.3.7, runtime
contract `fip0013-1`, OCI archive SHA-256
`8acd80db302c20c3cfe626b0cdebf8899ead35ae1e7d482723ba8efdbd101697`,
runtime seed SHA-256
`508e9c861808dd63ee6967b71ed015e9409004e15b88b4477ef08cdc593ec04d`,
Tact closure SHA-256
`7fb40fe4eaad08a5d4ce4dd780c1caccf0624b3ca31e6a0587cf61fd3e3fa3e3`,
and OCI manifest digest
`sha256:86f93316b38332d732e9092562319eb8af2eb4fb6ad23bf78db4ca357a37e9b0`.

## Hypothesis and Production Mechanism

The accepted strict/mirrored publisher, package-owned finalizer, trusted
canonical host seal, and relaxed/private read-only consumer should make the
schema-1 layer usable by the numeric non-root managed capsule without changing
Experiment 0054's accepted preparation and latency profile.

## Declared Scope

Use only exact owned root `/private/tmp/f58`, with workspace `p` at signed tip
`dd75b81cf8c7`, home `h`, Fortlet state `s`, Fortlet data `d`, isolated
MicroSandbox home `m`, synthetic auth file `auth.json`, and evidence beneath
`e`. Use the exact existing immutable package path above; do not rebuild it.
Never have more than one owned capsule.

Preflight MUST prove the signature and clean workspace, exact package and
artifact identities, absent experiment root before setup, empty isolated
inventories, mode-`0600` synthetic non-provider auth, and unchanged global
Fortlet inventory.

Run one cold `fortlet prepare tact` with `FORTLET_PREPARE_TIMINGS=1`, public
unauthenticated access only to the frozen schema-1 recipe inputs, and a
600-second ceiling. The measured unit must print exactly `tact<TAB>ready`, emit
the bounded cold phases, publish canonical modes, and leave no capsule.

Snapshot runtime, Tact, and project-layer marker hashes, sizes, and mtimes,
selected store and layer disk sizes, and isolated inventory. Run five
consecutive unchanged prepares without network use. Record each wall time;
every call must print the same ready line, emit no first-use event, create no
capsule, and leave the snapshot unchanged. Their median must be below one
second.

Run three fixed lifecycle cycles. Each starts absent and performs one timed
`fortlet run tact -- --version` from absent, one identical timed launch while
running, public stop, one timed launch from stopped, a second timed launch while
running, then public stop/reset, absent status, and empty inventory. Set
`FORTLET_STARTUP_TIMINGS=1` on all twelve launches. Every launch must return
Tact 0.3.7 with status zero. Absent, stopped, and running medians must each be
below one second, and every running sample must be below one second.

During one running state, inspect the stored capsule configuration and require
the project store read-only mount plus the schema-1 relaxed/private
`ro,nosuid,nodev` policy with no root-symlink following. Compare marker
snapshots after every cycle. Prepared launch may perform no image pull,
artifact download, preparation capsule, Nix evaluation, closure import,
provider realization, or project-store write.

No schema-2 preparation, provider or model request, project edit, checkpoint
inside the testbed, real credential read, authenticated input, public-project
mutation, remote mutation, package rebuild, or publication outside draft PR
#15 is permitted.

## Risks and Stop Rules

The complete campaign ends after 30 minutes. A cold failure, warm failure,
launch failure, timing rejection, marker mutation, mount mismatch, credential
anomaly, project change, remaining capsule, inventory mismatch, or cleanup
failure is terminal product evidence. Do not change inputs, ordering,
thresholds, or successful sample counts.

## Acceptance Criteria

1. Exact preflight passes against fresh isolated state.
2. Cold preparation succeeds within 600 seconds, publishes a verified
   canonical schema-1 layer, and leaves no capsule.
3. Five unchanged offline prepares have a sub-second median without mutation
   or capsule creation.
4. All twelve launches return Tact 0.3.7; each state median is sub-second and
   every running sample is below one second.
5. Mount policy, no-prepared-work, marker integrity, project integrity,
   per-cycle absence, empty inventory, and exact cleanup all pass.

## Budget and Cleanup

Budget: 30 minutes of measured work, one cold unit, five warm prepares, twelve
fixed launches, one owned capsule at a time, zero provider calls, zero model
calls, zero real credentials, zero project edits, and $0. After terminal
evidence, use public stop/reset, forget only workspace `exp0058`, restore
cleanup-only owner access if necessary, remove only `/private/tmp/f58`, and
prove absence and unchanged global inventory.

## Rehearsal

Read-only inspection confirmed that the exact Experiment 0057 package and its
MicroSandbox executable remain present, the signed public implementation is
unchanged, and `/private/tmp/f58` is absent. No workspace, experiment state,
image, capsule, credential fixture, network request, provider, harness, model,
project mutation, or remote mutation was created during rehearsal.

## Results

Preflight passed against signed implementation
`dd75b81cf8c7736ad57864c0af792ce636e17765` and exact immutable package
`/nix/store/l3a280n1rnsnazchhqhzc359xngl4v4r-fortlet-0.1.0`. The workspace
was clean, all declared artifact identities matched, synthetic auth passed
structural and mode checks without disclosure, isolated capsule and image
inventories were empty, and global inventory matched baseline.

Cold preparation returned exactly `tact<TAB>ready` in 200.95 seconds, below
the 600-second ceiling:

| Phase | Milliseconds |
| --- | ---: |
| Local image verification/load | 1,183 |
| Runtime-store seed | 2,736 |
| Tact closure import | 1,075 |
| Schema-1 layer | 195,918 |
| Final verification | 0 |
| Bounded event total | 200,914 |

The published layer was canonical: its root, directories, and executable
files were `0555`; ordinary files and the marker were `0444`. Status remained
absent and raw capsule inventory was empty. The isolated image inventory
contained only the declared 104,326,438-byte local image. Fortlet data used
1,258,928 KiB, Fortlet state used 0 KiB, and isolated MicroSandbox state used
676,316 KiB.

The runtime, Tact, and project-layer markers had these baseline identities:

| Marker | Size | SHA-256 |
| --- | ---: | --- |
| Runtime | 6,873 bytes | `5f27f2d2d201a4ff818befea5418deaf8778edeeaade338030eab4391825af57` |
| Tact | 748 bytes | `38bb9fc890b5e918b9c05332c1e54f426493753ff7f4a9bdf0e8da93d5e7d3c9` |
| Project layer | 252 bytes | `50b10e3f965b64ff2b451565b5b9b1eea2ddcebc9aada5b2f4aeaa26522a6521` |

All five unchanged prepares succeeded, printed the ready line, created no
capsule, emitted no image, runtime-store, or harness first-use phase, and left
every marker hash, size, mtime, and mode unchanged. They failed the latency
requirement:

| Sample | Schema-1 verification ms | Real seconds |
| --- | ---: | ---: |
| 1 | 5,501 | 5.51 |
| 2 | 5,125 | 5.14 |
| 3 | 5,124 | 5.14 |
| 4 | 5,077 | 5.09 |
| 5 | 5,202 | 5.22 |

The 5.14-second median exceeds the required sub-second median. Static
attribution confirms that a published schema-1 cache hit calls
`verify_published`, which recursively validates modes and then recursively
reads and hashes the complete sealed output. For this 1.2-GiB tool layer, that
full reuse traversal dominates every prepare. The permission correction is
functionally valid, but its reuse verification violates FIP-0006's amended
no-traversal cache-hit requirement and FIP-0013's latency gate.

The campaign stopped before lifecycle sampling, so absent, stopped, and
running launch distributions remain unavailable. No provider request, model
request, schema-2 preparation, project edit, real credential read, authenticated
input, package rebuild, or remote mutation occurred.

Public stop and reset reported absence. Raw capsule inventory was empty, the
testbed workspace remained clean and was forgotten, `/private/tmp/f58` was
removed and proved absent, and final global inventory matched preflight.

## Terminal Closure

1. Outcome: rejected; cold preparation passed at 200.95 seconds, but five
   successful unchanged prepares had a 5.14-second median against the
   sub-second requirement.
2. Root cause: reuse performs recursive mode validation and a complete
   content-digest traversal of the sealed schema-1 output. The 1.2-GiB Fortlet
   tool layer makes that integrity work incompatible with the cache-hit latency
   contract.
3. Actual cost: about five minutes versus the 30-minute ceiling, one cold
   prepare, five warm prepares, zero lifecycle launches, zero provider calls,
   zero model calls, zero real credentials, and $0.
4. Cleanup: public stop/reset reported absence, isolated capsule inventory was
   empty, the exact workspace was clean and forgotten, the exact experiment
   root was removed, and global inventory was unchanged.
5. Next action: stop for operator review. A successor must reconcile FIP-0006's
   integrity verification with its no-traversal cache-hit requirement before
   repeating lifecycle evidence. Schema-2 preparation and provider or model
   dispatch remain unauthorized.
