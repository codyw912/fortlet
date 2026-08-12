# Session Handoff — Public alpha repository is established

Audience: a fresh agent session. `GOAL.md` is normative and complete. No
experiment is active after the terminal publication push; Experiments 0001
through 0011 are terminally closed.

## Verified result

1. The complete Fortlet history is public at
   `https://github.com/codyw912/fortlet`, with SSH origin
   `git@github.com:codyw912/fortlet.git` and default branch `main`.
2. The repository readiness audit found dual MIT/Apache-2.0 licensing, GitHub
   no-reply authorship, no credential/private-key patterns in the tree or
   history, and only operator-accepted personal paths in retained experiment
   evidence.
3. Publication preserved the full decision and negative-experiment history.
   Jujutsu added SSH signatures to previously unsigned commits on initial
   push; Experiment 0011 maps every cited pre-signing checkpoint ID to its
   signed public object ID.
4. Initial publication produced exact equality among local `main`,
   `main@origin`, and GitHub `refs/heads/main`. Only `main` existed; GitHub
   reported zero releases, Actions workflows, webhooks, and deployments.
5. No CI, release, package, issue, tag, secret, deploy key, repository policy,
   or other remote resource was added. Those require separate authorization.

## Product state

FIP-0003 is accepted and conformant. Implemented daily-use surfaces are
`doctor`, explicit `run`, optional package-owned `codex` and `tact` shims,
explicit `native`, project-scoped `status`, and project-scoped `stop`.
Reusable project+harness capsules, immutable base and harness layers, brokered
ChatGPT credentials, safe project resolution, broad-root protection, and
deterministic pre-runtime diagnostics are established.

Experiment 0010 proved one immutable public-CLI capsule path from absent to
running to stopped to absent with exact ownership verification and complete
cleanup. The full standard gate passed again immediately before public
publication.

## Remaining gaps

FIP-0001 remains partial. Exact gaps are harness-owned persistent paths and
credential policy; live capsule-reconciliation and terminal-attachment failure
evidence; capsule topology, concurrency, and terminal coverage; restart, logs,
and tool-update commands; explicit interactive/background leases; declarative
project environments and private overlays; host-side publication; standalone
non-Nix installation; and native `x86_64-linux` package verification. FIP-0002
retains the documented automated Codex exit-status evidence gap.

## Successor boundary

The operator selected project-scoped reset/recovery as the next product
direction. Design it under a new accepted FIP before implementation. The
smallest likely contract is an owned, project+harness-scoped reset that refuses
active capsules, removes only disposable MicroSandbox state, preserves project
files and persistent harness state, and replaces raw `msb` recovery guidance.
Do not infer global inventory, state purging, restart, logs, leases, CI,
distribution, or additional GitHub mutation as part of that goal.
