# Experiment 0008: Human-paced Codex exit parity

Status: in-flight
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0007 wrote atomic `/exit\r` bytes to native and packaged Codex
0.147.0. Both reached the action and stayed alive beyond the same 15-second
bound, supplying no termination status and no Fortlet-specific discrepancy.

Official Codex source now explains that control. Tag `rust-v0.147.0` is version
0.147.0 and peels to commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`. Its paste-burst classifier uses an
8-millisecond inter-key threshold and a 9-millisecond recommended flush delay.
Enter during an active paste burst becomes a composer newline rather than a
submit event. The source regression test for rapid ASCII plus Enter confirms
that it does not submit. Experiment 0007 therefore exercised Codex paste
handling rather than the accepted typed command path.

Source identities:

1. Release tag: <https://github.com/openai/codex/releases/tag/rust-v0.147.0>
2. Peeled commit:
   <https://github.com/openai/codex/commit/be6e8eac029b183056b7e4402879f15d2c85f61b>
3. Paste thresholds and Enter handling:
   <https://github.com/openai/codex/blob/be6e8eac029b183056b7e4402879f15d2c85f61b/codex-rs/tui/src/bottom_pane/paste_burst.rs#L152-L165>
4. Exact rapid-input regression test:
   <https://github.com/openai/codex/blob/be6e8eac029b183056b7e4402879f15d2c85f61b/codex-rs/tui/src/bottom_pane/chat_composer.rs#L8483-L8529>

The exact executable identities are:

1. Observer checkpoint `31b4f073`, built binary
   `/Users/cody/dev/fortlet/target/debug/examples/pty_observer`, SHA-256
   `77037127acef6b3d28d7e120eac6bf6dcef705ddde0c4201ce027201875e98c4`.
2. Native npm launcher
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js`,
   SHA-256
   `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477`.
3. Its selected native binary at
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex`,
   SHA-256
   `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37`.
4. Immutable package output
   `/nix/store/50kwlpmbsr34qh50lpvb87ila07c4gvw-fortlet-0.1.0`, whose
   `libexec/fortlet/shims/codex` is a relative symlink to its packaged
   `bin/fortlet`.

## Hypothesis and Production Mechanism

Native Codex will treat separately delivered `/`, `e`, `x`, `i`, and `t` keys,
each followed by a 20-millisecond delay, and a separately delivered Enter key
as a typed local `/exit` command. The pacing exceeds Codex's paste-burst and
recommended-flush thresholds, so Enter reaches normal composer submission and
dispatches the local exit action.

Fortlet's packaged terminal attachment will carry the same timed key sequence
to guest Codex. Its session transaction should return the guest's exact normal
exit status to the observer. Matching code 0 with no signal establishes parity
for this action. A mismatch identifies a packaged-path discrepancy without
distinguishing Fortlet orchestration from MicroSandbox attachment and without
authorizing repair.

## Declared Scope

The operator runs two units from `/Users/cody/dev/fortlet`, in this fixed order,
from the same external Fish shell:

```fish
target/debug/examples/pty_observer observe-typed-exit \
    /Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js \
    /Users/cody/dev/fortlet \
    20

target/debug/examples/pty_observer observe-typed-exit \
    /nix/store/50kwlpmbsr34qh50lpvb87ila07c4gvw-fortlet-0.1.0/libexec/fortlet/shims/codex \
    /Users/cody/dev/fortlet \
    20
