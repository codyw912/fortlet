# FIP-0009: Codex Apps capability handling

Status: Accepted
Recorded: 2026-08-19 from the daily-startup warning accepted in GOAL.md
Requires: FIP-0001, FIP-0002, FIP-0008

## Summary

Disable stock Codex 0.147.0's built-in Apps capability on Fortlet-managed
launches by injecting the pinned CLI's stable `--disable apps` feature toggle.
Do not change persistent Codex configuration or model authentication. Preserve
every user-configured MCP server except the upstream-reserved `codex_apps`
registration.

Fortlet does not claim Apps support until a later accepted FIP identifies a
supported, narrow authorization source that can remain host-owned. The native
escape hatch remains untouched.

## Motivation

The first successful daily Codex session proved model-backed edit/test work,
renewable host-owned credentials, terminal attachment, and public lifecycle
commands. It also reproduced this warning on every startup:

```text
MCP client for `codex_apps` failed to start
HTTP 451: {"message":"no_biscuit_no_service"}
```

That warning describes an unavailable optional capability, not a failure of
Fortlet's model-authentication boundary. Leaving it enabled makes a successful
daily launch look partially broken and incurs a network handshake that cannot
succeed with Fortlet's declared credentials.

FIP-0008 deliberately excludes cookies and MCP authorization and requires the
Apps biscuit question to be recorded separately. Supplying an unknown token,
copying browser state, or treating the existing refresh token as general MCP
authority would collapse that boundary without an upstream contract.

## Pinned upstream evidence

The source audit used official Codex tag `rust-v0.147.0`, whose peeled commit is
`be6e8eac029b183056b7e4402879f15d2c85f61b`.

Observed facts:

1. `codex-rs/features/src/lib.rs` declares feature key `apps` stable and
   default-enabled. `apps_enabled_for_auth` additionally requires ChatGPT auth.
2. `codex-rs/cli/src/main.rs` exposes repeatable global `--enable` and
   `--disable` flags. `--disable apps` becomes the final
   `features.apps=false` override after ordinary `-c` overrides.
3. `codex-rs/codex-mcp/src/mcp/mod.rs` removes only the reserved `codex_apps`
   entry when Apps is disabled. Other configured MCP servers remain in the
   effective server map.
4. The same module constructs `codex_apps` at
   `https://chatgpt.com/backend-api/ps/mcp` with ChatGPT authentication. It
   recognizes `CODEX_CONNECTORS_TOKEN` only when that environment variable is
   already present.
5. `codex-rs/codex-mcp/src/connection_manager.rs` explicitly calls
   `CODEX_CONNECTORS_TOKEN` a debug override. Otherwise the reserved server
   receives Codex's existing ChatGPT auth provider.
6. `codex-rs/login/src/server.rs` requests
   `api.connectors.read` and `api.connectors.invoke` during browser login. The
   open-source client contains no separate biscuit acquisition, persistence,
   refresh, or external-provider contract.
7. The installed pinned CLI accepts `--disable apps --version`. With an
   isolated synthetic `CODEX_HOME` whose config enables Apps and registers a
   `sample` stdio MCP server, `--disable apps mcp list` retains `sample`.

Primary sources:

- [Feature definition](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/features/src/lib.rs)
- [CLI feature toggles](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/cli/src/main.rs)
- [Apps MCP registration and filtering](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/codex-mcp/src/mcp/mod.rs)
- [Apps authentication selection](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/codex-mcp/src/connection_manager.rs)
- [Login scopes](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/login/src/server.rs)

Inference, not an observed client contract: the service's HTTP 451 text implies
an additional server-side authorization condition called a biscuit. The pinned
open-source client does not define how an external tool obtains or renews it.
Fortlet therefore cannot safely infer such a protocol from the error string.

## Decision

Add an adapter-owned launch-argument transform to the harness contract. The
default transform preserves requested arguments exactly. Codex 0.147.0 prepends
`--disable apps`; Tact retains the default.

Apply the transform at the single runtime attachment boundary for interactive
and non-interactive launches. Do not write `config.toml`, inspect host config,
modify the persistent guest Codex home, or change environment forwarding.

The disable is authoritative for Fortlet-managed Codex 0.147.0 launches.
User-supplied `--enable apps` or `-c features.apps=true` does not re-enable the
unsupported reserved client. A future opt-in requires a successor FIP with a
host-owned authorization source, renewal and revocation semantics, destination
restriction, and compatibility evidence.

## Specification

### Adapter and argument ownership

1. The harness adapter MUST own fixed launch arguments required by its pinned
   executable contract.
2. Generic runtime code MUST ask the adapter for effective launch arguments
   and MUST NOT branch on the harness name.
