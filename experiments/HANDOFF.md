# Session Handoff — Interactive evidence gained; signal exit blocked

Audience: a fresh agent session. `GOAL.md` is normative and blocked by the
charter's repeated-failure trigger. Do not run another signal variation until
the operator authorizes a control that tests the shared assumption.

## Verified implementation state

1. The predecessor transparent-shim mission remains complete. Packaged Codex
   and Tact non-interactive commands enter managed Linux capsules without host
   fallback or credential output.
2. Checkpoint `ad7fdd0e` adds a repository-local PTY observer. It is test
   tooling, not an installed Fortlet command or runtime abstraction.
3. Six focused observer tests cover parsing, PTY activity, actual resize,
   buffered-startup exclusion, structured event order, `SIGINT` status,
   timeout, and owned-process cleanup.
4. The deterministic fixture rehearsal emits the complete event sequence and
   terminates with signal 2. The observer never persists raw PTY screen bytes.
5. Before live dispatch, the observer tests, full Rust suite, formatting,
   strict all-target Clippy, explicit conformance test, exact-tree
   `nix flake check`, package integrity checks, and doctor all passed on
   `aarch64-darwin`.

## Experiment 0003 terminal result

Both prompt-free real UI units produced activity before and after resizing from
80 by 24 to 120 by 40. During each 20-second window, a concurrent packaged
`--version` invocation returned the pinned guest version and read-only label
queries returned one unchanged matching managed capsule.

For both Codex and Tact, the observer then successfully sent the one declared
`SIGINT` to its verified foreground process group, but the UI did not terminate
within 15 seconds. Each observer exited `1` without an `exited` event or numeric
summary. The observer's owned cleanup completed, and final packaged
`--version` checks proved both capsules remained responsive.

No prompt, newline, intentional model inference, paid quota, credential value,
native fallback, unexpected mount, unowned process control, or shell mutation
was observed. Experiment cost was zero money, two units, zero retries, and 134
seconds.

## Why work stopped

The two harness units are consecutive failures of the same assumption: that
one foreground-group `SIGINT` should terminate a real harness UI. Current
evidence does not attribute the result. Codex and Tact may both treat SIGINT as
an in-UI cancellation, or Fortlet/MicroSandbox may fail to propagate the signal
as intended. A third signal or timeout variation would repeat the mask-level
failure prohibited by the self-correction playbook.

## Next action requiring operator direction

Choose a bounded control that can distinguish native harness semantics from
the isolated terminal path before changing product code. Two plausible controls
are running native Codex and Tact UIs under the same observer, or attaching a
deterministic guest terminal probe through the existing SDK seam. Either choice
must be newly declared; do not resume Experiment 0003.

Preserve the optional shim activation, fail-closed behavior, credential
boundary, package runtime-integrity check, no-prompt rule, observer ownership
checks, and Jujutsu checkpoint discipline. Native Linux verification and the
broader FIP-0001 gaps remain outstanding.
