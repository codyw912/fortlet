# Experiment 0003: Interactive packaged sessions

Status: declared
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Accepted Experiment 0002 proved that immutable packaged `codex --version` and
`tact --version` shims enter Fortlet, create or attach managed capsules, and
return pinned Linux guest versions without host fallback. It did not exercise
the interactive SDK attachment path.

The treatment revision is Jujutsu checkpoint `ad7fdd0e` (`add bounded
interactive PTY observer`). Its exact package output is
`/nix/store/6aldv1saddj7fmsxdgj0nc66s7y2f66s-fortlet-0.1.0`. The observer is
repository test tooling and is not installed in that package.

## Hypothesis and Production Mechanism

With the immutable shim directory active, each real harness UI enters the same
Fortlet session transaction as its proven non-interactive command. The SDK
terminal attachment should retain PTY activity across a host resize, while a
concurrent invocation should reconcile to the same project-and-harness capsule.
Sending `SIGINT` to the observer-owned foreground process group should terminate
the attached host session within its bound without making the capsule
unresponsive.

## Declared Scope

Run Codex, then Tact, from `/Users/cody/dev/fortlet`. Each interactive unit uses
its exact shim path under the immutable package above and receives no arguments,
prompt text, newline, or adaptive input.

The observer protocol is fixed:

1. Initial PTY: 80 columns by 24 rows.
2. Startup activity timeout: 90 seconds.
3. Startup output must settle for 100 milliseconds before resize.
4. Resized PTY: 120 columns by 40 rows.
5. Post-resize activity timeout: 15 seconds.
6. Concurrent-attachment window: 20 seconds.
7. Signal: one `SIGINT` to the verified observer-owned process group.
8. Exit timeout: 15 seconds.

During each concurrent window, query the running capsule filtered by
`fortlet.managed=true` and the matching `fortlet.tool` label, run that same
packaged shim with only `--version`, then repeat the label query. The before and
after names must match. After the observer exits, run one final packaged
`--version` invocation to prove responsiveness.

The package, observer checkpoint, project, credentials, MicroSandbox 0.6.8,
harness versions, dimensions, timings, ordering, commands, labels, and signal
remain frozen. Local Fortlet-managed capsule state may be created or reused.
No model task, publication, remote execution, native escape, shell setup
mutation, or user-owned cleanup is in scope.

## Alternatives

1. Rely on the MicroSandbox SDK's inherited terminal implementation. Rejected
   because the remaining conformance gap is packaged-shim evidence, not an SDK
   claim.
2. Submit a harmless model prompt. Rejected because terminal transport can be
   tested without paid quota or intentional inference.
3. Persist raw PTY output. Rejected because screen content can contain account
   or project metadata; bounded structural events are sufficient.

## Risks

1. A harness may treat one `SIGINT` as UI cancellation rather than process
   termination. The unit then fails its fixed exit bound without retry.
2. A UI may not redraw observably after resize even if the guest received it.
   Missing post-resize bytes rejects the unit; exact UI wording is never used.
3. Automatic authentication or metadata traffic may occur on UI startup. It
   must remain host-brokered, expose no credential values, and use no paid
   quota.
4. The 20-second window may expire before all concurrent evidence completes.
   That settles the unit; timings are not adapted.
5. Allowed Fortlet-managed capsules may remain under their four-hour idle
   timeout. No lifecycle cleanup is attempted.

## Acceptance Criteria

1. The observer's six focused tests and deterministic fixture rehearsal pass.
2. The complete standard verification set, exact package checks, and
   `nix run . -- doctor` pass before dispatch.
3. Each observer emits, in order, `started`, initial activity, `resized`,
   resized activity, concurrent window, `signal`, `exited`, and a numeric
   summary.
4. Each summary reports positive initial and resized byte counts and records
   termination by signal 2 within the 15-second exit bound.
5. Concurrent `--version` exits zero with pinned guest version `0.147.0` for
   Codex and `0.3.7` for Tact. Matching before/after label queries return one
   unchanged managed capsule name.
6. Each final `--version` invocation exits zero with the same pinned version.
7. No raw UI output is persisted, no prompt or newline is submitted, and no
   host fallback, credential value, unexpected mount, unowned process control,
   paid quota, or shell mutation occurs.
8. Both units run even if one fails unless a hard invariant fires. Both must
   pass to accept the treatment.
9. The experiment terminates after both units, a hard-invariant anomaly, or the
   30-minute dispatch budget.

## Budget and Plan

Budget: zero money, zero paid quota, zero submitted prompts, zero remote
mutation, two harness units, zero retries, and at most 30 minutes after dispatch
begins. Run and record Codex completely, then Tact completely. Observer-created
processes are bounded and owned; all label queries are read-only.

## Rehearsal

On 2026-08-11, before declaration:

1. `cargo test --example pty_observer` passed six tests covering command parsing,
   initial PTY activity, real resize activity, structured event order, buffered
   startup exclusion, signal status, timeout, and owned-process cleanup.
2. `cargo run --quiet --example pty_observer -- fixture` emitted the complete
   event sequence and summary with positive byte counts and signal 2.
3. `cargo test`, formatting, strict all-target Clippy, the explicit conformance
   test, and exact-tree `nix flake check` passed.
4. The package build passed release tests, empty-environment shim checks, and
   byte-for-byte MicroSandbox runtime integrity checks.
5. `nix run . -- doctor` reported a ready SDK 0.6.8 host, valid credential
   metadata, the Jujutsu project, an outside-mount credential file, and both
   harnesses without printing credential values.

## Results

Pending.

## Terminal Closure

Pending.
