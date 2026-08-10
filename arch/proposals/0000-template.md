# FIP-0000: Proposal Template

Status: Final

## Summary

Use this template for architecture-track decisions. A FIP records what was
decided and why; implementation state lives in `arch/conformance.json`.

## Headers

Every FIP starts with a `Status:` line and may carry relationship headers:

1. `Status: Draft | Review | Accepted | Final | Superseded | Withdrawn`
2. `Supersedes: FIP-NNNN`
3. `Superseded-By: FIP-NNNN`
4. `Requires: FIP-NNNN`
5. `Extended-By: FIP-NNNN`

Once Final, a FIP is immutable except for adding `Superseded-By:` and appending
dated, operator-authorized amendments. Other design changes require a new FIP.

## Motivation

Explain why this decision matters now and what project risk it reduces.

## Decision

State the decision directly.

## Specification

Describe observable requirements using MUST, SHOULD, and MAY. Every requirement
must be checkable by conformance evidence.

## Consequences

List design obligations and tradeoffs, not current implementation status.

## Alternatives Considered

Record meaningful alternatives and why they were not selected.

## Open Questions

List follow-up design questions that should become future FIPs.

## Amendments

A Final FIP may gain dated amendments carrying their own status, operator
authorization, and explicit supersession boundary. Never rewrite earlier text.
