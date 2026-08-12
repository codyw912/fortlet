# Capsule status and stop design

Status: validated by the operator on 2026-08-12.
Governing design: FIP-0001 and FIP-0003.

## Purpose

Add the first capsule-management surface without expanding into global
inventory, removal, restart, logs, leases, or tool updates. The commands make
the existing project-and-harness capsule lifecycle visible and give the user a
safe way to stop a reusable capsule.

## User contract

1. `fortlet status [harness]` resolves the current project and reports either
   the selected registered harness or every registered harness in registry
   order.
2. `fortlet stop <harness>` resolves the same project and stops only that
   project's expected harness capsule.
3. Both commands accept `--project PATH` and `--allow-broad-mount` with the
   same project-boundary behavior as `fortlet run`.
4. Status rows use `harness<TAB>state`. States are `absent` or the lowercase
   MicroSandbox states `created`, `starting`, `running`, `draining`, `paused`,
   `stopped`, and `crashed`.
5. Stop reports `absent`, `already-stopped`, or `stopped`. Absence and terminal
   states are successful idempotent outcomes.
6. Successful observation of a crashed capsule is not itself a command error.
   Lookup, configuration, ownership, project, or stop failures return nonzero
   with an actionable diagnostic on stderr.

## Ownership and lifecycle

Launch, status, and stop share a pure capsule descriptor containing the
deterministic runtime name and expected labels. Management lookup does not
prepare capsule state, provision an environment, or read provider credentials.

Before reporting or mutating an existing capsule, Fortlet parses its stored
configuration and verifies the identity labels `fortlet.managed`,
`fortlet.schema`, `fortlet.project`, and `fortlet.tool`. A deterministic-name
collision or malformed configuration fails closed. `fortlet.version` is
diagnostic rather than an ownership label, so a capsule created by an older
Fortlet version remains stoppable after an upgrade.

Status never changes MicroSandbox state. Stop uses MicroSandbox's bounded
graceful-stop operation for a nonterminal capsule. The SDK waits for terminal
state and escalates to forced termination after its grace timeout. Fortlet
reports success only when the SDK succeeds. Stop does not remove the capsule,
its root disk, credentials, or persistent harness state; a later run may reuse
it.

## Implementation shape

Extract capsule identity and label construction from provisioning into a pure
descriptor used by launch and management. Add a narrow local MicroSandbox
management seam for lookup and stop so deterministic tests can supply observed
capsules and errors without a VM. This seam is not a generic runtime
abstraction and does not weaken FIP-0001's direct SDK requirement.

Keep output rendering and SDK status mapping pure. Keep harness selection in
the registry rather than duplicating command-specific harness lists.

## Verification

Unit tests cover descriptor stability, identity-versus-version ownership,
every SDK status mapping, output rendering, and idempotent stop decisions. CLI
tests cover argument parsing, harness selection, project failures, stable
stdout, actionable stderr, and nonzero error behavior through the management
seam.

After deterministic rehearsal and the complete standard gates, one
predeclared local experiment creates a uniquely scoped Fortlet-owned capsule,
observes it as running through the public CLI, stops it through the public CLI,
observes it as stopped, and cleans it up. Ownership and absence are checked
before creation and after cleanup. The experiment records real SDK behavior;
it does not substitute for deterministic tests.

## Completion boundary

Update the runbook and conformance map with only the status-and-stop evidence.
Retain gaps for restart, logs, tool updates, explicit leases, topology and
concurrency, live failure paths, environments, publication, standalone
installation, and native Linux verification. Run the complete standard
verification set, close the experiment, rewrite the handoff, complete the
mission, inspect `main..@`, then stop.
