# Session Handoff — Goal-scoped publication autonomy

Audience: a fresh agent session. `GOAL.md` is normative and active. Read
FIP-0001 and FIP-0005 in full before implementation. Experiments 0001 through
0023 are terminally closed; there is no active experiment.

## Verified landed baseline

PR #5 squash-merged the first daily-session GOAL to protected `main` as
`bee396569bd7aa047905c435d219ce28f3d49b64`. Its tree is byte-identical to
reviewed signed tip `51e531a9dd74c543895f85d477e62b3a9e11679e`.
The hosted Rust gate passed after one diagnosed Linux repair, the goal branch
was removed locally and remotely, and the working copy began this GOAL as an
empty change directly on fetched `main`.

Fortlet's daily product path is proven: explicit credential-free `prepare`,
the optional Codex shim, host-owned renewable ChatGPT credentials, immutable
project Cargo and Jujutsu tools, one real model-backed edit/test loop, and
public stop/reset all succeeded. FIP-0007 and FIP-0008 are conformant. MCP
biscuit authorization remains a separate future product concern.

## Recorded workflow blocker

FIP-0005's first amendment reduced publication approval to one exact packet,
but any changed diff, additional push, or failed hosted run invalidated that
authority. PR #5 consequently required a second full packet for a small,
well-diagnosed Linux inode-reuse repair. The operator identified that ceremony
as disproportionate for a greenfield repository with no users, protected
`main`, required checks, and manual merge.

## Accepted contract

The 2026-08-19 amendments to FIP-0001, FIP-0005, and the charter make GOAL
acceptance the sole routine publication authorization. One goal branch and
draft PR may iterate through in-scope pushes, diagnosed CI fixes, replacement
runs, PR updates, readiness, landing verification, and exact bookmark cleanup
without more approval. Only the final branch tip requires a verified
signature. The operator remains the sole merge authority.

Standing authority does not cover a changed GOAL, another PR, a different base
or repository, `main` mutation, settings, releases, tags, packages, secrets,
external spend, destructive actions, or unrelated resources. Two failures
sharing an unresolved cause still stop under the charter.

## What to do next

1. Checkpoint the accepted amendments before changing operational rules.
2. Replace packet language consistently in `AGENTS.md`, `WORKFLOW.md`, and the
   runbook; update publication tests and conformance in the same checkpoint.
3. Run the complete local gate, sign only the publishable tip, then exercise
   the delegated workflow on one branch and draft PR. Mark it ready after the
   hosted gate succeeds; the operator squash-merges manually.

Do not change product code, hosted CI, branch protection, merge settings,
experiment mechanics, release policy, secrets, or the operator merge boundary.
