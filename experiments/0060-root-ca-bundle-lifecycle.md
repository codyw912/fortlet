# Experiment 0060: Root CA bundle lifecycle successor

Status: authorized — implementation and deterministic gate pending
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0013

## Baseline / Control

Experiment 0059 accepted 186.27-second cold schema-1 preparation and a
0.02-second median across five unchanged prepares. Its first prepared absent
launch failed before Tact: MicroSandbox 0.6.8 installed bind identity maps,
then its guest agent reported a read-only filesystem during initialization and
the VM shut down.

Read-only inspection of the pinned guest-agent source narrows the failing
operation. Initialization installs the capsule CA after mounts and networking;
it copies the certificate to distribution trust locations best-effort, then
fatally appends it to the selected CA bundle. The frozen OCI root maps
`/etc/ssl` into the Nix store, while managed capsules deliberately mount the
project store read-only at `/nix`. The log reached the CA copies but not the
guest agent's final CA-installation message. This makes the immutable
store-backed bundle the production hypothesis; the live control must still
falsify or accept it.

## Hypothesis and Production Mechanism

The guest agent should initialize when the OCI root contains a real
`/etc/ssl/certs` directory and regular `ca-certificates.crt` copied from the
pinned Nix CA package at image construction. The image and Fortlet-managed
environment must select that root-level bundle. MicroSandbox may then append
its per-capsule CA only in the disposable root overlay while `/nix` remains
read-only and immutable.

Advance the internal FIP-0013 runtime contract so the old image, project
stores, harness markers, and capsules cannot be selected as prepared state.
Do not change the common tool set, harness closures, project-layer contract,
store mount policy, provider contract, or public command surface.

## Frozen Inputs

Use the candidate produced by the authorized implementation checkpoint, with
its exact signed revision, immutable package path, executable hash, OCI archive
hash and manifest digest, runtime seed hash, Tact closure hash, MicroSandbox
0.6.8, and Tact 0.3.7 recorded before the first live capsule. No source or
package rebuild is permitted after live execution begins.

Use only exact owned root `/private/tmp/f60`. Use separate fresh control and
schema-1 project roots, Fortlet state, Fortlet data, isolated MicroSandbox
home, mode-`0600` synthetic non-provider auth fixtures, and evidence beneath
that root. If a repository workspace is needed, name it `exp0060` and pin it to
the signed candidate. Never have more than one owned capsule.

## Method

1. Deterministic evidence MUST unpack the produced OCI filesystem and prove
   `/etc/ssl` and `/etc/ssl/certs` are real directories, the selected CA bundle
   is a non-symlink regular file with the declared initial bytes, the OCI
   environment selects that path, the Nix CA source remains in the declared
   seed closure, and managed capsule configuration still mounts `/nix`
   read-only.
2. The complete `docs/RUNBOOK.md` gate MUST pass. Freeze and sign the candidate,
   build its exact immutable package, then preflight the signature, clean
   inputs, artifact identities, absent experiment root, empty isolated image
   and capsule inventories, structurally valid synthetic auth without
   disclosure, and global inventory baseline.
3. In fresh manifest-free control state, prepare only Tact, then run one timed
   packaged `fortlet run tact -- --version` from absence. Require status zero,
   exact Tact 0.3.7 output, guest-agent readiness, a real root-level CA bundle
   containing the injected capsule CA, and failed mutation through the
   read-only `/nix` mount.
4. Publicly stop the same control capsule and run the identical timed command
   once from stopped state and once while running. Require status zero and
   exact Tact 0.3.7 output for both. Each control launch MUST remain below one
   second. Publicly stop and reset it, prove absence and empty inventory, and
   compare prepared markers before continuing.
5. Only if the control passes, begin a separate fresh schema-1 campaign. Run
   one cold `fortlet prepare tact` with `FORTLET_PREPARE_TIMINGS=1`, public
   unauthenticated access only to the frozen recipe inputs, and a 600-second
   ceiling. Require exact `tact<TAB>ready`, bounded cold phases, no capsule,
   and canonical `0555`/`0444` schema-1 publication.
