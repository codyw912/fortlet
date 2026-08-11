# Experiment 0007: Exact Codex exit parity

Status: completed — rejected
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0003 proved that packaged Codex 0.147.0 reaches initial activity,
redraws after a real PTY resize, shares its managed capsule during concurrent
attachment, and remains responsive. It did not capture a terminal status
because the UI stayed alive after the experiment's first foreground-group
`SIGINT`.

Experiment 0006 then proved that the same first-interrupt non-exit occurs in
native Codex 0.147.0 outside the active Codex runner. That result removes the
basis for a Fortlet-specific signal defect but leaves exact eventual
termination and packaged status preservation unresolved.

The exact identities for this experiment are:

1. Observer checkpoint `147e956d`, built binary
   `/Users/cody/dev/fortlet/target/debug/examples/pty_observer`, SHA-256
   `57a7727f1c4ce162759a0e4254c44c8cd4994bae14951f63c15e00f2a7044081`.
2. Native npm launcher
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js`,
   SHA-256 `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477`.
3. Its selected native binary at
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex`,
   SHA-256 `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37`.
4. Immutable package output
   `/nix/store/yp879lslc04v48l31hpfnfqmlz6wd2ik-fortlet-0.1.0`, whose
   `libexec/fortlet/shims/codex` is a relative symlink to its packaged
   `bin/fortlet`.

Codex's official CLI reference identifies `/exit` followed by Enter as an
immediate local exit action and states that `/quit` is equivalent. This
experiment uses `/exit` only.

## Hypothesis and Production Mechanism

Native Codex will interpret the exact PTY bytes `/exit\r` as its local exit
command and terminate with an exact status. Fortlet's packaged terminal attach
will carry the same bytes to guest Codex, and its session transaction will
return the guest's identical code or signal to the observer.

Matching termination establishes the FIP-0001/FIP-0002 exit-status behavior
for this Codex action. A mismatch isolates a packaged-path discrepancy without
distinguishing Fortlet orchestration from the MicroSandbox attachment layer and
without authorizing a repair.

## Declared Scope

The operator runs two units from `/Users/cody/dev/fortlet` in this fixed order
from the same external Fish shell:

```fish
target/debug/examples/pty_observer observe-exit \
    /Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js \
    /Users/cody/dev/fortlet \
    20

target/debug/examples/pty_observer observe-exit \
    /nix/store/yp879lslc04v48l31hpfnfqmlz6wd2ik-fortlet-0.1.0/libexec/fortlet/shims/codex \
    /Users/cody/dev/fortlet \
    20
```

Before the first unit, a name-only check must report `CODEX_THREAD_ID`,
`CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` absent. No
environment values are read or recorded.

Each unit uses an 80-column by 24-row PTY, 90-second startup-activity bound,
100-millisecond startup settle, resize to 120 by 40, 15-second resize-activity
bound, 20-second unattended hold, exact `/exit\r` observer input, and
15-second exit bound. The operator supplies no input after starting a command
and returns the complete structural output verbatim.

The observer, identities, project, external shell, ordering, dimensions,
timings, action, and zero-retry rule remain frozen. The two local exit commands
are not model prompts. No intentional inference, paid quota, raw PTY capture,
Fortlet product change, harness configuration change, shell setup mutation,
remote execution, publication, or cleanup outside observer-owned processes is
in scope. Ordinary Codex-managed state and Fortlet-managed capsule writes are
allowed for the two launches.

## Alternatives

1. Send a second `SIGINT`. Rejected because the prior experiments established
   first-interrupt semantics, not the UI's explicit termination action.
2. Send terminal EOF. Rejected because it is not the documented Codex action
   selected by the validated design.
3. Have the operator type `/exit` manually. Rejected because observer-owned
   input is exact, repeatable, and produces a structural action boundary.
4. Match prompt or screen text before writing. Rejected because raw UI content
   may expose account or project metadata and is unnecessary for this bounded
   comparison.

## Risks

1. Native or guest Codex may not be ready to accept the slash command despite
   producing settled activity and a resize redraw. A timeout settles that unit;
   the input timing is not adapted and there is no retry.
2. A UI may close before `exit_command`. That unit supplies no exit-action
   evidence and still consumes its run.
3. Native startup may perform ordinary metadata or state activity, and the
   packaged unit may reuse its managed capsule. Credential values must remain
   host-brokered and unobserved; no intentional inference or paid quota is used.
4. The packaged status may differ even if both UIs close. That is a reportable
   discrepancy, not permission to change Fortlet under this goal.
5. Any observer timeout invokes bounded cleanup of only its verified child
   process group. Fortlet-managed capsule state may remain under its existing
   idle policy.

## Acceptance Criteria

1. Nine focused observer tests pass, including exact command bytes, code 23
   preservation, event order, timeout, and owned cleanup. Both deterministic
   fixture modes emit their complete expected summaries.
2. The complete runbook gate, exact package build, native hashes, observer hash,
   marker-name check, and `nix run . -- doctor` pass before dispatch.
3. Each unit emits `started`, `activity_initial`, `resized`,
   `activity_resized`, `concurrent_window`, and `exit_command` in order.
