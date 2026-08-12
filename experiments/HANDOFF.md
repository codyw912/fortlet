# Session Handoff — Blind Codex exit timing is exhausted

Audience: a fresh agent session. `GOAL.md` is normative and complete.
Experiments 0003 through 0008 are terminally closed. Do not resume them, adapt
their input timings, or repair Fortlet from their rejected assumptions.

## Verified product evidence

1. Packaged Codex and Tact shims enter managed Linux capsules without host
   fallback, attach through real PTYs, produce activity after resize, reconcile
   concurrent invocations to the same harness capsule, and remain responsive.
2. Both packaged UIs remained alive after one foreground-group `SIGINT`.
   Native Tact and externally launched native Codex reproduced that behavior,
   removing the evidence basis for a Fortlet signal defect.
3. The PTY observer retains separate signal, atomic-exit, and typed-exit modes.
   Twelve focused tests prove structured event order, exact atomic bytes, six
   separate typed key writes, five fixed 20-millisecond delays, distinctive
   fixture code 23, both timeout paths, and owned cleanup.
4. Native and packaged Codex 0.147.0 both remained alive after atomic
   `/exit\r` in Experiment 0007 and after the fixed paced typed interaction in
   Experiment 0008. Neither comparison supplied an exit status, and neither
   exposed a Fortlet-specific discrepancy.

## Source finding and Experiment 0008 integrity

Official Codex tag `rust-v0.147.0` peels to commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`. Source inspection established that
Codex classifies characters no more than 8 milliseconds apart as a paste burst
and turns Enter during that burst into a composer newline. This explains why
Experiment 0007's atomic write did not submit the local command.

Experiment 0008 conservatively wrote `/`, `e`, `x`, `i`, and `t` separately
with 20 milliseconds after each, then wrote Enter separately. Before dispatch,
all twelve tests, all three fixtures, the full Rust gate, strict Clippy,
conformance, exact-tree `nix flake check`, package smoke, and doctor passed. The
observer and native hashes matched, the immutable packaged shim returned
`codex-cli 0.147.0`, Experiment 0008 was the sole active record, and the external
Fish shell reported all four runner marker names absent.

The operator ran native and packaged units once each, in order, without a
reported retry or additional input. Both emitted the same sequence through
`typed_exit_command`, then timed out after 15 seconds. Narrowed process checks
found no matching live observer or launch command after either unit. Both also
ended with the same AppleScript-style `-2741` diagnostic seen in prior records;
its provenance is unknown and it is excluded from decisions.

Actual experiment cost was zero money, zero paid quota, zero model prompts, two
units, and zero retries. Numeric wall time was not captured and must not be
backfilled.

## Successor boundary

No Fortlet repair is justified. Blind timing changes are exhausted: do not
retry atomic input, change the paced delay, add prompt matching, send EOF or
signals, or ask the operator to repeat these experiments.

If exact daily-use termination remains release-critical, a successor must first
directly establish the native UI's composer readiness or receipt and handling
of input events without retaining raw screen content or submitting a model
prompt. That likely requires a source-instrumented native control or another
mechanism that observes the input state itself, not another externally timed
byte sequence. It needs a new mission and experiment.

Otherwise, return to the broader FIP-0001/FIP-0002 gaps: native Linux package
verification, standalone installation, failure-stage evidence, project-root
and broad-root behavior, capsule topology, lifecycle leases, or management
commands. Preserve optional shim activation, fail-closed behavior, credential
isolation, package runtime integrity, and Jujutsu discipline.