3. The Codex 0.147.0 adapter MUST add exactly `--disable apps` ahead of requested
   arguments. All requested arguments MUST retain their order and content.
4. The Tact adapter and native escape hatch MUST remain unchanged.
5. The transform MUST apply equally to explicit `fortlet run codex -- ...` and
   transparent Codex-shim launches, including interactive and non-interactive
   attachment.

### Configuration and persistence

1. Fortlet MUST NOT create, overwrite, merge, parse, or persist Codex
   `config.toml` for this capability.
2. Fortlet MUST NOT delete user MCP registrations, caches, plugins, skills, or
   other persistent Codex state.
3. Apps disablement MUST target upstream feature key `apps`, which controls the
   reserved `codex_apps` entry. It MUST NOT disable the general MCP subsystem.
4. A user registration named `codex_apps` is not preserved because upstream
   reserves and filters that name. Other user-configured server names MUST
   remain governed by Codex's ordinary configuration.

### Credential boundary

1. FIP-0008's model access token, account identifier, refresh lifecycle, and
   destination-bound substitution MUST remain unchanged.
2. Fortlet MUST NOT read or project a browser cookie, ChatGPT session cookie,
   raw Apps biscuit, `CODEX_CONNECTORS_TOKEN`, host Codex config, or a new MCP
   credential class.
3. The known Apps 451 MUST NOT be recast as a model login failure or corrected
   by asking the user to repeat `codex login`.
4. No Apps network request SHOULD occur from a managed launch while the feature
   is disabled.

### Compatibility and failure behavior

1. The fixed argument is pinned to Codex 0.147.0. A Codex adapter update MUST
   verify the feature key, CLI flag, precedence, and reserved-server filtering
   before publication.
2. If the pinned executable rejects the fixed argument, launch MUST fail at the
   terminal stage with the existing actionable correction. Fortlet MUST NOT
   retry without the disable or silently launch an unsupported Apps client.
3. Native Codex MAY expose Apps according to its own host configuration because
   it is the explicit unisolated escape hatch.

### Evidence

1. Unit evidence MUST prove Codex adds exactly the fixed pair, preserves all
   requested arguments, Tact remains unchanged, and runtime attachment uses
   adapter-effective arguments.
2. Existing FIP-0008 projection and credential tests MUST remain green.
3. A version-pinned compatibility check MUST prove the installed stock CLI
   accepts the fixed flag without reading provider credentials.
4. Experiment 0024 MUST launch packaged Codex through Fortlet, observe no
   `codex_apps` HTTP 451 or MCP-startup-incomplete warning, complete one ordinary
   model prompt, and close with public stop/reset evidence. It MUST not inspect
   credential content or invoke an App.

## Consequences

Managed daily startup becomes quiet and accurately represents Fortlet's
supported capability surface. The change is ephemeral, reversible, and
isolated to the pinned adapter. Existing persistent Codex state and configured
MCP servers are not rewritten.

Apps are unavailable inside Fortlet even if the user has a working native Apps
session. This is preferable to presenting a known-broken client or guessing at
an undisclosed credential. Supporting Apps later remains possible without
undoing persistent state because this FIP writes none.

The currently installed Spreadsheets plugin's live Microsoft Excel control and
the Sites plugin's hosting workflow depend on Apps and are therefore unavailable
inside Fortlet. Local spreadsheet-file work and local site development remain
available, as do skill-only plugins and independently configured MCP servers.

## Alternatives Considered

1. Copy browser or ChatGPT cookies into the guest. Rejected because no pinned
   client contract requires or bounds them, and they would expand guest account
   authority beyond FIP-0008.
2. Broker `CODEX_CONNECTORS_TOKEN`. Rejected because upstream labels it a debug
   override and supplies no supported source, minting, renewal, or revocation
   contract for Fortlet.
3. Reuse the model access token explicitly. Rejected because Codex already does
   so on the normal path and the observed service still returns HTTP 451.
4. Disable all MCP servers. Rejected because the defect is the reserved Apps
   client and user-configured MCP capability must remain intact.
5. Write `[features] apps = false` into persistent `config.toml`. Rejected
   because Fortlet need not own or merge user configuration when a supported
   launch-local flag exists.
6. Ignore or suppress the warning text. Rejected because the failed request and
   unavailable tools would remain, and filtering terminal output would damage
   native UX.
7. Add iron-proxy, Agent Proxy, or a Codex fork. Rejected as unnecessary for a
   targeted upstream feature toggle and outside this GOAL.

## Open Questions

1. Whether a future Codex release will publish a stable external Apps
   authorization contract suitable for a host-owned Fortlet broker.
2. Whether future daily use needs an explicit Fortlet option that fails when
   Apps is requested but unsupported, rather than keeping it disabled.
