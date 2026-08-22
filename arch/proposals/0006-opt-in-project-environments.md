# FIP-0006: Opt-in project environments

Status: Accepted
Recorded: 2026-08-17 from the operator-validated project-environment design
Requires: FIP-0001

## Summary

A project may opt into one immutable tool layer through a manifest and fixed
adjacent recipe at its resolved root. Fortlet snapshots both files and executes
the recipe only inside a credential-free provisioning capsule, then mounts the
verified output read-only into every project+harness capsule. Projects without
the manifest retain the existing base and harness layers unchanged.

## Motivation

Fortlet can isolate Codex and Tact, but its capsules currently contain only the
base operating system and the selected harness. An agent can edit a Rust, Go,
or Python repository yet cannot assume that the repository's compiler, version
control client, or test tools exist. This is the main barrier between the
current product surface and ordinary daily use.

Project tooling also introduces a new trust boundary. Running a repository
setup script on the host would violate Fortlet's host authority model. Mounting
a mutable tool directory would make capsule identity unreliable, while putting
provider credentials or the live repository into a provisioning capsule would
give setup code authority it does not need.

## Decision

Use an isolated project-layer recipe for the first project-environment
contract. A root manifest declares guest path entries and literal environment
variables; a fixed adjacent file contains one POSIX shell recipe. Fortlet
reads, validates, snapshots, and hashes both files on the host but never
executes them there. The exact in-memory recipe snapshot runs in a dedicated
MicroSandbox provisioning capsule with only an empty writable output mount.

The successful output is published atomically under an identity derived from
both files and is mounted read-only at `/opt/fortlet/project`.
Project-environment identity becomes part of capsule configuration, so a
configuration change uses the existing explicit stop and reset lifecycle
rather than mutating a running capsule.

This chooses a guest-only recipe over a project OCI image or Nix activation
because it reuses Fortlet's existing immutable-layer boundary without requiring
users to build and publish images, log into a registry, install Nix, or bridge
host-platform Nix closures into a Linux guest.

## Specification

### Discovery and manifest

1. Fortlet MUST recognize only `.fortlet/environment.json` at the final
   canonical project root. It MUST NOT search outside that root or activate
   environment files from a broader ancestor.
2. Absence of the manifest MUST preserve the current base-plus-harness capsule
   configuration and MUST NOT create a project layer or provisioning capsule.
3. The initial manifest schema MUST contain exactly one schema version, an
   ordered bounded list of layer-relative `PATH` entries, and a bounded map of
   literal environment variables. Unknown fields and unsupported schema
   versions MUST fail closed.

   ```json
   {
     "schema": 1,
     "path": ["bin"],
     "environment": { "RUST_BACKTRACE": "1" }
   }
   ```

   `path` and `environment` MAY be empty but MUST be present. No other initial
   fields are defined.
4. Manifest presence MUST require `.fortlet/environment.sh` at the same root.
   Both fixed paths MUST resolve to readable regular files inside the canonical
   project root; special files and links resolving outside the root MUST fail
   before provisioning.
5. Fortlet MUST read the manifest and recipe into owned immutable snapshots and
   derive identity from their length-delimited exact bytes,
   project-environment schema, provisioning base, and target guest platform.
   Parsing and provisioning MUST consume those snapshots rather than reopen
   live project paths.
6. Repository configuration MUST NOT interpolate host environment variables,
   execute command substitutions, source host files, name additional input
   files, or discover implicit inputs.

### Isolated provisioning

1. The recipe MUST execute as POSIX shell text only inside a dedicated
   MicroSandbox provisioning capsule. The host process MUST NOT execute or
   source the recipe, a hook, or generated repository text.
2. The provisioning capsule MUST receive only the exact recipe text, an empty
   writable `/out`, `FORTLET_OUTPUT=/out`, a fixed target-platform value, and
   the network behavior declared by this FIP. It MUST NOT mount the live
   project, the manifest, a home directory, harness state, Fortlet state, a
   host package store, or a host socket.
