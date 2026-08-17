# FIP-0006: Opt-in project environments

Status: Review
Recorded: 2026-08-17 for operator review
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
