# FIP-0013: Nix-built project runtime substrate

Status: Accepted
Recorded: 2026-08-21 from the operator-approved runtime-substrate direction
Requires: FIP-0001, FIP-0006, FIP-0007, FIP-0012

## Summary

Replace Fortlet's general-purpose Node/Debian base and separately installed
tool directories with one small Nix-built OCI runtime and one isolated Nix
store per project. The runtime image supplies the exact closure needed to boot,
prepare, inspect, and launch every supported harness. Fortlet seeds that closure
into the project store, adds only the selected harness closure, and lets a
schema-2 provider add the locked project closure to the same store. Runtime
capsules mount the completed store read-only.

Harness packages are pinned build artifacts, not language-package-manager
installation scripts. Codex, Tact, and future harnesses use the same runtime
image; selecting a harness changes its closure and adapter contract, not the
image. Prepared launch remains a sub-second operation, while cold preparation
has separately measured image, seed, harness, and provider phases.

## Motivation

The current `node:24-bookworm` image provides substantially more policy than
Fortlet needs while still requiring Fortlet to reconstruct basic operating
state through `apt`, install Codex through `npm`, fetch Tact independently, and
bootstrap Nix again for schema 2. Experiment 0053 proved the corrected Debian
package lifecycle, but it also made the layering problem visible: one selected
language runtime is acting as the common operating substrate, and multiple
package mechanisms independently assemble related immutable tools.

FIP-0012 already gives each project an isolated `/nix` store. Mounting that
store over a Nix-built image would hide the image's own store, including the
paths needed to boot it. The runtime closure must therefore be an explicit
seeded input to every project store rather than an accidental dependency on
the unmounted image filesystem.

The desired daily-use path must also retain the lifecycle latency established
by Experiments 0043 and 0045. Nix evaluation, package installation, artifact
download, and store mutation belong in `prepare`, never in a prepared launch.

## Decision

Fortlet owns one Nix expression that produces a minimal Linux OCI image for
each supported guest architecture. The build also produces a manifest of the
image's runtime closure and exact boot paths. The installation supplies the OCI
archive locally; Fortlet loads it through MicroSandbox's image API, verifies
the manifest digest, and launches it with network pulling disabled. A public
anonymous artifact MAY be an additional delivery mechanism, but ordinary use
MUST NOT require a registry account, registry credential, Docker daemon, or
host Nix.

Before any capsule mounts a project store at `/nix`, a credential-free seed
capsule boots from the image's unhidden store and copies the declared runtime
closure and store metadata into an owned project store. Fortlet then imports or
realizes the selected adapter's pinned harness closure. Schema-2 provider
preparation uses the already-seeded Nix runtime to archive, evaluate, and
realize the locked project closure. Schema-1 recipes retain their separate
validated output layer and run against the same fixed runtime-tool contract.

The resulting project store may be written only by credential-free preparation
capsules. Managed harness capsules mount it read-only. Project stores are never
shared across projects, although the immutable OCI cache and verified public
input artifacts may be shared as sources from which isolated stores are
materialized.

This FIP replaces FIP-0012's Debian common-base construction, per-harness
provisioning scripts, and in-provider Nix installation mechanisms. It preserves
FIP-0012's provider selection, bounded activation, inspection, identity,
credential, workspace, and reset contracts.

## Specification

### Runtime artifacts and identity

1. Fortlet MUST build one OCI runtime definition through pinned Nix inputs for
   each supported Linux guest architecture. Supported harnesses MUST NOT select
   different base images.
2. The runtime closure MUST contain the fixed boot and hold commands, POSIX
   shell, Nix runtime, CA roots, Git, download and archive bootstrap tools,
   managed-shell activation, and only the additional system utilities required
   by Fortlet's runtime contract.
3. Node, npm, and other language runtimes or package managers MUST NOT be part
   of the common contract merely because one harness uses their ecosystem.
   They MAY appear only when required by a selected, pinned harness or project
   closure.
4. The build MUST produce an OCI archive, its OCI manifest digest, and a
   canonical runtime-seed manifest. The seed manifest MUST bind the guest
   architecture, Nix inputs, store paths, boot and hold paths, managed-shell
   path, common-tool paths, CA bundle, and complete closure identity.
5. Fortlet MUST load the archive through MicroSandbox's local image interface,
   verify the cached image's manifest digest and platform, and use a local
   immutable reference with pull policy `never`. A mismatch or absent verified
   artifact MUST fail before project-store or capsule mutation.
6. The user-facing Fortlet installation MUST carry, or fetch anonymously by
   exact digest, every runtime and harness artifact required for its declared
   guest platform. It MUST NOT require a Docker daemon, registry login, host
   Nix installation, mutable tag, or authenticated package source.
7. Runtime image digest, seed-manifest identity, selected harness closure,
   project-provider identity, and target guest platform MUST participate in
   preparation and capsule configuration identity.

### Project-store assembly

