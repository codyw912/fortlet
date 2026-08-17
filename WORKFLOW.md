# The Workflow

This file is copied into an instantiated project verbatim. It defines the
durable rules; `ADAPTATION.md` defines how to bind them to a concrete project.

## 0. Principles

1. Decisions are append-only; implementation state is mutable and checked.
   Never edit history to look better; supersede it.
2. Honest measurement is absolute. No fabrication, backfill, estimation
   presented as actuals, or subset selection. Absent data stays absent, with
   coverage stated. A loss recorded honestly is a successful deliverable;
   a win recorded dishonestly is a repository-poisoning event.
3. Autonomy comes from rails, not permission-seeking. The operator grants a
   budgeted charter with falsifiable done-criteria; inside it the agent does
   not ask, outside it the agent stops.
4. The deliverable is the deliverable. Process, tooling, and integrity
   machinery are instrumental and justified only insofar as they unblock the
   thing the charter names. Building ceremony instead of finishing is a
   failure mode, and the operator will call it out.
5. Everything expensive gets a free rehearsal first. Every claim gets a
   verification command. Every artifact that supports a claim is immutable
   and regenerable.

## 1. The four tracks

### 1.1 Architecture track (`arch/`)

Significant design decisions are recorded as numbered improvement proposals
(see `ADAPTATION.md` for naming). A proposal is a PURE design document: it
records what was decided and why — never whether the code currently conforms.

Lifecycle (Status header): Draft -> Review -> Accepted -> Final, with
Superseded / Withdrawn as exits. Once Final, a proposal is immutable except:

