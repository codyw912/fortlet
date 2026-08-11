# GOAL: Verify human-paced Codex exit parity

Status: active on 2026-08-11.

Determine whether Fortlet's packaged Codex shim preserves native Codex's exact
termination result when both receive Codex's accepted human-paced `/exit`
interaction through the same PTY observer. Extend only repository test tooling;
do not change Fortlet product behavior, package behavior, harness configuration,
or accepted architecture.

Before implementation, read FIP-0001 and FIP-0002 in full, the validated design
at `docs/plans/2026-08-11-human-paced-codex-exit-parity-design.md`, and
Experiment 0007. Preserve every terminal result from Experiments 0003 through
0007.

## Source-established premise

OpenAI Codex tag `rust-v0.147.0`, peeled commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`, explains Experiment 0007 without a
Fortlet hypothesis. Codex classifies character bursts no more than 8
milliseconds apart as paste. Enter during an active burst becomes a newline in
the pasted composer text rather than a submit action. The observer's one atomic
`/exit\r` write therefore tested paste handling, not the native UI's accepted
typed exit interaction.

## Deliverable 1 — Add a paced typed-exit observer path

Completed at checkpoint `31b4f073`: all three observer actions pass twelve
focused tests. The new action proves six separate key writes, five fixed
20-millisecond delays, distinctive code 23, structural events, and owned
timeout cleanup while preserving both earlier actions.

1. Preserve the existing signal and atomic exit modes and all evidence they
   produced.
2. Add separate deterministic and live typed-exit modes. After the unchanged
   startup, resize, and hold sequence, write `/`, `e`, `x`, `i`, and `t` as
   separate PTY writes, sleeping 20 milliseconds after each character, then
   write Enter as a separate `\r` write.
3. Emit a distinct `typed_exit_command` structural event, discard all raw PTY
   screen bytes, and retain the existing exit bound and owned cleanup.
4. Use test-first vertical slices to prove the public modes, exact write chunks,
   five exact requested delays, event order, distinctive fixture exit code 23,
   timeout cleanup, and unchanged existing modes.
5. Simplify the completed observer without changing behavior, run the focused
   gate, and checkpoint the tested observer before declaring a live experiment.

## Deliverable 2 — Qualify Experiment 0008

1. Freeze the official Codex source tag and commit that establish the input
   mechanism, plus the exact native launcher and selected-binary hashes.
2. Freeze one exact packaged Codex shim produced from the tested observer
   checkpoint.
3. Declare Experiment 0008 with the exact commands, identities, timings,
   decision rules, and two-unit zero-retry budget.
4. Run the complete standard verification set, all three deterministic observer
   rehearsals, exact package checks, and `nix run . -- doctor` before dispatch.
5. Confirm Experiment 0008 is the sole active record and checkpoint the
   rehearsed declaration before giving the operator either live command.

## Deliverable 3 — Compare native and packaged typed exit

The operator runs native Codex, then packaged Codex, from
`/Users/cody/dev/fortlet` in the same external Fish shell. The four known runner
marker names must be absent. Each unit uses an 80-by-24 PTY, 100-millisecond
settle, 120-by-40 resize, 20-second unattended hold, the fixed paced typed-exit
action, and a 15-second exit bound.

The operator supplies no additional input and returns each complete structural
result verbatim. Both declared units run once unless a hard invariant fires:

1. Exact `exit_code:0` and `exit_signal:null` for both units accepts packaged
   typed-exit parity.
2. Any native-versus-packaged termination or status mismatch records a
   packaged-path discrepancy but authorizes no repair under this goal.
3. Failure of native Codex to terminate normally rejects the native interaction
   premise; record the packaged unit too, without retry or adaptation.

After the results, verify owned cleanup, close Experiment 0008 terminally,
update conformance and the experiment index, rewrite the handoff, mark this goal
complete or blocked, then STOP and report.

## Definition of Done

1. Deterministic evidence proves separate key writes, the fixed 20-millisecond
   pacing, event order, status preservation, and owned cleanup.
2. One exact native Codex identity and one exact packaged Codex shim receive the
   same source-established interaction from the same external Fish shell.
3. The full gate passes before live dispatch, and both unedited structural
   results are recorded with zero retries.
4. The two `/exit` interactions are local control input, not model prompts; no
   intentional inference or paid quota occurs.
5. Conformance states only what the results prove; then STOP.

## Binding rules

1. Preserve every charter invariant and FIP-0001/FIP-0002 constraint.
2. Keep the new typed action explicit and fixed. Do not use EOF, signals,
   adaptive input, prompt matching, screen-text matching, or a retry.
3. Keep the atomic `observe-exit` path available unchanged for reproducibility.
4. Do not store or print raw PTY content, environment values, account metadata,
   or credential values.
5. Do not change Fortlet, the package contract, live timings, or observer
   behavior after Experiment 0008 is declared.
6. The observer may control only the process group it created and verified.
7. Ordinary Codex-managed and Fortlet-managed state writes are allowed for the
   two launches. Shell startup-file, Fish, Nix, PATH-manager,
   Codex-configuration, and outside-project mutation are not authorized.
8. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: one hour from mission setup.
2. Experiment ceiling: zero money, zero paid quota, zero model prompts, two
   units, zero retries, and at most 15 minutes after dispatch begins.
3. Stop on credential output, unexpected external mutation, unowned process
   control, paid activity, or any need to change a declared live command.

## Verification

Run before issuing either operator command:

- `cargo test --example pty_observer`
- All deterministic observer fixture modes declared by Experiment 0008
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- Exact source, native identity, package identity, marker-name, unchanged-path,
  and one-active-experiment checks declared in Experiment 0008
