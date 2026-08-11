# GOAL: Attribute native Codex first-interrupt behavior

Status: completed on 2026-08-11 with the declared mechanism rejected. Removing
the four outer-Codex marker names did not change the native UI's pre-signal PTY
close. Codex first-interrupt attribution remains outside this completed goal;
do not retry Experiment 0005.

Determine whether inherited outer-Codex session markers caused Experiment
0004's native Codex UI to close before its signal boundary. Run one bounded
native Codex control with only those markers removed. Do not change Fortlet,
the PTY observer, accepted architecture, or native harness state.

Before dispatch, read FIP-0001 and FIP-0002 in full, the validated design, and
Experiments 0003 and 0004.

## Deliverable 1 — Freeze and qualify the control

Completed on 2026-08-11 at checkpoint `cd5445f6`: both hashes matched, marker
presence was recorded by name only, the product and observer were unchanged,
and the complete verification set plus fixture rehearsal passed.

1. Verify the exact Codex 0.147.0 launcher and native-binary hashes declared in
   Experiment 0005.
2. Record only whether the four declared marker names are initially present;
   never record environment values.
3. Confirm the product and observer are unchanged from the verified Experiment
   0004 stack.
4. Run the complete standard verification set, the six observer tests, and the
   deterministic fixture rehearsal before dispatch.
5. Checkpoint the predeclared experiment before any native UI launch.

## Deliverable 2 — Run one native Codex unit

Completed as a terminally rejected mechanism on 2026-08-11. The sole unit
closed during startup settling before resize or signal, matching Experiment
0004 despite the four-name removal. See Experiment 0005.

Launch the exact native Codex npm entry point under the unchanged observer from
`/Users/cody/dev/fortlet`, removing only these inherited names from the
observer and child environment:

- `CODEX_THREAD_ID`
- `CODEX_SANDBOX`
- `CODEX_SANDBOX_NETWORK_DISABLED`
- `CODEX_CI`

Keep the real workspace sandbox and `CODEX_MANAGED_*` launcher behavior intact.
Use an 80-by-24 PTY, settle for 100 milliseconds, resize to 120 by 40, allow a
20-second unattended hold, send one `SIGINT` to the verified observer-owned
foreground process group, and wait at most 15 seconds for exit. Write no prompt,
newline, or other input. Do not retry or adapt the environment, signal, or
timings.

Close the unit by the first matching rule:

1. If it reaches signal and remains alive, native Codex matches the packaged
   first-interrupt behavior; do not attribute Experiment 0003 to Fortlet.
2. If it reaches signal and exits, record the exact status and the narrowed
   packaged-path discrepancy; do not repair it under this goal.
3. If it closes before signal again, reject the marker-removal mechanism. A
   future external-terminal control requires a new goal and experiment.

After the unit, verify owned cleanup, close Experiment 0005 terminally, update
conformance and the handoff, mark this goal complete or blocked, then STOP and
report.

## Definition of Done

1. The experiment freezes one exact Codex identity and changes only the four
   declared environment names relative to Experiment 0004.
2. The unchanged observer is qualified immediately before dispatch.
3. The single unit is recorded without raw PTY bytes, retries, or adaptive
   input, and one of the three decision rules is applied without inference.
4. No provider credential value, paid quota, host setup mutation, unowned
   process control, native fallback, or product change occurs.
5. Conformance and the handoff state only what the result establishes; then
   STOP and report.

## Binding rules

1. Preserve every charter invariant and FIP-0001/FIP-0002 constraint.
2. Unsetting a marker must not weaken or bypass the execution sandbox; the
   workspace-only write boundary remains enforced externally.
3. Do not unset `CODEX_MANAGED_BY_NPM` or `CODEX_MANAGED_PACKAGE_ROOT`; the npm
   launcher may manage them normally.
4. Never persist raw native UI output, environment values, account metadata, or
   credentials.
5. Do not alter the observer, Fortlet, package, harness configuration, shell
   startup files, or native Codex state.
6. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: 30 minutes from implementation start.
2. Experiment ceiling: zero money, zero paid quota, zero prompts, one unit,
   zero retries, and at most 10 minutes after dispatch begins.
3. Stop on any need to weaken an invariant, initiate model inference, change an
   accepted FIP, mutate state outside the project, or control an unowned
   process.
4. Any early close settles the unit; it does not authorize a second launch.

## Verification

Run from `nix develop` before dispatch:

- `cargo test --example pty_observer`
- `cargo run --quiet --example pty_observer -- fixture`
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- Exact identity and marker-name checks declared in Experiment 0005
