# Experiment 0000: <name>

Status: declared | in-flight | completed — <accepted/rejected/terminal> 
Design: <FIP-NNNN, if the mechanism has one>

## Baseline / Control

What the current, measured state of the world is, with identities (versions,
commits, configuration hashes, dataset/cohort ids) exact enough that the
comparison is reproducible. If the baseline is a prior experiment's result,
cite the record and its artifacts; do not restate numbers from memory.

## Hypothesis and Production Mechanism

What you believe will happen and — critically — the MECHANISM that produces
it. "X will be faster" is not a hypothesis; "removing the cold-start turn
eliminates N seconds of serial time on the critical path" is. Attribution
later must trace back to a mechanism declared here; wins from undeclared
mechanisms are observations for the next experiment, not claims for this one.

## Declared Scope

Exactly what the treatment changes, and an explicit list of everything that
stays frozen (inputs, environment, versions, prompts, cohorts...). Scope
creep discovered mid-flight terminates the experiment.

## Alternatives

Meaningful alternatives considered and why they were not selected. Include
the cheaper option you rejected — the reviewer will ask.

## Risks

What could go wrong, what it would cost, and what failure would look like in
the data (so it is recognizable, not rationalizable).

## Acceptance Criteria

Falsifiable, pre-declared, including:

1. Verification gates that must stay green.
2. The decision rule: what result accepts the treatment, what rejects it,
   and what sample size a real claim requires (label anything smaller
   PRELIMINARY or screen-only).
3. Failure handling: single-unit failures settle their unit; the
   experiment-fatal thresholds (budget, failure rate) are stated here.

## Budget and Plan

Spend cap in the project's spend unit, unit counts, ordering/counterbalancing
if applicable, and what triggers an early stop. The plan is declared up
front; adaptive scheduling mid-experiment is a scope violation.

## Rehearsal

Evidence that the COMPLETE path — including publication/settlement/reporting
stages — passed in zero-cost mode before real dispatch. Date, command,
outcome.

## Results

Filled in as reality arrives. Raw observations first, with identities;
interpretation after. Failed attempts are recorded here verbatim (what
happened, what it cost), never overwritten.

## Terminal Closure

Mandatory when the experiment ends, success or failure:

1. Outcome: accepted / rejected / terminal-failure, in one line.
2. Root cause in one or two sentences.
3. Actual total cost vs budget.
4. Next action (successor experiment, rollout, or stop).
