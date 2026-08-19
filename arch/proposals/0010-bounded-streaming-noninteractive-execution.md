# FIP-0010: Bounded streaming non-interactive execution

Status: Accepted
Recorded: 2026-08-19 from the Experiment 0024 non-interactive hang diagnosis
Requires: FIP-0001, FIP-0002, FIP-0008, FIP-0009

## Summary

Stream non-interactive harness output as it is produced instead of collecting
it until process exit. Give the Codex adapter a ten-minute inactivity ceiling
for managed non-interactive commands. Activity on stdout or stderr renews the
ceiling; expiry kills only the guest command, preserves the reusable capsule,
ends the host credential-renewal lease, and reports one actionable correction.

Tact, interactive attachment, and the native escape hatch retain their current
behavior. The first slice has no user-configurable timeout.

## Motivation

Experiment 0024 ran packaged Codex 0.147.0 through Fortlet's non-interactive
MicroSandbox attachment. It remained completely silent for ten minutes. When
the operator stopped the capsule, the waiting host command reported that its
exec session had ended without an exit event. The same package and model path
worked interactively in Experiment 0025.

The follow-up source audit and credential-free controls separated the layers:

1. Fortlet uses MicroSandbox `exec_with` whenever host stdin or stdout is not a
   terminal. That API collects stdout and stderr and returns them only after an
   exit event.
2. MicroSandbox 0.6.8 defaults collected exec stdin to `/dev/null`; Fortlet was
   not accidentally keeping Codex stdin open.
3. A network-disabled MicroSandbox running the prepared stock Codex 0.147.0
   layer forwarded exact output and exit status for a shell command. Codex
   reached an in-guest HTTP 500 fixture, printed its failure, and exited 1.
4. Against an in-guest successful Responses SSE fixture, the same stock Codex
   printed `local-fixture-ok` and exited 0.
5. Against an in-guest server that accepted the request but never answered,
   Codex remained alive. Collected execution exposed no partial output before a
   six-second MicroSandbox timeout. Streaming execution exposed Codex startup
   and request progress immediately, then the same timeout killed the command.
6. Codex 0.147.0 gives model streams a five-minute idle timeout and up to five
   stream retries by default. Those upstream retries can legitimately outlive
   Experiment 0024's ten-minute observation window.
7. Fortlet's credential-renewal task is selected alongside runtime attachment.
   It is aborted after attachment completes; it cannot prevent an attachment
   future from completing.

The observed failure is therefore composed behavior, not a missing normal exit
event: Codex can remain alive in an upstream request while Fortlet withholds
all progress until exit. A bounded streaming boundary makes that state visible
and prevents a silent upstream request from owning a Fortlet invocation
indefinitely.

Pinned sources:

- Codex tag `rust-v0.147.0`, peeled commit
  `be6e8eac029b183056b7e4402879f15d2c85f61b`, especially
  `codex-rs/exec/src/lib.rs` and `codex-rs/model-provider-info/src/lib.rs`.
- MicroSandbox crates `microsandbox`, `microsandbox-protocol`, and
  `microsandbox-agent-client` version 0.6.8, especially
  `lib/sandbox/exec.rs` and `lib/sandbox/mod.rs`.

## Decision

Replace Fortlet's collected non-interactive attachment with MicroSandbox's
streaming exec surface. Generic runtime code forwards stdout and stderr events
as they arrive and returns only the explicit guest exit status.

Extend the harness adapter contract with an optional non-interactive inactivity
ceiling. Codex 0.147.0 selects ten minutes. Tact selects no Fortlet-owned
ceiling. The deadline starts when the exec handle is acquired and renews after
every stdout or stderr event. It does not limit a command that continues to
produce observable output.

On expiry, Fortlet kills the timed-out guest process group, drains the exec
session for a short bounded cleanup, and returns a terminal-stage error whose
single correction is to retry interactively when more than ten silent minutes
are expected. It does not stop or reset the capsule and does not retry Codex.

## Specification

### Mode selection and adapter ownership

1. The existing host terminal test MUST continue to select interactive versus
   non-interactive attachment for explicit and shim launches.
2. Generic runtime code MUST ask the harness adapter for its optional
   non-interactive inactivity ceiling and MUST NOT branch on the harness name.
3. The Codex 0.147.0 adapter MUST select a ten-minute ceiling for every managed
   non-interactive invocation.
4. The Tact adapter MUST select no Fortlet-owned inactivity ceiling.
5. Interactive attachment and `fortlet native` MUST remain unchanged.

### Streaming and completion

1. Non-interactive execution MUST use a non-PTY MicroSandbox streaming exec
   session with null stdin and the existing effective arguments and working
   directory.