3. Provisioning MUST NOT receive provider secrets, a credential broker, SSH or
   signing material, publication credentials, registry credentials, or host
   environment values. The first contract MAY use unauthenticated public
   network access; authenticated downloads and private overlays require a later
   FIP.
4. The recipe MUST install its complete layer-relative output beneath `/out`
   and MUST exit successfully. Fortlet MUST reject missing declared path
   directories, absolute or escaping links, unsupported output file types, or
   output that exceeds declared implementation bounds.
5. Fortlet MUST compute and record an output-tree digest after validation. A
   published marker MUST bind the input identity, output digest, schema,
   provisioning base, and target platform. Fortlet MUST verify that binding and
   output digest before every reuse or mount; the marker itself is excluded
   from the digested tree. The digest MUST cover each normalized relative path,
   file type, executable permission, file bytes, and link target.
6. Publication MUST use an owned temporary directory and atomic rename under an
   identity-specific lock. No capsule may observe a partial output. An existing
   published identity with a missing or invalid marker MUST fail closed rather
   than be overwritten.
7. A recipe failure, validation failure, interruption, or changed identity MUST
   leave every previously published layer intact. Temporary provisioning state
   and the owned provisioning capsule MUST receive bounded cleanup without
   deleting a prior layer.

### Activation and reconciliation

1. Fortlet MUST mount a selected project layer read-only at
   `/opt/fortlet/project` in addition to the existing immutable base and harness
   layers. It MUST NOT replace the capsule's image or persistent harness home.
2. Each manifest `PATH` entry MUST be normalized beneath the project layer and
   prepended in declared order to Fortlet's fixed guest `PATH`. The harness
   executable itself MUST continue to launch by its absolute Fortlet-owned
   path.
3. Manifest environment values MUST remain literal strings. Fortlet MUST reject
   invalid names and MUST reserve `HOME`, `PATH`, terminal-forwarded names,
   credential-broker names, Fortlet-owned names, and harness-owned names from
   project override.
4. The same validated project paths and variables MUST be applied independently
   to Codex and Tact capsules. Harness adapters MAY add their own environment
   afterward but MUST NOT receive project authority over credentials or
   persistent-state ownership.
5. Capsule labels and launch validation MUST include either the selected
   project-environment identity or an explicit no-environment value. A mismatch
   MUST fail closed with the established `fortlet stop <harness>` followed by
   `fortlet reset <harness>` recovery guidance.
6. Provisioning MUST complete before Fortlet creates or mutates a capsule for
   the new identity. A failed update MUST leave an existing capsule and its
   prior layer usable after the project manifest is restored.

### Evidence and user contract

1. Errors MUST identify the project-environment stage and one actionable next
   step without exposing internal capsule names, project identity hashes,
   credential values, or recipe output that may contain terminal control data.
2. Automated evidence MUST cover manifest absence and validation, recipe
   containment, snapshot consistency, complete identity inputs, provisioning
   mounts and credential absence, output validation and digest, atomic
   publication, concurrent ensure, failed updates, activation ordering,
   reserved variables, both harnesses, and stale-capsule reconciliation.
3. A repository fixture MUST establish that the project layer can provide
   `cargo`, `rustc`, and `jj` and execute a focused Fortlet test inside the
   guest boundary.
4. One predeclared bounded local experiment MUST exercise the immutable public
   launch path with Fortlet's own manifest. It MUST record exact ownership and
   cleanup, whether the unit succeeds or fails, and MUST NOT silently retry.
5. Documentation MUST explain opt-in discovery, first-use provisioning,
   immutable updates, stop/reset recovery, network behavior, the absence path,
   and that repository recipes execute in a guest but remain trusted to define
   commands available to the coding agent.

## Consequences

Fortlet gains a practical toolchain path without changing installation or
requiring a registry. Repository-controlled setup code is still code execution,
but its authority is limited to a disposable guest, its own snapshotted text,
public network access, and an empty output. The resulting tools are trusted by
the agent inside the project capsule and therefore must remain an explicit
project choice.

