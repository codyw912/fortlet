# FIP-0003: Project capsule status and stop

Status: Accepted
Recorded: 2026-08-12 from the operator-validated capsule-management design
Requires: FIP-0001

## Summary

Fortlet exposes project-scoped `status` and `stop` commands for its existing
project-and-harness MicroSandbox capsules. Both commands reuse Fortlet's
project resolution and capsule identity, verify ownership before acting, and
avoid global or destructive lifecycle operations.

## Motivation

Fortlet can create and reuse project-and-harness capsules, but users cannot
inspect their lifecycle state or deliberately stop one through the product.
Requiring direct MicroSandbox commands exposes internal capsule names and
bypasses Fortlet's project and ownership boundaries. A small management
surface makes day-to-day capsule behavior visible without prematurely defining
global inventory, deletion, restart, logs, or lease policy.

## Decision

Add `fortlet status [harness]` and `fortlet stop <harness>`. Both commands are
scoped to the resolved project, derive the same deterministic capsule identity
as launch, and validate Fortlet ownership from stored capsule configuration.

Status is observational. Stop is bounded, idempotent, and non-destructive: it
stops the runtime but preserves the capsule record and persistent state for
reuse. Fortlet version metadata does not prevent management of an otherwise
owned capsule created by an older release.

## Specification

### Scope and selection

1. `fortlet status [harness]` MUST report the selected registered harness, or
   every registered harness in registry order when no harness is supplied.
2. `fortlet stop <harness>` MUST target exactly one registered harness.
3. Both commands MUST resolve the project using FIP-0001's canonicalization,
   discovery, working-directory, scratch, and explicit broad-mount rules.
4. Both commands MUST accept explicit project selection and explicit broad-root
   authorization equivalent to `fortlet run`.
5. An unknown harness or project-resolution failure MUST fail before runtime
   mutation.
6. Management commands MUST NOT require provider credentials, provision tool
   layers, launch a harness, or prepare persistent capsule state.

### Discovery and ownership

1. Launch and management MUST derive capsule names and expected labels from one
   shared capsule descriptor.
2. Management MUST look up only the deterministic capsule name for the
   resolved project and selected harness. This FIP MUST NOT add global
   inventory or management by raw capsule name.
3. A runtime not-found result MUST be represented as `absent`.
4. Before reporting or mutating an existing capsule, Fortlet MUST parse its
   stored configuration and verify `fortlet.managed`, `fortlet.schema`,
   `fortlet.project`, and `fortlet.tool` against the expected descriptor.
5. Malformed stored configuration, a deterministic-name collision, or an
   ownership-label mismatch MUST fail closed without mutation.
6. `fortlet.version` MUST remain diagnostic metadata and MUST NOT prevent
   status or stop when the ownership labels match.

### Status

1. Status MUST NOT change capsule runtime state.
2. Status MUST represent every MicroSandbox lifecycle state without collapsing
   a running, transitional, stopped, or crashed capsule into `absent`.
3. The observable states MUST be `absent`, `created`, `starting`, `running`,
   `draining`, `paused`, `stopped`, and `crashed`.
4. Successfully observing `crashed` MUST be a successful status operation.
5. Each stdout row MUST contain the harness name, one tab, and its lowercase
   state. Status MUST NOT expose the internal capsule name or project identity.

### Stop

1. Stop MUST treat an absent capsule and a capsule already in a terminal
   stopped or crashed state as successful idempotent outcomes.
2. Stop MUST use MicroSandbox's bounded graceful-stop operation for a
   nonterminal capsule and MUST report success only after that operation
   succeeds.
3. Stop MAY rely on the SDK's bounded escalation from graceful shutdown to
   forced termination.
4. Stop MUST NOT remove the capsule record, root disk, credentials, project
   files, or persistent harness state.
5. Stop stdout MUST report the harness followed by a tab and exactly one of
   `absent`, `already-stopped`, or `stopped`.
6. Stop failure MUST return nonzero and MUST identify one actionable next step.

### Evidence

1. Automated evidence MUST cover shared descriptor derivation, ownership
   acceptance and rejection, version skew, every status mapping, stable output,
   idempotent outcomes, command parsing, and failure behavior.
2. Local runtime evidence MUST verify through the public CLI that an owned
   running capsule is reported running, can be stopped, remains present as
   stopped, and is not confused with an absent or unowned capsule.
3. Local runtime evidence MUST verify ownership and exact scope before creating
   or removing disposable test state.

## Consequences

Users gain the minimum controls needed to understand and stop a reusable
capsule without learning MicroSandbox naming. The surface stays project-local,
which limits accidental cross-project effects but does not help users discover
capsules for projects they cannot identify.

Separating ownership labels from version metadata permits safe cleanup after
upgrades. It also requires lifecycle code to keep authorization checks distinct
from launch-time revision compatibility checks.

The tab-delimited output is intentionally small and stable. Structured output,
additional metadata, and richer global inspection require a later design.

## Alternatives Considered

1. Global `status --all` and management by capsule name would aid inventory but
   expose internal identities and broaden mutation scope before a global
   ownership and presentation model exists.
2. A nested `fortlet capsule status|stop` namespace would leave room for more
   commands but adds ceremony to a two-command surface.
3. Directing users to `msb` would avoid product code but bypass Fortlet's
   project resolution and ownership checks and make internal naming part of the
   user workflow.
4. Removing a capsule during stop would reclaim more resources but conflate
   stopping with destructive cleanup and undermine reuse.
5. Requiring an exact `fortlet.version` label would make older owned capsules
   difficult to manage precisely when lifecycle control is most useful.

## Open Questions

1. The global inventory and safe capsule-removal contract.
2. Restart, logs, and explicit tool-update commands.
3. The observable lease representation and grace-period policy required by
   FIP-0001.
4. Whether a future structured-output mode should cover all management
   commands or the complete Fortlet CLI.
