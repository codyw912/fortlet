# FIP-0007: Explicit environment preparation

Status: Draft
Recorded: 2026-08-18 from the operator-selected daily-use hardening direction
Requires: FIP-0001, FIP-0002, FIP-0006

## Summary

Add `fortlet prepare <harness>` as the explicit, credential-free way to ensure
the immutable base, selected harness, and optional project-tool layers before
an interactive launch. Preparation reuses existing provisioning and validation
mechanisms but never creates the reusable project+harness capsule, persistent
harness home, guest credential projection, or terminal session.

## Motivation

Fortlet can now provide a real repository toolchain, but first use may spend
minutes downloading and validating immutable layers while the user is trying to
start an agent. That work currently occurs only inside `run`, after Fortlet has
already required a provider credential and prepared capsule-specific state.
This makes an environment problem look like an agent-startup problem and gives
users no deliberate way to stage a project before beginning work.

The environment store already has the correct narrow mechanism: it ensures the
global base, one adapter-owned harness layer, and the selected project's
content-verified layer. A public preparation command can expose that mechanism
without widening repository authority or inventing a background service.

## Decision

Provide one harness-scoped `prepare` command. It resolves the project exactly as
launch does, snapshots and validates the optional FIP-0006 inputs, and calls the
existing environment store. It reports readiness but stops before credential,
persistent harness-state, managed-capsule, or terminal boundaries.

Preparation is explicit and synchronous. Cache misses retain the existing
bounded first-use progress messages on stderr; success has one stable stdout
result. Cache hits verify immutable project output as required by FIP-0006 but
perform no provisioning contact.

## Specification

### Public command and selection

1. Fortlet MUST provide:

   ```text
   fortlet prepare <harness> [--project <path>] [--allow-broad-mount]
   ```

2. `<harness>` MUST name one registered adapter. Unknown harnesses MUST fail
   before project resolution or state creation with the same registered-harness
   correction used by launch.
3. Project discovery, explicit selection, canonicalization, nested working
   directory behavior, marker priority, broad-root scratch protection, and
   `--allow-broad-mount` MUST match `fortlet run`.
4. The command MUST discover and validate FIP-0006 only at the resolved project
   root. An unconfigured project MUST prepare only the existing base and
   selected harness layers.
5. `prepare` MUST NOT change explicit `run`, optional shim, `native`, `doctor`,
   `status`, `stop`, or `reset` syntax or semantics.

### Preparation boundary

1. Preparation MUST ensure exactly the immutable base layer, the selected
   adapter's pinned harness layer, and the optional selected project layer using
   the existing environment store, locks, validation, atomic publication, and
   cleanup paths.
2. Preparation MUST NOT read a provider credential or fingerprint, validate a
   provider login, create a guest auth projection, receive a credential broker,
   inspect or construct a reusable managed capsule, create a persistent
   project+harness home, or attach a terminal.
3. Dedicated disposable provisioning capsules remain permitted only for a
   missing immutable layer. They MUST retain FIP-0001 and FIP-0006 mount,
   credential, network, ownership, and cleanup constraints.
4. A successful cache hit MUST verify every required marker and the FIP-0006
   content digest, MUST NOT start a provisioning capsule or contact the network,
   and MUST NOT rewrite a published layer.
5. A missing layer MAY be provisioned synchronously. Concurrent preparation or
   launch MUST converge through the existing layer locks without publishing
   partial output or duplicating a completed identity.
6. A failure MUST leave every previously published layer and any existing
   managed capsule or persistent harness state unchanged. Fortlet MUST NOT
   silently fall back to an unprepared or host-native harness.
7. Preparing a changed project environment MUST NOT stop, reset, reconfigure, or
   replace a capsule for the prior identity. The subsequent launch retains
   FIP-0006's explicit stale-capsule stop/reset recovery.

### Output and failure behavior

1. On success, stdout MUST contain exactly:

   ```text
   <harness><TAB>ready
   ```

2. A cache hit SHOULD otherwise be silent. Existing broad-root scratch guidance
   and first-use preparation messages MAY appear on stderr when applicable.
3. First-use messages MUST name the user-facing layer class and MUST NOT expose
   internal capsule names, project identities, credential material, or host
   temporary paths.
4. Errors MUST identify one of the harness, project, project-environment, or
   environment stages and provide one actionable correction. Repository recipe
   diagnostics MUST retain FIP-0006's bounding and terminal-control sanitation.
5. Preparation MUST return nonzero unless every required layer is present and
   verified.

### Daily-use evidence

1. Automated evidence MUST cover CLI parsing and output, harness-before-project
   ordering, project-resolution parity, configured and unconfigured projects,
   both adapters, credential and runtime absence, cache-hit idempotence,
   provisioning-plan reuse, concurrency, failure preservation, and stage
   diagnostics.
2. Isolated CLI failure tests MUST prove invalid requests do not read provider
   credentials or create persistent harness state, managed capsules, or layer
   artifacts outside the declared failing stage.
3. One predeclared bounded local experiment MUST exercise the immutable packaged
   `prepare` command and one ordinary interactive project session. It MUST
   distinguish automatic evidence from operator-observed UX, record actual
   cache reuse and public cleanup, and MUST NOT silently retry.

## Consequences

Users gain a deliberate setup step that can run before they are ready to spend
a model prompt or debug an interactive terminal. A subsequent launch still owns
credentials, persistent state, capsule reconciliation, and attachment; prepare
does not claim the project is runnable beyond its immutable inputs.

Harness scope avoids downloading tools the user did not select. Synchronous
execution remains potentially slow on a first miss, but it is explicit and does
not require a daemon, task model, progress protocol, or new persistence class.
The stable ready line is intentionally small; richer inventory belongs in a
later lifecycle FIP if daily use demonstrates the need.

## Alternatives Considered

1. Keep preparation only inside `run`. This preserves the smallest CLI but
   continues to conflate potentially slow environment work with credentialed
   interactive startup.
2. Add `fortlet environment prepare`. A namespace may become useful with future
   inventory or pruning, but it adds hierarchy before a second environment
   operation exists.
3. Make `status` prepare missing layers. Rejected because an observation command
   must not download content, create VMs, or mutate layer state.
4. Prepare every harness with one command. Rejected because it downloads
   unrequested adapter tools and weakens the harness-owned boundary.
5. Provision automatically in the background. Rejected because it requires a
   task, lease, progress, cancellation, and failure-persistence model that daily
   use has not justified.
6. Add environment inventory or pruning first. Deferred because the recorded
   blocker is opaque first-use latency, while existing public stop/reset already
   covers the reusable capsule lifecycle.

## Open Questions

1. Whether repeated daily use justifies a read-only global environment and
   capsule inventory.
2. Whether first-use durations eventually justify structured progress or
   cancellation rather than the current synchronous messages.
3. Whether standalone distribution should expose an install-time prepare step.
