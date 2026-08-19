# GOAL: Harden the first daily Fortlet session

Status: active. FIP-0007 was accepted and its deterministic implementation is
complete. Experiment 0021 rejected the first daily session at credential
refresh. FIP-0008 is accepted for a host-owned renewable credential lease and
its implementation is authorized.

Turn the proven isolated runtime into a deliberate, understandable daily-use
workflow. Add the smallest explicit preparation surface, then dogfood one
ordinary interactive project session and fix only blockers established by that
path. Do not expand into general lifecycle administration or distribution.

Before implementation, read FIP-0001, FIP-0002, FIP-0005, FIP-0006, FIP-0007,
and FIP-0008 in full.

## Deliverable 0 — Decide renewable credential ownership

1. Compare a trusted host renewal lease over the existing MicroSandbox broker,
   iron-proxy plus a response-retry handler, Codex external authentication, and
   static guest projection.
2. Accept only a contract that keeps access and refresh credentials
   guest-unreadable, preserves the native Codex terminal, renews an active
   session without silently restarting a shared capsule, and fails closed.
3. FIP-0008 is accepted after review of Codex, MicroSandbox, iron-proxy, and
   Infisical Agent Proxy. Keep its conformance `unimplemented` until code and
   tests establish the public contract.

## Deliverable 1 — Accept explicit preparation

1. Compare preparation only as a side effect of `run`, a dedicated
   harness-scoped command, mutation through `status`, and background
   provisioning.
2. Accept only a contract that stages the exact immutable inputs needed by one
   harness without reading credentials, creating persistent harness state, or
   creating a reusable project+harness capsule.
3. Preserve current `run`, shim, native, project-resolution, status, stop,
   reset, and unconfigured-project behavior.
4. Keep FIP-0007 conformance `unimplemented` until code and tests establish the
   public contract. STOP before implementation if the operator does not accept
   the proposal.

## Deliverable 2 — Implement credential-free prepare

1. Add `fortlet prepare <harness>` with the same explicit project-selection
   options as `run`.
2. Resolve and validate the harness, project, and optional project environment,
   then ensure the base, harness, and project layers through the existing
   immutable provisioning paths.
3. Do not read provider credentials, create a guest auth projection, prepare a
   persistent harness home, construct a managed runtime capsule, or attach a
   terminal.
4. Make repeated preparation a verified cache hit with no network, VM, or
   first-use progress output. Print one stable ready result on success.
5. Preserve every prior published layer on failure and report the failed stage
   plus one actionable correction without exposing internal identities or
   unsafe recipe output.

## Deliverable 3 — Prove one ordinary work session

1. Add deterministic evidence for parsing, project parity with launch,
   credential absence, provisioning inputs, cache-hit idempotence, both
   harnesses, unconfigured projects, failure cleanup, and absence of persistent
   capsule or harness state.
2. Predeclare one bounded daily-session experiment only after the deterministic
   and package gates pass. Separate the explicit prepare observation from the
   operator-run interactive work observation in its record.
3. Exercise an ordinary Codex session from a nested Fortlet directory, use the
   project-provided Cargo and Jujutsu tools for one real edit/test loop, and
   verify a subsequent invocation reuses the capsule and persistent Cargo
   cache.
4. Close through public stop/reset and record user-visible friction honestly.
   Fix only a blocker inside the accepted contract; a new public mechanism
   requires a successor FIP and experiment rather than silent scope growth.

## Deliverable 4 — Document, conform, and publish

1. Document explicit preparation, cache-hit behavior, credential independence,
   and the boundary between layer readiness and capsule lifecycle.
2. Mark FIP-0007 conformant only after deterministic evidence and the terminal
   daily-session unit establish the contract.
3. Run the complete standard verification set, rewrite the handoff, shape and
   sign `main..@`, and prepare one exact FIP-0005 publication packet.
4. STOP for operator approval and manual squash merge.

## Definition of Done

1. A user can prepare one registered harness and the selected project's
   immutable tools before starting an interactive agent.
2. Preparation succeeds without a provider credential and cannot create or
   mutate a reusable project+harness capsule or persistent harness home.
3. A cache hit is fast, idempotent, and performs no provisioning contact.
4. The following interactive launch reuses the prepared layers; one real
   edit/test loop and one subsequent attachment work through the ordinary
   Fortlet surface.
5. Existing launch and management behavior remains compatible, and public
   stop/reset returns the test project to absence without removing durable
   state or immutable layers.
6. FIP-0007 is conformant, its experiment is terminal, the complete verification
   set is green, and the stack is reviewable; then STOP.

## Excluded scope

Do not add global inventory or pruning, restart, logs, background provisioning,
progress protocols, workload leases, private overlays, general Nix/devenv
activation, standalone installation, releases, packages, registry login,
remote execution, new harnesses, services, or repository settings.

## Binding rules

1. Preserve the charter and every accepted FIP, especially credential
   delegation, host-execution, project-boundary, and fail-closed rules.
2. Use the existing MicroSandbox SDK and environment store directly; do not add
   a generic runtime, task, or provisioning abstraction.
3. Preparation may run only the already-authorized immutable layer provisioning
   mechanisms. It must not read credentials or call runtime capsule creation,
   attachment, stop, or removal.
4. Keep preparation harness-scoped. Do not implicitly download every registered
   harness or make `status` mutating.
5. Architecture-track changes require accepted FIP-0007 before code. Keep
   conformance changes with the code and tests that establish them.
6. Use Jujutsu checkpoints and inspect `main..@` before handoff. Publication
   follows one exact FIP-0005 packet; the operator merges manually.

## Budget and escalation

1. Proposed engineering ceiling: two hours after FIP-0007 acceptance,
   excluding operator review, hosted CI, and manual merge waits.
2. External budget before an accepted experiment: zero money, zero paid quota,
   zero model prompts, zero remote mutation, and no live runtime dispatch.
3. The later experiment may use at most one operator-authorized interactive
   Codex session, one prepared harness, one owned capsule, and zero silent
   retries; its exact budget requires separate pre-dispatch acceptance.
4. Stop on credential access during prepare, persistent capsule or harness-state
   creation, unowned state, need to change an accepted FIP, repeated live
   failure, scope growth, or any undeclared external mutation.

## Verification

Run focused prepare tests during development and the complete standard
verification set from `docs/RUNBOOK.md` before experiment dispatch and again
before publication.
