# Session Handoff — Global capsule inventory proposed

Audience: a fresh agent session. `GOAL.md` is normative and active. Verify
the publication, merged `main`, complete gate, and conformance before relying
on this summary. Read FIP-0001, FIP-0003, FIP-0004, FIP-0005, and FIP-0011 in
full before implementation.

## Verified baseline

The operator squash-merged PR #9 as
`a7829362fbe13e82dbe4b1cddfac6ecd5f726a6f`. Its tree exactly matched reviewed
tip `c576b9c883ab643a316e76b982b137b73500be2e`; the
`daily-codex-work-loop` bookmark was removed locally and remotely. The working
copy then began clean directly above merged `main`.

On 2026-08-19 the complete `docs/RUNBOOK.md` gate passed again through
`nix develop`: all 83 unit tests and every enabled integration test,
formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check` on aarch64-darwin. Nix emitted only the known incompatible
`x86_64-linux` omission notice. Conformance remains honest: FIP-0001 and
FIP-0002 are partial; FIP-0003 through FIP-0010 are conformant.

## Product state and selected direction

Fortlet now reaches an ordinary managed Codex edit loop, streams output,
refreshes host-owned credentials, suppresses only unsupported Apps, supplies
project tools, and preserves those tools through non-interactive Bash login
shells. The last model-backed unit made the intended test edit but its guest
test could not find Cargo; the subsequently merged FIP-0006 correction has
deterministic and real composed-path evidence, but no second provider prompt
was sent under that completed goal.

The operator wants Codex feasible for real work before broad harness expansion,
then expects OMP, Pi, and Claude Code to expose additional adapter classes.
The agreed sequence is global capsule inventory, one Codex work-readiness
closure, then the first new harness chosen from a concrete use case rather than
a speculative generalized adapter abstraction.

## Proposed lifecycle slice

FIP-0011 is Accepted and `GOAL.md` is operator-approved. The selected
surface is only `fortlet list`: a read-only global inventory of fully validated
Fortlet-owned capsules with project, harness, and lifecycle state. It paginates
the typed SDK, reconstructs the project from FIP-0001's same-path bind, verifies
the stored identity and name, safely renders paths, and ignores unrelated
MicroSandbox capsules.

Cleanup deliberately reuses existing explicit
`status`/`stop`/`reset --project` commands. There is no global mutation, raw
capsule selector, automatic expiry, or workload lease in this slice. A missing
project root remains visible but not globally removable. That is the principal
honest limitation to evaluate during FIP review.

## Next action

The `capsule-inventory` bookmark and draft PR #10 exist. Deterministic
implementation is complete at `866007214df6b45030a4bfae71e89a4083679ff8`;
focused inventory, management-failure, conformance, formatting, and strict
Clippy gates pass. Experiment 0033 is predeclared but has not dispatched.

Experiment 0033 is accepted. Immutable package
`/nix/store/72qrzh32iaxcvhvi97p8bi1gsj41gmqr-fortlet-0.1.0` reported the sole
owned running Tact capsule from `/private/tmp`; the listed project selected
packaged public status/stop/reset through final absence, `no capsules`, and raw
inventory `[]`. The synthetic auth document is deleted, the working copy was
unchanged by the live unit, and FIP-0011 is conformant.

Close the GOAL, run the final complete standard gate, shape and sign the exact
publishable tip, update and push only `capsule-inventory`, require hosted Rust
verification, update PR #10 evidence, and mark it ready. Leave merge to the
operator.

Do not send a provider prompt; resume Experiments 0030–0032; add a harness;
add bulk cleanup, leases, logs, restart, standalone installation, or Linux CI;
expose internal capsule names; mutate an unowned capsule; push `main`; or merge.
