# Session Handoff — Native Tact attributed; Codex control blocked

Audience: a fresh agent session. `GOAL.md` is normative and blocked after the
terminal closure of Experiment 0004. Do not resume Experiments 0003 or 0004,
retry a signal variation, or change Fortlet from the incomplete attribution.

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

## Experiment 0004 terminal result

The unchanged observer and fixture re-qualified immediately before dispatch.
The direct native identities were Codex 0.147.0 and Tact 0.3.7; exact paths and
SHA-256 hashes are frozen in the experiment record.

Native Codex emitted initial activity, then closed its PTY while startup output
was settling. It never reached resize or signal. The host environment contained
Codex nested-session marker names, but because raw UI bytes were intentionally
not retained, the early close is unattributed. No retry was made.

Native Tact emitted activity before and after resize, reached the 20-second
hold, received the one observer-owned foreground-group `SIGINT`, and remained
alive beyond the 15-second exit bound. This matches packaged Tact and rejects
the assumption that its first interrupt should terminate. It is affirmative
evidence against attributing Experiment 0003's Tact result to Fortlet.

Owned cleanup left no matching observer or harness process. The two-unit
control cost zero money, zero paid quota, zero prompts, zero retries, and 63
seconds. After closure, the complete runbook verification set and doctor
passed; the known app-`meta` warning and native `x86_64-linux` omission remain.

## Successor boundary

The Codex signal comparison and exact packaged exit-status preservation remain
unproven. A successor goal may predeclare either a native Codex control outside
an existing Codex session or a deterministic guest terminal probe. It must not
resume either terminal experiment, infer a Fortlet repair from Tact's matching
native behavior, or weaken the no-prompt and process-ownership rules.

Preserve the optional shim activation, fail-closed behavior, credential
boundary, package runtime-integrity check, no-prompt rule, observer ownership
checks, and Jujutsu checkpoint discipline. Native Linux verification and the
broader FIP-0001 gaps remain outstanding.
