# Session Handoff — External native Codex control declared

Audience: a fresh agent session. `GOAL.md` is normative. Experiment 0006 is the
only active experiment and has not been dispatched. Do not resume Experiments
0003 through 0005.

## Verified predecessor state

1. Both packaged shims attach through real PTYs, show activity after resize,
   share their expected capsules with concurrent invocations, and remain
   responsive.
2. Both packaged UIs remained alive after one foreground-group `SIGINT`.
3. Native Tact reproduced that first-interrupt timeout, so the Tact result is
   not evidence of a Fortlet-specific defect.
4. Native Codex closed during startup settling in Experiments 0004 and 0005.
   Removing inherited Codex marker names did not alter the close, so those
   names are not the mechanism.
5. The observer at checkpoint `ad7fdd0e` remains unchanged and stores only
   structural events.

## Active operator control

The operator opened a separate Fish shell and reported all four checked Codex
runner markers absent by name. Experiment 0006 launches the exact native Codex
0.147.0 npm entry point from that shell under the unchanged observer. The
operator authorized ordinary Codex-managed state writes for this one no-input
launch; shell startup, Fish, Nix, and PATH-manager edits remain unauthorized.

Before giving the live command, re-verify both hashes, the unchanged product
and observer paths, the one-active-experiment invariant, the complete runbook
gate, and the deterministic fixture. Record and checkpoint that rehearsal.

The operator then runs one exact command, supplies no input or interrupt, waits
for automatic completion, and returns the complete structured output. There is
no retry. Reaching signal and timing out matches native and packaged behavior;
reaching signal and exiting narrows a packaged-path discrepancy; another
pre-signal close rejects the external-runner mechanism.

After the result, confirm owned cleanup, close Experiment 0006, update
conformance and this handoff, mark the goal complete or blocked, and stop. No
product repair is authorized.

Preserve optional shim activation, fail-closed behavior, credential isolation,
package runtime integrity, no-prompt operation, and Jujutsu discipline.
