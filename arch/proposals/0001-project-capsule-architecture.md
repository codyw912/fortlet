# FIP-0001: Project-scoped capsule architecture

Status: Accepted
Recorded: 2026-08-10, retroactively from the operator-validated product design

## Summary

Fortlet transparently launches supported coding-agent harnesses inside
project-and-harness-scoped MicroSandbox capsules. It preserves native terminal
behavior while keeping host credentials and publication authority outside the
guest. Local execution is the first implementation; remote execution remains a
separate future design problem.

## Motivation

Coding agents need broad access to project files and development tools, but
running them directly on the host exposes unrelated files, durable credentials,
and unrestricted publication authority. A useful boundary must reduce that
exposure without adding enough command and setup friction that users bypass it.

Multiple agent harnesses and concurrent sessions must work without bespoke
aliases, per-project setup rituals, or opaque long-lived virtual machines.

## Decision

Fortlet owns a fail-closed capsule-session transaction: resolve the project,
validate credential placement, provision immutable environment layers,
reconcile one capsule for the project and harness, then attach the harness to
the caller's terminal.

MicroSandbox is the sole local runtime. Harness-specific versions,
provisioning, state, credentials, and launch behavior live behind harness
adapters. Capsules have disposable roots and explicit persistent mounts.

## Specification

### Command and terminal behavior

1. Registered harness commands MUST launch through Fortlet by default while
   preserving arguments, current working directory, interactive TTY behavior,
   terminal resize, signals, and exit status.
2. Successful wrapper startup MUST be silent.
3. Failure to establish the declared project, environment, credential, capsule,
   or terminal boundary MUST prevent the harness from starting.
4. A failure MUST identify its stage and one actionable next step.
5. An explicit native escape hatch MAY exist, but Fortlet MUST NOT silently
   fall back to it.

### Project identity and mounts

1. Project resolution MUST prefer the nearest Jujutsu root, then Git root, then
   recognized development-environment root, then the current directory.
2. Project roots MUST be canonicalized on the host before capsule creation.
3. The project MUST be mounted at the same absolute path in the local guest and
   a launch from a subdirectory MUST start in the corresponding guest path.
4. Home-directory and filesystem-root launches MUST use persistent safe scratch
   space unless the user explicitly authorizes the broad mount.
5. Project identity MUST NOT expose local-path derivation as a contract, so a
   future remote workspace mode can define a portable identity.

### Capsule topology and persistence

1. The default isolation unit MUST be one capsule per project and harness.
2. Concurrent sessions of one harness in one project MUST attach to the same
   capsule and MAY share that harness's process and coordination state.
3. Different harnesses MUST NOT share private process or harness-state mounts.
4. Workspace and selected harness state writes MUST be durable continuously;
   capsule teardown MUST NOT be treated as a save operation.
5. Capsule roots, temporary files, and unpromoted system changes MUST be
   disposable.
6. Reusable packages and services SHOULD be promoted into declarative project,
   private-overlay, or Fortlet-owned environment layers.

### Environments and harnesses

1. Harness adapters MUST own the Linux executable, pinned version,
   provisioning, launch environment, persistent paths, and credential policy.
2. Host macOS harness binaries MUST NOT be executed or mounted as guest tools.
3. Tool layers MUST be versioned, immutable after publication, and safe to reuse
   across compatible capsules.
4. Tool updates MUST be explicit and MUST leave the last working version
   available after failure.

### Credentials and publication

1. Provider credentials SHOULD be delegated through a host broker and MUST NOT
   be written to persistent guest state.
2. Any ephemeral credential fallback MUST be explicitly declared by its harness
   adapter; there MUST be no silent fallback to a guest-readable credential.
3. SSH private keys, the 1Password SSH agent socket, and unrestricted GitHub
   publication credentials MUST remain outside capsules.
4. Agents MAY create unsigned local VCS checkpoints.
5. A future publication operation MUST be an explicit host transaction that
   presents the exact outgoing revisions, diff, destination, and PR metadata
   before approval, then signs, pushes, and creates or updates the PR.
6. The trusted publication process MUST NOT execute repository-controlled hooks,
   aliases, pagers, credential helpers, or generated shell text.
7. Fortlet MUST NOT add agent provenance or local setup identifiers to commits,
   PRs, or repository metadata.

### Lifecycle and future execution hosts

1. Interactive sessions and background workloads MUST hold observable leases.
2. Capsules SHOULD be removed after their final lease and a grace period;
   orphaned workloads MUST have a finite default maximum lifetime unless project
   policy explicitly overrides it.
3. Local execution MUST use the MicroSandbox SDK directly rather than a generic
   runtime abstraction.
4. Remote execution MUST receive a separate FIP covering workspace authority,
   encrypted interactive transport, execution-host trust, credential
   delegation, and return of revisions for host-side publication.

## Consequences

The capsule boundary stays understandable and harness behavior remains
extensible, but MicroSandbox is a deliberate product dependency. Native-feeling
commands require careful shim installation, terminal forwarding, and lifecycle
work. Remote execution cannot be added as a trivial endpoint switch because it
changes workspace and credential authority.

## Alternatives Considered

1. Docker and Docker Sandboxes offer strong UX references but introduce a
   Docker account or daemon dependency that is not acceptable as a requirement.
2. SmolVM provides a stronger VM-shaped boundary but its startup latency does
   not meet the daily command UX target.
3. A subprocess wrapper around `msb` was useful for prototyping but loses typed
   lifecycle integration and couples Fortlet to presentation-oriented CLI text.
4. One capsule per project would improve cross-harness process sharing but would
   weaken harness isolation and make private state ownership ambiguous.
5. Agent-local signing keys add provenance machinery without improving the
   chosen host approval boundary.

## Open Questions

1. The exact transparent shim installation and native escape-hatch contract.
2. The first management commands and lifecycle lease representation.
3. Private project overlay discovery and declarative environment activation.
4. Remote workspace authority and transport.
