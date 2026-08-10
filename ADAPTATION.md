# Adaptation Guidance

How an agent binds `WORKFLOW.md` to a concrete project. Input: this file plus
the project's description/brief. Output: an instantiated workflow committed
before product code. There are intentionally no per-project-type variants;
derive the bindings below from what the project actually is.

## 1. Derive the bindings

Work through these in order; record the answers in the instantiated
`OVERVIEW.md` under a "Workflow bindings" section.

### B1. Proposal identity

Pick a short prefix from the project name (e.g. project "Foo" -> FIP,
"Bar" -> BIP). Proposals are zero-padded 4-digit numbered, filenames
`NNNN-slug.md`. Proposal 0000 is always the template itself (status Final,
conformance `n-a`).

### B2. What counts as an architecture-track decision

Derive from the project's risk surface. The test: "would two parallel
sessions build incompatible things without this being written down?" or
"would reversing this cost more than a day?" Typical members: module
boundaries, external contracts (APIs, schemas, file formats), state and
persistence models, measurement/quality semantics, security boundaries,
irreversible tooling choices. Typical non-members: naming, layout of a
single module, anything a refactor can undo cheaply — those go in commit
messages or code comments.

### B3. What counts as an experiment

Anything whose outcome is learned from reality rather than from the test
suite, especially if it costs money, quota, reputation, or user attention.
Derive the project's spend unit: provider dollars for LLM-heavy projects, CI
minutes or cloud cost for infra projects, user sessions for product
experiments, wall-clock days for pure-research spikes. If the project has
genuinely NO expensive contact with reality, keep the track anyway (the
records still capture hypotheses and outcomes of risky refactors/spikes) but
collapse budget fields to effort estimates.

### B4. Verification set

Bind concrete commands from the project's stack: test runner, type checker /
linter, conformance checker, build. Put them in `docs/RUNBOOK.md` and in
`GOAL.md`'s Verification section. If the stack lacks one (e.g. no types), say
so explicitly in the runbook rather than leaving the slot silently empty.
Add project-specific gates as they emerge (schema validation, e2e smoke,
report regeneration) — the set only grows.

### B5. Charter thresholds

Scale to the project's stakes. Derive: total budget and per-experiment cap
(in the B3 spend unit), consecutive-terminal-failure trigger (2 is a good
default — the third attempt after two identical deaths is where masks get
re-tried), failure-rate threshold for aborting a multi-unit campaign (>25%
is the tested default), and reporting cadence (per deliverable + per
terminal closure as floor). If there is no external operator, the charter
still gets written — self-imposed rails fail more gracefully than no rails.

### B6. Generated artifacts

List every file that a generator owns (reports, dashboards, indexes). Mark
them "generator-owned — never hand-edit" in the runbook, and name the
regeneration command next to each.

### B7. Holdouts and reserved surfaces

If the project has evaluation assets (benchmark suites, test cohorts,
validation datasets), reserve at least one as a holdout that day-to-day
iteration must not touch, and name it in the handoff's "what NOT to do"
list. Optimizing against your only validation surface is a slow-motion
version of training on the test set.

## 2. Instantiate the tree

1. Copy everything except this repo's `README.md`.
2. Rename `*.template.md` -> real names (`OVERVIEW.template.md` ->
   `OVERVIEW.md`, `governance/CHARTER.template.md` ->
   `governance/CHARTER.md`, etc.).
3. Fill every `{{PLACEHOLDER}}`; grep for `{{` afterwards (excluding this
   file, which mentions the marker in prose) to confirm none remain.
4. Rename `arch/proposals/` files to the B1 prefix convention in
   `arch/README.md`'s text (directory name stays `proposals/`).
5. Seed `arch/conformance.json` with entry 0000 (`n-a`).
6. Write the first real `GOAL.md` for the bootstrap mission, with a real
   Definition of Done ("repo builds, verification set green, first N
   proposals accepted" is a fine first mission).
7. Commit: `Instantiate agentic workflow (from SPAWN.md)`.

## 3. Right-sizing (read this twice)

The workflow scales DOWN by collapsing documents, never by dropping the
four questions (decided? tried? allowed? handover?).

- Tiny project (one contributor-agent, no spend): charter and GOAL.md can be
  one file; experiments may live as sections in one EXPERIMENTS.md;
  conformance can be a table in arch/README.md. Keep proposal immutability
  and terminal closure — they cost nothing and are the load-bearing parts.
- Medium project: the full tree as given.
- Large/multi-fleet project: split HANDOFF.md per workstream; charters may
  nest (program charter -> per-mission GOAL.md), but hard invariants only
  ever tighten downward, never loosen.

Anti-patterns the operator will reject:

1. Instantiating ceremony the project cannot feed (e.g. elaborate budget
   ledgers for a project with no spend). Empty process is negative signal.
2. Writing proposals after the code as documentation theater. If the
   decision was already made in code, record it honestly as such
   (status Accepted, dated retroactively-recorded) — once.
3. Deleting failed experiment records or handoff "what NOT to do" items to
   make the repo look cleaner. The failure history IS the value.
