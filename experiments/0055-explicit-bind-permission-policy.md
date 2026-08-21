# Experiment 0055: Explicit bind-permission publication policy

Status: completed — rejected
Design: FIP-0001, FIP-0006, FIP-0007, FIP-0013

## Baseline / Control

Experiment 0054 used packaged Fortlet 0.1.0, MicroSandbox 0.6.8, runtime
contract `fip0013-1` for `aarch64-linux`, and local OCI manifest digest
`sha256:86f93316b38332d732e9092562319eb8af2eb4fb6ad23bf78db4ca357a37e9b0`.
Its schema-1 publisher and consumer both used MicroSandbox's default bind
policy: strict stat virtualization and private host permissions. Guest-created
entries reached the host as owner-only modes; Fortlet's host-side recursive
write-bit removal then produced representative `0500` directories and `0400`
executables. The first read-only consumer mount failed before its command.

This experiment uses the same already-built local OCI archive and MicroSandbox
binary. The fixture is one public-neutral tree containing an executable
`bin/probe`, an ordinary `share/data` file, and a relative `bin/probe-link`
symlink. It contains no repository text, package input, provider state,
credential, identity, or harness.

## Hypothesis and Production Mechanism

Private host permissions deliberately keep guest `chmod` changes in
MicroSandbox's metadata overlay instead of mirroring ordinary rwx bits to the
host inode. Sealing that host view therefore preserves owner-only modes and
mixes two permission authorities. Explicit mirrored permissions on the
writable publication bind will instead make the guest the sole authority for
portable published modes. A trusted finalization step in that same guest will
canonicalize directories and executable files to `0555`, ordinary files to
`0444`, retain only relative symlinks, and reject special files. The host can
then validate and publish those literal modes.

The consumer will keep host propagation private, but use relaxed stat
virtualization so an already-sealed read-only backing tree does not require a
successful metadata-xattr probe. The mount remains `ro,nosuid,nodev`; relaxed
stat handling changes metadata fallback only and does not weaken host- or
guest-enforced read-only behavior.

## Declared Scope

Use exact owned root `/private/tmp/f55`, isolated `MSB_HOME` beneath it, local
image reference `fortlet-runtime:fip0013-1-aarch64-linux`, one vCPU, 512 MiB
memory, a 1 GiB root disk, no network, pull policy `never`, and at most one
owned capsule at a time. The four exact capsule names are
`f55-control-publish`, `f55-control-consume`, `f55-candidate-publish`, and
`f55-candidate-consume`. Each command has a 30-second bound.

Load the declared local OCI archive into the fresh isolated image store. The
control publisher mounts a fresh host directory at `/out` with explicit
`stat-virt=strict,host-perms=private`; root creates the fixture with guest modes
`0755` for directories and the executable, `0644` for data, and a relative
symlink. After the publisher exits and is removed, record literal host modes
and extended-attribute names, remove write bits recursively exactly as the
current host sealer does, and record them again. Mount the sealed tree read-only
with the same strict/private policy and attempt to execute the probe. This is a
reproduction control, not a production alternative.

The candidate publisher uses a distinct fresh directory and
`stat-virt=strict,host-perms=mirror`. Root creates the identical fixture, then
the trusted guest finalizer sets the root and both directories to `0555`, the
probe to `0555`, and data to `0444`; verifies the symlink is relative; rejects
any block, character, FIFO, or socket entry; and exits. After removal, record
literal host modes, symlink target, extended-attribute names, and content
hashes. The candidate consumer mounts the tree with
`ro,nosuid,nodev,stat-virt=relaxed,host-perms=private`, runs as numeric
`1000:1000`, executes both the probe and relative symlink, reads data, and
requires both an append and a `chmod` attempt to fail. Record post-consumer
modes and hashes and require exact equality.

Each attached `msb run` wall duration is recorded. These direct CLI samples
bound gross regressions but do not replace Experiment 0054's Fortlet lifecycle
thresholds; an accepted mount mechanism still requires the original prepared
absent, stopped, and running latency campaign before adoption.

No source, production implementation, FIP, schema-2 state, project checkout,
provider state, harness closure, model request, real credential, personal
identity, remote, package, or publication may be changed or contacted.

## Alternatives

