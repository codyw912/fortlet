# Session Handoff — First-interrupt behavior attributed

Audience: a fresh agent session. `GOAL.md` is normative and complete.
Experiments 0003 through 0006 are terminally closed. Do not resume them or
repair Fortlet from their rejected assumptions.

## Verified product evidence

1. Packaged Codex and Tact shims enter managed Linux capsules without host
   fallback, attach through real PTYs, produce activity after resize, reconcile
   concurrent invocations to the same harness capsule, and remain responsive.
2. Both packaged UIs remained alive after one foreground-group `SIGINT`.
3. Native Tact reproduced that result under the unchanged observer.
4. Native Codex closed before signal when the observer ran inside an active
   Codex runner, even after known marker names were removed.
5. In Experiment 0006, the operator ran the exact native Codex 0.147.0 launcher
   from a separate Fish shell where all four runner markers were absent. Native
   Codex reached initial activity, resize activity, the unattended hold, and
   signal, then remained alive beyond the same 15-second exit bound.

The evidence now attributes both packaged timeouts to native first-interrupt
semantics rather than a Fortlet-specific propagation defect. It does not prove
the exact eventual termination action or exit-status preservation for either
packaged UI.

## Experiment 0006 integrity

Before dispatch, both native hashes matched, product and observer paths were
unchanged since `ad7fdd0e`, Experiment 0006 was the sole active record, and all
six observer tests plus the deterministic fixture passed. The full Rust suite,
formatting, strict Clippy, explicit conformance, exact-tree `nix flake check`,
and doctor passed. The known app-`meta` warning and incompatible
`x86_64-linux` omission remain.

The operator reported no input or retry and returned the complete structural
output. A final AppleScript-style `-2741` diagnostic appeared after the observer
timeout; its provenance is unknown and it did not affect the declared event
sequence or decision. Read-only process inspection found no matching observer
or npm launcher afterward.

Actual cost was zero money, zero paid quota, zero prompts, one unit, and zero
retries. Numeric wall time was not captured; do not backfill an estimate.

## Successor boundary

No product repair is justified for first-interrupt handling. If daily-use
acceptance requires exact termination and exit status, a successor must first
identify the native UI's actual termination action and compare that same action
through the packaged path. Do not assume a first `SIGINT` should exit.

Preserve optional shim activation, fail-closed behavior, credential isolation,
package runtime integrity, no-prompt operation, observer ownership checks, and
Jujutsu discipline. Native Linux verification and broader FIP-0001 gaps remain
outstanding.