1. Every resolved project MUST receive one Fortlet-owned Nix store, including
   projects with schema 1 or no project provider. Codex and Tact MAY share that
   store only when they resolve to the same project identity.
2. A new project store MUST be seeded before it is mounted at guest `/nix`.
   The seed capsule MUST boot from the verified OCI image without a `/nix`
   override and receive only an owned writable staging store, fixed manifest,
   disposable home and temporary state, and the inputs required to copy and
   verify the runtime closure.
3. Seeding MUST copy exactly the declared runtime closure and required Nix
   store metadata, then prove that every declared boot, shell, common-tool, CA,
   and Nix path resolves through the staged store. Publication MUST be atomic;
   partial staged state MUST never become launchable.
4. Shared OCI and public artifact caches MAY avoid repeated downloads and MAY
   use filesystem cloning or equivalent copy acceleration. A resulting project
   store MUST have independent ownership and mutation state and MUST NOT reveal
   another project's source, closure additions, garbage-collection roots, or
   inventory.
5. Only credential-free preparation capsules MAY write a project store. A
   managed harness capsule MUST mount the verified store read-only at `/nix`.
   The host MUST NOT execute repository-controlled text or provider evaluation
   while assembling it.
6. Store mutation MUST serialize by project and selected input identity. A
   failed seed, import, realization, or verification MUST preserve the last
   verified runtime, harness, and provider records. Unreferenced additions MAY
   remain for later bounded garbage collection but MUST NOT become selected
   without a complete verified record.
7. Stop and start MUST retain the project store. FIP-0004 reset MUST retain it.
   Destructive store purge, global inventory, and cross-project garbage
   collection require a separate public contract.

### Harness closures

1. Each harness adapter MUST select a pinned Nix derivation or equivalently
   content-addressed Nix closure for every supported guest architecture. Its
   executable path, closure identity, version, persistent paths, launch
   environment, and credential policy remain adapter-owned.
2. Harness construction MAY consume a language ecosystem upstream, but normal
   Fortlet preparation MUST import or realize a prebuilt verified closure. It
   MUST NOT run `npm`, another language package installer, or an upstream
   install script to populate a mutable output directory.
3. Preparing one harness MUST add only that harness's closure and MUST NOT
   download or expose another harness's executable or private state.
4. A verified harness cache hit MUST perform no network request, Nix
   evaluation, closure import, store mutation, or provisioning-capsule launch.
5. Updating one harness MUST change its closure identity without changing the
   common OCI image unless the runtime contract itself changed. The prior
   verified harness closure MUST remain usable after a failed update.
6. The absolute harness executable MUST resolve inside the verified project
   store. The separate `/opt/fortlet/tool` mount and adapter provisioning
   script cease to be runtime requirements under this FIP.

### Project providers and schema compatibility

1. FIP-0006 schema 1 MUST retain its strict manifest, snapshot, guest-only
   recipe, output validation, publication, and `/opt/fortlet/project`
   activation contracts. Its fixed provisioning base identity MUST change to
   the Nix-built runtime contract.
2. The schema-1 provisioning contract MUST guarantee only the declared common
   runtime tools. A repository recipe MUST NOT rely on Debian `apt`, npm, or
   another undeclared package manager. Existing public fixtures in this goal
   MUST be updated to the new explicit contract rather than emulating the old
   image.
3. FIP-0012 schema-2 preparation MUST use the Nix executable and store metadata
   already present in the seeded project store. It MUST NOT download a Nix
   distribution, run the Nix installer, install operating-system packages, or
   create a second Nix store.
4. Schema-2 project archiving, locked evaluation, realization, bounded
   activation, record identity, cache behavior, and failure redaction remain
   governed by FIP-0012. Provider preparation MAY add closures to the project
   store but runtime launch MUST NOT evaluate or realize them.
5. No-provider, schema-1, and schema-2 projects MUST use the same runtime image,
   seed mechanism, selected-harness mechanism, and runtime read-only mount.

### Preparation and launch performance

1. `prepare` MUST own all image loading, store seeding, closure import, Nix
   evaluation, provider realization, and writable store access. Automatic
   first launch MAY invoke the same ensure operation before credential and
   managed-capsule reconciliation, as required by FIP-0007.
2. A fully prepared launch MUST perform no image pull, artifact download,
   provisioning-capsule creation, Nix evaluation, closure import, provider
   realization, or project-store write.
3. Repeated `prepare` for an unchanged plan MUST work without network access,
   create no capsule, rewrite no marker or store path, and have a sub-second
   median on the declared reference host.
4. Bounded lifecycle evidence MUST measure unchanged prepared launches from
   absent, stopped, and running capsule states. Every state MUST retain a
   sub-second median, and every running-capsule sample MUST remain below one
   second. Any slower sample MUST be retained and attributed rather than
   discarded.
5. Cold preparation MUST emit opt-in bounded timing events for local image
   verification or load, runtime-store seeding, harness closure preparation,
   schema-1 layer preparation or schema-2 provider realization, and final
   verification. Events MUST follow the existing startup-diagnostic redaction
   and terminal-safety contract.
