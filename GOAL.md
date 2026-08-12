# GOAL: Publish the public alpha repository

Status: authorized on 2026-08-12.

Publish Fortlet's complete verified history as a public GitHub repository at
`github.com/codyw912/fortlet`. This mission establishes a durable remote before
the next product GOAL; it does not add product behavior, CI, releases,
distribution artifacts, issues, or project policy.

## Deliverable 1 — Verify public readiness

Completed before authorization:

1. The working copy is clean and the complete history uses the operator's
   GitHub no-reply author address.
2. MIT and Apache-2.0 license texts are present and Cargo declares the matching
   dual license.
3. Current-tree and full-history scans found no credential, private-key, GitHub
   token, bearer-token, or API-key patterns.
4. All `/Users/cody` occurrences were shown to and accepted by the operator.
   They occur only in retained experiment/design evidence and disclose no
   credential value.
5. GitHub identity `codyw912` is authenticated over SSH and
   `github.com/codyw912/fortlet` does not exist.

## Deliverable 2 — Rehearse and publish

1. Declare Experiment 0011 before remote mutation.
2. Run the complete standard verification set against the exact outgoing tree.
3. Advance the local `main` bookmark by fast-forward from its existing
   checkpoint to the publication tip.
4. Inspect the complete outgoing history and exact destination.
5. Create public repository `codyw912/fortlet` with description
   `Project-scoped isolation for coding-agent CLIs` and add it as `origin`.
6. Push only bookmark `main` through Jujutsu. Do not use mutating Git commands.
7. Verify the remote default branch, visibility, URL, and exact tip through
   read-only GitHub queries.

## Deliverable 3 — Close and publish the record

1. Close Experiment 0011 with exact remote and revision evidence.
2. Rewrite `experiments/HANDOFF.md`, mark this GOAL complete, and checkpoint
   the terminal record.
3. Advance `main` to the closure checkpoint, inspect the one-checkpoint
   outgoing delta, push `main` once more, and verify exact remote equality.
4. Leave a clean working copy and STOP. Reset/recovery remains the intended
   next product GOAL but is not authorized under this mission.

## Definition of Done

1. `https://github.com/codyw912/fortlet` exists and is public.
2. Remote `main` contains the complete local history through the terminal
   publication checkpoint.
3. Local `main`, remote `main`, and the verified closure revision are equal.
4. No branch, tag, release, issue, package, workflow, or other remote resource
   is created.
5. The complete standard verification set is green before initial publication.
6. Experiment 0011 is terminally closed; then STOP.

## Binding rules

1. Preserve every charter invariant and accepted FIP.
2. Publish the complete history; do not rewrite, squash, redact, or omit failed
   experiments.
3. Use Jujutsu for bookmarks and pushes. Do not use `git add`, `git commit`, or
   another mutating Git command.
4. Do not expose credential values in commands, output, records, or remote
   metadata.
5. Repository creation and the two declared `main` pushes are the only remote
   mutations authorized.
6. If SSH signing blocks on 1Password, pause for operator approval and retry
   only after approval.
7. Stop on a destination mismatch, non-fast-forward, unexpected remote
   resource, credential anomaly, failed verification gate, or need to rewrite
   history.

## Budget and escalation

1. Engineering ceiling: one hour.
2. External budget: zero money, one public repository creation, two `main`
   pushes, zero other remote mutations, and no paid quota.
3. Experiment 0011 is the only active experiment.

## Verification

Before initial publication run:

- `cargo test`;
- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --test conformance`;
- `nix flake check`.

After each push, verify GitHub visibility, default branch, and exact remote tip.
