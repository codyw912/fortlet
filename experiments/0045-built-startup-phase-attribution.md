# Experiment 0045: Built startup phase attribution

Status: completed — accepted
Design: FIP-0001, FIP-0002

## Baseline / Control

Experiment 0043 measured packaged lifecycle totals: absent median 0.46 seconds,
stopped median 0.41 seconds, and running median 0.06 seconds. Experiment 0044
failed terminally because `cargo test` had built the test harness but not the
selected `target/debug/fortlet` product executable; its three commands emitted
no timing events and were excluded from phase attribution.

The successor exact executable was built explicitly from local revision
`20a7a4a1` with `nix develop -c cargo build --bin fortlet`. A credential-free
failure probe then invoked that exact path with `FORTLET_STARTUP_TIMINGS=1` and
observed the bounded event
`phase=resolve delta_ms=2 total_ms=2` before the expected missing-auth failure.
The isolated rehearsal directory was removed. No VM or harness launched.

## Hypothesis and Production Mechanism

MicroSandbox reconciliation dominates the ordinary absent and stopped launch
cost. Fortlet's preceding phases are local resolution, fresh-token credential
read, capsule-state projection, and warm layer verification. Therefore the
`runtime` phase should account for at least 60 percent of every absent and
stopped total. Running reconciliation and command completion should keep the
complete path below one second.

## Declared Scope

Run three fixed lifecycle cycles against AGD with the exact built development
binary and `FORTLET_STARTUP_TIMINGS=1`. Each cycle performs one instrumented
explicit Codex version launch from absent, one while running, public stop, one
instrumented launch from stopped, a second while running, then public stop and
reset followed by absent status and empty inventory.

Capture all six timing events and independent `/usr/bin/time -p` totals. The
binary, project, host, image, credentials, warm layers, command, terminal mode,
and ordering stay frozen. Management uses the landed packaged Fortlet. No model
request, credential inspection, or AGD file edit is permitted.

## Alternatives

Reusing Experiment 0044 is prohibited because it is terminal. Unrestricted SDK
tracing remains rejected because it can emit unrelated configuration and path
fields. Direct `msb` controls and a custom filtered subscriber remain successor
options only if Fortlet's coarse phases attribute cost to runtime but cannot
separate guest readiness from host preparation.

## Risks

The development binary could fail only during VM creation despite passing the
credential-free probe. A firewall prompt, token renewal, host load, ownership
mismatch, or cleanup failure can contaminate a unit. Millisecond truncation can
hide sub-millisecond phases but not a material delay. Any missing or extra
timing event, failed version command, or lifecycle mismatch settles its unit;
two failures with one unresolved assumption terminate the experiment.

## Acceptance Criteria

1. All twelve launches return Codex 0.147.0 with status zero.
2. Each launch emits exactly one ordered event for `resolve`, `credentials`,
   `capsule-state`, `environment`, `runtime`, and `command`; totals are
   monotonic and the timing lines contain no path, environment value,
   credential, user configuration, or command output.
3. Accept the hypothesis only if `runtime` is at least 60 percent of the final
   total for every absent and stopped launch. Otherwise reject it and identify
   the actual dominant phase before treatment.
4. Every running final total remains below one second. Any running sample above
   one second remains a daily-use blocker.
5. Every cycle ends absent with `no capsules`, and AGD retains exactly its three
   pre-existing environment-file modifications.
6. Any ownership or credential anomaly, inability to restore absence, or two
   failures sharing an unresolved assumption terminates the experiment.

## Budget and Plan

Budget: 30 minutes, three absent capsules, twelve credential-free version
commands, zero model prompts, zero external money, and one Fortlet-owned
capsule at a time. Use the fixed order above and do not add samples or treatment
after seeing results.

## Rehearsal

On 2026-08-20, `nix develop -c cargo build --bin fortlet` rebuilt the exact
product path. An isolated missing-auth launch with exact opt-in emitted one
bounded `resolve` timing event before the expected credentials-stage failure.
The full session unit group and strict Clippy had already passed. Rehearsal
created no capsule, launched no harness, and made no model request.

## Results

All twelve instrumented launches returned Codex 0.147.0 with status zero. Each
emitted exactly the six declared timing events in order. Independent
`/usr/bin/time -p` totals agreed with the final integer-millisecond event after
expected rounding.

The primary results were:

| Cycle | State | Runtime ms | Command ms | Final total ms | Real seconds |
| --- | --- | ---: | ---: | ---: | ---: |
| 1 | absent | 628 | 167 | 810 | 0.81 |
| 1 | running | 8 | 35 | 52 | 0.06 |
| 1 | stopped | 318 | 70 | 412 | 0.42 |
| 1 | running | 7 | 35 | 57 | 0.06 |
| 2 | absent | 520 | 69 | 600 | 0.60 |
| 2 | running | 8 | 36 | 55 | 0.06 |
| 2 | stopped | 311 | 84 | 410 | 0.42 |
| 2 | running | 7 | 40 | 60 | 0.06 |
| 3 | absent | 526 | 70 | 610 | 0.61 |
| 3 | running | 8 | 36 | 54 | 0.06 |
| 3 | stopped | 311 | 76 | 396 | 0.40 |
| 3 | running | 7 | 36 | 55 | 0.06 |

Absent median final time was 610 milliseconds, stopped median was 410
milliseconds, and running median was 55 milliseconds. Runtime reconciliation
accounted for 77.5, 86.7, and 86.2 percent of absent totals and 77.2, 75.9,
and 78.5 percent of stopped totals, satisfying the predeclared 60-percent rule
in every unit.

Outside the first absent command's 167-millisecond command phase, command
completion was 69–84 milliseconds after a boot and 35–40 milliseconds while
running. Resolution was always below one reported millisecond, credentials
took 3–6 milliseconds, capsule-state projection took 5–19 milliseconds, and
warm environment verification was below one reported millisecond.

The timing lines contained only their fixed phase and duration fields. All
three cycles ended with public `absent` status and `no capsules`. AGD retained
exactly its three pre-existing environment-file modifications. There was no
model request, credential output, file edit, failed unit, or unexpected prompt.

## Terminal Closure

1. Outcome: accepted; runtime reconciliation dominates absent and stopped
   launch, while running launch is suitable for daily use at a 55-millisecond
   median.
2. Root cause: ordinary restart cost is the MicroSandbox reconciliation and
   relay-ready boundary, not Fortlet resolution, credentials, warm layers, or
   repeated attachment. The earlier approximately nine-second observation did
   not reproduce across either packaged or instrumented screens and cannot be
   attributed as a persistent product behavior.
3. Actual cost: 10 minutes, three absent capsules, twelve version commands,
   zero model prompts, and zero external money versus the 30-minute ceiling.
4. Next action: retain and document the exact opt-in timing seam so a future
   outlier can carry phase evidence. Do not apply a speculative optimization;
   complete the goal gate and report the measured lifecycle distributions.
