# Session Handoff — Codex Apps capability handling

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001, FIP-0005, and FIP-0008 in full before design or implementation.
Experiments 0001 through 0023 are terminally closed; there is no active
experiment.

## Verified landed baseline

PR #6 squash-merged the delegated-publication GOAL to protected `main` as
`118671e0f744b4168de86bcc4c6f326c3d89a768`. Its tree is byte-identical to the
reviewed signed tip `a7c0d0a63a9bed5e3802e70b7ed5eb0578b13d17`.
Hosted Rust verification passed, the exact goal bookmark was removed locally
and remotely, and the working copy began this GOAL as an empty change directly
on fetched `main`.

GOAL acceptance now grants standing authority for one branch and PR through
ordinary pushes, diagnosed CI repairs, evidence updates, readiness, landing
verification, and exact bookmark cleanup. Only the final publishable tip must
be signed. The operator remains the sole merge authority.

## Product baseline

Fortlet's daily Codex path is proven: explicit credential-free `prepare`, the
optional shim, host-owned renewable ChatGPT credentials, immutable project
Cargo and Jujutsu tools, one real model-backed edit/test loop, and public
stop/reset all succeeded. FIP-0007 and FIP-0008 are conformant.

The remaining visible startup defect is separate from model authentication.
Codex 0.147.0 starts the built-in `codex_apps` MCP client, which returns HTTP
451 `no_biscuit_no_service` because Fortlet supplies no Apps biscuit. The model
request itself succeeds after the FIP-0008 bearer-projection repair. FIP-0008
explicitly excludes cookies and MCP authorization and leaves the separate
capability as an open question.

## Accepted direction

The new GOAL is source-first. Verify how pinned Codex registers, authorizes,
and disables `codex_apps`; then draft FIP-0009. If no narrow supported external
authorization surface exists, prefer disabling only the unavailable built-in
Apps client. Preserve user-configured MCP servers and the existing model-token
boundary. Do not project raw cookies, biscuits, host configuration, or a
general MCP credential into the guest.

The two-hour ceiling includes research, design, the smallest implementation,
deterministic evidence, one bounded packaged startup check, and the normal
single-PR publication lifecycle. FIP-0009 acceptance is the one required design
stop before implementation.

## What to do next

The source audit is complete and FIP-0009 is proposed. Codex feature `apps` is
stable and default-on; `--disable apps` is a supported global CLI override.
The effective MCP map removes only reserved server `codex_apps` when disabled.
`CODEX_CONNECTORS_TOKEN` is explicitly a debug override, while normal Apps
startup reuses the ChatGPT auth provider. The open client contains no external
biscuit acquisition or renewal contract.

The recommended implementation adds an adapter-owned launch-argument transform:
Codex 0.147.0 prepends `--disable apps`, Tact and native execution remain
unchanged, and runtime attachment consumes the transformed arguments without a
harness-name branch. No persistent config or credential changes are proposed.

Next: obtain operator acceptance of FIP-0009. Then implement its smallest slice,
add deterministic evidence, declare Experiment 0024, and run the packaged check.

Do not resume a closed experiment, inspect credential contents, change model
authentication, disable all MCP servers, fork Codex, introduce a proxy, mutate
repository settings, merge, or expand beyond the accepted GOAL.
