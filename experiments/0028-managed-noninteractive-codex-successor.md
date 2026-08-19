# Experiment 0028: Managed non-interactive Codex successor

Status: declared — 2026-08-19
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

Pending.

## Terminal Closure

Pending.
