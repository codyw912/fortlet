# Session Handoff — Warning-free daily Codex baseline

Audience: a fresh agent session. The Codex Apps capability GOAL is complete;
choose and obtain operator acceptance for a successor GOAL before product work.
Experiments 0001 through 0025 are terminally closed, and no experiment is
active.

## Verified product baseline

Fortlet's daily Codex path now includes optional explicit preparation,
transparent shims, host-owned renewable ChatGPT credentials, immutable project
tools, ordinary interactive model work, warning-free startup, and public
status/stop/reset. FIP-0007, FIP-0008, and FIP-0009 are conformant.

Managed Codex 0.147.0 prepends `--disable apps` through the harness adapter at
the shared runtime attachment boundary. The fixed disable is authoritative over
user attempts to re-enable Apps, targets only the upstream-reserved
`codex_apps` registration, and preserves requested arguments and independently
configured MCP servers. Tact and `fortlet native codex` are unchanged. Fortlet
does not write or inspect host Codex configuration and introduces no Apps
credential.

The practical limitation is narrow and explicit: connector-backed live
Microsoft Excel control and Sites hosting are unavailable inside managed
sessions. Local spreadsheet-file work, local site development, skill-only
plugins, and direct MCP servers remain available. The operator confirmed those
connector workflows are not part of current daily use; native Codex remains the
escape hatch if that changes.

## Evidence and publication state

The complete local gate passed through `nix develop` on aarch64-darwin: 72 unit
tests, 29 non-ignored integration tests, formatting, strict all-target/all-
feature Clippy, conformance, and `nix flake check`. Both ignored stock-Codex
0.147.0 fixtures passed separately against the pinned native binary. Native
x86_64-linux package verification remains outstanding.

Experiment 0024 is terminally rejected: its one non-interactive `codex exec`
attempt emitted no Apps warning but produced no model response before the
ten-minute bound. Public cleanup succeeded and it was not retried. Experiment
0025 is terminally accepted: the packaged interactive session started without
the Apps warning, returned exactly `fortlet-apps-disabled-ok` for its sole
prompt, exited normally on one idle Ctrl-C, and completed public stop/reset
cleanup. The non-interactive timeout's cause remains unresolved; interactive
daily use is the proven product path.

PR #7 is the sole authorized goal pull request on bookmark
`codex-apps-capability`. Its initial hosted Rust verification passed on reviewed
revision `07e82b7d410864ed3057ce4e00d3f8cf4c1fd23e`. Verify the final check and
PR state rather than relying on this snapshot. The operator is the sole merge
authority. After merge, fetch `main`, prove tree equality with the reviewed
tip, and remove only this goal's local and remote bookmark.

## What to do next

If PR #7 has not merged, finish only its accepted FIP-0005 publication closure.
If it has merged, verify and clean the exact bookmark, then discuss the next
GOAL with the operator. Product-facing candidates already identified in prior
planning include global inventory/safe cross-project cleanup, restart/logs,
explicit workload leases, and eventually standalone distribution. Select one
based on the operator's current daily-use friction rather than speculative
architecture.

Do not resume Experiments 0024 or 0025, re-enable Apps inside Fortlet, infer a
biscuit protocol, project cookies or host config, retry the non-interactive
model unit silently, merge PR #7, mutate repository settings, or begin a new
GOAL without operator acceptance.
