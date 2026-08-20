# Experiment 0044: Managed startup phase attribution

Status: completed — terminal-failure
Design: FIP-0001, FIP-0002

## Baseline / Control

Experiment 0043 used packaged Fortlet 0.1.0 and three fixed lifecycle cycles on
the operator's aarch64-darwin host. Absent launches were 0.71, 0.46, and 0.44
seconds; stopped launches were 0.48, 0.41, and 0.40 seconds; all six running
launches were 0.06 seconds. Its strict materialization hypothesis was rejected
because absent and stopped distributions overlapped. The approximately
nine-second Experiment 0042 observation did not reproduce.

Local revision `20a7a4a1` adds an exact opt-in timing seam to the same launch
transaction. `FORTLET_STARTUP_TIMINGS=1` emits only fixed phase names and
integer delta and total milliseconds. Normal startup remains silent. The
complete session unit group and strict all-target/all-feature Clippy pass.

## Hypothesis and Production Mechanism

MicroSandbox reconciliation dominates the ordinary absent and stopped launch
cost. Fortlet's resolution, fresh-token credential read, capsule-state
projection, and warm environment-layer verification are local file operations,
while the runtime phase creates or boots the VM and waits for its relay.
Therefore the `runtime` phase should account for at least 60 percent of every
absent and stopped total. The running runtime and completed command phases
should remain small enough for the total to stay below one second.

## Declared Scope

Repeat Experiment 0043's three fixed lifecycle cycles against AGD, substituting
the instrumented development binary's explicit `fortlet run codex --
--version` for the packaged shim. Capture its six timing events and independent
`/usr/bin/time -p` total for every launch. The binary, project, host, image,
credentials, warm layers, command, terminal mode, and ordering stay frozen.

The exact sequence per cycle is absent launch, running launch, public stop,
stopped launch, second running launch, public stop, public reset, absent status,
and empty inventory. Lifecycle management uses the already-built packaged
Fortlet. No model request or AGD file edit is permitted.

## Alternatives

Unrestricted SDK tracing was rejected because it can emit paths,
configuration, and other fields unrelated to this measurement. A custom SDK
subscriber and direct `msb` control were deferred until Fortlet's coarse phases
show they are necessary. Sampling profilers are disproportionate for a
sub-second path with known serial boundaries.

## Risks

The development binary might resolve a different MicroSandbox artifact or fail
the packaged runtime boundary; that settles the first unit and terminates the
experiment without retry. Millisecond truncation can make very small phases
appear as zero but cannot conceal a multi-second phase. Host load, token
renewal, a firewall prompt, ownership mismatch, or cleanup failure invalidates
the affected unit and is retained.

## Acceptance Criteria

1. The instrumented binary reports MicroSandbox 0.6.8-compatible lifecycle
   state before dispatch, and all twelve launches return Codex 0.147.0 with
   status zero.
2. Each launch emits exactly one ordered event for `resolve`, `credentials`,
   `capsule-state`, `environment`, `runtime`, and `command`; totals are
   monotonic and contain no path, environment value, credential, or command
   output.
3. Accept the hypothesis only if `runtime` is at least 60 percent of the final
   total for every absent and stopped launch. Otherwise reject it and localize
   the actual dominant phase before treatment.
4. Every running total remains below one second. Any running sample above one
   second remains a daily-use blocker.
5. Every cycle ends absent with `no capsules` and AGD retains exactly its three
   pre-existing environment-file modifications.
6. Two failures sharing an unresolved assumption, a credential or ownership
   anomaly, or inability to restore absence terminates the experiment.

## Budget and Plan

Budget: 30 minutes, three absent capsules, twelve credential-free version
commands, zero model prompts, zero external money, and one Fortlet-owned
capsule at a time. Use the fixed ordering from Experiment 0043 and do not add
samples or treatment after seeing results.

## Rehearsal

On 2026-08-20, exact-output unit tests proved the opt-in gate and bounded event
format, the full session unit group passed, strict Clippy passed, and
Experiment 0043 proved the complete lifecycle and cleanup sequence with the
same runtime state. No instrumented VM launch occurred during rehearsal.

## Results

The first absent, running, and stopped commands returned Codex 0.147.0 with
status zero and independent real durations of 0.83, 0.06, and 0.46 seconds,
respectively. None emitted a startup-timing event. The sequence was terminated
before its second running command or later cycles.

Inspection of the dispatch assumption showed that `cargo test` had compiled
the test harness containing the new code, while the selected
`target/debug/fortlet` executable remained an older successfully built product
binary. The preflight checked runtime compatibility and absent state but did
not prove the opt-in event from the exact executable that would dispatch.

Public stop and reset restored `absent`, and global inventory reported
`no capsules`. No model request, credential output, AGD edit, ownership
anomaly, or cleanup failure occurred.

## Terminal Closure

1. Outcome: terminal-failure; the selected executable was stale and produced
   no declared phase evidence.
2. Root cause: the rehearsal proved the test binary but omitted an exact
   product-binary opt-in probe; `cargo test` does not relink
   `target/debug/fortlet`.
3. Actual cost: 6 minutes, one absent capsule, three version commands, zero
   model prompts, and zero external money versus the 30-minute ceiling.
4. Next action: a successor must explicitly `cargo build --bin fortlet`, prove
   the exact built executable emits the expected bounded events in a
   credential-free failure fixture before VM dispatch, and only then repeat
   phase attribution.
