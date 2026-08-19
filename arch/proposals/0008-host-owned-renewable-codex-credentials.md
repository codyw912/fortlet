# FIP-0008: Host-owned renewable Codex credentials

Status: Draft
Recorded: 2026-08-18 from the credential-refresh blocker in Experiment 0021
Requires: FIP-0001, FIP-0002

## Summary

Keep Codex ChatGPT access and refresh credentials on the host, renew them
through one Codex-specific Fortlet lease, and rotate only the access token in
MicroSandbox's existing live secret broker. The guest receives a persistent
external-token metadata projection containing broker placeholders and no usable
refresh credential. A Fortlet invocation maintains renewal only while it owns
an active harness attachment or command.

## Motivation

Experiment 0021 reached the packaged Codex UI but failed before model work.
Fortlet projected a broker placeholder as the access token and a deliberately
fake refresh token. Codex treated the projection as managed ChatGPT OAuth,
attempted its own refresh, and rejected the fake credential. Static
request-time replacement therefore establishes secret containment but not a
renewable session.

The missing behavior must not be fixed by exposing the host refresh token to
the guest. It also should not replace the native Codex terminal with a new UI,
require a second provider account, or introduce an always-running service just
to keep an inactive capsule's token current.

The relevant implementation boundaries were verified against Codex 0.147.0
and MicroSandbox 0.6.8 before this proposal was drafted:

1. Codex defines `chatgptAuthTokens` for ChatGPT tokens supplied by an external
   host and gives unauthorized recovery to an external authentication provider
   instead of its managed refresh-token path. Its app-server bridge exists, but
   the protocol labels that login mode unstable and for OpenAI internal use
   only.
2. Codex's `headers` authentication cannot be loaded by the stock CLI from
   `auth.json`, so it is not a native-TUI configuration surface.
3. MicroSandbox already performs destination-bound TLS secret substitution and
   supports live rotation of an existing secret without changing its
   guest-visible placeholder.
4. iron-proxy supplies a strong general egress boundary, rotating file-backed
   secret sources, and an externally authorized one-replay response handler.
   It does not own OAuth refresh. Using it here would add a second TLS proxy,
   CA distribution, enforced routing, process lifecycle, and another secret
   configuration around capabilities already present in MicroSandbox.

Sources checked for this decision:

