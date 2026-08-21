# FIP-0012: Resolved provider-backed project environments

Status: Accepted
Recorded: 2026-08-21 from the operator-accepted portable project-capability design
Requires: FIP-0001, FIP-0004, FIP-0006, FIP-0007
Extended-By: FIP-0013

## Summary

Extend Fortlet's opt-in project environment into one harness-neutral resolved
plan. FIP-0006's isolated recipe remains schema 1. Schema 2 explicitly selects
one locked Nix dev shell that Fortlet prepares inside a credential-free guest,
captures as a bounded activation, and reuses through a project-scoped Nix
store. A read-only command reports the selected and prepared plan without
printing values or internal host identities.

Fortlet also supplies minimum common version-control tooling and projects only
the operator's effective name and email through generated Git and Jujutsu
configuration. Host configuration, credentials, signing, hooks, helpers, and
publication authority remain outside the capsule.

## Motivation

FIP-0006 proved that an isolated two-file recipe can provide real project
tools, but it requires every repository to hand-build a complete immutable
layer. That is a valuable escape hatch, not an ergonomic way to use the
environment definitions already maintained by ordinary projects.

The next risk is complete local work rather than another harness or a general
policy system. Codex and Tact need the same compilers, package managers,
version-control commands, literal activation, and reusable dependency state.
Users also need to see what Fortlet selected before spending a model prompt.

Nix dev shells are the smallest provider relevant to the operator's current
projects. Nix can resolve a locked Linux tool closure and expose its build
environment without making host Nix a product requirement. Its activation
surface is nevertheless broader than FIP-0006: it reads a project source tree,
owns a store and cache, and may contain shell functions or hooks. Those changes
need an explicit contract rather than an amendment that makes schema 1's
already-conformant isolation ambiguous.

## Decision

Keep `.fortlet/environment.json` as the sole opt-in point. Schema 1 retains
FIP-0006 exactly. Schema 2 contains only an explicit `nix-dev-shell` provider
and a dev-shell name. It requires root `flake.nix` and `flake.lock`; Fortlet
does not infer Nix, devenv, Dev Containers, language ecosystems, or competing
environment files.

Fortlet resolves schema 2 in a dedicated credential-free preparation capsule.
A pinned single-user Nix runtime archives the read-only project into a
project-scoped `/nix` store, evaluates only that content-addressed source, and
realizes `devShells.<guest-system>.<name>`. Fortlet captures only exported
scalar variables, discards non-exported Nix and stdenv functions, arrays, and
variables as inert provider metadata, rejects executable activation, protected
names, and host-only paths, and binds the resulting activation and derivation
to the project environment identity. Launch mounts the verified project store
and applies the captured activation directly; it does not evaluate Nix or run
activation shell text again.

Add `fortlet plan [--project <path>] [--allow-broad-mount]` as the read-only
inspection surface. It reports stable public classes and names, not environment
values, host paths, identity values, or internal hashes.

The common base supplies Git and bootstrap utilities. Fortlet reads only the
effective host `user.name` and `user.email`, writes separate minimal Git and
Jujutsu configuration artifacts, and mounts them read-only. The guest receives
no host configuration file or signing and publication authority.

## Specification

### Plan and discovery

1. Fortlet MUST resolve one `ProjectCapabilityPlan` before preparation or
   launch. The plan MUST be independent of a harness and MUST contain the
   canonical project, target guest system, provider class, declared inputs,
   activation class, cache classes, identity-projection state, and unsupported
   requirements.
2. `.fortlet/environment.json` MUST remain the only project opt-in point.
   Absence MUST produce an explicit no-project-provider plan and preserve the
   existing base-plus-harness behavior.
3. Schema 1 discovery, snapshotting, identity, provisioning, validation,
   publication, activation, and failure behavior MUST remain exactly as
   specified by FIP-0006.
4. Schema 2 MUST contain exactly:

   ```json
   {
     "schema": 2,
     "provider": {
       "kind": "nix-dev-shell",
       "name": "default"
     }
   }
   ```

   `name` MUST be a bounded Nix attribute component. Unknown fields, kinds,
   versions, path selectors, registry references, and empty names MUST fail
   closed.
5. Schema 2 MUST require regular, readable root `flake.nix` and `flake.lock`
   files inside the canonical project and MUST reject an adjacent schema-1
   recipe as ambiguous. Missing locks, competing provider declarations, or an
   input that resolves outside the content-addressed project source MUST fail
   before provider preparation.
6. Fortlet MUST choose `devShells.<guest-system>.<name>` from its fixed guest
   target. The repository MUST NOT select a host system, arbitrary installable,
   impure evaluation, flake registry alias, or writable lock update.

### Credential-free Nix preparation

1. Fortlet MUST pin the Nix runtime version and distribution digest as part of
   the provider contract. Ordinary use MUST NOT require Nix on the host.
