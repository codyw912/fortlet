# Session Handoff — Codex daily work closure active

Audience: a fresh agent session. `GOAL.md` is normative and active. Verify the
merged `main`, clean Jujutsu working copy, complete gate, and conformance before
relying on this summary. Read FIP-0001 through FIP-0011 in full before
implementation or experiment dispatch.

## Verified baseline

The operator squash-merged PR #10 as
`bca9438b2bb97b8f79728627b687e8d1ccc943d3`. Its tree exactly matched reviewed
signed tip `47c6e9575a7cd9d30dc081c7e48a6bf22ac0c81c`; the
`capsule-inventory` bookmark was removed locally and remotely. The working copy
then began clean directly above merged `main`.

On 2026-08-19 the complete `docs/RUNBOOK.md` gate passed through `nix develop`
on aarch64-darwin: all 88 unit tests and every enabled integration test,
formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check`. Nix emitted only the known incompatible `x86_64-linux`
omission notice. FIP-0001 and FIP-0002 remain partial with explicit gaps;
FIP-0003 through FIP-0011 are conformant.

## Product state

Fortlet now resolves safe project roots, supplies immutable project tools,
optionally prepares layers, launches Codex and Tact explicitly or through
package-owned shims, renews file-backed host Codex credentials, suppresses only
the unsupported Apps client, streams non-interactive output with explicit EOF
and a bounded Codex inactivity ceiling, and exposes project lifecycle controls
plus global owned-capsule inventory.

The remaining Codex uncertainty is composed evidence, not a known missing
feature. Experiment 0030's sole provider-backed non-interactive unit streamed,
edited correctly, and exited normally, but Codex's `bash -lc` could not find
Cargo. Experiment 0031 proved Debian's login profile dropped the managed PATH
and that a process-scoped `BASH_ENV` restored it. Experiment 0032 then observed
the production hook expose Cargo 1.97.1 and pass the exact focused guest test;
that experiment was rejected only because undeclared public crate downloads
occurred and a planned read-only inspection was omitted before cleanup. No
provider-backed work unit has run after this correction.

Earlier accepted evidence remains complementary: Experiment 0023 completed an
interactive packaged shim edit/test loop with brokered model authentication;
Experiment 0025 proved quiet interactive Apps disablement; Experiments 0027 and
0029 proved packaged streaming, exact exit behavior, and explicit EOF without
provider credentials. Experiment 0033 proved the global inventory and public
cleanup path.

## Selected direction

The operator accepted a two-hour evidence-first goal: complete one useful,
bounded repository edit/test task through an immutable package's transparent
Codex shim in non-interactive mode. Passing without product code is success. A
failure is terminal for its experiment; only a demonstrated Fortlet defect may
receive the smallest deterministic correction and at most one newly declared
successor model unit.

This closure comes before workload leases, missing-root cleanup, standalone
installation, or another harness. If it passes, select the first new harness
from a concrete OMP, Pi, or Claude Code use case rather than generalizing the
adapter contract speculatively.

## Next action

Create the single `codex-work-readiness` bookmark and draft pull request. Then
predeclare Experiment 0034 with an exact useful maintenance task, frozen prompt,
permitted paths, guest command, immutable revision/package, clean baseline,
provider-process bound, and public cleanup. Do not dispatch until the complete
preflight is recorded. If Codex nesting markers are present in the active agent
environment, have the operator run the one exact command from a marker-free
outer shell.

Do not resume or retry Experiments 0030–0033; add a harness; add leases, global
mutation, Apps credentials, standalone packaging, Linux CI, remote execution,
or unrelated product work; inspect or print credential content; mutate an
unowned capsule; push `main`; or merge.
