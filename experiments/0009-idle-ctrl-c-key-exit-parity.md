# Experiment 0009: Idle Ctrl-C key exit parity

Status: in-flight
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiments 0003, 0004, and 0006 sent operating-system `SIGINT` to observer-
owned process groups. Native and packaged Codex remained alive because a
process signal is not the TUI's Ctrl-C key interaction. Experiments 0007 and
0008 then sent atomic and paced `/exit` PTY input; native and packaged Codex
again matched, but neither supplied a status.

The operator manually confirmed a distinct native Codex 0.147.0 control: one
Ctrl-C exits at an idle, empty composer. During an active conversation, the
first Ctrl-C interrupts work and a later Ctrl-C exits.

Official Codex source agrees. Tag `rust-v0.147.0` peels to commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`. Its formal one-second double-press
shortcut is disabled. The TUI offers Ctrl-C first to modal, history-search, and
composer-text cancellation. With none of those active, it interrupts
cancellable work or immediately requests shutdown-first exit when idle.

Source identities:

1. Release tag: <https://github.com/openai/codex/releases/tag/rust-v0.147.0>
2. Peeled commit:
   <https://github.com/openai/codex/commit/be6e8eac029b183056b7e4402879f15d2c85f61b>
3. Ctrl-C state machine:
   <https://github.com/openai/codex/blob/be6e8eac029b183056b7e4402879f15d2c85f61b/codex-rs/tui/src/chatwidget/interaction.rs#L378-L440>
4. Disabled double-press flag and one-second dormant timeout:
   <https://github.com/openai/codex/blob/be6e8eac029b183056b7e4402879f15d2c85f61b/codex-rs/tui/src/bottom_pane/mod.rs#L167-L181>

The exact executable identities are:

1. Observer checkpoint `cc52a2e9`, built binary
   `/Users/cody/dev/fortlet/target/debug/examples/pty_observer`, SHA-256
   `55c7b6d0c1a73dbc55b3c36ebbe300a2f1af59a4daec39732fa358518a439470`.
2. Native npm launcher
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js`,
   SHA-256
   `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477`.
3. Its selected native binary at
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/node_modules/@openai/codex-darwin-arm64/vendor/aarch64-apple-darwin/bin/codex`,
   SHA-256
   `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37`.
4. Immutable package output
   `/nix/store/60bqrm3v3rflzlfhr2y59f1d6z2f3gbl-fortlet-0.1.0`, whose
   `libexec/fortlet/shims/codex` is a relative symlink to its packaged
   `bin/fortlet`.

## Hypothesis and Production Mechanism

Native Codex will decode one `0x03` PTY byte as the Ctrl-C key at an idle,
empty composer and request normal shutdown. Fortlet's packaged terminal attach
will carry the same byte to guest Codex, and its session transaction will
return the guest's identical termination status to the observer.

Matching exact exit status establishes FIP-0001/FIP-0002 exit preservation for
this ordinary idle interaction. A mismatch identifies a packaged-path
discrepancy without distinguishing Fortlet orchestration from MicroSandbox
attachment and without authorizing repair.

## Declared Scope

The operator runs two units from `/Users/cody/dev/fortlet`, in this fixed order,
from the same external Fish shell:

```fish
target/debug/examples/pty_observer observe-ctrl-c \
    /Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js \
    /Users/cody/dev/fortlet \
    5

target/debug/examples/pty_observer observe-ctrl-c \
    /nix/store/60bqrm3v3rflzlfhr2y59f1d6z2f3gbl-fortlet-0.1.0/libexec/fortlet/shims/codex \
    /Users/cody/dev/fortlet \
    5
