# GOAL: Observe native Codex outside the Codex runner

Status: active. Authorized by the operator on 2026-08-11 after confirming from
an external Fish shell that `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
`CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` are all absent.

Run one native Codex 0.147.0 UI control from the operator's external terminal
under the unchanged PTY observer. Determine whether native Codex reaches and
responds to the same first-interrupt boundary as packaged Codex when no Codex
runner contains the observer. Do not change Fortlet, the observer, the native
harness, or accepted architecture.

Before dispatch, read FIP-0001 and FIP-0002 in full, the validated design at
`docs/plans/2026-08-11-external-native-codex-control-design.md`, and Experiments
0003 through 0005.

## Deliverable 1 — Qualify the operator control

1. Verify the frozen Codex launcher and native-binary hashes.
2. Confirm the operator's name-only marker check reported all four runner
   markers absent from the external Fish shell.
3. Confirm Fortlet and the observer remain unchanged from checkpoint
   `ad7fdd0e`.
4. Run the complete standard verification set, all six observer tests, and the
   deterministic fixture rehearsal before dispatch.
5. Checkpoint the rehearsed Experiment 0006 before giving the operator the live
   command.

## Deliverable 2 — Observe the external native UI

The operator runs the exact command declared in Experiment 0006 from
`/Users/cody/dev/fortlet` in the already-checked external Fish shell. The unit
uses an 80-by-24 PTY, 100-millisecond settle, 120-by-40 resize, 20-second
unattended hold, one `SIGINT` to the verified observer-owned foreground process
group, and a 15-second exit bound.

The operator must not type, press Enter, press Ctrl-C, or retry. The observer
stores no native screen bytes. The operator returns the complete structural
output verbatim.

Close by the first matching rule:

1. If native Codex reaches signal and remains alive, its first-interrupt
   behavior matches packaged Codex; do not attribute that timeout to Fortlet.
2. If native Codex reaches signal and exits, record the exact status and a
   packaged-path discrepancy; do not repair it under this goal.
3. If native Codex closes before signal, the external-runner mechanism is
   rejected and no signal comparison is established.

After receiving the result, verify or obtain operator confirmation of owned
cleanup, close Experiment 0006 terminally, update conformance and the handoff,
mark this goal complete or blocked, then STOP and report.

## Definition of Done

1. One exact native Codex identity runs under the unchanged observer from a
   marker-free external Fish shell.
2. The complete gate and deterministic rehearsal pass before the live command
   is issued.
3. The operator supplies one unedited structural result with no retry or input.
4. Ordinary Codex-managed state writes are allowed for this one native launch;
   shell startup-file, Fish configuration, Nix configuration, and PATH-manager
   edits are not authorized.
5. No prompt, intentional inference, paid quota, credential value, unowned
   process control, Fortlet invocation, or product change occurs.
6. Conformance and the handoff state only what the result proves; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001/FIP-0002 constraint.
2. Use the exact native launcher path, not `codex` through `PATH`, a Fortlet
   shim, `fortlet native`, or another wrapper.
3. Do not add a sandbox around the external control; that would change the
   runner-context variable being tested.
4. Never persist raw UI output, environment values, account metadata, or
   credential values.
5. Do not change product code, observer code, native Codex configuration, or
   timings during the mission.
6. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: 30 minutes from mission setup.
2. Experiment ceiling: zero money, zero paid quota, zero prompts, one unit,
   zero retries, and at most 10 minutes after dispatch begins.
3. The operator explicitly authorizes ordinary Codex-managed state writes for
   this one launch, but no shell or declarative-environment mutation.
4. Stop on any credential output, unexpected external mutation, unowned process
   control, paid activity, or need to change the declared command.

## Verification

Run before issuing the operator command:

- `cargo test --example pty_observer`
- `cargo run --quiet --example pty_observer -- fixture`
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- Exact identity, unchanged-path, and one-active-experiment checks declared in
  Experiment 0006