6. Before adoption, one predeclared feasibility experiment MUST measure the
   existing mechanism as a control and the candidate on the same declared
   platform and network condition. It MUST record elapsed time for every cold
   phase, repeated preparation, prepared absent/stopped/running launches,
   downloaded and materialized bytes where available, and artifact/store disk
   size.
7. The experiment MUST declare a finite cold-preparation ceiling before the
   candidate run. Exceeding that ceiling or any prepared-launch requirement
   rejects adoption but does not require abandoning the architecture; a later
   optimization MUST receive a fresh experiment identity.

### Failure behavior and evidence

1. Failures MUST identify image, runtime seed, harness closure, project
   provider, or prepared launch as the failing stage and provide one actionable
   correction without exposing host paths, project identities, store hashes,
   environment values, personal identity, or credentials.
2. Automated evidence MUST cover artifact and platform identity, local
   no-pull image selection, seed completeness, atomic publication, project
   isolation, selected-harness closure, no package-manager installation,
   schema-1 compatibility, seeded schema-2 use, read-only runtime stores,
   cache-hit idempotence, stale-capsule reconciliation, and failure
   preservation.
3. Build evidence MUST prove the OCI archive and runtime manifest are
   reproducible for every declared guest architecture and that their runtime
   closure contains no undeclared harness or language-runtime dependency.
4. The feasibility campaign MUST remain credential-free and use only public
   project and artifact identities. Provider and model work require their own
   authority after the substrate is accepted by evidence.
5. Conformance MUST remain partial until both supported guest architectures,
   the performance requirements, one schema-1 path, both harness closures, and
   one live schema-2 preparation have passed their declared evidence.

## Consequences

Fortlet has one reproducible Linux substrate instead of inheriting Node and
Debian package policy from a harness convenience image. Nix becomes an internal
runtime and packaging mechanism, but users do not need host Nix and projects do
not need to adopt Nix. Harness selection remains adapter-owned while ordinary
preparation no longer depends on npm or a per-harness installer.

Every project pays for an isolated copy of the runtime closure. That increases
initial materialization and disk use compared with mounting one global mutable
store, but it keeps untrusted project evaluation and source archives from
crossing project boundaries. Shared public source artifacts and filesystem
cloning can improve cost later without changing the isolation model.

The OCI image and project store deliberately contain overlapping runtime
content: the image must boot before `/nix` is mounted, and the project store
must contain those same paths after the mount hides the image store. The
feasibility experiment decides whether that duplication is acceptable before
adoption. Later snapshot or disk-image optimizations may remove physical
duplication only if they preserve logical store isolation and the latency
contract.

Schema-1 recipes lose accidental access to Debian's package manager. Their
documented contract becomes smaller and more portable: fixed shell and
bootstrap tools, public network when authorized, and an empty output. The
Fortlet fixture must demonstrate that this remains useful.

## Alternatives Considered

1. Keep `node:24-bookworm` and correct only its common tools. Rejected because
   it retains a harness-oriented language runtime, repeated package mechanisms,
   and a second Nix bootstrap without a product requirement for Node or npm.
2. Choose a different conventional distribution image. This could improve
   size or package availability but preserves the same mutable package-manager
   assembly and does not solve the hidden `/nix` store or harness packaging
   model.
3. Use a full NixOS system image. Rejected because Fortlet overrides init and
   entrypoint behavior and needs a small OCI root filesystem, not systemd,
   activation, services, or a configured machine.
4. Use one image per harness. Rejected because common runtime updates would
   multiply images, capsule identity would conflate runtime and harness
   selection, and projects using multiple harnesses would pull duplicate bases.
5. Put all harnesses in one image. Rejected because preparing one harness would
   download and expose unselected tools and make harness updates change the
   shared base.
6. Share one writable Nix store globally. Rejected by FIP-0012 because project
   source enumeration, untrusted mutation, corruption, and garbage collection
   would cross project boundaries.
7. Run host Nix and bind its store. Rejected because ordinary use would require
   host Nix, Darwin cannot supply the selected Linux closure directly, and host
   evaluation would violate the repository-code boundary.
8. Require a public OCI registry. Rejected because MicroSandbox can load local
   OCI archives and Fortlet's charter forbids a required registry account or
   login. Anonymous digest-pinned distribution may remain an optimization.

## Open Questions

1. Whether runtime seeding should use direct verified copying, a NAR export, a
   preformatted store disk, or a MicroSandbox snapshot after the first
   feasibility measurements.
2. Whether installation should bundle harness closure archives or retrieve
   them anonymously from a content-addressed cache.
3. Which filesystem clone or sparse-image mechanism can reduce per-project
   physical storage without introducing cross-project mutation or inventory.
4. Whether cold-preparation measurements justify cancellation and structured
   progress beyond the initial opt-in timing events.
5. When public store inventory, size reporting, pruning, or destructive purge
   becomes necessary.