6. Snapshot runtime, Tact, and project-layer marker hashes, sizes, mtimes, and
   modes; selected store and layer disk sizes; project status; and isolated
   inventory. Run five consecutive unchanged prepares without network use.
   Every call MUST print the exact ready line, create no capsule, emit no
   image, runtime-store, or harness first-use phase, and leave every snapshot
   unchanged. Their median MUST be below one second.
7. If the warm gate passes, run three fixed lifecycle cycles. Each starts
   absent and performs one timed `fortlet run tact -- --version` from absent,
   one identical timed launch while running, public stop, one timed launch from
   stopped, a second timed launch while running, then public stop/reset, absent
   status, and empty inventory. Set `FORTLET_STARTUP_TIMINGS=1` on all twelve
   launches. Every launch MUST return Tact 0.3.7 with status zero.
8. During one running state, inspect the stored capsule configuration and
   require the project store read-only mount plus schema-1 relaxed/private
   `ro,nosuid,nodev` policy with no root-symlink following. Compare every
   marker snapshot after every cycle.

Absent, stopped, and running schema-1 medians MUST each be below one second,
and every running sample MUST be below one second. Prepared launch may perform
no image pull, artifact download, preparation capsule, Nix evaluation, closure
import, provider realization, project-store write, or schema-1 tree traversal.

No schema-2 preparation, provider or model request, project edit, testbed
checkpoint, real credential read, authenticated input, public-project mutation,
or remote mutation outside draft PR #15 is permitted.

## Risks and Stop Rules

The complete experiment ends after 40 minutes of measured live work. A
deterministic artifact failure, control failure, cold failure, warm failure,
launch failure, timing rejection, marker mutation, mount mismatch, credential
anomaly, project change, remaining capsule, inventory mismatch, or cleanup
failure is terminal product evidence. Do not change inputs, ordering,
thresholds, or successful sample counts. Do not retry a terminal product unit.

## Acceptance Criteria

1. Deterministic OCI inspection and the complete local gate pass.
2. The fresh manifest-free absent, stopped, and running controls each return
   Tact 0.3.7 below one second while proving the intended CA and `/nix`
   boundaries.
3. Cold schema-1 preparation succeeds within 600 seconds and five unchanged
   prepares have a sub-second median without traversal, mutation, or capsule
   creation.
4. All twelve schema-1 launches return Tact 0.3.7; each lifecycle-state median
   is sub-second and every running sample is below one second.
5. Mount policy, no-prepared-work, marker integrity, project integrity,
   per-cycle absence, empty inventory, and exact cleanup all pass.

## Alternatives Considered

1. Make `/nix` writable in managed capsules. Rejected because it would weaken
   FIP-0013's project-store integrity and let credentialed workloads mutate the
   prepared runtime and provider state.
2. Use a conventional distribution or full NixOS image. Rejected because the
   failure is a specific writable-root contract mismatch; changing the common
   runtime model would add unrelated package and system policy.
3. Disable MicroSandbox CA installation. Rejected because its destination-bound
   secret substitution depends on the guest trusting the capsule CA.
4. Add a dedicated writable mount over `/etc/ssl`. Deferred because the
   disposable image root already owns mutable system initialization state and
   needs no new persistent state class.

## Budget and Cleanup

Budget: 40 minutes of measured live work, one manifest-free prepare, three
control launches, one cold schema-1 prepare, five warm prepares, twelve fixed
schema-1 launches, one owned capsule at a time, zero provider calls, zero model
calls, zero real credentials, zero project edits, and $0. After terminal
evidence, use public stop/reset for every owned capsule, forget only workspace
`exp0060` if created, restore cleanup-only owner access if necessary, remove
only `/private/tmp/f60`, and prove absence and unchanged global inventory.

## Rehearsal

Read-only source and image inspection established the production hypothesis.
Repository baseline verification passed before implementation: 106 unit tests
and all enabled integration tests, formatting, strict Clippy, conformance, the
Nix package, and shim activation were green; the two stock Codex compatibility
tests remained intentionally ignored, and Nix reported only the known native
`x86_64-linux` omission. No experiment root, workspace, capsule, credential
fixture, package rebuild, provider, harness, model, project mutation, or remote
mutation was created by this rehearsal.

## Results

Pending.

## Terminal Closure

Pending.