2. Each project identity MUST receive a Fortlet-owned persistent Nix store at
   guest `/nix`. It MAY be shared by Codex and Tact for that project but MUST
   NOT contain harness state, provider credentials, host configuration, SSH or
   signing material, or publication authority.
3. Provider archiving, evaluation, and realization MUST occur only in a
   dedicated provisioning capsule containing the pinned Nix runtime, the live
   project mounted read-only, the project Nix store, a disposable home and
   temporary directory, fixed target values, and public unauthenticated network
   access. It MUST NOT receive a writable project mount, harness state, a
   reusable managed capsule, host environment, provider broker, host sockets,
   or credentials.
4. Preparation MUST use the locked root flake without updating it, using an
   impure mode, consulting a mutable registry, or accepting authenticated
   substituters or inputs. A required private input, authentication, service,
   host platform, or unsupported feature MUST fail with one correction.
5. Fortlet MUST archive the read-only project through pinned Nix without
   updating its lock, then evaluate only the returned store source. It MUST
   resolve the selected dev shell once and bind provider identity to the
   schema, fixed target, pinned Nix runtime, archived source, locked inputs,
   resolved derivation, captured activation, and provider-store contract.
   Parsing, validation, and later launch MUST consume that resolved record
   rather than reopen live project inputs or provider output opportunistically.
6. Concurrent prepare and launch MUST serialize by resolved project/provider
   identity. Failure or interruption MUST preserve the prior verified record
   and store and MUST remove only owned temporary provisioning state.

### Bounded activation

1. Fortlet MUST capture the selected dev shell through the pinned Nix
   structured environment interface. It MUST accept only exported scalar
   variables with bounded names and values. It MUST discard non-exported
   variables, arrays, and Nix stdenv shell functions without executing,
   persisting, or projecting them.
2. A non-empty `shellHook`, command alias, service, task, explicitly required
   shell function, or other executable activation MUST fail as unsupported.
   Projects that require shell functions MUST use schema 1. Repository shell
   text MUST NOT execute on the host or during a credentialed launch.
3. `HOME`, `PATH`, `BASH_ENV`, terminal variables, Fortlet and MicroSandbox
   variables, credential variables, Git and Jujutsu configuration selectors,
   harness-owned variables, host paths, and other protected names MUST NOT be
   accepted from provider activation. Provider PATH entries MUST resolve under
   the verified project Nix store.
4. Launch MUST mount the verified project Nix store at `/nix`, prepend the
   captured provider PATH before Fortlet's fixed base and harness PATH, and
   apply accepted scalar variables before harness-owned variables. The harness
   executable MUST continue to launch by its absolute Fortlet-owned path.
5. `prepare` and automatic first launch MUST use the same plan and provider
   ensure operation. A verified cache hit MUST avoid provider network contact,
   evaluation, realization, marker rewrite, and managed-capsule creation.
6. The resolved provider identity MUST participate in capsule launch
   configuration. A changed plan MUST fail against an existing capsule with
   the established explicit stop/reset correction; it MUST NOT mutate or
   replace a running capsule silently.

### Inspection

1. Fortlet MUST provide:

   ```text
   fortlet plan [--project <path>] [--allow-broad-mount]
   ```

   Project selection and broad-root handling MUST match `run` and `prepare`.
2. Plan inspection MUST be read-only. It MUST NOT read provider credentials or
   personal identity values, contact the network, evaluate a provider, verify
   a content tree expensively, prepare a layer, create state, or inspect or
   mutate a managed capsule.
3. Output MUST report the project kind, guest target, provider class,
   declaration paths, selected dev-shell name where applicable, preparation
   state, activation variable names, cache classes, identity-projection state,
   and unsupported requirement classes in a stable bounded format. Identity
   state before a launch-time check MUST be `unchecked`, not inferred by
   reading personal values.
4. Output MUST NOT contain environment values, name or email values, host state
   or cache paths, provider output, project identity hashes, internal capsule
   names, or credentials. Unsafe project text MUST receive terminal-safe
   rendering.
5. An invalid declaration MUST fail at the project-environment stage with one
   actionable correction. A valid but unprepared plan MUST inspect
   successfully as unprepared without causing preparation.

### Common tools, identity, and mutable state

1. The common base MUST provide Git, CA certificates, download and archive
   bootstrap utilities, and the fixed managed-shell activation. Project
   compilers, language package managers, and Jujutsu MAY remain provider or
   schema-1 recipe tools.
2. Fortlet MAY read only the host's effective `user.name` and `user.email` for
   local checkpoint identity. It MUST use a trusted non-interactive read that
   disables includes, system configuration, pagers, aliases, hooks, helpers,
   and repository configuration. Missing or invalid values MUST produce no
   identity artifact and MUST NOT block an otherwise valid harness launch or
   trigger an invented or copied fallback.
3. Fortlet MUST generate separate minimal Git and Jujutsu configuration files
   containing only those two literal values. It MUST validate bounds, write
   them atomically as mode 0600 regular files outside the project, mount them
   read-only, and select them explicitly for guest commands.
