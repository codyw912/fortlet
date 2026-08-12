# Session Handoff — Codex-specific automated exit work is closed

Audience: a fresh agent session. `GOAL.md` is normative and complete.
Experiments 0003 through 0009 are terminally closed. Do not resume them, adapt
their input timings, or repair Fortlet from their rejected assumptions.

## Verified product evidence

1. Packaged Codex and Tact shims enter managed Linux capsules without host
   fallback, attach through real PTYs, produce activity after resize, reconcile
   concurrent invocations to the same harness capsule, and remain responsive.
2. Both packaged UIs remained alive after one foreground-group `SIGINT`.
   Native Tact and externally launched native Codex reproduced that behavior,
   removing the evidence basis for a Fortlet signal defect.
3. The PTY observer now retains four separate actions: process-group signal,
   atomic `/exit`, paced `/exit`, and one Ctrl-C key byte. Fifteen focused tests
   prove exact action delivery, structured event order, distinctive fixture
   status, timeout boundaries, and owned cleanup.
4. Native and packaged Codex 0.147.0 matched under all three attempted exit
   controls: atomic `/exit`, paced `/exit`, and one `0x03` PTY key. All six live
   units reached their structural action and timed out. No comparison supplied
   an exit status or exposed a Fortlet-specific discrepancy.
5. The operator manually confirmed native Codex exits after one Ctrl-C at an
   idle, empty composer. During active work the first Ctrl-C interrupts and a
   later Ctrl-C exits. This daily-use behavior is not contradicted by the
   observer's inability to establish the same UI state.

## Source finding and Experiment 0009 integrity

Official Codex tag `rust-v0.147.0` peels to commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`. Its TUI routes Ctrl-C through
modal, history-search, composer-text, and active-work cancellation before idle
quit. The formal one-second double-press shortcut is disabled in this version,
so one Ctrl-C at a truly idle empty composer requests exit.

Experiment 0009's observer wrote exactly one `0x03` byte after initial
activity, quiet settle, resize activity, and a fixed 5-second hold. Before
dispatch, all fifteen tests, all four fixtures, the full Rust gate, strict
Clippy, conformance, exact-tree `nix flake check`, package smoke, and doctor
passed. Exact observer and native hashes matched; Experiment 0009 was the sole
active record; and the external Fish shell reported all four runner marker
names absent.

The operator ran native and packaged units once each, in order, without a
reported retry or additional input. Both emitted the same sequence through
`ctrl_c_key`, then timed out after 15 seconds. Narrowed process checks found no
matching live observer or launch command after either unit. Both also ended
with the AppleScript-style `-2741` diagnostic seen in prior records; its
provenance is unknown and it is excluded from decisions.

Actual experiment cost was zero money, zero paid quota, zero model prompts, two
units, and zero retries. Numeric wall time was not captured and must not be
backfilled.

## Successor boundary

No Fortlet repair is justified. Stop Codex-specific automated exit work: do
not retry or vary slash input, Ctrl-C keys, process signals, holds, or readiness
timings. Further work would require observing native UI state, which is
disproportionate to the current product risk and conflicts with the deliberate
raw-content boundary.

For ordinary use, retain manual idle Ctrl-C as a lightweight smoke check. For
automated exit-status propagation, prefer a deterministic guest process that
returns a distinctive code and isolates Fortlet/MicroSandbox transport from
harness UI state.

The next goal should return to a broader FIP-0001/FIP-0002 gap: native Linux
package verification, standalone installation, failure-stage evidence,
project-root and broad-root behavior, capsule topology, lifecycle leases, or
management commands. Preserve optional shim activation, fail-closed behavior,
credential isolation, package runtime integrity, and Jujutsu discipline.
