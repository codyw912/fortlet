# Session Handoff — Interactive acceptance mission authorized

Audience: a fresh agent session. `GOAL.md` is normative and active. Verify the
claimed baseline before relying on it, then implement only the bounded
interactive acceptance mission.

## Verified predecessor state

1. FIP-0002 is Accepted and defines optional package-owned `codex` and `tact`
   shims plus the explicit recursion-safe `fortlet native` escape hatch.
2. Checkpoint `29d42a82` implements transparent dispatch, native execution,
   staged failures, package shims, and focused tests.
3. Checkpoint `85d4766b` preserves the fixed-output MicroSandbox runtime and
   checks `msb` and libkrunfw byte-for-byte against the release archive.
4. Accepted Experiment 0002 proved packaged `codex --version` and
   `tact --version` return pinned Linux guest versions without native fallback
   or credential output.
5. The predecessor mission's complete verification set passed on
   `aarch64-darwin`; native `x86_64-linux` verification remains outstanding.

## Why this mission exists

The package, shims, VM creation, provisioning, guest tools, and non-interactive
exit path are proven. Interactive TTY, resize, signal, and concurrent-attachment
behavior is still inherited from the explicit session path rather than
independently exercised through packaged shims. That is the nearest remaining
daily-use uncertainty.

The operator validated
`docs/plans/2026-08-11-interactive-session-acceptance-design.md` and authorized
one prompt-free real UI launch for each harness. The engineering ceiling is two
hours.

## Intended evidence path

1. Build a test-side PTY observer and rehearse it end-to-end with a deterministic
   fixture before trusting it against Fortlet.
2. Emit structured events rather than repository-persisted raw UI content.
3. Run all standard gates and doctor before declaring live dispatch.
4. Declare a new experiment; never resume terminal Experiment 0001 or accepted
   Experiment 0002.
5. Exercise Codex and Tact with fixed initial and resized dimensions, a
   concurrent `--version` attach, managed-label evidence, `SIGINT`, bounded exit,
   and a final responsiveness check.

## Safety and stopping rules

Do not write prompt text or a newline to either real UI or intentionally
initiate model inference. Automatic authentication or metadata traffic remains
host-brokered and must use no paid quota. Do not touch shell startup files,
invoke native fallback, inspect or record credential values, terminate unowned
processes, or clean up user-owned MicroSandbox state. Both declared harness
units run unless a hard invariant fires; there are no retries.

If evidence exposes a local defect inside accepted behavior, repair it with
focused tests. If the repair would change an accepted FIP or broaden into
lifecycle, distribution, Linux, remote, publication, or a generic abstraction,
stop. Close the experiment, update conformance and this handoff, inspect the
Jujutsu stack, then stop at the goal boundary.
