# GOAL: Verify idle Ctrl-C key exit parity

Status: active on 2026-08-11.

Determine whether Fortlet's packaged Codex shim preserves native Codex's exact
termination result when both receive one Ctrl-C terminal key at an idle, empty
composer. Extend only repository test tooling; do not change Fortlet product
behavior, package behavior, harness configuration, or accepted architecture.

Before implementation, read FIP-0001 and FIP-0002 in full, the validated design
at `docs/plans/2026-08-11-idle-ctrl-c-exit-parity-design.md`, and Experiment
0008. Preserve every terminal result from Experiments 0003 through 0008.

## Source-established premise

The operator confirmed that native Codex 0.147.0 exits after one manually typed
Ctrl-C at an idle, empty composer. During an active conversation, the first
Ctrl-C interrupts work and a later Ctrl-C exits.

Official Codex tag `rust-v0.147.0`, peeled commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`, agrees. Its formal double-press
quit shortcut is disabled. With no modal, composer text, or cancellable work,
one Ctrl-C key requests an immediate shutdown-first exit. This is not the same
mechanism as sending operating-system `SIGINT` to a process group.

## Deliverable 1 — Add one Ctrl-C key observer path

1. Preserve the signal, atomic-exit, and typed-exit modes and all evidence they
   produced.
2. Add separate deterministic and live Ctrl-C-key modes. After the unchanged
   startup, resize, and fixed hold sequence, write exactly one `0x03` byte to
   the observer-owned PTY.
3. Emit a distinct `ctrl_c_key` structural event, discard all raw PTY screen
   bytes, and retain the existing exit bound and owned cleanup.
4. Use test-first vertical slices to prove the public modes, exact single-byte
   and single-write delivery, event order, distinctive fixture exit code 23,
   timeout cleanup, and unchanged existing modes.
5. Simplify without changing behavior, run the focused gate, and checkpoint the
   tested observer before declaring a live experiment.

## Deliverable 2 — Qualify Experiment 0009

1. Freeze the official Codex source tag and commit that establish the idle
   Ctrl-C mechanism, plus the exact native launcher and selected-binary hashes.
2. Freeze one exact packaged Codex shim produced from the tested observer
   checkpoint.
3. Declare Experiment 0009 with exact commands, identities, timings, decision
   rules, and a two-unit zero-retry budget.
4. Run the complete standard verification set, all deterministic observer
   rehearsals, exact package checks, and `nix run . -- doctor` before dispatch.
5. Confirm Experiment 0009 is the sole active record and checkpoint the
   rehearsed declaration before giving the operator either live command.

## Deliverable 3 — Compare native and packaged Ctrl-C key exit

The operator runs native Codex, then packaged Codex, from
`/Users/cody/dev/fortlet` in the same external Fish shell. The four known runner
marker names must be absent. Each unit uses an 80-by-24 PTY, 100-millisecond
settle, 120-by-40 resize, a 5-second unattended hold, one `0x03` PTY write, and
a 15-second exit bound.

The operator supplies no additional input and returns each complete structural
result verbatim. Both declared units run once unless a hard invariant fires:

1. Exact matching exit codes and signals accept packaged idle Ctrl-C parity;
   the source-established expected result is `exit_code:0` and
   `exit_signal:null` for both.
2. Any native-versus-packaged termination or status mismatch records a
   packaged-path discrepancy but authorizes no repair under this goal.
3. Failure of native Codex to terminate normally rejects the automated idle
   readiness premise; record the packaged unit too, without retry or adaptation.

After the results, verify owned cleanup, close Experiment 0009 terminally,
update conformance and the experiment index, rewrite the handoff, mark this goal
complete or blocked, then STOP and report.

## Definition of Done

1. Deterministic evidence proves one `0x03` byte in one PTY write, event order,
   status preservation, and owned cleanup.
2. One exact native Codex identity and one exact packaged Codex shim receive the
   same source-established idle interaction from the same external Fish shell.
3. The full gate passes before live dispatch, and both unedited structural
   results are recorded with zero retries.
4. No model prompt, intentional inference, or paid quota occurs.
5. Conformance states only what the results prove; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001/FIP-0002 constraint.
2. Send exactly one Ctrl-C terminal key byte. Do not send process signals, a
   second key, EOF, slash commands, adaptive input, or screen-text matching.
3. Keep all existing observer paths available unchanged for reproducibility.
4. Do not store or print raw PTY content, environment values, account metadata,
   or credential values.
5. Do not change Fortlet, the package contract, live timings, or observer
   behavior after Experiment 0009 is declared.
6. The observer may control only the process group it created and verified.
7. Ordinary Codex-managed and Fortlet-managed state writes are allowed for the
   two launches. Shell startup-file, Fish, Nix, PATH-manager,
   Codex-configuration, and outside-project mutation are not authorized.
8. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: 45 minutes from mission setup.
2. Experiment ceiling: zero money, zero paid quota, zero model prompts, two
   units, zero retries, and at most 10 minutes after dispatch begins.
3. Stop on credential output, unexpected external mutation, unowned process
   control, paid activity, or any need to change a declared live command.

## Verification

Run before issuing either operator command:

- `cargo test --example pty_observer`
- All deterministic observer fixture modes declared by Experiment 0009
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- Exact source, native identity, package identity, marker-name, unchanged-path,
  and one-active-experiment checks declared in Experiment 0009
