# Experiment 0056: Trusted host canonical seal

Status: declared
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0013

## Baseline / Control

Experiment 0055 reproduced the production failure with packaged Fortlet 0.1.0,
MicroSandbox 0.6.8, runtime contract `fip0013-1` for `aarch64-linux`, and local
OCI manifest digest
`sha256:86f93316b38332d732e9092562319eb8af2eb4fb6ad23bf78db4ca357a37e9b0`.
Under strict stat virtualization and private host permissions, guest
`0755`/`0644` became literal host `0700`/`0600`; current write-bit removal made
the tree `0500`/`0400`, and a strict read-only consumer failed to mount it.

Experiment 0055 also proved that mirrored host permissions deliberately retain
an owner-access floor: guest-finalized `0555` directories and executables appear
literally as `0755`, while guest-finalized `0444` files appear as `0644`. That
candidate stopped before consumption. This experiment uses that exact local
binary and image with one fresh public-neutral synthetic fixture and no
repository, provider, harness, credential, or identity input.

The consumption control is one strict/private read-only mount of the correctly
sealed candidate tree. Its result determines whether relaxed stat handling is
necessary, but it does not suppress the separately declared relaxed consumer
unless a shared host failure invalidates both.

## Hypothesis and Production Mechanism

Mirrored publication preserves the guest's group/other readability and
executability while adding only MicroSandbox's deliberate host owner-write
floor. After the trusted guest has validated and finalized the tree, the
trusted host can remove write bits without interpreting repository text or
inventing executable permissions. This transforms literal host `0755` to
canonical `0555` and `0644` to canonical `0444`, aligning the host inode modes
with MicroSandbox's existing guest stat overrides.

The resulting tree should be executable and readable by a numeric non-root
consumer while remaining immutable. Strict consumption may fail because its
xattr capability probe touches a sealed root. Relaxed consumption should use
the existing stat overrides when available, tolerate an unwritable backing
tree, and retain `ro,nosuid,nodev` enforcement. If strict also succeeds, it is
the preferred production policy because no relaxation is needed.

## Declared Scope

Use exact owned root `/private/tmp/f56`, isolated `MSB_HOME` beneath it, local
image reference `fortlet-runtime:fip0013-1-aarch64-linux`, one vCPU, 512 MiB
memory, a 1 GiB root disk, no network, pull policy `never`, and at most one
owned capsule at a time. The exact capsule names are `f56-publish`,
`f56-strict-consume`, and `f56-relaxed-consume`. Each command has a 30-second
bound.

Load the exact local OCI archive into the fresh isolated image store. Mount one
fresh host directory at `/out` with
`stat-virt=strict,host-perms=mirror`. Root creates `bin/probe`, ordinary
`share/data`, and relative `bin/probe-link`. The trusted guest finalizer rejects
special files and an absolute link, then sets the root, directories, and probe
to `0555` and data to `0444`. Record the guest view, remove the publisher, and
record literal host modes, symlink target, xattr names, and file hashes.

The trusted host validates that only the declared tree exists, the symlink is
relative, no special file exists, literal modes are exactly `0755` for the
root, directories, and probe and `0644` for data, and hashes match. It then
runs only `chmod -R a-w` on that exact tree. Revalidate exact canonical
`0555`/`0444` modes, link, types, and hashes before consumption.

Run the strict consumer first and relaxed consumer second, each from a fresh
capsule, numeric user `1000:1000`, and the same sealed tree. Both use
`ro,nosuid,nodev,host-perms=private`; only `stat-virt=strict` versus
`stat-virt=relaxed` differs. Each command attempts both executable paths, reads
data, requires append and chmod to fail, and prints four fixed success lines
only after all checks pass. Remove each capsule and compare modes, symlink,
hashes, and xattr names with the pre-consumer record.

Record all attached `msb run` wall durations. These single direct-CLI samples
screen for gross overhead only and do not waive the GOAL's full Fortlet
prepared-lifecycle latency requirements.

