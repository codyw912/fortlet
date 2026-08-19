# Experiment 0029: Packaged Codex explicit EOF regression

Status: declared — 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0008, FIP-0009, and FIP-0010
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0028 is terminally rejected and must not be retried. Its sole
packaged Codex process streamed `Reading additional input from stdin...`, then
remained there until Fortlet's 600-second inactivity bound. Pinned-source
inspection attributed that exact state to MicroSandbox 0.6.8 streaming
`StdinMode::Null`: the SDK does not transmit its protocol's empty `ExecStdin`
EOF frame on that path.

Treatment revision `673f3206b51c9ce08a898d1852d3f12142f8b04e`, including
checkpoint `71d1cfc0`, instead requests a streaming stdin pipe and explicitly
closes its `ExecSink` before processing events. Its immutable aarch64-darwin
package is `/nix/store/m6lrd4a8slw5r627k5xkday9l3nkdj7d-fortlet-0.1.0`.
A focused test was red before the call and green afterward; all non-interactive
deadline/streaming tests, strict Clippy, and formatting pass.

## Hypothesis and Production Mechanism

An explicitly closed MicroSandbox stdin pipe will let stock Codex 0.147.0's
`read_to_end(stdin)` observe EOF. The process will advance beyond its initial
stdin diagnostic, attempt only a deliberately unreachable in-guest loopback
model endpoint, emit the resulting local transport failure, and exit nonzero.
Fortlet will stream those events and return the exact guest status rather than
waiting for its inactivity ceiling.

## Declared Scope

1. Build one immutable package from the frozen treatment and record its store
   path and full revision.
2. Create one fixed temporary synthetic ChatGPT auth document outside every
   guest mount. Its far-future unsigned test JWT and dummy refresh token are
   not provider credentials and must not be refreshed.
3. Require public Codex absence and an empty MicroSandbox inventory, then run
   one packaged `codex exec` with one inert prompt and CLI configuration that
   selects a custom Responses provider at `http://127.0.0.1:9/v1`, requires
   external auth, and sets request and stream retries to zero.
4. Record complete stdout/stderr and exact host status. Use only packaged
   public status, stop, and reset for cleanup; require final absence and empty
   inventory, then delete the synthetic document.

No provider address, real credential read, DNS dependency, second process,
retry, model response, interactive/native fallback, tool call, guest
repository edit, private capsule mutation, remote mutation, release, merge, or
package publication is in scope.

## Alternatives

1. Send another real model prompt. Rejected because the GOAL authorizes exactly
   one and Experiment 0028 terminally consumed it.
2. Repeat Tact `--version`. Rejected because Tact does not read stdin and could
   not detect the defect exposed by Experiment 0028.
3. Use SDK null stdin again. Rejected because pinned source and the live result
   show that it does not close streaming guest stdin.

## Risks

1. CLI configuration syntax could fail before Codex reads stdin. This still
   proves completion but rejects the transport-stage criterion; do not retry.
2. Loopback port 9 could unexpectedly be occupied. Any successful connection
   or model-like response is an invariant anomaly; clean up and stop.
3. Synthetic-auth validation or an unexpected refresh could fail before
   attachment. Record the first result without weakening the credential
   boundary or retrying.

## Acceptance Criteria

1. Revision/package are exact, working copy is empty, Codex starts absent, and
   MicroSandbox inventory is empty.
2. The sole command emits the stdin diagnostic and then additional Codex
   configuration or local-transport output, proving EOF advanced the process.
3. It reaches only `127.0.0.1:9`, exits nonzero before the 600-second Fortlet
   ceiling, and Fortlet returns that exact nonzero status without an inactivity
   error.
4. Public status shows the capsule running; public stop/reset restore absence;
   final inventory is empty and the synthetic auth file is deleted.
5. The first failure or invariant anomaly settles the unit. There is no retry.

## Budget and Plan

One credential-free local Codex process, one owned capsule, zero provider
requests, zero paid quota, zero retries, and at most the accepted 600-second
inactivity ceiling. Package build and read-only baseline checks do not consume
the treatment unit.

## Rehearsal

The focused explicit-EOF test failed against the first implementation and
passed after the correction. Six non-interactive forwarding/exit/timeout tests,
the output-renewal test, strict Clippy, and formatting then passed through
`nix develop`. Experiment 0026 already proved stock Codex's completed local
failure path emits an explicit MicroSandbox exit event.

## Results

Pending.

## Terminal Closure

Pending.
