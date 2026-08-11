# Session Handoff — Explicit Codex exit still unresolved

Audience: a fresh agent session. `GOAL.md` is normative and complete.
Experiments 0003 through 0007 are terminally closed. Do not resume them or
repair Fortlet from their rejected assumptions.

## Verified product evidence

1. Packaged Codex and Tact shims enter managed Linux capsules without host
   fallback, attach through real PTYs, produce activity after resize, reconcile
   concurrent invocations to the same harness capsule, and remain responsive.
2. Both packaged UIs remained alive after one foreground-group `SIGINT`.
3. Native Tact and externally launched native Codex reproduced that first-
   interrupt behavior, removing the evidence basis for a Fortlet signal defect.
4. The PTY observer now has a separate `observe-exit` action. Deterministic
   evidence proves it writes exactly `/exit\r`, emits an `exit_command` event,
   preserves fixture exit code 23, and cleans up an owned process that ignores
   the command.
5. In Experiment 0007, native and packaged Codex 0.147.0 each reached initial
   activity, real resize activity, the fixed hold, and `exit_command`, then
   remained alive beyond the same 15-second exit bound.

The matching explicit-input timeouts expose no Fortlet-specific discrepancy.
They do not establish that either UI accepted `/exit\r` as a command, and they
provide no exit status with which to verify packaged status preservation.

## Experiment 0007 integrity

The observer was checkpointed before declaration. Nine focused tests, both
deterministic fixtures, the full Rust suite, formatting, strict Clippy,
explicit conformance, exact-tree `nix flake check`, exact packaged
`codex --version`, and doctor passed before dispatch. Both native hashes and
the observer hash matched, product paths were unchanged, Experiment 0007 was
the sole active record, and the operator reported all four known runner marker
names absent from the external Fish shell.

The operator ran native and packaged units once each, in order, without a
reported retry or additional input. Both returned the same structural sequence
through `exit_command` and the same timeout. Both also ended with an identical
AppleScript-style `90:91 ... (-2741)` line whose provenance remains unknown;
it is not part of the decision. Read-only checks found no matching owned
process after either unit.

Actual cost was zero money, zero paid quota, zero model prompts, two units, and
zero retries. Numeric wall time was not captured; do not backfill an estimate.

## Successor boundary

No product repair is justified. Do not retry first `SIGINT`, marker removal,
external first-signal control, or exact `/exit\r` injection: those protocols
are settled.

If exact daily-use termination remains release-critical, first establish the
native UI's accepted termination interaction under a protocol that can
distinguish command readiness or key interpretation without persisting raw UI
content, submitting a model prompt, or adapting a failed unit. Source-level
inspection of the pinned Codex input handling or a new bounded native-only
control may supply that mechanism. It requires a new mission and experiment.

Preserve optional shim activation, fail-closed behavior, credential isolation,
package runtime integrity, observer ownership checks, and Jujutsu discipline.
Native Linux verification and broader FIP-0001 gaps remain outstanding.
