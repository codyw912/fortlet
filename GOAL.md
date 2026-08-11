# GOAL: Interactive packaged session acceptance

Status: active on 2026-08-11 under operator-authorized Deliverable 4. Both real
packaged UIs passed PTY, resize, concurrent-capsule, and responsiveness checks,
but neither terminated within 15 seconds after the single declared `SIGINT`.
The repeated-failure trigger remains active against another packaged signal
variation; the authorized next action is a native-harness control that tests
the shared one-`SIGINT` assumption without changing Fortlet.

Prove that packaged `codex` and `tact` shims support ordinary interactive
terminal use through Fortlet's existing fail-closed capsule-session path. Build
only the test-side machinery and product repairs required by falsifiable PTY,
resize, concurrent-attachment, signal, and exit evidence.

Before designing or implementing, read FIP-0001 and FIP-0002 in full and the
validated design at
`docs/plans/2026-08-11-interactive-session-acceptance-design.md`.

## Deliverable 1 — Rehearse the PTY observer

Completed 2026-08-11: checkpoint `ad7fdd0e` adds the bounded observer, six
focused tests, deterministic fixture rehearsal, structured events, buffered-
startup exclusion, and owned cleanup.

1. Implement the smallest repository test tool that can launch an immutable
   packaged shim under a PTY, set and change its dimensions, control its child
   process group, send a signal, bound execution, and record exit status.
2. Make the tool emit bounded structured events. Do not persist raw harness
   screen content or account/project metadata in repository records.
3. Rehearse the complete observation and cleanup path against a deterministic
   fixture that reports terminal dimensions, reacts to resize, traps signals,
   and exits predictably.
4. Add focused automated coverage for the driver. It is test tooling, not a
   Fortlet command or a generic runtime abstraction.
5. The driver may terminate only its own process group and remove only
   temporary state it created.

## Deliverable 2 — Prepare the interactive screen

Completed 2026-08-11: all standard gates, exact package checks, and doctor
passed; Experiment 0003 was declared at checkpoint `920aac22`.

1. Run the complete standard verification set and package checks.
2. Run `nix run . -- doctor` without printing credential values.
3. If the rehearsal exposes a Fortlet defect, repair only that defect and ship
   focused regression evidence and conformance in the same checkpoint.
4. Stop for a proposal if a repair would change an accepted public, terminal,
   credential, or capsule contract. Otherwise FIP-0001 and FIP-0002 already
   authorize this mission.
5. Predeclare the next numbered experiment with one fixed Codex unit and one
   fixed Tact unit, an exact immutable package output, zero prompts, zero
   retries, and the protocol below.

## Deliverable 3 — Exercise both real harness UIs

Completed as a terminally rejected experiment on 2026-08-11. Both units ran
without retry. See `experiments/0003-interactive-packaged-sessions.md`.

For each harness unit, in fixed order:

1. Launch its packaged shim UI in an 80 by 24 PTY without submitting a prompt
   or newline.
2. Observe sustained process life and terminal output without gating on exact
   UI wording.
3. Resize to 120 by 40 and require subsequent terminal activity consistent
   with a redraw.
4. While the UI remains attached, run the same packaged shim with `--version`
   and use a read-only label query to verify both invocations select the same
   project-and-harness capsule.
5. Send `SIGINT` to the foreground process group, require bounded termination,
   and record the exact exit status.
6. Use a final non-interactive invocation to prove the managed capsule remains
   responsive.

Both units run even if one fails unless a hard invariant fires. Afterward,
close the experiment terminally, update conformance, rewrite the handoff, mark
this goal complete or blocked, then STOP and report.

## Deliverable 4 — Falsify the shared signal assumption

Authorized by the operator on 2026-08-11 after Experiment 0003's repeated
failure trigger.

1. Predeclare a new bounded control; do not resume Experiment 0003 and do not
   change product code or the observer.
2. Run the directly resolved native Codex and Tact UIs under the unchanged PTY
   observer with Experiment 0003's dimensions, settling, hold, one-`SIGINT`,
   exit bound, zero-input rule, order, and zero-retry discipline.
3. Record exact native executable identities before dispatch. Do not activate
   Fortlet shims or use `fortlet native` for the control.
4. If both native UIs also remain alive, reject the shared termination
   assumption rather than attributing the packaged result to Fortlet. If a
   native UI exits while its packaged counterpart did not, record the narrowed
   propagation defect. Mixed evidence establishes no broad cross-harness claim.
5. Close the control terminally, update conformance and the handoff honestly,
   mark this goal complete or blocked, then STOP and report. Any product repair
   requires a successor mission justified by the control.

## Definition of Done

1. The PTY observer proves its own dimensions, resize, signal, status, timeout,
   and owned-cleanup behavior against a deterministic fixture.
2. Both real packaged shims exercise the interactive SDK attachment path with
   observable activity before and after resize.
3. Each concurrent invocation selects the same managed capsule as its matching
   interactive session.
4. Signal delivery terminates each observed session within its declared bound,
   with exact status recorded, and the capsule remains responsive afterward.
5. No prompt is submitted, no intentional model inference or paid quota is
   used, and no native fallback, credential value, unexpected mount, unowned
   process control, or shell-configuration mutation occurs.
6. Product defects discovered by the protocol have focused regression evidence;
   claims not established remain explicit conformance gaps.
7. The complete verification set is green; then STOP and report.

Deliverable 4 may close this diagnostic continuation honestly without making
criterion 4 true. In that case the terminal record and conformance map must
state whether the missing claim is a Fortlet defect, a rejected acceptance
assumption, or still unattributed.

## Binding rules

1. Preserve every charter invariant and every FIP-0001/FIP-0002 constraint.
2. Never write prompt text or a newline to a real harness UI during this goal.
3. Never silently fall back to a host harness or expose provider credentials,
   SSH keys, signing agents, or publication authority to a capsule.
4. Successful wrapper startup remains silent; first-use provisioning progress
   is allowed and must not be mistaken for a wrapper banner.
5. Do not introduce lifecycle commands, lease redesign, standalone
   distribution, Linux verification, new harnesses, remote execution,
   publication, or a generic runtime/terminal abstraction.
6. Use reviewable Jujutsu checkpoints and inspect `main..@` before handoff.

## Budget and escalation

1. Engineering ceiling: 2 hours. Stop earlier as soon as the Definition of Done
   is met; report actual measured effort.
2. Each experiment ceiling: zero money, zero paid quota, zero submitted
   prompts, two harness units, zero retries, and at most 30 minutes after
   dispatch begins. Deliverable 4 remains inside the original cumulative
   two-hour engineering ceiling.
3. Local Fortlet-owned test capsules, processes, temporary files, and state are
   allowed. Mutation or termination of unowned state is not.
4. Stop on any need to weaken an invariant, change an accepted FIP,
   intentionally initiate model inference, or expand the product boundary.
5. Two consecutive terminal failures sharing an assumption trigger escalation.

## Verification

Run from `nix develop`:

- PTY observer fixture tests and rehearsal command declared by Deliverable 1
- `cargo test`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --test conformance`
- `nix flake check`
- `nix run . -- doctor`
- The packaged interactive commands and label queries declared in the
  experiment record