2. Every stdout event MUST be written to host stdout, and every stderr event
   MUST be written to host stderr, without content rewriting.
3. Forwarded output SHOULD be flushed promptly so progress is visible while
   the command remains alive.
4. An explicit guest exit event MUST return its exact exit code after all
   preceding output events have been forwarded.
5. A spawn failure or a stream that closes without an exit event MUST remain a
   terminal-stage failure. Fortlet MUST NOT invent a successful exit status.
6. Requested arguments and adapter-owned arguments MUST retain the ordering
   required by FIP-0009.

### Inactivity and cleanup

1. For a harness with a ceiling, the inactivity deadline MUST begin when the
   streaming exec handle is acquired and MUST renew after each stdout or stderr
   event.
2. A `Started` event MAY renew the initial deadline but MUST NOT disable it.
3. When the deadline expires, Fortlet MUST send MicroSandbox's kill operation
   to the exec session and MUST wait only a short bounded interval for its
   terminal event.
4. Timeout MUST fail at the terminal stage and give exactly one correction:
   retry interactively when more than ten silent minutes are expected.
5. Timeout MUST NOT retry the command, fall back to native execution, stop the
   capsule, reset persistent state, or alter credentials.
6. Normal exit, spawn failure, stream loss, timeout, and host-side attachment
   error MUST all end the invocation's FIP-0008 credential-renewal task within
   its existing bounded cleanup.

### Evidence

1. Credential-free tests MUST prove incremental stdout and stderr forwarding,
   exact zero and non-zero exit status, missing-exit failure, inactivity reset,
   timeout kill, and bounded timeout cleanup.
2. Credential-free session tests MUST prove normal completion and timeout both
   cancel the renewal task rather than waiting for its next refresh.
3. Existing interactive, Tact, argument-transform, credential projection,
   Apps-disablement, and lifecycle tests MUST remain green.
4. A packaged local fixture MUST reproduce successful non-interactive output
   and exit without provider credentials before any model-backed experiment.
5. One terminal successor experiment MAY then prove a short ordinary Codex
   prompt, clean exit, no Apps warning, and public stop/reset cleanup.

## Consequences

Non-interactive users see Codex configuration, warnings, and progress before
the process exits. A silent stalled Codex command no longer owns a Fortlet
invocation beyond ten minutes, while active long-running commands can continue.

The fixed ceiling is intentionally conservative and simple for the first
reliable daily-use slice. A legitimate model request that produces no terminal
output for ten minutes will be killed and must be retried interactively. The
capsule and its persistent state survive that timeout.

Streaming preserves each output stream's bytes and event order, but it does not
claim a total ordering between independently delivered stdout and stderr.

## Alternatives Considered

1. Keep collected output and add only MicroSandbox's total timeout. Rejected
   because it repeats Experiment 0024's misleading silence and discards useful
   partial output on timeout.
2. Stream without a Fortlet ceiling. Rejected because it improves visibility
   but does not satisfy the accepted GOAL's bounded-failure requirement.
3. Override Codex provider retry and stream-timeout configuration. Rejected
   because it would couple Fortlet to provider-specific config merging and
   would not bound hangs outside the model stream.
4. Add a public timeout flag or environment variable now. Rejected because
   shim parity would require a broader configuration design before the basic
   reliable path is proven.
5. Apply the ceiling to Tact. Rejected because no Tact failure motivates that
   behavior and this proposal should change only the diagnosed boundary.
6. Use a total-duration ceiling. Rejected because actively progressing coding
   work should not be killed solely for being long-running.
7. Fall back to interactive or native Codex automatically. Rejected because it
   changes terminal semantics and violates the explicit isolation boundary.

## Open Questions

1. Whether daily use needs a user-configurable inactivity ceiling shared by
   explicit and shim launches.
2. Whether a later Codex release makes its upstream request retry behavior
   sufficiently visible and bounded to remove Fortlet's adapter ceiling.
3. Whether Tact or future harnesses should opt into the same ceiling after
   independent evidence.

## Amendment — 2026-08-19 explicit EOF for MicroSandbox 0.6.8

Experiment 0028 showed that the pinned SDK's streaming `StdinMode::Null` does
not send the empty `ExecStdin` frame that closes guest stdin. Stock Codex
therefore blocked in `read_to_end(stdin)` before making its model request.
Experiment 0029 proved the correction against an immutable package and an
in-guest loopback-only provider.

For MicroSandbox 0.6.8, Specification “Streaming and completion” item 1's null
stdin requirement is implemented as an explicit stdin pipe closed immediately
after the streaming handle is acquired. No input bytes are sent. This amendment
changes only that pinned-SDK mechanism; all mode, streaming, inactivity,
cleanup, credential, and user-facing semantics remain unchanged.
