# Session Handoff — Codex marker mechanism rejected

Audience: a fresh agent session. `GOAL.md` is normative and complete.
Experiment 0005 is terminally closed. Do not resume Experiments 0003, 0004, or
0005, and do not change Fortlet from the remaining incomplete attribution.

## Verified implementation state

1. Packaged Codex and Tact shims enter managed Linux capsules, attach through
   real PTYs, show activity after resize, reconcile concurrent invocations to
   the same harness capsule, and remain responsive afterward.
2. The repository observer at checkpoint `ad7fdd0e` remains unchanged. Six
   focused tests and its deterministic fixture prove resize, signal, status,
   timeout, and owned cleanup without persisting raw PTY bytes.
3. Experiment 0003's packaged Codex and Tact UIs both remained alive after one
   foreground-group `SIGINT`.
4. Experiment 0004 established that native Tact also remains alive, so the Tact
   timeout is not evidence of a Fortlet-specific first-interrupt defect.
5. Experiment 0004's native Codex control closed its PTY during startup settling
   before resize or signal. The environment contained outer-Codex marker names,
   leaving their causal role open.

## Experiment 0005 terminal result

The exact Codex 0.147.0 launcher and native-binary hashes matched. Product and
observer paths were unchanged since `ad7fdd0e`. Before dispatch, all six
observer tests, the fixture rehearsal, the full Rust suite, formatting, strict
Clippy, conformance, exact-tree `nix flake check`, and doctor passed. The known
app-`meta` warning and incompatible `x86_64-linux` omission remain.

The single launch removed exactly `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
`CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI`, while preserving the actual
workspace sandbox and npm-managed environment behavior. Native Codex again
emitted initial activity and closed its PTY during startup settling, before
resize or signal. No retry was made. Owned cleanup left no matching process.

This falsifies the declared marker-removal mechanism. It does not establish why
native Codex closes in this runner context, whether native Codex normally exits
after its first interrupt, or whether packaged Codex preserves exact eventual
exit status. No product repair is attributed.

The experiment cost zero money, zero paid quota, zero prompts, one unit, zero
retries, and 11 seconds through cleanup verification.

## Successor boundary

The most direct remaining control is a separately declared native Codex run
from a terminal outside an existing Codex runner. It must explicitly bound or
authorize native Codex's possible host-state writes before dispatch. A
deterministic guest terminal probe is the alternative if the next question is
SDK signal transport rather than native UI semantics.

Preserve optional shim activation, fail-closed behavior, credential isolation,
package runtime integrity, no-prompt operation, observer ownership checks, and
Jujutsu checkpoint discipline. Native Linux verification and broader FIP-0001
gaps remain outstanding.
