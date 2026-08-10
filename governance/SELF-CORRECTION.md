# Self-Correction Playbook

Copied into instantiated projects verbatim. These are distilled,
generalized failure lessons from a real agentic project's history — most
cost actual money or weeks. Read this file when: an experiment dies
terminally, a failure repeats under a different guise, a review keeps
bouncing, or the deliverable has not moved in days while the repo has.

## 1. When a failure repeats with a new mask, question the assumption

The single most expensive pattern on record: one philosophy error (demanding
byte-exact equality from stochastic output) was "fixed" four times under
different masks — case stability, wall sizing, timeout tuning, chunk
anchoring — killing four consecutive campaigns. Each fix was locally
reasonable; the assumption underneath was wrong.

Rule: after TWO terminal failures that rhyme, you are not allowed a third
attempt at the mask level. Write down the assumption both attempts shared,
construct the cheapest test that could falsify it, and run that instead.
If the charter pre-authorizes a fallback path, activate it.

## 2. Ceremony is not the deliverable

Agents under uncertainty drift toward building process: more validation,
more integrity checks, more lifecycle states, more tests of tests. This
feels like progress and is individually justifiable — and it is the primary
way missions silently die. In the source project, the apparatus was proven
after seven experiments and ~2% of the budget; the remaining risk was
DIRECTION, yet the pull toward more apparatus persisted.

Rule: before adding machinery, name the recorded blocker that requires it.
No blocker on record, no machinery. If a week of commits contains no unit of
the actual deliverable, report that as an anomaly instead of explaining it.

## 3. Don't gate stochastic processes on exact content

Any check of the form "the model output must equal/contain exactly X" is a
time bomb. Validate rates, structure, bands, and machine-checkable outcomes
(exit codes, passing tests, schema validity). A unit producing on-task
output for its full window is a valid measurement even when content drifts.

## 4. Per-unit failure is data, not death

A failed sample/record/run settles its own unit, gets its cause recorded,
and the campaign continues. Campaigns terminate on budget, a pre-declared
failure-rate threshold (default >25%), or a hard-invariant hit — never on a
single unlucky unit. The inverse discipline also holds: no silent retries,
no replacement of failed units to prettify the set, no complete-case subset
selection at analysis time.

## 5. Rehearse the LAST steps, not just the core

Multiple campaigns executed their expensive core perfectly and then died in
unrehearsed publication/settlement stages, wasting the spend. A zero-cost
end-to-end rehearsal of the COMPLETE path — through reporting, persistence,
and closure — is mandatory before real dispatch. "The interesting part
works" is not rehearsal.

## 6. Attribute wins to declared mechanisms only

If the treatment won but not through the mechanism you pre-declared, you
have an observation, not a result. Bisect bundles before attributing
components. Never re-narrate an accidental early-exit or a lucky path as
the designed win — the record's future readers can't tell the difference,
which is exactly the problem.

## 7. Claims and evidence are different substances

Anything the system says about itself (model output asserting success,
self-reported metrics without provenance) is a claim. Evidence is a
runner-executed gate, an externally reported actual, or a regenerable
artifact. Label the two differently everywhere; never let a claim flow into
a headline number. When only claims are available, the honest statement is
"no result", not the claim with a caveat.

## 8. Losses are deliverables

An honestly measured loss defines the prize for the next cycle and is
recorded with the same care as a win. Deleting negative results, resuming
terminally closed experiments, or iterating past the mission boundary to
turn a loss into a win are all the same corruption. Expectation-setting
belongs up front: if a baseline can lose, the mission text says so, so
nobody is tempted to fix reality.

## 9. Freeze what you compare against

Comparisons are only meaningful when scope is frozen: same inputs, same
environment, same everything except the declared treatment. Discovering
mid-flight that something varied silently doesn't downgrade the result — it
terminates it. Diffs between successive plans beat re-freezing from scratch:
re-authoring invites unnoticed drift.

## 10. The handoff is part of the work

A mission is not done when the code works; it is done when a fresh session
can verify the state and continue without inheriting myths. Write the
handoff's "current state" as verifiable claims, keep advisory guidance
labeled advisory, and grow the "what NOT to do" list. Assume your successor
knows nothing and trusts nothing — because the good ones don't.
