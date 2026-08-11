# External native Codex control

Status: validated with the operator on 2026-08-11.

## Outcome

Run one native Codex UI outside the active Codex runner to establish whether
normal native startup reaches the same first-interrupt boundary as packaged
Codex. Prior controls closed before signal inside the runner, even after its
known marker names were removed. This control changes the containing execution
context rather than another marker or signal variation.

## Controlled path

The operator uses a separate Fish shell and has reported `CODEX_THREAD_ID`,
`CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` absent by
name. The exact Codex 0.147.0 npm entry point runs under the unchanged observer
from `/Users/cody/dev/fortlet`, with the same PTY dimensions, settling, resize,
hold, one foreground-group `SIGINT`, and exit bound as Experiments 0003 through
0005. The operator supplies no input and does not retry.

An additional sandbox is deliberately omitted because it would reintroduce a
runner-context difference. The operator authorizes ordinary Codex-managed state
writes for this one native launch. No shell startup-file, Fish configuration,
Nix configuration, or PATH-manager mutation is authorized.

## Evidence and decisions

Before dispatch, exact hashes, unchanged source paths, the full repository gate,
and the deterministic observer fixture must pass and be checkpointed. The
observer suppresses raw native screen bytes; the operator returns only its
complete structural output.

If native Codex reaches signal and stays alive, the packaged timeout matches
native first-interrupt behavior. If it exits, the exact status establishes a
packaged-path discrepancy. If it closes before signal again, the external
context mechanism is rejected. Each result is terminal and authorizes no
repair. The mission has one unit, zero retries, zero prompts, a ten-minute
dispatch ceiling, and a thirty-minute engineering ceiling.
