# GOAL: Add project capsule reset and recovery

Status: completed on 2026-08-12.

Archived during the one-time pull-request workflow bootstrap so the first
squash merge retains this completed mission as a durable repository record.

Implement accepted FIP-0004: let users remove an owned terminal capsule's
disposable MicroSandbox state without raw `msb` commands, implicit process
termination, or persistent-state loss. Do not expand into automatic stop,
persistent-state purge, global inventory, restart, logs, leases, CI,
distribution, or remote publication.

Before implementation, read FIP-0001, FIP-0003, and FIP-0004 in full and the
validated design at
`docs/plans/2026-08-12-project-capsule-reset-design.md`. Preserve every
terminal experiment result and the public repository established by Experiment
0011; no remote mutation is authorized.

## Deliverable 1 — Extend the locked management lifecycle

1. Add reset to the existing narrow local MicroSandbox management seam.
2. Derive reset targets from the shared capsule descriptor used by launch,
   status, and stop.
3. Return absence before creating Fortlet state or a capsule lock.
4. For a present target, acquire the shared per-capsule lock, fetch current
   state again, parse stored configuration, and verify name plus managed,
   schema, project, and tool ownership before any mutation.
5. Accept version skew for reset while preserving launch's exact-version
   reconciliation.

## Deliverable 2 — Implement explicit terminal reset

1. Add `fortlet reset <harness>` with `--project` and
   `--allow-broad-mount`.
2. Remove only owned `stopped` or `crashed` capsules through the SDK handle.
3. Refuse `created`, `starting`, `running`, `draining`, and `paused` without
   stopping or killing anything, and direct the user to
   `fortlet stop <harness>` before retrying.
4. Print exactly `harness<TAB>reset` after removal or
   `harness<TAB>absent` when no capsule exists.
5. Preserve project files, project+harness state, credential projection,
   credential fingerprint, and immutable base and harness layers.

## Deliverable 3 — Replace raw-runtime recovery guidance

1. Change launch's stale-capsule correction from raw `msb` removal to
   `fortlet stop <harness>` followed by `fortlet reset <harness>`.
2. Keep internal capsule names and project identities out of normal reset
   output and recovery guidance.
3. Document that explicit project selection must be repeated across launch,
   stop, and reset.

## Deliverable 4 — Prove, document, conform, and close

1. Add deterministic evidence for every SDK state decision, side-effect-free
   absence, lock/refetch ordering, ownership and version skew, malformed
   configuration, output, parsing, and lookup/lock/removal failures.
2. Add isolated real-CLI failure evidence for harness and project boundaries
   without a VM or credential.
3. Update README, overview, runbook, and conformance with only established
   reset behavior.
4. Rehearse Experiment 0012 without runtime contact, run the complete standard
   verification set, then dispatch its one owned local capsule unit exactly as
   declared.
5. Close Experiment 0012 terminally, rewrite `experiments/HANDOFF.md`, mark
   this GOAL complete, inspect `main..@`, then STOP and report.

## Definition of Done

1. The public CLI implements FIP-0004 exactly.
2. Reset cannot terminate active work and cannot remove an unowned capsule.
3. Absence is idempotent and does not create state or locks.
4. Terminal reset removes only disposable runtime state and preserves the
   project and persistent harness state.
5. Launch recovery guidance uses only Fortlet commands.
6. Deterministic tests cover the complete decision surface without a VM or
   real credential.
7. Experiment 0012 records the actual public-CLI refusal, stop, reset,
   preservation, idempotence, and cleanup path, whether accepted or honestly
   rejected.
8. FIP-0004 conformance and the complete standard verification set are green;
   then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001, FIP-0002, FIP-0003, and
   FIP-0004 constraint.
2. Use the MicroSandbox SDK directly. Extend only the existing narrow local
   management seam; do not add a generic runtime abstraction.
3. Do not read credentials, provision layers, prepare harness state, launch a
   harness, or create a capsule from reset.
4. Do not add force, automatic stop, persistent-state deletion, raw-name
   management, multi-harness reset, global inventory, restart, logs, leases,
   structured output, CI, packaging, or distribution work.
5. Tests may mutate only owned temporary directories and subprocess
   environments. They MUST NOT start a VM, use a real credential, launch a
   harness, or contact a network service.
6. Experiment 0012 is the only authorized live runtime contact. Verify its
   exact generated target and ownership before removal and cleanup.
7. Keep conformance changes in the same checkpoint as code and tests. Keep
   experiment results in their later terminal-closure checkpoint.
8. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.
9. Do not push, create a PR, or perform any GitHub mutation.

## Budget and escalation

1. Engineering ceiling: two hours from implementation start.
2. External budget: zero money, zero paid quota, zero model prompts, zero
   remote mutation, one local Codex `--version` launch, one Fortlet-owned local
   capsule, and zero retries.
3. Stop on any undeclared network attempt, credential anomaly, outside-scope
   mutation, unowned capsule, need to change an accepted FIP, repeated live
   failure, experiment scope change, or need for remote mutation.

## Verification

Run before claiming completion:

- focused reset tests during development;
- `cargo test`;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --test conformance`;
- `nix flake check`.

## Completion

Implementation checkpoint `3eada810` added the accepted reset contract, its
deterministic evidence, recovery guidance, documentation, and partial
conformance. The complete standard verification set passed before dispatch.
Experiment 0012 then accepted one immutable public-CLI unit covering active
refusal, explicit stop, terminal reset, persistence, idempotence, and exact
cleanup. Its terminal-closure checkpoint marks FIP-0004 conformant. No remote
mutation occurred.
