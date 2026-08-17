# GOAL: Add opt-in project environments

Status: active. FIP-0006 accepted on 2026-08-17.

Make Fortlet useful for ordinary work in real repositories by adding the
smallest safe project-tool environment: projects that opt in can supply a
versioned immutable layer whose tools and declared environment reach both
supported harnesses. A project without configuration must behave exactly as it
does today.

Before implementation, read FIP-0001, FIP-0005, and FIP-0006 in full.

## Deliverable 1 — Accept the project-environment contract

1. Compare an isolated project-layer recipe, a pinned OCI project image, and
   Nix/devenv activation against daily-use friction, isolation, identity,
   update safety, and FIP-0001's optional-Nix boundary.
2. Select and accept only the smallest contract that can provide real project
   tools without executing repository-controlled text on the host or exposing
   provider credentials during provisioning.
3. Keep conformance `unimplemented` until code and tests establish observable
   requirements. STOP if the implementation is materially larger than
   FIP-0006 describes.

## Deliverable 2 — Build immutable project layers

1. Discover only the opt-in manifest at the resolved project root; absence
   preserves the current base-plus-harness environment.
2. Read the fixed manifest and adjacent recipe into owned snapshots before
   isolated provisioning, and derive environment identity from their exact
   bytes plus the provisioning contract and target platform.
3. Run repository-controlled provisioning only inside a dedicated capsule
   that receives no project mount, harness state, provider credential, SSH
   agent, publication authority, or host shell execution.
4. Publish a successful output atomically as an immutable content-verified
   layer. A failed or changed build must not overwrite or remove the last
   working layer.

## Deliverable 3 — Activate and reconcile the environment

1. Mount the project layer read-only at one stable guest path and add only its
   validated path entries and literal environment values to Codex and Tact.
2. Reserve Fortlet, credential, terminal, home, and harness-owned variables so
   project configuration cannot replace the isolation or credential boundary.
3. Include project-environment identity in capsule configuration. A changed
   environment must fail closed against an existing stale capsule and use the
   established `stop` then `reset` recovery path.
4. Keep explicit `fortlet run`, optional shims, native escape, project
   resolution, management commands, and unconfigured projects compatible.

## Deliverable 4 — Prove one useful repository path

1. Add deterministic evidence for discovery, parsing, path containment,
   identity, snapshotting, isolation inputs, atomic publication, update
   failure, activation, reserved variables, and capsule reconciliation.
2. Add a repository-owned environment for Fortlet that exposes `cargo`,
   `rustc`, and `jj` in the guest and successfully runs a focused project test.
3. Predeclare and execute at most one bounded local acceptance experiment for
   the immutable public path. Record honest success or failure and perform no
   silent retry.
4. Update user documentation and conformance only for established behavior;
   run the complete standard verification set, rewrite the handoff, prepare
   one FIP-0005 publication packet, then STOP for operator merge.

## Definition of Done

1. Unconfigured repositories retain current Fortlet behavior.
2. One checked-in manifest produces one isolated, immutable, content-verified
   project layer without making provider or host credentials guest-readable.
3. Both project-controlled provisioning files affect environment identity, and
   a failed update preserves the prior published layer and capsule.
4. Both harness configurations receive the same validated project tool paths
   and variables without allowing protected-variable overrides.
5. Fortlet's own configured capsule can execute `cargo`, `rustc`, `jj`, and a
   focused repository test.
6. FIP-0006 is conformant, its experiment is terminal, the complete runbook
   verification set is green, and the outgoing stack is reviewable; then STOP.

## Excluded scope

Do not add background services, workload leases, private overlays, general Nix
or devenv activation, standalone installation, logs, restart, tool-update
commands, extra harnesses, remote execution, releases, packages, registry
authentication, or repository settings.

## Binding rules

1. Preserve the charter and every accepted FIP, especially FIP-0001's
   credential, host-execution, optional-Nix, and fail-closed boundaries.
2. Use the MicroSandbox SDK directly; do not add a generic runtime abstraction.
3. The host may read, validate, hash, and snapshot the two fixed files but MUST
   NOT execute repository-controlled shell text or hooks.
4. Provisioning may receive only the recipe snapshot and an empty output mount.
   It MUST NOT receive the live project, persistent state, host environment,
   provider credentials, SSH material, or publication credentials.
5. Keep conformance changes with the code and tests that establish them. Use
   Jujutsu checkpoints and inspect `main..@` before handoff.
6. Publication follows one exact FIP-0005 packet. The operator remains the sole
   merge authority unless they explicitly authorize that specific merge.

## Budget and escalation

1. Engineering ceiling: three hours after FIP-0006 acceptance, excluding
   operator review, hosted CI, and manual merge waits.
2. External budget: zero money, zero paid quota, no registry login, no release,
   and at most one predeclared local acceptance unit.
3. Stop on any credential anomaly, host execution of project-controlled text,
   input outside the project root, undeclared provisioning mount, inability to
   preserve the prior layer, need to change an accepted FIP, repeated live
   failure, or scope materially larger than FIP-0006.

## Verification

Run focused project-environment tests during development and the complete
standard verification set from `docs/RUNBOOK.md` before publication.
