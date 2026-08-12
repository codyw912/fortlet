# GOAL: Add project capsule status and stop

Status: authorized on 2026-08-12.

Implement the accepted FIP-0003 management slice: let users observe the
existing project-and-harness capsule states and safely stop one owned capsule
without removing reusable state. Do not expand into global inventory, remove,
restart, logs, tool updates, explicit leases, or other product surfaces.

Before implementation, read FIP-0001 and FIP-0003 in full and the validated
design at `docs/plans/2026-08-12-capsule-status-and-stop-design.md`. Preserve
FIP-0002's optional shim and native-escape behavior and every terminal result
from Experiments 0001 through 0009.

## Deliverable 1 — Share capsule identity and ownership

1. Extract a pure capsule descriptor that derives the existing deterministic
   capsule name and labels without preparing state, provisioning, reading
   credentials, or contacting MicroSandbox.
2. Use the descriptor from launch, status, and stop so management cannot drift
   from the capsule that launch owns.
3. Separate ownership labels from diagnostic version metadata. Launch MAY
   retain its current exact-version reconciliation rule; management MUST accept
   otherwise owned capsules across Fortlet version changes.
4. Parse stored runtime configuration and fail closed on malformed data, name
   collision, or ownership mismatch before mutation.

## Deliverable 2 — Implement project-scoped status

1. Add `fortlet status [harness]` with `--project` and
   `--allow-broad-mount`.
2. With no harness, report every registered harness in registry order; with a
   harness, report only that registered harness.
3. Report exactly `harness<TAB>state`, using `absent` or the lowercase
   MicroSandbox lifecycle state.
4. Keep status observational: no credentials, environment provisioning,
   capsule-state preparation, harness launch, or runtime mutation.
5. Treat a successfully observed crashed capsule as successful status output.

## Deliverable 3 — Implement bounded idempotent stop

1. Add `fortlet stop <harness>` with the same project options.
2. Treat absent and terminal capsules as successful idempotent outcomes.
3. Stop a nonterminal owned capsule with MicroSandbox's bounded graceful-stop
   operation and report success only after the SDK succeeds.
4. Print exactly `harness<TAB>absent`, `harness<TAB>already-stopped`, or
   `harness<TAB>stopped`.
5. Never remove capsule records, roots, credentials, project files, or
   persistent harness state.

## Deliverable 4 — Prove, document, conform, and close

1. Add deterministic evidence for descriptor reuse, ownership and version
   skew, every status mapping, rendering, idempotent stop, CLI parsing,
   project/harness failures, and actionable runtime failures.
2. Document status and stop in `docs/RUNBOOK.md` and update stale README or
   overview descriptions that materially misstate the implemented product
   surface.
3. Update `arch/conformance.json` in the implementation checkpoint. Remove
   only the status-and-stop gap; retain exact gaps for the remaining FIP-0001
   and FIP-0002 work.
4. Rehearse Experiment 0010 without runtime contact, run the complete standard
   verification set, then dispatch its one owned local capsule unit exactly as
   declared.
5. Close Experiment 0010 terminally, rewrite `experiments/HANDOFF.md`, mark
   this GOAL complete, inspect `main..@`, then STOP and report.

## Definition of Done

1. The public CLI implements FIP-0003 exactly and remains usable without
   transparent shims.
2. Management cannot act on a capsule until Fortlet ownership is verified.
3. Status has no capsule/runtime mutation and stop preserves reusable state.
4. Deterministic tests cover the complete command decision surface without a
   VM or real credential.
5. Experiment 0010 records the actual public-CLI running -> stopped path and
   owned cleanup, whether accepted or honestly rejected.
6. Documentation and conformance state only the evidence obtained.
7. The complete standard verification set is green; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001, FIP-0002, and FIP-0003
   constraint.
2. Use the MicroSandbox SDK directly. A narrow local management seam for
   deterministic tests is allowed; a generic runtime abstraction is not.
3. Do not read credentials from status or stop, expose capsule names or project
   identities in normal command output, or silently fall back to native tools.
4. Do not add removal, restart, logs, global inventory, management by raw
   capsule name, structured output, lease representation, or packaging work.
5. Tests may mutate only owned temporary directories and subprocess
   environments. They MUST NOT start a VM, use a real credential, launch a
   harness, or contact a network service.
6. Experiment 0010 is the only authorized live runtime contact. Verify its
   exact generated target and ownership before creation and cleanup.
7. Keep conformance changes in the same checkpoint as code and tests. Keep
   experiment results in their later terminal-closure checkpoint.
8. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: two hours from implementation start.
2. External budget: zero money, zero paid quota, zero model prompts, zero
   remote mutation, one local Codex `--version` launch, one Fortlet-owned local
   capsule, and zero retries.
3. Stop on any undeclared network attempt, credential anomaly, outside-scope
   mutation, unowned capsule, need to change an accepted FIP, repeated live
   failure, or experiment scope change.

## Verification

Run before claiming completion:

- focused management tests during development;
- `cargo test`;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --test conformance`;
- `nix flake check`.