- [Codex 0.147.0 authentication modes](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/protocol/src/auth.rs)
- [Codex 0.147.0 authentication manager](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/login/src/auth/manager.rs)
- [Codex 0.147.0 external-token protocol](https://github.com/openai/codex/blob/rust-v0.147.0/codex-rs/app-server-protocol/src/protocol/v2/account.rs)
- [iron-proxy](https://github.com/paradigmxyz/iron-proxy)

## Decision

Extend the Codex adapter with a host-owned credential lease. Before a Codex
launch can create, start, or attach to a capsule, the lease loads the current
file-backed ChatGPT credential and renews it when it cannot cover a bounded
safety window. While the Fortlet process is running a Codex command, the lease
renews before expiry and rotates the access token and account identifier through
MicroSandbox's existing live secret modification API.

The guest projection uses Codex 0.147.0's external ChatGPT-token storage shape.
It contains an opaque broker placeholder as its access token, an empty refresh
field only where required by Codex's serialized schema, the brokered account
identifier, and non-secret metadata required to load the record. It contains no
host access token, host refresh token, API key, personal access token, agent
identity, cookie, or MCP authorization.

This is deliberately Codex-specific. It does not introduce a general OAuth
framework or change Tact credential behavior. iron-proxy remains a credible
future replacement if MicroSandbox cannot provide the required live rotation
or if a recorded failure establishes a need for response-triggered exact
replay, broader egress policy, or cross-runtime secret providers.

## Specification

### Host credential source

1. Fortlet MUST support only the existing file-backed Codex ChatGPT login for
   this contract. An absent, unreadable, oversized, non-regular, symlinked,
   malformed, non-ChatGPT, or incomplete source MUST fail before capsule
   creation, start, or attachment with one `codex login` correction.
2. The source MUST contain a non-empty access token, refresh token, and account
   identifier. Fortlet MUST validate the access-token expiry and the account
   binding without printing credential material.
3. The source path MUST remain outside every guest mount. The source document,
   refresh token, ID token, and other durable provider material MUST NOT enter
   a capsule, broker configuration, process environment inherited by the
   harness, diagnostic, trace, or persistent Fortlet harness state.
4. Keyring-backed Codex authentication, API keys, personal access tokens,
   agent identities, and provider-generic OAuth are outside this initial
   contract and MUST fail with an actionable unsupported-source message rather
   than silently changing authentication modes.

### Renewal and host persistence

1. Before launch, Fortlet MUST hold a source-scoped exclusive renewal lock,
   reread the credential beneath that lock, and use the existing access token
   only when it remains valid beyond a declared safety window.
2. If renewal is required, Fortlet MUST use Codex-compatible OAuth refresh
   parameters and the configured Codex refresh endpoint. It MUST bound connect,
   response, and total request time and MUST NOT retry silently.
3. A successful response MUST contain a valid access token, refresh token, and
   account binding matching the locked source. Fortlet MUST reject missing,
   malformed, expired, or account-changing responses before modifying either
   host storage or broker state.
4. Host persistence MUST preserve unrelated source fields, use a mode-0600
   owned temporary regular file, sync the complete replacement, compare the
   source identity observed under the lock, and atomically rename over the
   original file. Failure before rename MUST leave the original source intact.
5. Concurrent Fortlet invocations MUST converge on the newest source beneath
   the same lock. A waiter MUST reread after acquiring the lock and MUST NOT
   reuse the refresh token from an earlier snapshot.
6. A refresh failure MUST leave the host source, guest projection, existing
   capsule, and last brokered value unchanged. Initial renewal failure MUST
   prevent harness start. Fortlet MUST report whether login is required or a
   bounded transport failure occurred without including response bodies or
   token-derived identifiers.

### Guest projection and broker boundary

1. The Codex projection MUST use the pinned harness version's external
   ChatGPT-token mode (`chatgptAuthTokens` for Codex 0.147.0), not managed
   `chatgpt` mode.
2. The projection MUST contain only broker placeholders for the access token
   and account identifier, an empty refresh field if the pinned schema requires
   that field, and the minimum non-secret metadata needed for Codex to load the
   record. It MUST NOT contain a usable provider credential.
3. Projection creation MUST remain atomic, mode 0600, symlink-safe, and
   persistent in the adapter-owned harness home. Login run inside a capsule
   MUST NOT be the supported way to populate or renew it.
4. MicroSandbox MUST substitute the current host access token and account
   identifier only for the declared OpenAI and ChatGPT TLS destinations. A
   placeholder sent to any other destination MUST retain the existing
   fail-closed violation behavior.
5. The guest-visible placeholders and projection path MUST remain stable across
   access-token rotation. Renewal MUST NOT require rewriting the projection,
   restarting Codex, or exposing a second token to the guest.
6. The host refresh token MUST never be registered as a MicroSandbox secret.

### Active credential lease

1. Every Codex `run` or shim invocation MUST create one renewal lease after
   project and immutable-layer validation and before runtime reconciliation.
   `prepare`, `doctor`, `native`, `status`, `stop`, and `reset` MUST NOT start a
   lease or perform network renewal.
2. The lease lifetime MUST be bounded by the Fortlet process that owns the
   attached interactive or non-interactive harness command. It MUST NOT create
   an always-running daemon, service, capsule workload, or renewal task for an
   inactive command.
3. The lease MUST schedule renewal before the brokered access token reaches the
   safety window. It MUST reread beneath the source lock before refreshing so a
   token renewed by another process is reused.
4. After successful host renewal, the lease MUST rotate the existing access
   token and account identifier with MicroSandbox's live secret modification
   path while preserving placeholders and destination restrictions. It MUST
   verify that the runtime reports the changes live; a restart-backed or
   unsupported disposition MUST be treated as failure.
5. Concurrent leases for the same capsule MUST be idempotent. They MAY each
   apply the same current secret after converging on the shared host source,
   but MUST NOT produce competing refresh requests from the same source
   snapshot.
6. If in-session renewal or live rotation fails, Fortlet MUST NOT place a real
   credential in the projection, fall back to unbrokered egress, restart a
   shared capsule silently, or hide the failure. The current command MAY
   continue only while the already-brokered token remains outside the safety
   window; otherwise Fortlet MUST return a nonzero credential-stage failure
   with login or retry guidance.
7. Normal harness exit MUST cancel and join the lease without delaying terminal
   exit by more than a short bounded cleanup interval. Lease cancellation MUST
   NOT stop or reset the reusable capsule.

### Compatibility and evidence

1. The implementation MUST pin projection compatibility to each supported
   Codex adapter version. An adapter update MUST prove that its external-token
   record loads in the stock native TUI before publication.
2. Automated evidence MUST cover source validation, expiry scheduling, no-op
   reuse, successful rotation, host-file atomicity, account mismatch, malformed
   responses, lock convergence, single refresh, live-modification disposition,
   cancellation, and redacted failures.
3. Isolated CLI evidence MUST prove every non-launch command remains free of
   renewal network activity and that invalid credentials fail before capsule
   mutation.
4. A compatibility fixture MUST start the pinned stock Codex CLI with a
   placeholder-only external-token projection and prove it reaches the first
   backend request without attempting the OAuth refresh endpoint. The fixture
   MUST use a fake local backend and no provider credential or paid request.
5. One new predeclared experiment MUST exercise an ordinary prompt and edit/test
   loop through the packaged shim. It MUST NOT resume Experiment 0021 or retry
   silently. MCP biscuit authorization MUST be recorded separately and MUST
   NOT be treated as evidence for this credential contract.

## Consequences

Codex can keep its native terminal while Fortlet owns the lifecycle of the
host login and MicroSandbox remains the only guest egress secret boundary. A
long-running command can receive a rotated access token without a capsule
restart, and an inactive reusable capsule requires no background process.

Fortlet now mutates the user's host Codex login during a successful refresh.
That is a narrower but more consequential authority than the current read-only
credential loader, so atomic replacement, locking, field preservation, and
failure redaction become part of the product contract. File-backed login is
the only first implementation; keyring support must be designed separately.

The projection depends on a pinned Codex compatibility surface that upstream
labels external and, in its app-server protocol, unstable. Version-specific
fixtures make that dependency explicit. If the native CLI stops loading the
external-token record, Fortlet must fail its adapter compatibility gate rather
than reverting to a fake managed refresh token.

## Alternatives Considered

1. Put the real refresh token in the guest projection. Rejected because a
   compromised agent or project could read and exfiltrate a durable provider
   credential, violating the charter.
2. Keep static MicroSandbox substitution and require periodic native login.
   Rejected because Experiment 0021 proved that it cannot support an ordinary
   prompt reliably and turns provider expiry into unexplained guest failure.
3. Add iron-proxy plus a trusted response-retry handler now. Deferred because
   it still needs a host OAuth owner and would duplicate MicroSandbox's TLS
   substitution while adding CA, routing, configuration, and process lifecycle.
   Its exact one-replay boundary is preferable if proactive live rotation later
   proves insufficient.
4. Drive Codex's external authentication through `codex app-server`. Rejected
   for the native CLI because it requires Fortlet to become an app-server
   client and terminal UI; the relevant 0.147.0 protocol is explicitly
   unstable and for internal use.
5. Use Codex `headers` authentication. Rejected because the stock CLI refuses
   to load that mode from `auth.json`.
6. Use `CODEX_ACCESS_TOKEN`, a personal access token, agent identity, or an API
   key. Rejected as the default because those are different credential and
   billing contracts and do not renew the user's existing ChatGPT OAuth login.
7. Refresh only at launch and restart the capsule when the token changes.
   Rejected because an interactive session can outlive an access token and a
   silent restart would disrupt concurrent attachments.

## Open Questions

1. Whether Codex will expose a stable external-auth surface for the native CLI,
   eliminating the version-specific projection contract.
2. Whether a later runtime needs response-triggered replay in addition to
   proactive renewal, justifying iron-proxy or an equivalent upstream boundary.
3. Whether keyring-backed Codex storage can be supported without expanding
   Fortlet into a general credential-store abstraction.
4. What separate capability should supply the `codex_apps` MCP biscuit without
   coupling MCP authorization to model authentication.
