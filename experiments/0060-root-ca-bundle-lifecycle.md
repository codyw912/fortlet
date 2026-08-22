# Experiment 0060: Root CA bundle lifecycle successor

Status: terminal — accepted
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

Use signed candidate
`eb363924fde5f2f540dcf557f26ba88bc0a40d89` and exact immutable package
`/nix/store/f2wf5fb7dgzn66ylccr24c5598ml3yjr-fortlet-0.1.0`. Its SSH
signature verifies as `good` for the declared GitHub noreply identity. The
package contains Fortlet 0.1.0, MicroSandbox 0.6.8, Tact 0.3.7, runtime
contract `fip0013-2`, and these identities:

- Fortlet executable SHA-256
  `7d367bc8c175d7be8b4b915957343de9d6879f850fa9934ac757bc65bf3bd867`;
- MicroSandbox executable SHA-256
  `799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`;
- OCI archive SHA-256
  `79bd08cbe230c5a676d2e29cbf4fbef8ff83c75eb0ac42dccf3c3eef8529bf46`;
- OCI manifest digest
  `sha256:1833500921de187517faff311532608dbba63af310530daad28c855d281dd2b8`;
- runtime seed SHA-256
  `8a9a38b4960987021757672d6a95b05e2c6a525737fbec493a7842d2274e6655`;
  and
- Tact closure SHA-256
  `7fb40fe4eaad08a5d4ce4dd780c1caccf0624b3ca31e6a0587cf61fd3e3fa3e3`.

The manifest selects root bundle
`/etc/ssl/certs/ca-certificates.crt` and immutable source
`/nix/store/xqm9bp646askgkfwsgsifvmzv0sc34yg-nss-cacert-3.126/etc/ssl/certs/ca-bundle.crt`.
No source or package rebuild is permitted after live execution begins.

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
`x86_64-linux` omission. The corrected candidate then passed 107 unit tests and
the same complete gate; its artifact build rootlessly unpacked the OCI image
and checked the real root-level bundle and four CA environment selectors. The
exact signed package was built and hashed while `/private/tmp/f60` remained
absent and the sole Jujutsu workspace remained clean. No experiment root,
capsule, credential fixture, provider, harness, model, project mutation, or
remote mutation was created by this rehearsal.

## Results

Deterministic implementation `eb363924fde5f2f540dcf557f26ba88bc0a40d89`
advanced the runtime contract to `fip0013-2` and verified with a good SSH
signature. Its OCI artifact build unpacked the produced root filesystem and
proved that `/etc/ssl` and `/etc/ssl/certs` are real directories, the active
bundle is a regular mode-`0644` file with the exact immutable initial bytes,
all four CA environment selectors use it, and the Nix source remains in the
seed closure. The complete pre-live gate passed: 107 unit tests and every
enabled integration test, formatting, strict all-target/all-feature Clippy,
conformance, the Nix package, and shim activation were green. The two external
stock Codex tests remained intentionally ignored, and Nix reported only the
known native `x86_64-linux` omission.

Preflight passed against the signed implementation and exact immutable package
declared above. Both isolated image and capsule inventories began empty, both
isolated Fortlet inventories reported `no capsules`, synthetic auth passed
mode and structural checks without disclosure, the schema-1 workspace was a
clean empty child of the signed candidate, and the hashed global inventory
matched its later cleanup value.

The manifest-free control preparation completed in 6.31 seconds:

| Phase | Milliseconds |
| --- | ---: |
| Local image verification/load | 1,144 |
| Runtime-store seed | 4,072 |
| Tact closure import | 1,082 |
| Final verification | 0 |
| Bounded event total | 6,299 |

Its absent, stopped, and running launches returned Tact 0.3.7 in 0.47, 0.35,
and 0.03 seconds. The running capsule exposed real root-level `/etc/ssl`
directories and a regular active bundle. The immutable source contained
472,033 bytes and 121 certificates; the active bundle retained that exact
prefix and contained 472,668 bytes and 122 certificates after MicroSandbox
injected its capsule CA. Creating `/nix/fortlet-exp0060-write` failed with a
read-only-filesystem error and left no entry. Public stop/reset restored
absence and empty inventory, and its runtime and Tact markers remained
byte-for-byte unchanged.