Turning stat virtualization off for the consumer was rejected because it
cannot be combined with mirrored permissions and discards MicroSandbox's
portable metadata layer entirely. Leaving publication private and repairing
modes on the host was rejected because the host would remain responsible for
interpreting guest output and could diverge from the guest's metadata view.
Promoting every schema-1 output into the Nix store remains a possible later
architecture, but is too broad for attribution of this concrete bind-policy
failure. Snapshots were rejected because MicroSandbox snapshots exclude bind
mount data and therefore do not solve schema-1 publication.

## Risks

The control may reproduce as a mount failure or as an executable-permission
failure depending on exactly where strict stat probing encounters the sealed
tree; either is retained verbatim. If it executes successfully, the proposed
attribution is rejected and the candidate does not run. Mirroring makes guest
permission changes real host changes, so only the disposable exact candidate
root is exposed and no untrusted recipe text is used. Cleanup of sealed trees
may require restoring owner write and execute bits on only the exact experiment
directories; that cleanup-only mutation is predeclared and must occur after
all terminal measurements.

A VM, image-store, or host failure shared by both policies stops the experiment
without claiming a permission result. Any undeclared file, absolute symlink,
special file, successful consumer mutation, changed hash or mode, remaining
capsule, nonempty isolated inventory, or cleanup failure is a visible
candidate rejection. No failed unit is resumed or replaced.

## Acceptance Criteria

1. Preflight verifies the exact MicroSandbox version, OCI archive identity and
   manifest digest, fresh exact root, empty isolated inventories, and the four
   absent capsule names.
2. The control publisher sees the declared guest modes, while literal host
   modes demonstrate the private-policy divergence. After current-style
   sealing, the strict/private consumer must fail before successfully executing
   `probe-ok`; otherwise reject the attribution and stop.
3. The candidate publisher exits zero after its trusted finalizer. Literal
   host inspection must show exactly `0555` for the root, `bin`, `share`, and
   `bin/probe`; `0444` for `share/data`; and `probe` as the relative symlink
   target, with no special file.
4. The numeric non-root candidate consumer must print exactly two `probe-ok`
   lines and one `data-ok` line, prove append and chmod fail, and exit zero.
   Post-consumer hashes, modes, and symlink target must exactly equal the
   pre-consumer record.
5. All four run durations are retained. No policy-latency claim is made from
   one sample, and acceptance does not waive the GOAL's full lifecycle limits.
6. Final isolated capsule inventory is empty, exact image inventory contains
   only the locally loaded declared image, the exact experiment root is
   removed and absent, and global Fortlet inventory is unchanged.
7. A failed unit is terminal. Reject the candidate on any integrity,
   readability, executability, read-only, isolation, inventory, or cleanup
   criterion; do not alter the fixture, mount policy, or thresholds in flight.

## Budget and Plan

Budget: 15 minutes total, one local image load, one control publication, one
control consumption, one candidate publication, one candidate consumption,
zero network, zero provider calls, zero harness calls, zero model calls, zero
real credentials, and $0. Stop on the first experiment-fatal anomaly or time
ceiling. Remove each capsule before creating the next; after terminal
measurements restore cleanup-only owner access on the two exact trees, remove
the exact isolated root, and prove absence.

## Rehearsal

On 2026-08-21 read-only inspection verified MicroSandbox 0.6.8 exposes
`strict`, `relaxed`, and `off` stat virtualization; `private` and `mirror` host
permission propagation; and `ro`, `nosuid`, and `nodev` mount options. The
packaged runtime manifest reports the declared reference, digest, architecture,
shell, and archive identity, and the archive is locally present. CLI help
accepted the declared command and mount-option grammar. No experiment root,
image, sandbox, VM, or fixture was created during rehearsal.

## Results

Preflight verified MicroSandbox 0.6.8, the exact
`8acd80db302c20c3cfe626b0cdebf8899ead35ae1e7d482723ba8efdbd101697`
OCI archive SHA-256, the declared manifest digest, an absent exact experiment
root, and empty isolated image and capsule inventories. The archive loaded
locally as the declared 99.5 MiB image without network access. Read-only global
Fortlet inventory contained the one pre-existing unrelated stopped capsule
named by the handoff.