No source, production implementation, FIP, schema-2 state, project checkout,
provider state, harness closure, model request, real credential, personal
identity, network, remote, package, or publication may be changed or contacted.

## Alternatives

Removing the owner floor inside MicroSandbox was rejected because the floor is
an intentional host-safety property and changing a dependency is unnecessary
if a bounded trusted publication step suffices. Clearing stat-override xattrs
was rejected because it discards the guest's validated metadata and broadens
the host transformation. Reconstructing modes solely on the host was rejected
because the host should remove only the known safety floor, not infer guest
intent. Promoting schema-1 output into the Nix store remains a later
architecture option but is too broad for this failure.

## Risks

The host seal is recursive, so it is permitted only after exact-root,
tree-shape, type, link, mode, and hash validation. Cleanup may require restoring
owner access on only `/private/tmp/f56/layer`; that cleanup-only mutation occurs
after terminal measurements. Strict probing could succeed, fail cleanly, or
leave a stopped capsule; every outcome is retained and the exact capsule is
removed before the relaxed run. A strict result is not evidence for or against
relaxed behavior.

Any undeclared entry, absolute or escaping link, special file, unexpected
pre-seal or post-seal mode, hash change, successful consumer mutation,
consumer-output mismatch, remaining capsule, nonempty final isolated
inventory, changed global inventory, or cleanup failure rejects the candidate.
A VM, image-store, or host failure shared by both consumers stops the
experiment without a mount-policy claim. No failed unit is resumed or replaced.

## Acceptance Criteria

1. Preflight verifies MicroSandbox 0.6.8, exact OCI archive and manifest
   identities, an absent exact root, empty isolated inventories, and unchanged
   global Fortlet inventory.
2. The publisher exits zero and reports guest `0555` for the root,
   directories, and probe and `0444` for data. After removal, host modes must
   show only the expected owner-floor difference: `0755` and `0644`.
3. Trusted host validation passes before `chmod -R a-w`. Afterward literal
   modes must be exactly `0555` for the root, directories, and probe and `0444`
   for data, with the same relative link, types, and hashes.
4. The strict consumer result is retained. If it exits zero with all checks,
   strict is selected over relaxed for production; if it fails solely while
   starting the sealed mount and relaxed later succeeds, relaxed is selected.
   Any integrity or mutation failure rejects the mechanism.
5. The relaxed consumer must exit zero and print exactly `probe-ok`,
   `probe-ok`, `data-ok`, and `readonly-ok` on separate lines. Append and chmod
   must fail. Post-consumer modes, link, hashes, and xattr names must exactly
   match the sealed record.
6. All three run durations are retained without a latency claim. Acceptance
   does not waive the full prepared lifecycle campaign.
7. Final isolated capsule inventory is empty, isolated image inventory contains
   only the locally loaded declared image before cleanup, `/private/tmp/f56` is
   removed and absent, and global Fortlet inventory is unchanged.
8. A failed unit is terminal. Do not alter the fixture, seal, ordering, mount
   policies, or thresholds in flight.

## Budget and Plan

Budget: 15 minutes total, one local image load, one mirrored publication, one
trusted host seal, one strict consumption, one relaxed consumption, zero
network, zero provider calls, zero harness calls, zero model calls, zero real
credentials, and $0. Stop on the first candidate-fatal anomaly or time ceiling.
Remove each capsule before creating the next; after terminal measurements
restore cleanup-only owner access on the exact layer, remove the exact isolated
root, and prove absence.

## Rehearsal

On 2026-08-21 read-only source inspection verified MicroSandbox 0.6.8's exact
`0700` directory and `0600` regular-file mirror floors, strict xattr probe,
relaxed xattr fallback, and host- plus guest-enforced read-only mount path. CLI
help accepts the declared mount grammar. The exact archive, image manifest,
shell, and runtime root were verified in Experiment 0055, and its complete
post-cleanup `aarch64-darwin` standard gate passed. No Experiment 0056 root,
image, capsule, VM, or fixture existed during rehearsal.

## Results

Pending.

## Terminal Closure

Pending.
