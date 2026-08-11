# GOAL: Verify exact Codex exit parity

Status: completed on 2026-08-11 — rejected as an exact termination proof.
Native and packaged Codex both reached the exact `/exit\r` action and remained
alive beyond the same 15-second bound. No Fortlet discrepancy was observed,
but neither unit supplied an exit status to compare.

Determine whether the packaged Codex shim preserves native Codex's exact
termination result when both receive the documented local `/exit` command
through the same PTY observer. Extend only the repository test observer needed
to make that comparison; do not change Fortlet product behavior or accepted
architecture.

Before implementation, read FIP-0001 and FIP-0002 in full, the validated design
at `docs/plans/2026-08-11-exact-codex-exit-parity-design.md`, and Experiments
0003 through 0006.

## Deliverable 1 — Add the bounded exit-command observer path

Completed at checkpoint `147e956d`: both observer actions pass nine focused
tests, including exact `/exit\r` bytes, distinctive exit code 23, structured
event order, and owned cleanup after an ignored exit command.

1. Preserve the existing signal observer and all evidence it produced.
2. Add an explicit observer mode that writes exactly `/exit\r` to its owned PTY
   after the existing startup, resize, and hold sequence.
3. Emit a structural `exit_command` event, discard all raw screen bytes, wait
   within the existing exit bound, and report the exact exit code or signal.
4. Extend the deterministic fixture and focused tests to prove exact command
   delivery, event order, distinctive exit-code preservation, timeout cleanup,
   and compatibility with the existing signal mode.
5. Checkpoint the tested observer before declaring the live experiment.

## Deliverable 2 — Qualify the comparison

Completed at checkpoint `4ffb622f`: exact identities, both deterministic
rehearsals, the complete gate, package smoke, doctor, the sole-active-record
check, and the external Fish marker check passed before dispatch.

1. Freeze the native Codex 0.147.0 launcher and selected-binary hashes.
2. Freeze one exact packaged Codex shim produced from the observer checkpoint.
3. Declare Experiment 0007 with the exact commands, identities, timings,
   decision rules, and two-unit zero-retry budget.
4. Run the complete standard verification set, both deterministic observer
   rehearsals, package checks, and `nix run . -- doctor` before dispatch.
5. Confirm Experiment 0007 is the sole active record and checkpoint the
   rehearsed declaration before giving the operator either live command.

## Deliverable 3 — Compare native and packaged exit

Completed as rejected evidence in Experiment 0007. Both zero-retry units
reached `exit_command`, timed out identically, and left no matching owned
process. Exact termination and status preservation remain unresolved.

The operator runs native Codex, then packaged Codex, from
`/Users/cody/dev/fortlet` in the same external Fish shell. The four known
runner marker names must be absent. Each unit uses an 80-by-24 PTY,
100-millisecond settle, 120-by-40 resize, 20-second unattended hold, exact
`/exit\r` observer input, and a 15-second exit bound.

The operator supplies no additional input and returns each complete structural
result verbatim. Close after both units unless a hard invariant fires:

1. Matching exact exit codes and signals accept packaged exit parity.
2. Any native-versus-packaged termination or status mismatch records a
   packaged-path discrepancy but authorizes no repair under this goal.
3. Failure of native Codex to reach or respond to `exit_command` leaves the
   native termination protocol unestablished; record both declared units
   without retry.

After the results, verify owned cleanup, close Experiment 0007 terminally,
update conformance and the experiment index, rewrite the handoff, mark this goal
complete or blocked, then STOP and report.

## Definition of Done

1. Deterministic evidence proves that the observer writes exactly `/exit\r`
   and preserves a distinctive fixture exit status.
2. One exact native Codex identity and one exact packaged Codex shim receive
   the same action under the same observer protocol from the same external
   Fish shell.
3. The full gate passes before live dispatch, and both unedited structural
   results are recorded with zero retries.
4. The two `/exit` slash commands are local control input, not model prompts;
   no intentional inference or paid quota occurs.
5. Ordinary Codex-managed state writes are allowed for these launches, but
   shell startup-file, Fish, Nix, PATH-manager, Codex-configuration, and
   outside-project mutation are not authorized.
6. Conformance states only what the results prove; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001/FIP-0002 constraint.
2. Keep the exit action explicit; do not replace it with EOF, a second signal,
   adaptive input, or screen-text matching.
3. Do not store or print raw PTY content, environment values, account metadata,
   or credential values.
4. Do not change Fortlet, the package contract, harness configuration, live
   timings, or observer behavior after Experiment 0007 is declared.
5. The observer may control only the process group it created and verified.
6. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: one hour from mission setup.
2. Experiment ceiling: zero money, zero paid quota, zero prompts, two units,
   zero retries, and at most 15 minutes after dispatch begins.
3. Stop on credential output, unexpected external mutation, unowned process
   control, paid activity, or any need to change a declared live command.

## Verification

Run before issuing either operator command:

- `cargo test --example pty_observer`
- Both deterministic observer fixture modes declared by Experiment 0007
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- Exact native identity, package identity, marker-name, unchanged-path, and
  one-active-experiment checks declared in Experiment 0007
