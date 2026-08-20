# GOAL: Make capsule launch latency suitable for daily use

Status: active — locally complete; publication and hosted gate pending

## Outcome

Measure the managed launch critical path on Apple silicon, identify the phase
responsible for the approximately nine-second absent-capsule observation, and
remove any material Fortlet-owned delay that can be corrected without changing
the accepted capsule architecture. Do not add another harness or product
surface until running-capsule launch is demonstrably suitable for ordinary
daily use.

This is a performance and observability refinement of accepted FIP-0001 and
FIP-0002, published under FIP-0005. Measurement and non-architectural
corrections need no new FIP. A change to capsule persistence, isolation,
credential delegation, or the selected runtime requires a proposal before
implementation.

## Deliverables

1. Verify merged `main`, clean Jujutsu state, conformance, and the complete
   `docs/RUNBOOK.md` gate through `nix develop` before measurement or code.
2. Predeclare and run a credential-safe lifecycle screen using the exact
   packaged Fortlet and cached inputs. Measure completed `codex --version`
   launches from absent, stopped, and already-running capsule states on the
   same project and host. Use no model prompts and clean up through public
   Fortlet lifecycle commands.
3. Attribute the dominant delay before changing behavior. Distinguish Fortlet
   preparation, MicroSandbox creation or restart, guest readiness, terminal or
   streaming attachment, and harness startup. Treat MicroSandbox's guest-boot
   metric separately from Fortlet's end-to-end command latency.
4. If the lifecycle screen is insufficient, add the smallest opt-in,
   secret-safe timing seam needed to expose phase durations. Successful normal
   startup remains silent, and timing output must not contain paths,
   credentials, environment values, command output, or user configuration.
5. Implement only a measured, FIP-conformant correction. Preserve project and
   harness capsule identity, fail-closed behavior, credential boundaries,
   disposable roots, persistent project and harness state, and explicit native
   escape. If the correction requires an architecture change or an upstream
   MicroSandbox fix, stop with exact evidence and the smallest successor
   proposal or upstream report.
6. Re-run the same lifecycle screen after treatment. Record distributions and
   lifecycle state rather than one aggregate shell duration. A running-capsule
   managed `--version` median above one second remains a blocking daily-use
   failure; do not rationalize it with cold-start measurements.
7. Run the complete local and hosted gates, publish one goal-scoped draft PR,
   and leave merge authority with the operator.

## Definition of Done

1. The approximately nine-second observation is decomposed into named phases
   with commands and exact runtime, package, host, image, and lifecycle state.
2. Running-capsule launch has at least five valid samples and a median no
   greater than one second on the operator's Apple-silicon host.
3. Absent and stopped launch distributions are recorded separately, and their
   dominant costs are identified. No provider marketing boundary is presented
   as Fortlet end-to-end latency.
4. Every implemented optimization has before-and-after evidence from the same
   declared experiment mechanism and preserves the accepted security and
   lifecycle contracts.
5. Every experiment is terminally closed, conformance is accurate, and the
   complete `docs/RUNBOOK.md` verification set passes.

## Excluded scope

No new harness, standalone installer, global shim activation, model-backed
task, credential redesign, remote runtime, telemetry, provider replacement,
release, repository setting, or merge is in scope. Do not optimize first-use
Nix evaluation or immutable tool-layer provisioning; Experiment 0042's
nine-second observation occurred after those layers were warm.

## Budget and authority

The engineering ceiling is two hours. Experiments may create one verified
Fortlet-owned capsule at a time, perform at most twelve credential-free
`--version` launches per declared experiment, and use zero model prompts, zero
external money, and zero paid quota. Public stop and reset must restore the
starting capsule state after each destructive measurement sequence.

Acceptance authorizes one `capsule-launch-latency` bookmark and one pull
request targeting `main` under FIP-0005. The operator remains the sole merge
authority.

## Verification

Run the complete standard verification set from `docs/RUNBOOK.md` through
`nix develop` before publication readiness.

The final aarch64-darwin local gate passes: 91 unit tests and every enabled
integration test, formatting, strict all-target/all-feature Clippy,
conformance, the curated package, and the shim-activation check. The two
stock-Codex compatibility tests remain explicitly ignored because they require
an external binary. Nix reports only the expected incompatible x86_64-linux
omission warning.

Experiments 0043 and 0045 establish running medians of 0.06 seconds through the
landed package and 55 milliseconds under phase instrumentation. Instrumented
absent and stopped medians are 610 and 410 milliseconds. Runtime
reconciliation accounts for 76–87 percent of every absent and stopped total;
the earlier approximately nine-second observation did not reproduce. No
speculative performance treatment is justified.

Then STOP. New harnesses and other product work belong to a successor GOAL.