The first two direct diagnostic execs stopped before completing that boundary
check because they attempted to resolve `cmp` first through the absent managed
PATH and then through a runtime root that does not link Diffutils. These were
retained apparatus errors, not retried Fortlet launches. Host inspection of the
frozen manifest selected only present `head` and `sha256sum` commands; the
corrected read-only observation then passed. Similarly, the first mount JSON
predicates expected lowercase types and an option array, returned `false`, and
were corrected against the bounded typed object before the cycle closed.

Fresh schema-1 cold preparation completed in 155.00 seconds, below the
600-second ceiling:

| Phase | Milliseconds |
| --- | ---: |
| Local image verification/load | 1,064 |
| Runtime-store seed | 2,453 |
| Tact closure import | 1,042 |
| Schema-1 layer | 150,435 |
| Final verification | 1 |
| Bounded event total | 154,997 |

Complete post-cold inspection accepted the canonical `0555` directories and
executables, `0444` ordinary files and marker, and supported directory, file,
and link types. Fortlet data used 1,263,068 KiB, of which the project layer used
1,263,056 KiB; isolated MicroSandbox state used 676,452 KiB and Fortlet state
used 0 KiB. The isolated image was 104,486,136 bytes.

The runtime, Tact, and schema-1 marker identities remained unchanged through
all later prepares, lifecycle cycles, and cleanup preflight:

| Marker | Size | Mode | SHA-256 |
| --- | ---: | ---: | --- |
| Runtime | 6,929 bytes | `0600` | `ba62bf089abcaa664c38c437bd5627b62596af70abb51edf582fb38fb9626b24` |
| Tact | 748 bytes | `0600` | `4aa3d255d81ddbba73a170029fbf1cf0827644f0ba601c764557affc9cbb1a10` |
| Project layer | 252 bytes | `0444` | `e1dce142f43663c44e55abeeef5ba507ba5487c8cdc8b1c6ad47c59cd41eaa79` |

All five unchanged prepares returned the exact ready line, emitted only
zero-millisecond schema-1 and final-verification events, created no capsule,
and preserved every marker property. Wall times were 0.01, 0.00, 0.00, 0.00,
and 0.00 seconds; the 0.00-second median passes the sub-second hard gate.

All twelve fixed lifecycle launches returned Tact 0.3.7:

| Cycle | Absent | Running 1 | Stopped | Running 2 |
| --- | ---: | ---: | ---: | ---: |
| 1 | 0.42 s | 0.03 s | 0.34 s | 0.03 s |
| 2 | 0.40 s | 0.02 s | 0.32 s | 0.02 s |
| 3 | 0.39 s | 0.02 s | 0.32 s | 0.02 s |

The absent median was 0.40 seconds, the stopped median was 0.32 seconds, and
the running median was 0.02 seconds. Every running sample was at most 0.03
seconds. The inspected configuration had a named read-only `/nix` mount and a
schema-1 bind with read-only, `nosuid`, `nodev`, relaxed stat virtualization,
private host permissions, and root-symlink following disabled. Every cycle
ended absent with empty isolated inventory and unchanged markers.

Final public stop/reset, workspace status, both isolated inventories, marker
comparison, and global inventory comparison passed. Workspace `exp0060` was
forgotten, cleanup-only owner access was restored beneath the exact disposable
root, and only `/private/tmp/f60` was removed and proved absent. No schema-2
preparation, provider or model request, real credential read, project edit,
remote-project mutation, or publication outside the authorized goal branch
occurred.

## Terminal Closure

1. Outcome: accepted. The manifest-free control and fresh schema-1 campaign
   both passed guest-agent initialization, immutable-store enforcement,
   preparation, lifecycle, integrity, mount-policy, and latency criteria.
2. Root cause: the prior image exposed `/etc/ssl` through the Nix store, so
   MicroSandbox's required CA append reached the read-only `/nix` mount. The
   corrected contract keeps the immutable source in the seed closure but
   materializes the active bundle in disposable root state.
3. Actual cost: one manifest-free prepare, three control launches, one cold
   schema-1 prepare, five warm prepares, twelve schema-1 launches, zero
   provider calls, zero model calls, zero real credentials, zero project edits,
   and $0, within the 40-minute live ceiling.
4. Cleanup: every owned capsule was publicly stopped and reset; both isolated
   inventories were empty; markers, workspace, and global inventory were
   unchanged; workspace `exp0060` was forgotten; and `/private/tmp/f60` was
   removed and proved absent.
5. Next action: stop for operator review. FIP-0013's prepared lifecycle gate is
   accepted on the declared `aarch64-darwin` host, but schema-2 provider work,
   model dispatch, and native `x86_64-linux` verification remain separate.