- adding `Superseded-By:` when replaced, and
- appending dated, operator-authorized Amendment sections that narrow or
  extend the decision without rewriting it. Amendments state their own
  supersession boundary explicitly ("this changes X only under conditions Y;
  everything else stays true").

Design changes to a Final proposal otherwise require a NEW proposal with a
`Supersedes:` header. Decision history is append-only.

Relationship headers: `Supersedes:`, `Superseded-By:`, `Requires:`,
`Extended-By:`.

A proposal's Specification section uses MUST/SHOULD/MAY language and must be
checkable — a proposal that cannot be tested cannot be tracked.

Architecture-track decisions are made BEFORE implementation: a nontrivial
mechanism gets its proposal accepted first, then built. This is what lets
parallel worker sessions build against a stable contract.

### 1.2 Conformance map (`arch/conformance.json`)

Whether the code conforms to each proposal lives in one machine-readable file,
separate from the proposals: per-proposal status
(`conformant` / `partial` / `unimplemented` / `n-a`), the modules and tests
that satisfy it, and explicit gaps. Rules:

1. `conformant` requires listed tests. `partial` requires listed gaps.
2. Update conformance in the SAME commit as the code that changes it.
3. A checker script validates the map (no dangling file references, no
   missing entries) and runs in the standard verification set.

### 1.3 Experiment track (`experiments/`)

Anything tried against reality — a paid run, a benchmark, a user test, a
deployment canary, a performance screen — is an experiment and gets a
numbered record. Proposals hold durable decisions; experiment records hold
mutable status, declared scope, outcomes, attribution, and rollout decisions.

Non-negotiable discipline:

1. PRE-DECLARE before spending: hypothesis and production mechanism, declared
   scope (what changes, what is frozen), alternatives considered, risks,
   falsifiable acceptance criteria, and budget. A record without these is not
   authorization to spend.
2. TERMINAL CLOSURE after: outcome, root cause in one or two sentences,
   actual cost, and next action. Losses and invalid evidence are recorded as
   carefully as wins — negative results are retained, not deleted.
3. Never edit raw artifacts to improve a result. Never retry silently: a
   failed attempt is closed and a successor experiment gets a NEW record and
   identity; never resume a terminally closed one.
4. One experiment in flight at a time unless the charter says otherwise.
5. Rehearse the complete expensive path in a zero-cost mode (dry-run, mock
   provider, staging) before the real dispatch — including the LAST steps
   (publication, settlement, reporting). Most real-world campaign deaths in
   the source project were unrehearsed final stages, not unrehearsed cores.
6. A single failed unit is data, not death: it settles its own unit; a
   campaign terminates only on budget, a pre-declared failure-rate threshold,
   or a hard-invariant hit.

The `experiments/README.md` index lists every record with a one-line outcome,
so the failure history is scannable in one screen.

### 1.4 Governance track (`governance/`, `GOAL.md`, `experiments/HANDOFF.md`)

Work is granted through charters and missions, not ad-hoc chat:

1. `governance/CHARTER.md` — the operator's standing grant: mission,
   falsifiable Definition of Done, autonomy grants (spend, decision scope),
   hard invariants (violating one voids the charter), direction constraints
   (encode already-diagnosed failure vectors), escalation triggers (stop and
   ask), and reporting cadence. Superseded charters are marked and retained.
2. `GOAL.md` at the repo root — the CURRENT mission for a fresh session:
   ordered deliverables, binding rules, budget, verification commands, and an
   explicit "then STOP" boundary. Normative; wins over everything except the
   charter's hard invariants.
3. `experiments/HANDOFF.md` — descriptive briefing for the next session: why
   the mission exists, current verified state, conventions that keep working,
   advisory design guidance, known quirks, and a "what NOT to do" section of
   paid-for lessons. Descriptive, not normative: `GOAL.md` wins on conflict.
   Replaced (not appended) when a mission completes; old versions live in git.

Expectation-setting belongs in the mission text: if a deliverable can "lose"
honestly (a losing baseline, a rejected hypothesis), say so up front so the
agent does not iterate ad infinitum trying to make reality look better.

## 2. Session lifecycle

Fresh session startup order:

1. Read `GOAL.md`, then `experiments/HANDOFF.md`, then the charter.
2. Verify the claimed current state (run the verification commands; check
   conformance) — the handoff says "verify before relying" and means it.
3. Read the specific proposals the mission names, in full, before designing.

While working:

1. Architecture-track decision -> proposal first, accepted, then code.
2. Semantic commits, bounded size (target: reviewable in one sitting,
   roughly <= 800 changed lines). Commit messages state intent, not mechanics.
3. Independent review before irreversible or paid steps, where available
   (a second session or agent reviewing the diff/plan cold).
4. Full verification gates green before any real-world dispatch.
5. Report to the operator at every deliverable completion and every terminal
   closure — compact: outcome, root cause, totals, next action. No permission
   requests inside the granted scope; no silent multi-day building either.
   Prolonged silence is itself a report-worthy anomaly.

### 2.1 Pull request publication

After a public remote exists, each authorized GOAL uses one descriptive
Jujutsu bookmark and one draft pull request targeting `main`. Semantic local
checkpoints remain reviewable on the branch; the operator squash-merges the
completed GOAL into one public commit. A recorded bootstrap MAY use a second
closure pull request for evidence that can exist only after the primary merge;
this exception does not apply to later GOALs.

After locally knowable work and verification are complete, present one reviewed
publication packet: the signed revisions and diff, named bookmark and `origin`
destination, complete draft-PR metadata, expected initial hosted check,
readiness criteria, known failures, and any exact landed bookmarks proposed for
cleanup. One approval authorizes only one push of that signed bookmark, creation
of that draft PR, observation of its initial hosted run, declared evidence-only
body updates, readiness after success, and named bookmark cleanup after the
operator merge and exact tree equality. The operator merges unless they
explicitly authorize the agent to merge one specific pull request.

Changed code or scope, another push or PR, a retry, substantive metadata change,
repository settings, an unexpected remote change, or any failed transaction
step is outside the packet and requires a new exact review and approval. Never
infer authority over unrelated refs or resources.

Before publication, complete all locally knowable GOAL work, close every
experiment, inspect and sign the bookmark stack, run the complete local
verification set, and record the local Nix host and result. The GOAL may remain
conditionally active only for hosted verification, operator merge, and landing
observation. Mark the PR ready only after its expected hosted check passes. A
failure stops publication; do not bypass a check, silently retry, or force-push
`main`.

After an operator squash merge, fetch `main` and prove its tree equals the exact
reviewed branch-tip tree. Commit identity is expected to change. If the
publication packet named landed bookmarks for cleanup, remove only those after
destination and tree equality are verified; cleanup may otherwise be deferred.

Session end (or mission completion):

1. Update conformance, close experiment records, regenerate derived docs
   via their generators (never hand-edit generated files).
2. Rewrite `experiments/HANDOFF.md` for the successor.
3. Update `GOAL.md` or mark it complete; STOP at the mission boundary even if
   momentum suggests continuing — follow-on work is a new mission.

## 3. Verification

Every project binds a standard verification set (see `ADAPTATION.md`), which
must include at minimum:

1. The full test suite.
2. Static checks (types/lint) as the stack provides.
3. The conformance checker.
4. Regenerability: any published report or derived doc can be regenerated
   byte-exact (or semantically identical, where the format is unstable) from
   committed artifacts.

Claims about the system's behavior in reports/docs must cite the command that
proves them. "It works" without a green gate is an agent claim, never
evidence; the two are labeled differently everywhere they appear.

## 4. Self-correction

`governance/SELF-CORRECTION.md` (copied verbatim) is the distilled failure
playbook. Read it when: a failure repeats under a different guise, an
experiment dies terminally, a review keeps bouncing, or you notice
apparatus-building displacing the deliverable. Its headline rule: when a
failure repeats with a new mask, question the assumption, not the mask.