The first control-publisher start was denied by the outer execution sandbox
with `Operation not permitted` before the VM or fixture ran. It took 0.05
seconds and left one stopped empty capsule. Following the execution
environment's required permission replay, that exact capsule was removed, the
empty control directory was verified, and the identical predeclared command
was run with host VM permission. No experimental input or threshold changed.

The control publisher then exited zero in 0.48 seconds and reported the
declared guest modes:

| Control entry | Guest mode | Literal host mode | Host mode after sealing |
| --- | ---: | ---: | ---: |
| root | `0755` | `0700` | `0500` |
| `bin` | `0755` | `0700` | `0500` |
| `share` | `0755` | `0700` | `0500` |
| `bin/probe` | `0755` | `0600` | `0400` |
| `share/data` | `0644` | `0600` | `0400` |

Every entry carried MicroSandbox's `user.msb.override_stat` xattr, and the
relative symlink target remained `probe`. The exact current-style host sealer
removed write bits. The strict/private read-only consumer then failed in 0.06
seconds before executing the probe:

```text
error: failed to start "f55-control-consume"
  → mount: mount layer_ae137427: Permission denied (os error 13)
  → the host path is not readable by msb (check permissions)
```

This reproduced Experiment 0054 and passed the attribution gate.

The candidate publisher exited zero in 0.29 seconds. Its trusted finalizer
reported exactly `0555` for the root, both directories, and executable, and
`0444` for data. It retained the relative symlink and found no special file.
Literal host inspection after capsule removal, however, reported `0755` for
the root, both directories, and executable, and `0644` for data. Every entry
again carried the stat-override xattr. The two file SHA-256 values were
`97e605bd53b3d47c8c9462fa174267c1961e3f44633cf7388a0003d4f54ab9e9`
and `ee5075a1707aea4f4d06583c57087a69955f856cee6358424f3e8fea706f5978`.

Read-only inspection of the exact MicroSandbox 0.6.8 source attributed the
divergence to an intentional safety property, not an ignored mount option.
`HostPermissions::Mirror` mirrors ordinary guest bits only after OR-ing an
owner-access floor of `0700` for directories and `0600` for regular files so
the host process cannot lock itself out. Guest `0555` therefore becomes host
`0755`, and guest `0444` becomes host `0644`. Because acceptance criterion 3
required literal canonical host modes, the candidate was rejected immediately
and `f55-candidate-consume` was never created.

Both completed and failed capsules were removed and isolated capsule inventory
was empty. Before cleanup, isolated image inventory contained only the exact
locally loaded image. Cleanup-only owner access was restored on the two exact
fixture trees, `/private/tmp/f55` was removed and proved absent, and global
Fortlet inventory exactly matched baseline. No network, project, provider,
harness, model, credential, identity, remote, source, package, or publication
operation occurred.

After terminal cleanup, the complete `aarch64-darwin` standard verification
set passed through `nix develop`: 102 unit tests and every enabled integration
test, formatting, strict all-target/all-feature Clippy, conformance, the Nix
package, and shim activation were green. The two stock-Codex compatibility
tests remained explicitly ignored because they require an external binary, and
Nix reported only the expected incompatible `x86_64-linux` omission.

## Terminal Closure

1. Outcome: rejected; the control reproduced, but mirrored publication could
   not make guest-finalized non-writable modes literal on the host.
2. Root cause: MicroSandbox 0.6.8 intentionally ORs a `0700` directory or
   `0600` regular-file owner-access floor into mirrored host modes. The proposed
   guest-only finalizer therefore cannot be the sole authority for a literally
   sealed host tree.
3. Actual cost: about 2 minutes 55 seconds of live work versus the 15-minute
   ceiling, one locally loaded image, one outer-sandbox-denied pre-VM start,
   one control publication, one expected-failed control consumption, one
   rejected candidate publication, no candidate consumption, zero network,
   provider, harness, model, or credential calls, and $0.
4. Cleanup: all exact capsules and the exact isolated root were removed,
   isolated inventory was empty, and global inventory was unchanged.
5. Next action: stop for operator review. The smallest successor is a distinct
   experiment that retains mirrored guest creation and validation, then applies
   a trusted host-side canonical seal to remove only MicroSandbox's deliberate
   owner-write floor before testing the relaxed read-only consumer. No such
   successor, production fix, lifecycle retry, schema-2 work, or model dispatch
   is authorized by this closure.
