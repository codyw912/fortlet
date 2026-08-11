# Session Handoff — Native Codex marker control declared

Audience: a fresh agent session. `GOAL.md` is normative. Experiment 0005 is the
only active experiment and has not been dispatched. Verify this state before
relying on it.

## Verified predecessor state

1. Packaged Codex and Tact shims enter managed Linux capsules, attach through
   real PTYs, show activity after resize, reconcile concurrent invocations to
   the same harness capsule, and remain responsive afterward.
2. The unchanged repository observer at checkpoint `ad7fdd0e` has six focused
   tests and a deterministic fixture proving resize, signal, status, timeout,
   and owned cleanup without persisting raw PTY bytes.
3. Experiment 0003's packaged UIs both remained alive after one `SIGINT`.
4. Experiment 0004 established that native Tact behaves the same way, so its
   timeout is not evidence of a Fortlet-specific first-interrupt defect.
5. Experiment 0004's native Codex unit closed its PTY during startup settling.
   The host environment contained outer-Codex session marker names, but their
   causal role was not tested.
6. The full runbook verification set and doctor passed after Experiment 0004.
   The known app-`meta` warning and native `x86_64-linux` omission remain.

## Active control

Experiment 0005 tests one mechanism: inherited outer-Codex markers caused the
native Codex early close. It launches the same exact Codex 0.147.0 npm entry
point under the same observer, project, sandbox, PTY dimensions, timings, and
one-`SIGINT` protocol as Experiment 0004. Only `CODEX_THREAD_ID`,
`CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` are removed
from the observer and child environment. Values are never recorded.

Do not remove `CODEX_MANAGED_*`, weaken the actual execution sandbox, write any
PTY input, alter timings, retry, or inspect raw UI content. Re-verify the exact
hashes, complete runbook gate, observer tests, and fixture before checkpointing
the rehearsed declaration and dispatching the one unit.

The unit has three terminally valid outcomes: reach signal and time out, reach
signal and exit with exact status, or close before signal again. The first
matches native and packaged first-interrupt behavior; the second narrows a
packaged-path discrepancy; the third rejects the marker hypothesis and leaves
an external-terminal control for a future mission.

Afterward, verify owned cleanup, close the experiment, update conformance and
this handoff, mark the goal complete or blocked, and stop. No product repair is
authorized by this mission.

Preserve optional shim activation, fail-closed behavior, credential isolation,
package runtime integrity, observer ownership checks, and Jujutsu checkpoint
discipline.
