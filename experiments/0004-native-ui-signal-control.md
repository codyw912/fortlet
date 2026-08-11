# Experiment 0004: Native UI signal control

Status: declared
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0003 is the baseline. Its immutable packaged Codex and Tact UIs
both produced activity before and after an 80-by-24 to 120-by-40 PTY resize,
then remained alive beyond 15 seconds after one `SIGINT` to the observer-owned
foreground process group. Because both native UIs may use the first interrupt
as an in-UI cancellation, that result does not identify a Fortlet or
MicroSandbox propagation defect.

This experiment is the operator-authorized control. It uses the unchanged PTY
observer at checkpoint `ad7fdd0e` and directly launches these host commands:

1. Codex 0.147.0: canonical launcher
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js`,
   SHA-256 `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477`.
   Its selected `aarch64-apple-darwin` binary has SHA-256
   `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37`.
2. Tact 0.3.7: `/Users/cody/.local/bin/tact`, SHA-256
   `c8b5eb923ef3f6edec7d240f7960c072e0db411896c947de09cd41da00398545`.

`command -v`, canonical-path resolution, SHA-256 checks, file inspection, and
direct `--version` invocations established these identities on 2026-08-11.

## Hypothesis and Production Mechanism

If one foreground-group `SIGINT` is native UI cancellation rather than a
termination request, each native UI will reproduce its packaged counterpart's
bounded-exit failure under the same observer. Matching native behavior rejects
the shared Experiment 0003 acceptance assumption and removes the evidence basis
for attributing that result to Fortlet.

If a native UI instead terminates within 15 seconds with an exact status while
its packaged counterpart did not, the differing isolation and SDK attachment
path becomes the declared mechanism that narrows attribution toward Fortlet or
MicroSandbox signal propagation. This control does not distinguish those two
components and authorizes no repair.

## Declared Scope

Run native Codex, then native Tact, directly from the exact paths above and
with working directory `/Users/cody/dev/fortlet`. Fortlet shims are not added
to `PATH`, and neither `fortlet run` nor `fortlet native` participates.

The unchanged observer protocol is frozen from Experiment 0003:

1. Initial PTY: 80 columns by 24 rows.
2. Startup activity timeout: 90 seconds.
3. Startup output settles for 100 milliseconds before resize.
4. Resized PTY: 120 columns by 40 rows.
5. Post-resize activity timeout: 15 seconds.
6. Unattended hold: 20 seconds.
7. Signal: one `SIGINT` to the verified observer-owned process group.
8. Exit timeout: 15 seconds.

The observer, project, host environment, harness versions, dimensions,
timings, ordering, signal, and zero-input rule remain frozen. No prompt text,
newline, adaptive input, model task, Fortlet capsule, publication, remote
execution, shell setup mutation, or cleanup outside observer-owned processes
is in scope. No raw PTY bytes are persisted.

## Alternatives

1. Send a second signal or increase the packaged timeout. Rejected because it
   repeats the failed assumption at mask level and violates the active
   repeated-failure stop.
2. Attach a deterministic guest terminal probe. Deferred because direct native
   UI behavior is the cheaper control for harness-level signal semantics.
3. Invoke `fortlet native`. Rejected because direct paths remove Fortlet and
   its resolution logic from the control.

## Risks

1. Native startup may behave differently because host credentials or local
   configuration differ from the guest. Structural PTY and exit events remain
   the only evidence; exact screen text is never an acceptance gate.
2. A host UI may attempt benign metadata or update checks. No prompt is sent,
   no intentional inference or paid quota is used, and any attempted shell or
   setup mutation is experiment-fatal.
3. The npm Codex launcher adds a Node parent around its native binary. Because
   the observer signals the entire owned foreground process group, both
   processes receive the signal; this is the installed host command users
   actually invoke and is therefore the relevant native behavior.
4. A timeout causes the observer's bounded owned cleanup. It is a settled unit,
   not authorization to retry.

## Acceptance Criteria

1. `cargo test --example pty_observer` and the deterministic fixture rehearsal
   pass immediately before dispatch with the unchanged observer.
2. Each observer emits, in order, `started`, initial activity, `resized`,
   resized activity, `concurrent_window`, and `signal`.
3. For a native UI that terminates, the observer also emits `exited` and an
   exact numeric summary within 15 seconds. A timeout is recorded verbatim.
4. If both native UIs time out like their packaged counterparts, reject the
   shared one-`SIGINT` termination assumption. Experiment 0003 then supplies no
   evidence of a Fortlet-specific signal defect.
5. If a native UI terminates while its packaged counterpart timed out, record
   a per-harness isolated-path discrepancy. Both must terminate to support a
   cross-harness propagation-discrepancy claim.
6. Mixed results support only their per-harness comparisons; they neither
   accept the shared assumption nor establish a broad Fortlet defect.
7. Both units run even if one fails unless a hard invariant fires. There are
   zero retries and no adaptive signal, input, or timeout changes.
8. No raw UI output is persisted, no prompt or newline is submitted, and no
   credential value, unowned process control, paid quota, or shell mutation
   occurs.

## Budget and Plan

Budget: zero money, zero paid quota, zero submitted prompts, zero remote
mutation, two harness units, zero retries, and at most 30 minutes after dispatch
begins. Run and record Codex completely, then Tact completely. The control
remains inside the goal's cumulative two-hour engineering ceiling.

## Rehearsal

Completed on 2026-08-11 immediately before dispatch without modifying the
observer:

1. `nix develop -c cargo test --test conformance` passed the conformance-map
   integrity check.
2. `nix develop -c cargo test --example pty_observer` passed all six focused
   observer tests.
3. `nix develop -c cargo run --quiet --example pty_observer -- fixture` emitted
   the complete declared event sequence and a summary with signal 2, 7 initial
   bytes, and 13 resized bytes.

## Results

Pending.

## Terminal Closure

Pending.
