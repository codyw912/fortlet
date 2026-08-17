# Agent instructions

Before writing code, read these files in order:

1. `GOAL.md`
2. `experiments/HANDOFF.md`
3. `governance/CHARTER.md`
4. Every FIP named by the goal

Follow `WORKFLOW.md` for durable project rules. Use Jujutsu (`jj`) for local
version control. Make checkpoints with `jj describe -m "<intent>"` followed by
`jj new`; do not use `git add` or `git commit`.

Run the complete verification set in `docs/RUNBOOK.md` before claiming work is
complete. Architecture-track changes require an accepted FIP before
implementation, and conformance changes belong in the same checkpoint as the
code and tests that establish them.

For publication, follow FIP-0005 and the runbook. Use one goal bookmark and one
draft pull request, except for its recorded two-PR bootstrap. Before every push
or GitHub mutation, present the exact revisions and diff, destination, complete
metadata or settings change, verification state, and known failures; wait for
explicit operator approval. The operator merges unless they explicitly
authorize the agent to merge one specific pull request. Never push directly or
force-push to `main`.
