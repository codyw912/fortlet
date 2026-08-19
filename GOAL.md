# GOAL: Make managed non-interactive Codex execution reliable

Status: complete — 2026-08-19, pending operator merge of PR #8. FIP-0010 is
accepted and conformant. Managed non-interactive execution now streams output,
closes stdin explicitly, preserves exact exit status, and gives Codex a fixed
600-second output-inactivity ceiling with bounded command-only cleanup.

Experiment 0028 consumed the one authorized model-backed successor and exposed
that MicroSandbox 0.6.8 streaming null stdin does not send guest EOF. It
terminally failed at the new supported bound and was not retried. The in-scope
explicit-EOF correction then passed the immutable packaged, credential-free
Codex regression in Experiment 0029. Post-correction model success remains
deliberately unclaimed.

Determine whether the non-interactive failure belongs to Fortlet,
MicroSandbox 0.6.8, or Codex 0.147.0. Make the existing managed non-interactive
surface complete with correct output and exit status, or fail within an
explicit supported bound with one actionable correction when Fortlet cannot
make the upstream path reliable.

Before implementation, read FIP-0001, FIP-0002, FIP-0005, FIP-0008, and
FIP-0009 in full.

## Deliverable 0 — Reproduce and attribute the hang

1. Preserve Experiment 0024 as terminal evidence; do not retry it or infer a
   cause from silence.
2. Reproduce with credential-free local fixtures before any provider request.
   Prefer the smallest faithful Fortlet, MicroSandbox, and stock-Codex controls
   that separate command completion from model authentication.
3. Inspect Fortlet's non-interactive attachment and renewal selection,
   MicroSandbox 0.6.8 exec completion semantics, and Codex 0.147.0 `exec`
   shutdown behavior from their pinned sources.
4. Determine whether the harness process remains alive, exits without an SDK
   event, or is kept open by Fortlet. Record observed facts separately from
   inference and retain exact commands and outcomes.

## Deliverable 1 — Fix only the owned boundary

1. If the defect is inside Fortlet's accepted adapter, runtime attachment, or
   credential-lease contracts, implement the smallest correction under the
   existing FIPs.
2. Preserve stdout, stderr, requested arguments, exit status, cancellation,
   current directory, and credential renewal for both interactive and
   non-interactive launches.
3. If diagnosis instead requires a new public timeout/configuration contract or
   changes workload-lease semantics, draft FIP-0010 and obtain operator
   acceptance before production implementation.
4. If the pinned upstream path cannot be made reliable within the accepted
   boundary, fail closed within a justified bound and report one actionable
   correction. Do not silently fall back to native Codex or hide output.

## Deliverable 2 — Establish deterministic evidence

1. Add a credential-free regression fixture that would fail for the observed
   hang and proves bounded completion, output forwarding, and exact exit status.
2. Cover the credential-renewal task's relationship to command completion and
   cancellation without reading, printing, or contacting provider credentials.
3. Keep interactive Codex, Tact, Apps disablement, capsule lifecycle commands,
   persistent state, and user-configured MCP behavior unchanged.
4. Run the pinned stock-Codex compatibility fixtures when diagnosis or code
   touches their boundary.

## Deliverable 3 — Verify and publish once

1. After deterministic evidence passes, declare one bounded successor
   experiment for exactly one short model-backed non-interactive prompt.
2. Require the prompt response, clean command termination, correct exit status,
   no Apps warning, and public stop/reset cleanup. Do not retry a failed unit.
3. Update conformance, runbook, GOAL, and handoff with the exact supported
   surface and any remaining upstream limitation.
4. Run the complete standard verification set through `nix develop`, then use
   one goal bookmark and pull request under FIP-0005. The operator performs the
   squash merge.

## Definition of Done

1. `fortlet run codex -- exec ...` no longer waits indefinitely after an
   otherwise completed or failed non-interactive request.
2. Successful output and the harness exit status reach the host exactly; a
   supported failure is bounded and gives one actionable correction.
3. Interactive Codex, Tact, Apps disablement, model authentication/renewal, and
   public lifecycle behavior retain their existing evidence.
4. No credential content, browser state, Apps authorization, host Codex config,
   or new secret class becomes guest-readable or enters test output.
5. Deterministic and packaged evidence passes, FIP conformance is honest, the
   local and hosted gates pass, and the PR is ready for operator merge.

## Excluded scope

Do not add general logs, restart, global inventory, cross-project cleanup,
standalone installation, a Codex fork, an alternative runtime, remote
execution, a proxy, a new credential class, hosted CI changes, repository
settings, release, tag, or package publication.

## Budget and escalation

Engineering ceiling: two hours. External money and paid quota remain zero
beyond one short operator-authorized model-backed non-interactive prompt after
deterministic evidence passes. This accepted GOAL authorizes one descriptive
bookmark and one draft PR targeting `main` under FIP-0005. Stop for FIP-0010
acceptance, material scope expansion, a second provider prompt, destructive or
unrelated mutation, merge, any credential or data-boundary anomaly, or two
failures sharing an unresolved assumption.

## Verification

Run focused source and regression controls during diagnosis. Before live
dispatch and readiness, run the complete standard verification set from
`docs/RUNBOOK.md` inside `nix develop` plus any pinned compatibility command
declared by the accepted design.