The two-file identity makes updates deterministic with respect to every
project-controlled provisioning input, while the output digest detects
corruption and records the actual published tree. Public network content can
still drift unless the recipe pins and verifies it; the first contract
documents that limitation instead of claiming cross-machine bit
reproducibility. Previously successful outputs are retained, so failure cannot
destroy the last working layer, but selecting an older layer is initially done
by restoring its manifest and recipe rather than a new rollback command.

The fixed mount and literal environment model is intentionally less expressive
than devcontainers, Nix modules, or arbitrary activation scripts. Services,
mutable package installation, authenticated inputs, and private overlays remain
separate design problems.

## Alternatives Considered

1. A project-selected OCI image pinned by digest gives the strongest existing
   distribution identity and avoids setup scripts during launch. It also makes
   every project build and publish an image, introduces registry lifecycle and
   possible login, changes the whole capsule base rather than adding tools, and
   provides no small path for Fortlet's own currently unpublished toolchain.
2. Nix flake or devenv activation fits this repository and can provide strong
   dependency closure. Making it the first runtime mechanism would require Nix
   knowledge or host-to-Linux closure plumbing, conflict with FIP-0001's
   default optional-Nix direction, and make a macOS development setup part of
   ordinary capsule startup.
3. A new declarative package DSL could prohibit arbitrary shell recipes, but
   Fortlet would immediately need to own package resolvers, artifact formats,
   checksums, platform selection, and error recovery across language
   ecosystems. That is larger than proving one useful environment layer.
4. Allowing the recipe to name arbitrary repository inputs or mounting the live
   project read-only supports lockfiles and arbitrary build systems, but makes
   snapshotting, containment, and cache identity substantially larger. Baking
   tool versions and checksums into the two fixed files is enough for the first
   toolchain slice.
5. Installing tools into the persistent harness home is simple but mixes
   project and harness ownership, permits mutation after activation, and makes
   failed updates capable of damaging the only working environment.

## Open Questions

1. Whether successful use justifies a declarative artifact schema that can
   disable provisioning network access entirely.
2. Whether a later schema should admit declared lockfiles or other snapshotted
   repository inputs without exposing the live project.
3. Whether private overlays need a host broker analogous to provider
   credentials or should use prebuilt authenticated artifacts.
4. Whether explicit prepare, inspect, rollback, garbage-collection, and update
   commands are warranted after the implicit first-use path is proven.
5. Whether project services should share one environment across harnesses or
   require a separate project workload capsule and lease model.

## Amendment — 2026-08-21 portable sealed layer permissions

Amendment status: Accepted. Operator-accepted after Experiments 0054 and 0055
rejected implicit and guest-only permission handling, and Experiment 0056
accepted the complete mirrored-publication, trusted-seal, and
relaxed-consumption mechanism.

This amendment changes only schema-1 output publication identity, finalization,
sealing, verification, and runtime mount policy. The public manifest remains
schema 1, repository recipes retain the same inputs and authority, and every
other discovery, snapshot, digest, atomicity, activation, credential, failure,
and user contract in this FIP remains unchanged.

### Publication contract

1. Schema-1 output publication MUST have an internal versioned contract
   identity distinct from the public manifest schema. That version MUST
   participate in the published-layer identity and marker binding. A layer
   produced under an older permission contract MUST NOT be selected as a cache
   hit for the amended contract.
2. The provisioning capsule's writable `/out` bind MUST use strict stat
   virtualization with mirrored host permissions. Mirroring applies only to
   the owned empty output root; it MUST NOT broaden permissions on the project,
   persistent state, credentials, sockets, or any other host path.
3. After the snapshotted repository recipe exits successfully, Fortlet MUST run
   a separate package-owned finalizer inside the same credential-free capsule.
   No repository-controlled command may run after that finalizer. The finalizer
   MUST operate only beneath `/out`, follow no symlink during traversal, and:

   - reject device nodes, FIFOs, sockets, and every other unsupported type;
   - preserve validated symlink targets without changing symlink modes;
   - set every real directory to `0555`;
   - set a regular file to `0555` when any executable bit was present, otherwise
     to `0444`; and
   - remove every write, set-user-ID, and set-group-ID bit.

