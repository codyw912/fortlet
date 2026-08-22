# Experiment 0059: Bounded schema-1 reuse lifecycle

Status: terminal — rejected on first prepared absent launch
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

## Results

Preflight passed against signed implementation
`95107fdbc686058ab4bb2fb11c9c3ffb9fc606a9` and exact immutable package
`/nix/store/6ldx5571fpz2gah2c9c5lydi46w36br2-fortlet-0.1.0`. The workspace
was clean, every declared package and artifact identity matched, synthetic auth
passed structural and mode checks without disclosure, isolated image and
capsule inventories were empty, and global Fortlet inventory matched baseline.

Cold preparation returned exactly `tact<TAB>ready` in 186.27 seconds, below
the 600-second ceiling:

| Phase | Milliseconds |
| --- | ---: |
| Local image verification/load | 1,063 |
| Runtime-store seed | 2,624 |
| Tact closure import | 1,151 |
| Schema-1 layer | 181,418 |
| Final verification | 0 |
| Bounded event total | 186,258 |

The published `fortlet-project-layer-v3` layer was canonical: its root and
directories were `0555`, executable files were `0555`, and ordinary files and
the marker were `0444`. A complete evidence traversal found no unsupported
type or mode. Status remained absent and raw capsule inventory was empty. The
isolated image inventory contained only the declared 104,326,438-byte image.
Fortlet data used 1,256,956 KiB, Fortlet state used 0 KiB, and isolated
MicroSandbox state used 676,060 KiB.

The runtime, Tact, and project-layer markers had these identities, which
remained unchanged through every later product unit and cleanup check:

| Marker | Size | Mode | SHA-256 |
| --- | ---: | ---: | --- |
| Runtime | 6,873 bytes | `0600` | `5f27f2d2d201a4ff818befea5418deaf8778edeeaade338030eab4391825af57` |
| Tact | 748 bytes | `0600` | `38bb9fc890b5e918b9c05332c1e54f426493753ff7f4a9bdf0e8da93d5e7d3c9` |
| Project layer | 252 bytes | `0444` | `f402b9b7596706d7b979148013894a3758280d654c25cee4990f7f29f2173cbd` |

All five unchanged prepares passed. Each printed the exact ready line, created
no capsule, emitted no image, runtime-store, or harness first-use phase, and
left every marker hash, size, mtime, and mode unchanged:

| Sample | Schema-1 verification ms | Real seconds |
| --- | ---: | ---: |
| 1 | 2 | 0.02 |
| 2 | 0 | 0.02 |
| 3 | 0 | 0.01 |
| 4 | 0 | 0.02 |
| 5 | 0 | 0.01 |

The 0.02-second median passes the required sub-second hard gate and improves
Experiment 0058's 5.14-second median by more than two orders of magnitude.
This establishes that ordinary `v3` reuse is bounded independently of the
1.2-GiB output size.

The first lifecycle launch, from prepared absence, failed before Tact or
command attachment. Fortlet emitted `resolve` in 1 millisecond, `credentials`
and `environment` in 0 milliseconds, and `capsule-state` in 52 milliseconds,
then reported that the sandbox process exited with status zero before the
agent relay became available. Total wall time was 0.55 seconds, but it is a
failed sample and provides no accepted launch latency.

Retained MicroSandbox logs narrow the failure to guest-agent initialization.
The VM entered, installed the five bind-identity mappings, and then the guest
reported `agentd: init failed: io error: Read-only file system (os error 30)`
before shutdown. The stopped configuration contained the intended read-only
project-store mount and the schema-1 relaxed/private
`ro,nosuid,nodev`, no-root-symlink-following mount. Because the capsule never
reached running state, that configuration is not runtime mount or mutation
evidence, and the log does not identify which guest path required a write.

The declared launch-failure rule ended the campaign after one of twelve
launches. No launch was retried, no stopped or running launch ran, and no
lifecycle-state median is available. No schema-2 preparation, provider or
model request, project edit, real credential read, authenticated input,
package rebuild, or remote mutation occurred.

The exact stopped provider capsule was inspected and removed during bounded
cleanup. Raw isolated inventory then returned `[]`; every marker snapshot was
unchanged; the testbed workspace remained clean and was forgotten; only
`/private/tmp/f59` was removed and proved absent; and the final global Fortlet
inventory matched its preflight hash.

## Terminal Closure

1. Outcome: rejected overall. Cold preparation passed at 186.27 seconds and
   bounded reuse passed with a 0.02-second median, but the first prepared absent
   launch failed before Tact because the guest agent could not initialize.
2. Root cause boundary: MicroSandbox entered the VM and installed bind identity
   maps, then `agentd` encountered a read-only filesystem and the VM shut down.
   The retained evidence does not identify the write target or establish
   whether the correction belongs in the runtime image, root-disk policy, or
   another writable mount.
3. Actual cost: one cold prepare, five warm prepares, one of twelve lifecycle
   launches, zero provider calls, zero model calls, zero real credentials, and
   $0, within the 30-minute ceiling.
4. Cleanup: the exact stopped capsule was removed, isolated inventory was
   empty, snapshots and project contents were unchanged, the exact clean
   workspace was forgotten, the exact root was removed, and global inventory
   was unchanged.
5. Next action: stop for operator review. A successor should first isolate the
   guest-agent write requirement against the frozen Nix runtime before any
   lifecycle retry, schema-2 preparation, provider request, or model dispatch.
