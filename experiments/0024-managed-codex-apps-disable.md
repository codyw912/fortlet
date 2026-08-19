# Experiment 0024: Managed Codex Apps disable

Status: rejected — terminal 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0008, and FIP-0009
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0023 terminally accepted the packaged daily Codex path and one
ordinary model-backed edit/test loop. Its only remaining visible startup defect
was the built-in `codex_apps` client failing with HTTP 451
`no_biscuit_no_service`. Native Codex remains the unmodified control and may
continue to expose Apps according to its host configuration.

The frozen treatment revision is
`a98288dff08db53e34127e51b27c817078cf20b5` (`Disable unsupported Codex
Apps client`). Its immutable aarch64-darwin package is
`/nix/store/d2qnnmsfvbchf4vz1j9j8iwngbiy97q8-fortlet-0.1.0`, with
MicroSandbox 0.6.8 and Codex 0.147.0. Codex's adapter prepends exactly
`--disable apps` at runtime attachment.

The complete local gate passed 72 unit tests, 29 non-ignored integration tests,
formatting, strict all-target/all-feature Clippy, conformance, and `nix flake
check` on aarch64-darwin. Both ignored stock-Codex fixtures passed separately
against the native 0.147.0 binary. Immediately before dispatch, the working
copy was an empty successor of the exact treatment, the public CLI reported
`codex<TAB>absent`, and the pinned MicroSandbox CLI reported no sandboxes. The
project-input SHA-256 values remain:

1. `.fortlet/environment.json`:
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh`:
   `5857604b2ceb0dda2117ee4a54256d342300b3832305ebd9a777cf6ecfe703e0`.

## Hypothesis and Production Mechanism

The pinned Codex feature toggle removes only the reserved `codex_apps` server
from Codex's effective MCP map. Therefore a Fortlet-managed packaged launch
will retain ordinary model authentication while omitting the HTTP 451 and
MCP-startup-incomplete warnings. Fortlet will not write persistent Codex
configuration or add an Apps credential.

## Declared Scope

After the deterministic tests and complete standard gate pass:

1. Freeze the exact signed revision, package path, project-input hashes, public
   Codex absence, and empty MicroSandbox inventory. Stop on ambiguity.
2. From `/Users/cody/dev/fortlet`, run one packaged non-interactive launch:

   ```text
   fortlet run codex --project /Users/cody/dev/fortlet -- exec --ephemeral --skip-git-repo-check "Return exactly fortlet-apps-disabled-ok without using tools."
   ```

3. Record the exact model response and all startup diagnostics. Require no
   `codex_apps`, HTTP 451, `no_biscuit_no_service`, or MCP-startup-incomplete
   warning.
4. Require the public CLI to report the same owned Codex capsule running, then
   stop and reset it through public packaged commands. Require final absence
   and an empty MicroSandbox inventory.

No App invocation, live Excel control, Sites hosting, credential-content read,
native harness launch, repository edit, retry, replacement prompt, second
session, remote mutation, settings change, release, or package publication is
in scope.

## Alternatives

1. Treat deterministic adapter and stock-CLI evidence as sufficient. Rejected
   because the GOAL explicitly requires the packaged startup warning and model
   path to be observed together.
2. Use an interactive agent edit/test loop. Rejected because Experiment 0023
   already proved that path and this unit needs only the changed startup
   capability boundary.
3. Invoke an App as a negative test. Rejected because the unsupported
   capability is intentionally absent and no Apps authorization belongs in the
   capsule.

## Risks

1. Host authentication or the provider may fail independently. Record the
   first result, perform public cleanup, and close the experiment without
   inspecting credentials or retrying.
2. Absence of a warning alone cannot prove absence of every network packet.
   Attribute that narrower claim to the pinned source and compatibility
   fixture, not to this terminal transcript.
3. Stop/reset may fail. Report the exact owned artifact and do not remove an
   uncertain capsule through private means.

## Acceptance Criteria

1. The complete local gate and exact immutable package build pass before
   dispatch.
2. Initial public Codex status is absent and MicroSandbox reports no sandbox.
3. The sole packaged prompt returns exactly `fortlet-apps-disabled-ok` through
   ordinary ChatGPT model authentication.
4. No known Apps 451 or MCP-startup-incomplete diagnostic appears.
5. Public stop/reset restores absence and MicroSandbox reports no sandbox.
6. The first failure, invariant anomaly, cleanup completion, or ten elapsed
   minutes after dispatch terminates the experiment.

## Budget and Plan

Zero external money or paid quota beyond the one model prompt already
authorized by GOAL.md; at most one bounded host OAuth refresh, one packaged
non-interactive session, one owned reusable capsule, no retry, and at most ten
elapsed minutes after dispatch. Deterministic tests and the package build are
the zero-provider rehearsal.

## Rehearsal

The adapter unit tests prove exact argument insertion, requested-argument
preservation, Tact preservation, and use at the shared runtime attachment seam.
The pinned stock-Codex fixture proves the fixed disable remains authoritative
over both a user `--enable apps` and `features.apps=true`, retains a separate
configured MCP server, accepts the fixed flag at version 0.147.0, preserves the
external model-token headers, and makes no OAuth refresh request. The complete
gate, immutable package build, exact revision, project inputs, public absence,
and empty runtime inventory are frozen above.

## Results

The complete local gate and immutable package build passed, and the frozen
baseline matched every declared revision, input, absence, and inventory check.
The sole packaged non-interactive command then remained silent for the full
ten-minute bound. It emitted no `codex_apps`, HTTP 451,
`no_biscuit_no_service`, MCP-startup-incomplete, credential, or model response
diagnostic, but it also did not return the required model response or exit.

At the bound, the packaged public CLI reported `codex<TAB>running`. Public
`stop` reported `codex<TAB>stopped`; the waiting launch then failed at the
terminal stage because the runtime exec session ended without an exit event.
Public `reset` reported `codex<TAB>reset`, final status reported
`codex<TAB>absent`, and the pinned MicroSandbox CLI reported no sandboxes. No
retry, replacement prompt, second session, credential-content read, App
invocation, repository edit, or remote mutation occurred.

## Terminal Closure

Rejected. The packaged treatment removed the previously visible Apps warning,
but the required ordinary model response was not observed before the frozen
timeout. The immediate failure mode was a non-interactive MicroSandbox exec
session that remained open without output; after public stop it ended without
an exit event. Whether the model request reached the provider is unknown, so
this unit cannot establish preserved model behavior or attribute the hang to
model authentication.

Actual cost was one packaged non-interactive attempt, one owned capsule, no
observable model response, ten elapsed minutes, and public stop/reset cleanup.
Keep the deterministic implementation evidence and FIP-0009's conformance
partial. Any further live check must be a newly declared experiment with a new
prompt authorization and should use the already-proven interactive packaged
path rather than repeat this non-interactive assumption.
