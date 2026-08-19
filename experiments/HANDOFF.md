# Session Handoff — Non-interactive Codex reliability

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001, FIP-0002, FIP-0005, FIP-0008, and FIP-0009 in full before production
implementation. Experiments 0001 through 0025 are terminally closed; there is
no active experiment.

## Verified landed baseline

PR #7 squash-merged the Codex Apps capability GOAL to protected `main` as
`f05da5ec8274e5905d8baf81fe61c48c8e76ecd4`. Its tree is byte-identical to the
signed reviewed tip `5739490942546a930975d51171ac43d112b4d32f`. Hosted Rust
verification passed, the exact `codex-apps-capability` bookmark was removed
locally and remotely, and the new GOAL begins from an empty working-copy change
directly above fetched `main`.

The complete merged product baseline includes optional preparation and shims,
host-owned renewable ChatGPT credentials, immutable project tools, public
status/stop/reset, and adapter-owned Apps disablement. FIP-0007, FIP-0008, and
FIP-0009 are conformant. Managed interactive Codex 0.147.0 starts without the
known Apps warning and completes ordinary model work. Tact and native Codex are
unchanged.

## Observed defect

Experiment 0024 is the exact failure evidence and must not be resumed. The
packaged command was:

```text
fortlet run codex --project /Users/cody/dev/fortlet -- exec --ephemeral --skip-git-repo-check "Return exactly fortlet-apps-disabled-ok without using tools."
```

It emitted no output or startup diagnostic for ten minutes. Public status then
reported the owned capsule running. Public stop caused the waiting host command
to fail at the terminal stage because the runtime exec session ended without an
exit event; reset restored absence and MicroSandbox reported no sandboxes.
Whether a provider request occurred is unknown.

Experiment 0025 changed only to interactive attachment. Startup was warning-
free, its sole prompt returned exactly `fortlet-apps-disabled-ok`, one idle
Ctrl-C exited normally, and public lifecycle cleanup succeeded. This proves the
model credential and Apps boundaries but does not explain Experiment 0024.

## Accepted direction

Diagnose source-first with credential-free controls. Separate Fortlet's
`attach_codex_with_lease` selection, MicroSandbox 0.6.8 non-interactive exec
completion, and stock Codex 0.147.0 `exec` shutdown. Determine whether the guest
process remains alive, exits without an SDK exit event, or is held open by the
host lease task.

Fix the smallest Fortlet-owned defect under accepted FIPs. A new public
timeout/configuration contract or workload-lease semantic requires proposed and
operator-accepted FIP-0010 before implementation. Interactive behavior,
credentials, Apps handling, Tact, lifecycle commands, and native escape hatches
remain frozen.

## What to do next

Verify the clean main baseline and conformance, then inspect the three pinned
source paths and construct the smallest credential-free reproduction. Do not
make a provider request until deterministic regression evidence and the full
pre-dispatch gate pass. The accepted GOAL permits exactly one final short
model-backed non-interactive prompt and one goal-scoped PR; the operator remains
the sole merge authority.

Do not resume Experiment 0024 or 0025, inspect session history or credential
content, retry the provider prompt during diagnosis, add an implicit timeout,
fall back to native Codex, introduce general logs/restart/inventory, or expand
beyond the accepted GOAL.
