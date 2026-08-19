# GOAL: Resolve the Codex Apps startup warning

Status: active — operator-accepted 2026-08-19. Fortlet's ordinary Codex model
and edit/test path works, but stock Codex 0.147.0 still starts its built-in
`codex_apps` MCP client without the separate biscuit authorization it expects.
Every daily launch consequently reports an HTTP 451 warning for a capability
Fortlet does not currently claim to support.

Make Codex startup capability-aware. Determine the actual pinned upstream
contract, then either delegate the capability through a narrow safe boundary or
disable only the unavailable built-in Apps client. Preserve ordinary model
authentication and user-configured MCP servers.

Before implementation, read FIP-0001, FIP-0005, and FIP-0008 in full.

## Deliverable 0 — Establish the upstream contract

1. Inspect the Codex 0.147.0 source and configuration schema that register
   `codex_apps`, acquire or attach its biscuit, and enable or disable the client.
2. Separate observed facts from inference. Record exact primary-source paths,
   configuration keys, authorization source and lifetime, and failure behavior.
3. Determine whether a supported external capability exists that Fortlet can
   delegate without exposing a durable browser cookie, ChatGPT session, host
   configuration, or general MCP credential to the guest.
4. Prefer a clean adapter-owned disable path when no narrow supported
   delegation surface exists. Do not patch or fork Codex for this GOAL.

## Deliverable 1 — Accept the capability boundary

1. Draft FIP-0009 for Codex Apps capability handling. Keep MCP authorization
   independent from FIP-0008 model authentication.
2. Specify ownership, persistence, configuration precedence, failure behavior,
   and version-pinned compatibility evidence.
3. Preserve user-configured MCP servers. A fallback MUST target only the
   unavailable built-in `codex_apps` client.
4. Obtain operator acceptance of the concrete FIP before implementation.

## Deliverable 2 — Implement the smallest safe behavior

1. Put Codex-specific behavior in the harness adapter or its owned session
   preparation boundary, not generic orchestration.
2. Avoid overwriting persistent user configuration. Any generated or injected
   setting MUST be minimal, deterministic, version-pinned, and reversible.
3. Keep model credentials, refresh behavior, terminal behavior, lifecycle
   commands, and non-Apps MCP configuration unchanged.
4. Fail closed with one actionable correction if upstream provides neither a
   safe delegation surface nor a targeted disable mechanism.

## Deliverable 3 — Verify daily startup and publish once

1. Add deterministic coverage for configuration precedence, targeted Apps
   handling, preservation of other MCP servers, and the unchanged credential
   projection.
2. Run a bounded packaged Codex startup check. It MUST show no misleading
   `codex_apps` 451 warning and MUST preserve an ordinary model-backed prompt
   when live authorization is available.
3. Update conformance, README, runbook, GOAL, and handoff evidence with the
   exact supported and unsupported capability surface.
4. Run the complete standard verification set through `nix develop`, then use
   one goal bookmark and pull request. The operator performs the squash merge.

## Definition of Done

1. A normal packaged Codex launch does not emit the known `codex_apps` HTTP 451
   warning merely because Fortlet lacks a separate Apps biscuit.
2. Ordinary ChatGPT model authentication and renewal still work.
3. User-configured MCP servers are not disabled, rewritten, or granted broader
   credentials.
4. No browser cookie, ChatGPT session cookie, raw biscuit, host Codex config,
   refresh token, or general-purpose MCP credential becomes guest-readable.
5. The pinned compatibility behavior has automated evidence, the local and
   hosted gates pass, and the PR is ready for operator merge.

## Excluded scope

Do not add a general proxy, Codex fork, browser-session projection, broad MCP
credential broker, MCP server manager, remote execution, standalone installer,
hosted CI change, repository setting, release, tag, or package publication.

## Budget and escalation

Engineering ceiling: two hours. External money and paid quota remain zero.
One bounded operator-observed model prompt MAY reuse the existing ChatGPT login
after deterministic evidence passes; do not inspect or print credential
content. This accepted GOAL authorizes one descriptive bookmark and one draft
PR targeting `main` under FIP-0005. Stop for FIP-0009 acceptance, material
scope expansion, any design requiring raw browser or biscuit material in the
guest, a new secret class, destructive or unrelated mutation, merge, or two
failures sharing an unresolved cause.

## Verification

Run focused adapter and credential-projection tests while editing. Before
readiness, run the complete standard verification set from `docs/RUNBOOK.md`
inside `nix develop` and the bounded packaged startup check declared by the
accepted FIP.
