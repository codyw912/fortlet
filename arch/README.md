# Fortlet Architecture Track

Architecture work is captured as Fortlet Improvement Proposals (FIPs) under
`proposals/`. Files use `NNNN-slug.md`; documents and discussion refer to them
as `FIP-NNNN`.

A FIP is a pure design document. It records a durable decision, its context,
tradeoffs, and a checkable specification; current implementation state belongs
only in `conformance.json`.

## Decision lifecycle

1. `Draft` — under active design.
2. `Review` — design complete, awaiting operator approval.
3. `Accepted` — approved; implementation may begin.
4. `Final` — immutable except for `Superseded-By:` and dated,
   operator-authorized amendments.
5. `Superseded` — replaced by a later FIP.
6. `Withdrawn` — abandoned without replacement.

Design changes to a Final FIP require a new proposal with a `Supersedes:`
header. Decision history is append-only.

## What requires a FIP

Write and accept a FIP before changing security or credential boundaries,
project/capsule identity, persistence, public CLI or configuration contracts,
harness adapter contracts, publication semantics, remote workspace or
transport models, or any choice whose reversal would cost more than one day.

Local refactors, internal naming, and reversible module layout do not require a
FIP unless parallel sessions would otherwise build incompatible behavior.

## Implementation conformance

`conformance.json` tracks whether code conforms to each FIP. Allowed statuses
are `conformant`, `partial`, `unimplemented`, and `n-a`. Conformant entries need
tests; partial entries need explicit gaps. Update the map in the same checkpoint
as code that changes conformance.

Validate it with:

```sh
cargo test --test conformance
```

## Index

1. [FIP-0000: Proposal Template](proposals/0000-template.md)
2. [FIP-0001: Project-scoped capsule architecture](proposals/0001-project-capsule-architecture.md)
3. [FIP-0002: Optional transparent harness shims](proposals/0002-optional-transparent-harness-shims.md)
4. [FIP-0003: Project capsule status and stop](proposals/0003-project-capsule-status-and-stop.md)
5. [FIP-0004: Project capsule reset](proposals/0004-project-capsule-reset.md)