4. Fortlet MUST stop or remove the provisioning capsule before trusted host
   validation or sealing begins. No guest process may retain the output mount
   while the host publishes it.
5. Before changing host modes, Fortlet MUST validate the exact owned temporary
   tree, declared paths, relative non-escaping links, supported types, output
   digest, and the literal host modes resulting from the finalized guest modes
   plus the pinned MicroSandbox owner-access floor. For MicroSandbox 0.6.8,
   those pre-seal host modes are exactly `0755` for real directories and
   executable regular files and `0644` for non-executable regular files. Any
   other mode or type MUST fail closed before publication.
6. The trusted host MUST write and sync the marker at ordinary non-executable
   file mode, then remove only write bits recursively from non-symlink entries
   beneath the exact validated temporary root. It MUST NOT add readability or
   executability, interpret repository text, follow symlinks, or mutate an
   existing published layer.
7. Before atomic rename, Fortlet MUST revalidate the complete tree. Every real
   directory and executable regular file MUST be exactly `0555`; every other
   regular file, including the marker, MUST be exactly `0444`; links, types,
   and the output digest MUST match the pre-seal record; and no entry may be
   writable. Publication MUST fail closed on any mismatch.
8. Reuse verification MUST enforce the amended contract identity, marker
   binding, canonical literal modes, supported types, links, declared paths,
   and output digest. It MUST reject owner-only `0500`/`0400` trees and other
   legacy or malformed outputs rather than treating non-writability alone as a
   valid seal.

### Runtime consumption

1. The schema-1 layer MUST be mounted with explicit read-only, `nosuid`, and
   `nodev` options, relaxed stat virtualization, private host-permission
   propagation, and no root-symlink following.
2. Relaxed stat virtualization MAY consume MicroSandbox stat-override metadata
   when present and MUST tolerate its absence by falling back to the canonical
   literal host modes. It does not relax Fortlet's publication verification,
   path containment, supported-type, or digest requirements.
3. The host and guest read-only enforcement MUST reject content writes,
   creation, removal, rename, and chmod through the runtime mount. A numeric
   non-root capsule user MUST be able to traverse declared directories, execute
   canonical executable files, follow validated relative links, and read
   canonical ordinary files.

### Failure, performance, and evidence

1. Recipe, guest-finalization, capsule-cleanup, pre-seal validation, marker,
   seal, or post-seal validation failure MUST preserve every previously
   published layer and MUST NOT publish the temporary tree. Cleanup MAY restore
   owner access only on that exact owned temporary root after terminal evidence
   has been retained.
2. Guest finalization and host validation/sealing are cold-preparation work.
   A verified cache hit and prepared launch MUST perform no traversal, chmod,
   marker rewrite, capsule creation, or layer mutation.
3. Automated evidence MUST cover executable-intent normalization, ordinary
   files, directories, links, hostile names, unsupported types, special bits,
   exact pre-seal and post-seal modes, marker mode, digest preservation,
   publication-contract invalidation, rejection of legacy owner-only layers,
   cleanup preservation, and the exact provisioning and runtime mount policies.
4. Experiment 0056 establishes direct mechanism feasibility for the pinned
   MicroSandbox release. Adoption still requires the credential-free Fortlet
   preparation and absent/stopped/running lifecycle evidence and latency limits
   required by FIP-0013.

The owner-access floor remains a useful MicroSandbox safety property while a
guest is actively publishing. Trusted host sealing occurs only after that guest
has lost the mount, so removing the floor at publication does not weaken the
host process during guest execution. Canonical literal modes also keep the
layer usable if stat-override xattrs are unavailable, while relaxed runtime stat
handling avoids the strict writable-xattr probe that rejected the sealed tree.