4. Fortlet MUST NOT copy or expose a host `.gitconfig`, Jujutsu config, include,
   signing setting or socket, SSH command, credential helper, URL rewrite,
   alias, pager, filter, hook path, editor command, remote credential, or
   publication configuration. Generated identity values MUST NOT appear in
   plan output, diagnostics, tests, experiments, or committed artifacts.
5. Git and Jujutsu signing MUST remain disabled for guest-created checkpoints.
   Repository-local VCS data MAY be updated inside the mounted project, but no
   guest operation gains authority to publish it.
6. The mounted workspace and project provider store MUST survive capsule stop
   and start. Harness homes remain private per project+harness. Capsule-root
   mutation remains unpromoted and disposable under FIP-0004 reset; reset MUST
   NOT delete the project, project provider store, immutable layers, or
   persistent harness home.

### Evidence and failure behavior

1. Automated evidence MUST cover schema-1 compatibility, schema-2 strict
   parsing, locked root selection, guest-system selection, credential-free
   provisioning inputs, pinned runtime identity, provider-store ownership,
   structured activation filtering, protected variables and paths, cache-hit
   idempotence, concurrency, failure preservation, stale-capsule detection,
   inspection safety, and both harnesses.
2. Isolated CLI evidence MUST prove inspection and invalid declarations do not
   read credentials or identity values, contact a provider, create persistent
   state, or mutate runtime resources.
3. Automated evidence MUST prove only the two identity fields reach generated
   Git and Jujutsu configuration, signing is disabled, personal values stay
   redacted, and absent identity creates no projection or invented author while
   leaving unrelated harness use available.
4. One bounded experiment campaign MUST use Fortlet's schema-1 recipe and one
   exact public repository with schema 2. Across one Codex unit and one Tact
   unit it MUST prove useful edit, project verification, unsigned local
   checkpoint, stop/start reuse, exact diff inspection, no remote mutation,
   and owned cleanup.
5. A provider failure MUST identify the project-environment or environment
   stage and one correction without printing provider output containing
   control characters, host paths, environment values, identity values, or
   internal resource names.

## Consequences

Fortlet can consume one real repository-owned environment standard while
retaining its small isolated recipe. Provider behavior becomes visible before
model use, and Codex and Tact share one project contract rather than growing
harness-specific setup.

The first Nix provider is intentionally strict. It admits locked, scalar-only
dev shells, ignores inert Nix stdenv metadata, and rejects non-empty shell hooks,
required shell functions, services, tasks, private inputs, and impure
evaluation. Some existing flakes will therefore require a small Fortlet-facing
dev shell or must keep using schema 1. The pinned structured environment
interface is experimental upstream, so its version and fixtures become
compatibility obligations.

A project-scoped Nix store consumes more disk than a global shared store but
prevents one repository's untrusted evaluation and cache mutation from
crossing into another project. Stop/start retains useful downloads; reset
continues to mean disposable capsule-root recovery rather than cache purge.

Projecting literal identity makes ordinary local checkpoints usable without
bringing publication or signing authority into the guest. It also creates a
small host-to-guest personal-data flow, so values remain absent from every
public diagnostic and experiment artifact.

## Alternatives Considered

1. Extend schema 1 with a larger shell recipe. Rejected because it preserves
   hand-written download logic rather than consuming a project's declared
   tool environment and cannot expose a provider plan honestly.
2. Run host `nix develop` and mount its closure. Rejected because ordinary use
   would require host Nix, Darwin cannot realize the selected Linux closure
   locally, and host evaluation would widen the repository-code boundary.
3. Run `nix develop --command <harness>` on every launch. Rejected because it
   re-evaluates project code and activation in a credentialed capsule, makes
   startup and failure less predictable, and cannot provide a stable inspected
   plan before harness execution.
4. Support devenv or Dev Containers simultaneously. Rejected because either
   adds lifecycle commands, services, mounts, features, and compatibility
   semantics unrelated to proving one provider. A later FIP can add another
   explicit provider to the same plan.
5. Share one global mutable Nix store. Rejected because corruption, garbage
   collection, and untrusted project additions would cross project boundaries.
6. Copy the host Git and Jujutsu configuration. Rejected because includes,
   helpers, signing, aliases, hooks, URL rewrites, SSH commands, and local paths
   would cross the guest boundary along with the two useful identity fields.
7. Add a general cache or package DSL. Rejected because the Nix store and
   existing persistent harness home are enough to measure reuse in this slice.

## Open Questions

1. Whether real projects justify a signed activation artifact format that can
   admit bounded shell hooks without running them during credentialed launch.
2. Whether a later provider should implement a safe Dev Container subset or a
   direct devenv contract.
3. Whether dependency caches should become project-shared declarations rather
   than remaining provider state plus harness-private home state.
4. Whether project provider stores need public inventory, pruning, size
   reporting, or destructive purge commands.
5. Whether an explicit non-personal agent identity should be offered alongside
   projection of the operator's local checkpoint identity.
