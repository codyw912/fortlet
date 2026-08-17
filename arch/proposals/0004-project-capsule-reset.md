# FIP-0004: Project capsule reset

Status: Accepted
Recorded: 2026-08-12 from the operator-validated reset and recovery design
Requires: FIP-0001, FIP-0003

## Summary

Fortlet exposes `fortlet reset <harness>` to remove an owned, terminal
project+harness capsule's disposable MicroSandbox state while preserving the
project, persistent harness state, credential projection, and immutable
environment layers. Reset refuses to terminate active or transitional
capsules and replaces raw `msb` recovery guidance with Fortlet commands.

## Motivation

Launch detects stale capsule configuration but currently directs users to
remove an internal capsule with `msb`. That exposes implementation naming,
bypasses Fortlet ownership checks, and makes safe recovery difficult after an
upgrade. Users also need a deliberate way to discard a capsule's disposable
root without erasing durable project or harness state.

Reset is destructive to runtime state and could terminate active work if
combined implicitly with stop. Its public contract therefore needs an explicit
ownership, state, persistence, and error boundary.

## Decision

Add an idempotent, project-scoped `fortlet reset <harness>` command. It removes
only an owned stopped or crashed capsule. Active and transitional capsules must
be stopped explicitly with the existing project-scoped stop command before
reset.

Reset shares Fortlet's project resolution, capsule descriptor, ownership
validation, and per-capsule lock. It uses the MicroSandbox SDK directly and
does not remove Fortlet's persistent harness-state directory or immutable tool
layers.

## Specification

### Scope and selection

1. `fortlet reset <harness>` MUST target exactly one registered harness for the
   resolved project.
2. Reset MUST use FIP-0001's canonicalization, discovery, working-directory,
   scratch, and explicit broad-mount rules.
3. Reset MUST accept explicit project selection and broad-root authorization
   equivalent to run, status, and stop.
4. An unknown harness or project-resolution failure MUST fail before runtime
   mutation.
5. Reset MUST NOT require provider credentials, provision environment layers,
   launch a harness, prepare persistent capsule state, or expose an internal
   capsule name or project identity in normal output.
6. This FIP MUST NOT add global inventory, management by raw capsule name, or
   multi-harness reset.

### Lookup, locking, and ownership

1. Reset MUST derive its target from the shared capsule descriptor used by
   launch, status, and stop.
2. A runtime not-found result MUST succeed as `absent` without creating a
   Fortlet state directory or capsule lock.
3. If the target exists, reset MUST acquire the same per-capsule lock used by
   launch and stop, then fetch the current target again before authorization or
   mutation.
4. Reset MUST parse stored capsule configuration and verify the deterministic
   name plus `fortlet.managed`, `fortlet.schema`, `fortlet.project`, and
   `fortlet.tool` under the lock.
5. Malformed stored configuration, a deterministic-name collision, or an
   ownership mismatch MUST fail closed without removal.
6. `fortlet.version` MUST remain diagnostic metadata and MUST NOT prevent reset
   when the ownership labels match.

### State decisions and removal

1. Reset MUST remove only an owned capsule whose fresh status is `stopped` or
   `crashed`.
2. Reset MUST refuse `created`, `starting`, `running`, `draining`, and `paused`
   without stopping, killing, starting, attaching, or removing the capsule.
3. Active or transitional refusal MUST return nonzero and direct the user to
   run `fortlet stop <harness>` before retrying reset.
4. Eligible removal MUST use the MicroSandbox SDK handle operation and MUST
   report success only after it succeeds.
5. Reset MUST NOT remove or modify project files, the persistent
   project+harness state directory, credential projection, credential
   fingerprint, or immutable base and harness layers.
6. A later run MUST be able to create a fresh capsule around the preserved
   project and harness state.

### Output and recovery guidance

1. Successful reset stdout MUST be exactly `harness<TAB>reset` after removal
   or `harness<TAB>absent` when no capsule exists.
2. Reset failures MUST identify their stage and one actionable next step.
3. Launch's stale-capsule recovery guidance MUST use public Fortlet stop and
   reset commands rather than raw `msb` removal.
4. Documentation MUST explain that an explicit project selection must be
   repeated when using stop and reset for that launch target.
5. Persistent-state deletion MUST NOT be named or implied as reset behavior; a
   future purge operation requires a different public command and accepted
   design.

### Evidence

1. Automated evidence MUST cover every SDK status decision, side-effect-free
   absence, shared descriptor targeting, locking, ownership and version skew,
   malformed configuration, stable output, parsing, and failure behavior.
2. Local runtime evidence MUST verify through the immutable public CLI that
   reset refuses an active capsule, removes the same owned capsule after
   explicit stop, reports subsequent absence idempotently, and preserves
   selected persistent state.
3. Local runtime evidence MUST verify exact ownership before removal or cleanup
   and MUST remove only generated disposable test state.

## Consequences

Users can recover from stale or unwanted disposable capsule roots without
learning MicroSandbox naming or risking persistent harness state. Requiring a
separate stop adds one command but makes process termination visible and keeps
reset's destructive authority narrow.

Version-tolerant ownership permits recovery after upgrades. Preserving the
entire harness-state directory also means reset cannot repair corruption in
that durable state; an explicit purge command would need a stronger warning,
confirmation, and separate persistence design.

## Alternatives Considered

1. Automatically stopping an active capsule before removal is more convenient
   but lets one recovery command terminate interactive sessions and background
   work implicitly.
2. Allowing reset only for stale-version capsules is narrower but prevents
   deliberate replacement of a healthy disposable root.
3. Directing users to `msb remove` avoids product code but exposes internal
   identity and bypasses Fortlet's ownership and project boundaries.
4. Removing the persistent harness-state directory would provide a deeper
   reset but risks data loss and conflates disposable runtime recovery with
   durable-state purge.
5. Combining reset with global inventory or raw capsule names broadens scope
   and authority before a global management contract exists.

## Open Questions

1. Whether a separate, explicitly destructive persistent-state purge command
   is justified by real recovery cases.
2. The global inventory and safe cross-project removal contract.
3. Restart, logs, tool updates, and observable workload leases.
