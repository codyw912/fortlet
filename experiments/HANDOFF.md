# Session Handoff — Fail-closed pre-runtime launch errors

Audience: a fresh agent session. `GOAL.md` is normative and active. The current
mission is deterministic, repository-local, and governed by FIP-0001. It must
not launch a real harness or MicroSandbox capsule.

## Verified baseline

1. Project resolution and broad-root safety closed at checkpoints `e764dfc1`
   and `77e8fd9c`. Fifteen project tests prove the selected-root contract.
2. The full standard gate passed at closure. The successor working copy was
   clean, and no experiment was active.
3. Experiments 0001 through 0009 remain terminally closed. Codex-specific exit
   automation must not be resumed.
4. `session::launch` wraps project, harness, credentials, capsule, environment,
   and terminal operations with a stage name and one correction. Only the
   generic `stage` formatter currently has direct test coverage.
5. Several real pre-runtime failures can be forced entirely through temporary
   paths: invalid harness/project/auth, invalid local capsule state or guest
   auth projection, and incomplete environment layers.

## Mission guidance

Read FIP-0001 and
`docs/plans/2026-08-12-pre-runtime-failure-evidence-design.md` in full before
implementation. Invoke the compiled binary from Unix integration tests. Clear
each subprocess environment and supply isolated paths plus fake credentials.

Advance each case only to its intended deterministic failure. Assert the outer
stage and correction, a stable cause fragment, nonzero exit, and absence of
later artifacts. Environment cases must reject preseeded incomplete layers
before any provisioning capsule or network operation can begin.

## What not to do

1. Do not add an injected runtime, orchestration trait, test-only product flag,
   generic error framework, or live fault campaign.
2. Do not use real credentials or inherit the operator's home, XDG paths,
   runner markers, or shell environment.
3. Do not launch MicroSandbox, Codex, Tact, provisioning, or a network request.
4. Do not claim live capsule reconciliation or terminal attachment failure
   evidence; those remain explicit conformance gaps.

## Completion boundary

Keep code, tests, runbook, and conformance changes in one reviewable checkpoint.
Run the complete verification set, rewrite this handoff for the successor, mark
`GOAL.md` complete, inspect `main..@`, then stop. The ceiling is two engineering
hours with no external budget or experiment.
