# Experiment 0028: Managed non-interactive Codex successor

Status: rejected — terminal 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0005, FIP-0008, FIP-0009, and FIP-0010
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0024 is terminally rejected and must not be retried. Its managed
non-interactive Codex process remained alive on an upstream request while
Fortlet's collected attachment exposed no progress. Experiment 0026 attributed
that composition with credential-free local fixtures. Experiment 0027 accepted
the corrected immutable production streaming/exit path without provider
credentials.

The treatment revision is
`7b915c246dd1fef37b0ab9ee2a1d907df9638da8`, packaged as
`/nix/store/md28p2a03dyklbms5rd34gqpqx1hqa9k-fortlet-0.1.0` for
aarch64-darwin. The complete standard gate and both ignored stock-Codex 0.147.0
local-server compatibility fixtures passed before declaration.

## Hypothesis and Production Mechanism

Fortlet's packaged non-terminal path will stream Codex 0.147.0 output as events
arrive, preserve the ordinary Apps-disabled ChatGPT authentication path, and
return the explicit guest zero exit status after one short response. If the
upstream request becomes silent instead, the adapter's accepted ten-minute
inactivity ceiling will kill only the command and return one interactive retry
correction while preserving the capsule.

## Declared Scope

1. Confirm the frozen package/revision, empty working copy, public Codex
   absence, and empty MicroSandbox inventory.
2. From `/Users/cody/dev/fortlet`, run exactly once:

   ```text
   <package>/bin/fortlet run codex --project /Users/cody/dev/fortlet -- \
     exec --ephemeral --skip-git-repo-check \
     "Return exactly fortlet-noninteractive-ok without using tools."
   ```

3. Record the complete visible output and exact host status. Do not alter the
   prompt, arguments, model, login, package, project, or timeout.
4. After success or failure, use only the same packaged public status, stop,
   and reset commands. Require final absence and an empty inventory.

No second prompt, retry, interactive or native fallback, tool call, repository
edit by Codex, credential-content inspection, Apps invocation, private capsule
mutation, remote mutation, release, tag, package publication, or merge is in
scope.

## Alternatives

1. Ask the operator to repeat the old Experiment 0024 command. Rejected because
   terminal units are immutable and this treatment has a new runtime boundary.
2. Use an interactive prompt. Rejected because Experiment 0025 already proved
   that control and this GOAL specifically closes non-interactive reliability.
3. Ask for an edit/test loop. Rejected because one exact text response is the
   smallest unit that proves model response, streaming completion, and exit.

## Risks

1. Host authentication or provider availability may fail independently. The
   first result settles the unit; clean up publicly without retry or credential
   inspection.
2. Codex may remain silent until Fortlet's ten-minute inactivity ceiling. That
   is an accepted bounded failure only if the terminal error gives the single
   interactive correction and cleanup preserves the capsule.
3. Stop/reset may fail. Record the exact owned artifact and do not use a private
   removal path.

## Acceptance Criteria

1. Revision, package, empty working copy, public absence, and empty inventory
   match before dispatch.
2. The sole prompt returns an exact `fortlet-noninteractive-ok` response through
   ordinary ChatGPT authentication, with visible output and no `codex_apps`,
   HTTP 451, `no_biscuit_no_service`, or MCP-startup-incomplete diagnostic.
3. The packaged Fortlet process exits cleanly with exact host status zero.
4. Public status reports the reusable Codex capsule running, public stop/reset
   restore absence, and final MicroSandbox inventory is empty.
5. The first request failure, invariant anomaly, cleanup failure, or accepted
   ten-minute inactivity failure terminates the experiment. There is no retry.

## Budget and Plan

Exactly one short operator-authorized model prompt, one managed Codex process,
one owned reusable capsule, zero tools, zero retries, and at most the accepted
ten-minute output-inactivity ceiling. No additional paid quota or external
spend is authorized.

## Rehearsal

Experiment 0027 passed the packaged generic non-terminal streaming and exact
exit path. Before declaration, 81 unit tests, 29 ordinary integration tests,
formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check` passed through `nix develop`; the two pinned ignored
stock-Codex compatibility fixtures then passed against local fake servers.

## Results

Immediately before dispatch, the working copy was empty, packaged public status
reported `codex<TAB>absent`, and the packaged MicroSandbox CLI reported `[]`.
The frozen command was launched once. It immediately streamed this stderr:

```text
Reading additional input from stdin...
```

It then emitted no more output. During the wait, the operator reported a macOS
firewall popup asking whether `msb` could reach the configured DNS server; the
popup's final disposition was not independently observed. Exactly 600 seconds
after the last output event, Fortlet killed the guest command and returned host
status 1 with:

```text
fortlet: terminal stage failed; retry interactively from a supported terminal: non-interactive command exceeded its 600-second inactivity ceiling
```

There was no model response and no `codex_apps`, HTTP 451,
`no_biscuit_no_service`, or MCP-startup-incomplete diagnostic. No second prompt
or process was launched. Packaged public cleanup reported
`codex<TAB>running`, `codex<TAB>stopped`, `codex<TAB>reset`, and finally
`codex<TAB>absent`; the final MicroSandbox inventory was `[]`.

Post-terminal source inspection found that Codex 0.147.0 emits the observed
line immediately before blocking in `read_to_end(stdin)`. MicroSandbox 0.6.8's
streaming dispatcher records `StdinMode::Null` in host-only options but neither
places a stdin mode in `ExecRequest` nor sends the empty `ExecStdin` frame that
means EOF. Its `StdinMode::Bytes` and `ExecSink::close` paths do send that
frame. The Codex process therefore had not advanced to its model request; the
firewall popup cannot explain this process-level wait.

## Terminal Closure

Rejected. The exact response and zero-exit criteria failed because the first
streaming implementation relied on MicroSandbox's documented null-stdin mode,
which does not close guest stdin in the pinned streaming path. Fortlet's new
visibility and failure bound worked: it exposed the wait immediately, killed
only the command at 600 seconds, returned the one interactive correction with
status 1, preserved the capsule, and cleaned up publicly.

Actual cost was one launched prompt process, one owned capsule, 600 seconds of
post-output inactivity, zero model responses, zero tool calls, no retry, and
unknown provider quota because Codex never passed its stdin read. The next
action is an in-scope explicit-EOF correction and credential-free regression;
the accepted GOAL prohibits another model-backed attempt.
