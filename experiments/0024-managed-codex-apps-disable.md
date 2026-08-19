# Experiment 0024: Managed Codex Apps disable

Status: active — declared 2026-08-19; dispatch awaits the complete local gate
Design: FIP-0001, FIP-0002, FIP-0008, and FIP-0009
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0023 terminally accepted the packaged daily Codex path and one
ordinary model-backed edit/test loop. Its only remaining visible startup defect
was the built-in `codex_apps` client failing with HTTP 451
`no_biscuit_no_service`. Native Codex remains the unmodified control and may
continue to expose Apps according to its host configuration.

The treatment is the final signed revision produced by this GOAL. It must use
MicroSandbox 0.6.8 and Codex 0.147.0, with Codex's adapter prepending exactly
`--disable apps` at runtime attachment. The exact revision and immutable Nix
package path will be frozen after the complete local verification set passes.

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
gate and immutable package result remain to be recorded before dispatch.

## Results

Pending.

## Terminal Closure

Pending.