4. A unit that exits also emits `exited` and a numeric summary within 15 seconds.
   A timeout or pre-action close is recorded verbatim.
5. Matching native and packaged `exit_code` and `exit_signal` values accept
   packaged Codex exit parity for the `/exit` action.
6. Any exact-status or termination mismatch rejects parity and records a
   packaged-path discrepancy without authorizing repair.
7. If native Codex does not reach or respond to `exit_command`, exact native
   termination remains unestablished. The packaged unit still runs unless a
   hard invariant fires, and neither unit is retried.
8. No raw UI output, environment value, model prompt, credential value, paid
   quota, shell mutation, unexpected external mutation, or unowned process
   control occurs.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, two units, zero
retries, and at most 15 minutes after dispatch begins. Run and record native
Codex completely, then packaged Codex completely. Engineering remains within
the goal's one-hour ceiling.

## Rehearsal

Local rehearsal completed on 2026-08-11 before any live UI dispatch:

1. `cargo test --example pty_observer` passed nine focused tests covering both
   command modes, exact `/exit\r` bytes, real resize activity, structured event
   order, signal 2, exit code 23, both timeout boundaries, and owned cleanup.
2. The unchanged signal fixture emitted its complete sequence with signal 2,
   7 initial bytes, and 13 resized bytes. The new exit fixture emitted its
   complete sequence with code 23, 7 initial bytes, and 12 resized bytes.
3. `cargo test`, formatting, strict all-target Clippy, and the explicit
   conformance test passed.
4. Exact-tree `nix flake check` passed. It retained the known warning that the
   app lacks `meta` and the declared omission of incompatible `x86_64-linux`
   checks.
5. `nix run . -- doctor` reported a ready SDK 0.6.8 host, valid credential
   metadata, the Jujutsu project, an outside-mount credential file, and both
   harnesses without printing credential values.
6. The exact immutable shim exited zero with `codex-cli 0.147.0`. Its symlink,
   both native hashes, and the observer hash matched the declared identities.
   A path-restricted Jujutsu diff from `147e956d` showed no observer, fixture,
   product, package, manifest, or lockfile change.
7. An anchored status query found Experiment 0007 as the sole active record;
   the record template was excluded from that query.

8. Immediately before dispatch, the operator's name-only check in the external
   Fish shell reported `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
   `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` all absent. No environment
   value was read or recorded.

The complete rehearsal is now qualified. No live command had been issued when
this result was recorded.

## Results

### Native Codex unit

The operator ran the exact native command once from the qualified external
Fish shell and returned this complete structural output:

```text
{"event":"started"}
{"event":"activity_initial"}
{"event":"resized"}
{"event":"activity_resized"}
{"event":"concurrent_window"}
{"event":"exit_command"}
Error: PTY child did not exit before timeout
90:91: syntax error: Expected “"” but found unknown token. (-2741)
```

Native Codex reached initial activity, actual resize activity, the unattended
hold, and the exact `/exit\r` action, then remained alive beyond the fixed
15-second exit bound. The observer emitted neither `exited` nor a numeric
summary, so this unit establishes no native exit status. It consumes the sole
native run and is not retried.

The final `-2741` line appeared after the observer timeout diagnostic, as a
similar line did in Experiment 0006. Its AppleScript-style provenance remains
unestablished, so it is retained as an unattributed post-observer diagnostic
and is not used in the decision. A subsequent read-only process-table query
found no matching observer or npm Codex-launcher process.

### Packaged Codex unit

The operator then ran the exact immutable packaged command once from the same
external Fish shell and returned this complete structural output:

```text
{"event":"started"}
{"event":"activity_initial"}
{"event":"resized"}
{"event":"activity_resized"}
{"event":"concurrent_window"}
{"event":"exit_command"}
Error: PTY child did not exit before timeout
90:91: syntax error: Expected “"” but found unknown token. (-2741)
```

Packaged Codex reached the same structural boundaries as native Codex and
remained alive beyond the same 15-second exit bound. It emitted neither
`exited` nor a numeric summary, so it also establishes no exit status. The
matching timeout supplies no evidence of a Fortlet-specific discrepancy.

The same unattributed `-2741` diagnostic followed the observer error. It is
retained but excluded from the decision. A subsequent read-only process-table
query found no matching observer or packaged Fortlet process.

## Terminal Closure

1. Outcome: rejected — exact `/exit\r` input did not terminate either native or
   packaged Codex within the fixed bound, so the experiment establishes no
   exact status to compare. The matching structural results expose no packaged
   discrepancy.
2. Root cause: the observer demonstrably wrote the declared bytes, but the
   experiment did not establish that either real UI accepted them as a
   terminating command. Because raw UI state was intentionally unavailable,
   command-readiness, key interpretation, and current CLI semantics remain
   indistinguishable.
3. Actual total cost: zero money, zero paid quota, zero model prompts, two of
   two units, and zero retries. Numeric wall time was not captured; no estimate
   is backfilled. Both observer invocations enforced every declared stage
   bound, and no overrun was reported.
4. Next action: stop this goal. No Fortlet repair is justified. Any successor
   must first establish the native UI's accepted termination interaction
   without retrying this exact `/exit\r` protocol or weakening the raw-content
   and zero-prompt boundaries.
