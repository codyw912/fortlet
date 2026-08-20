# Experiment 0045: Built startup phase attribution

Status: declared
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

Pending.

## Terminal Closure

Pending.