```

Before the first unit, a name-only check must report `CODEX_THREAD_ID`,
`CODEX_SANDBOX`, `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` absent. No
environment values are read or recorded.

Each unit uses an 80-column by 24-row PTY, 90-second startup-activity bound,
100-millisecond startup settle, resize to 120 by 40, 15-second resize-activity
bound, 5-second unattended hold, exactly one `0x03` PTY write, and a 15-second
exit bound. The operator supplies no input after starting a command and returns
the complete structural output verbatim.

The observer, identities, project, external shell, ordering, dimensions,
timings, action, and zero-retry rule remain frozen. No model prompt,
intentional inference, paid quota, raw PTY capture, Fortlet product change,
harness configuration change, shell setup mutation, remote execution,
publication, or cleanup outside observer-owned processes is in scope. Ordinary
Codex-managed state and Fortlet-managed capsule writes are allowed.

## Alternatives

1. Send two Ctrl-C bytes. Rejected because native Codex exits with one at the
   declared idle state; a second key could hide first-key loss or wrong state.
2. Send process-group `SIGINT`. Rejected because prior experiments settled that
   different mechanism and it does not exercise TUI input handling.
3. Retry `/exit`, use EOF, or inspect prompt text. Rejected because prior
   records exhausted blind slash-command timing, EOF is a different action,
   and raw screen content may expose account or project metadata.
4. Rely only on manual native behavior. Rejected because it supplies no
   packaged status comparison; the exact one-byte observer slice is small and
   deterministic.

## Risks

1. Structural activity may occur while a modal or cancellable startup task is
   still active. The first key could cancel that state instead of exiting. A
   timeout settles the unit; the hold and action are not adapted.
2. Native or packaged Codex may close before `ctrl_c_key`. That unit supplies no
   key-action evidence and still consumes its run.
3. Normal startup may perform ordinary metadata or state activity. Credential
   values must remain host-brokered and unobserved; no model prompt or paid
   activity is allowed.
4. The packaged status may differ even if both UIs close. That is a reportable
   discrepancy, not permission to change Fortlet under this goal.
5. Any observer timeout invokes bounded cleanup of only its verified child
   process group. Fortlet-managed capsule state may remain under existing idle
   policy.

## Acceptance Criteria

1. Fifteen focused observer tests pass, including exact one-byte/one-write
   delivery, event order, code 23 preservation, all three input-action timeout
   paths, and owned cleanup. All four deterministic fixtures emit complete
   expected summaries.
2. The complete runbook gate, exact package build, native hashes, observer hash,
   source identity, marker-name check, and `nix run . -- doctor` pass before
   dispatch.
3. Each live unit emits `started`, `activity_initial`, `resized`,
   `activity_resized`, `concurrent_window`, and `ctrl_c_key` in order.
4. A unit that exits also emits `exited` and a numeric summary within 15 seconds.
   A timeout or pre-action close is recorded verbatim.
5. Exact matching `exit_code` and `exit_signal` values accept packaged idle
   Ctrl-C parity. The source-established expected result is code 0 and no
   signal for both.
6. Any exact-status or termination mismatch rejects parity and records a
   packaged-path discrepancy without authorizing repair.
7. If native Codex does not terminate normally, reject the automated idle-
   readiness premise. The packaged unit still runs unless a hard invariant
   fires, and neither unit is retried.
8. No raw UI output, environment value, model prompt, credential value, paid
   quota, shell mutation, unexpected external mutation, or unowned process
   control occurs.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, two units, zero
retries, and at most 10 minutes after dispatch begins. Run and record native
Codex completely, then packaged Codex completely. Engineering remains within
the goal's 45-minute ceiling.

## Rehearsal

Local rehearsal completed on 2026-08-11 before any live UI dispatch:

1. Source inspection pinned official tag `rust-v0.147.0` to commit
   `be6e8eac029b183056b7e4402879f15d2c85f61b` and traced Ctrl-C through the
   bottom-pane cancellation routing, active-work interrupt branch, idle quit
   request, dormant one-second timeout, and disabled double-press flag. This
   matches the operator's manual native control.
2. `cargo test --example pty_observer` passed fifteen focused tests covering all
   modes, one-byte/one-write `0x03` delivery, real resize activity, structural
   events, signal 2, exit code 23, all three input-action timeout paths, and
   owned cleanup.
3. The signal fixture emitted its complete summary with signal 2, 7 initial
   bytes, and 13 resized bytes. Atomic exit, typed exit, and Ctrl-C-key fixtures
   each emitted their distinct action and a complete summary with code 23,
   7 initial bytes, and 12 resized bytes.
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
   declarations. A path-restricted Jujutsu diff from `cc52a2e9` showed no
   observer, fixture, product, package, manifest, or lockfile change. An
   anchored status query found Experiment 0009 as the sole active record.
8. Immediately before dispatch, the operator's name-only check in the external
   Fish shell reported `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
   `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` all absent. No environment
   value was read or recorded.

The complete rehearsal is qualified. No live UI unit had been dispatched when
this result was recorded.

## Results

Pending.

## Terminal Closure

Pending.