```

Before the first unit, a name-only check must report `CODEX_THREAD_ID`,
`CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` absent. No
environment values are read or recorded.

Each unit uses an 80-column by 24-row PTY, 90-second startup-activity bound,
100-millisecond startup settle, resize to 120 by 40, 15-second resize-activity
bound, 20-second unattended hold, five separate command-character writes with
20 milliseconds after each, one separate `\r` Enter write, and a 15-second
exit bound. The operator supplies no input after starting a command and returns
the complete structural output verbatim.

The observer, identities, project, external shell, ordering, dimensions,
timings, action, and zero-retry rule remain frozen. These two `/exit`
interactions are local control input, not model prompts. No intentional
inference, paid quota, raw PTY capture, Fortlet product change, harness
configuration change, shell setup mutation, remote execution, publication, or
cleanup outside observer-owned processes is in scope. Ordinary Codex-managed
state and Fortlet-managed capsule writes are allowed for the two launches.

## Alternatives

1. Add one delay only before Enter. Rejected because Codex's classifier also
   buffers a multi-character burst; separate paced keys follow its human-like
   source test and make the interaction unambiguous.
2. Use the minimum 9-millisecond recommended delay. Rejected because a fixed
   20 milliseconds remains human-like and supplies conservative scheduling
   margin without meaningfully increasing the experiment.
3. Reuse atomic `/exit\r`, use EOF or signals, or match raw prompt text.
   Rejected because Experiment 0007 settled atomic input, the other actions do
   not test typed `/exit`, and raw screen content may expose account or project
   metadata.
4. Have the operator type manually. Rejected because exact observer-owned
   timing and structural evidence are reproducible and require no adaptive
   interaction.

## Risks

1. Codex may still not be ready for composer input despite settled activity and
   resize output. A timeout settles the unit; timing and input are not adapted.
2. Native or packaged Codex may close before `typed_exit_command`. That unit
   supplies no exit-action evidence and still consumes its run.
3. Normal startup may perform ordinary metadata or state activity. Credential
   values must remain host-brokered and unobserved; no model prompt or paid
   activity is allowed.
4. The packaged status may differ even when both UIs close. That is a reportable
   discrepancy, not permission to change Fortlet under this goal.
5. Any observer timeout invokes bounded cleanup of only its verified child
   process group. Fortlet-managed capsule state may remain under its existing
   idle policy.

## Acceptance Criteria

1. Twelve focused observer tests pass, including separate key writes, five
   exact 20-millisecond requested delays, event order, code 23 preservation,
   both exit-action timeouts, and owned cleanup. All three deterministic fixture
   modes emit their complete expected summaries.
2. The complete runbook gate, exact package build, native hashes, observer hash,
   source identity, marker-name check, and `nix run . -- doctor` pass before
   dispatch.
3. Each live unit emits `started`, `activity_initial`, `resized`,
   `activity_resized`, `concurrent_window`, and `typed_exit_command` in order.
4. A unit that exits also emits `exited` and a numeric summary within 15 seconds.
   A timeout or pre-action close is recorded verbatim.
5. Exact `exit_code:0` and `exit_signal:null` for both units accepts packaged
   typed-exit parity.
6. Any exact-status or termination mismatch rejects parity and records a
   packaged-path discrepancy without authorizing repair.
7. If native Codex does not terminate normally, reject the native interaction
   premise. The packaged unit still runs unless a hard invariant fires, and
   neither unit is retried.
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

1. Source inspection pinned official tag `rust-v0.147.0` to commit
   `be6e8eac029b183056b7e4402879f15d2c85f61b` and traced the atomic-input
   failure through the 8-millisecond paste threshold, Enter-as-newline branch,
   exact rapid-input regression test, slash dispatch, and normal shutdown path.
2. `cargo test --example pty_observer` passed twelve focused tests covering all
   modes, six separate typed key writes, five requested 20-millisecond delays,
   real resize activity, structured events, signal 2, exit code 23, both exit
   timeouts, and owned cleanup.
3. The signal fixture emitted its complete summary with signal 2, 7 initial
   bytes, and 13 resized bytes. Atomic and typed exit fixtures each emitted
   their distinct action event and complete summary with code 23, 7 initial
   bytes, and 12 resized bytes.
4. `cargo test`, formatting, strict all-target Clippy, and the explicit
   conformance test passed.
5. Exact-tree `nix flake check` passed. It retained the known warning that the
   app lacks `meta` and the declared omission of incompatible `x86_64-linux`
   checks.
6. `nix run . -- doctor` reported a ready SDK 0.6.8 host, valid credential
   metadata, the Jujutsu project, an outside-mount credential file, and both
   harnesses without printing credential values.
7. The frozen packaged shim exited zero with `codex-cli 0.147.0` and remained a
   relative symlink to packaged `bin/fortlet`. Native version was
   `codex-cli 0.147.0`; both native hashes and observer hash matched their
   declarations. A path-restricted Jujutsu diff from `31b4f073` showed no
   observer, fixture, product, package, manifest, or lockfile change. An
   anchored status query found Experiment 0008 as the sole active record.
8. Immediately before dispatch, the operator's name-only check in the external
   Fish shell reported `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
   `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` all absent. No environment
   value was read or recorded.

The complete rehearsal is qualified. No live UI unit had been dispatched when
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
{"event":"typed_exit_command"}
Error: PTY child did not exit before timeout
96:97: syntax error: Expected “"” but found unknown token. (-2741)
```

Native Codex reached initial activity, actual resize activity, the unattended
hold, and the fixed paced typed-exit action, then remained alive beyond the
15-second exit bound. The observer emitted neither `exited` nor a numeric
summary, so this unit supplies no native exit status and rejects the declared
native normal-termination premise. It consumes the sole native run and is not
retried or adapted.

The final `-2741` line appeared after the observer timeout diagnostic, as in
Experiments 0006 and 0007. Its AppleScript-style provenance remains
unestablished, so it is retained but excluded from the decision. Narrowed,
read-only process-table checks found no matching live observer command or exact
native npm launcher after the result. The pre-existing process table did
contain an unrelated six-hour-old observer test executable under its Cargo
parent; it did not match the declared live command and was not controlled.

### Packaged Codex unit

Pending the second declared zero-retry unit.

## Terminal Closure

Pending.
