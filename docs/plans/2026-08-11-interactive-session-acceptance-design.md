# Interactive packaged session acceptance

Status: validated with the operator on 2026-08-11.

## Outcome

Fortlet will prove the remaining daily-use uncertainty in its transparent shim
path: whether packaged Codex and Tact sessions behave correctly through a real
interactive terminal. The mission is evidence-first. Product code changes only
when the declared evidence exposes a concrete defect.

The production path remains the accepted FIP-0001 and FIP-0002 transaction:

```text
packaged shim -> Fortlet session -> managed capsule -> SDK terminal attach
```

No public command, runtime abstraction, lifecycle mechanism, or architecture
contract is added by this design.

## Test-side PTY driver

A small repository test tool launches an exact immutable package shim under a
pseudoterminal. It owns the PTY, initial dimensions, resize operation, child
process group, signal delivery, timeouts, and exit observation. It emits a
bounded structured event log rather than storing raw harness screen content,
which may contain account or project metadata.

Before it is trusted against Fortlet, the driver runs a deterministic local
fixture process. The fixture reports its terminal dimensions, reacts to
resize, traps signals, and exits predictably. This rehearses event ordering,
timeout behavior, cleanup, and status capture without MicroSandbox, provider
traffic, or a harness UI.

The driver is test tooling, not a Fortlet subcommand. It may terminate only its
own child process group and remove only temporary state it created.

## Live evidence flow

One predeclared experiment contains fixed Codex and Tact units. Each unit:

1. Starts the real packaged shim UI in an 80 by 24 PTY.
2. Observes sustained process life and terminal output without matching exact
   UI wording.
3. Resizes the PTY to 120 by 40 and observes subsequent terminal activity
   consistent with a redraw.
4. Runs a concurrent packaged `--version` invocation and verifies through
   read-only labels that both resolve to the same project-and-harness capsule.
5. Sends `SIGINT` to the foreground process group without writing prompt text
   or a newline.
6. Records bounded termination and the exact observed exit status, then proves
   the managed capsule remains responsive with a final non-interactive attach.

Both harness units run even if one fails unless a hard invariant fires. There
are no retries and no adaptive UI input.

## Failure and safety boundaries

The unit fails on timeout, missing pre- or post-resize activity, capsule-label
mismatch, unbounded signal handling, unexpected host-harness output, or failure
of the final responsiveness check. Exact screen text is not an acceptance gate.

Any indication of prompt submission, intentional model inference, paid quota,
credential output, unexpected mount scope, native fallback, unowned process
control, or mutation outside the project terminates the experiment immediately.
Automatic authentication or metadata traffic remains host-brokered and must
not expose credentials. Fortlet-managed capsules may remain under their
declared idle timeout; the experiment does not invent lifecycle cleanup
semantics.

## Verification and stopping rule

Automated evidence covers the PTY driver's deterministic fixture behavior and
any product defect repaired during the mission. Before live dispatch, the
complete standard verification set, package checks, and `fortlet doctor` must
pass. The experiment record freezes the package output, project, dimensions,
ordering, signals, commands, timeouts, and zero-prompt rule.

After the two declared units, update conformance honestly, close the experiment
terminally, rewrite the handoff, mark the goal complete or blocked, then stop.
The engineering ceiling is two hours, with zero external spend or paid quota.

## Explicit non-goals

- Lifecycle commands, lease redesign, or capsule cleanup behavior.
- Broad-root and project-resolution hardening beyond defects encountered here.
- Standalone distribution or native `x86_64-linux` verification.
- Model-task quality, submitted prompts, publication, or remote execution.
- New harnesses, another runtime, or a generic terminal/runtime abstraction.
