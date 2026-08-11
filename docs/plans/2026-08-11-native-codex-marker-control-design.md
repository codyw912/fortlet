# Native Codex marker control

Status: validated with the operator on 2026-08-11.

## Outcome

Run one bounded control to determine whether Experiment 0004's native Codex UI
closed early because it inherited markers from the outer Codex session. This is
diagnostic evidence only. It changes no Fortlet code, observer behavior,
accepted architecture, package, or native harness state.

## Controlled difference

The control launches the exact Codex 0.147.0 npm entry point and uses the
unchanged PTY observer, project directory, execution sandbox, dimensions,
settling, resize, hold, signal, and exit bound from Experiment 0004. It removes
only `CODEX_THREAD_ID`, `CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`, and
`CODEX_CI` from the observer and child environment. Their values are never
read or recorded. `CODEX_MANAGED_*` remains available for the installed npm
launcher to manage normally. Removing marker names does not remove or weaken
the externally enforced workspace-only sandbox.

## Evidence flow

Before dispatch, verify the exact launcher and native-binary hashes, confirm the
four marker names are present by name only, run the complete repository gate,
and rehearse the unchanged observer against its deterministic fixture. Then
checkpoint the declared experiment and run one native Codex unit with zero
input and zero retries.

If the unit reaches the signal and remains alive, native Codex matches the
packaged first-interrupt behavior. If it reaches the signal and exits, the exact
status establishes a packaged-path discrepancy but authorizes no repair. If it
closes before signal again, the marker mechanism is rejected and any external-
terminal control becomes a separately declared successor. All outcomes close
the experiment terminally.

## Safety and stopping rule

The observer controls only its verified child process group and stores only
structural events. No prompt, newline, model inference, credential value, shell
mutation, host setup change, or unowned cleanup is allowed. The mission ends
after one unit, hard-invariant failure, or ten dispatch minutes, within a
30-minute engineering ceiling.
