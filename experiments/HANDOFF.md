# Session Handoff — Firewall-approved Codex readiness recovery active

Audience: a fresh agent session. `GOAL.md` is normative and active. Verify
`main`, the clean Jujutsu working copy, conformance, and the complete
`docs/RUNBOOK.md` gate before relying on this summary. Read FIP-0001 through
FIP-0011 in full before experiment dispatch.

## Verified merged baseline

The operator squash-merged PR #11 as
`be43d5909b6a65e9ca6984859b85b6d27c2601dc`. Bidirectional Jujutsu diffs proved
its tree byte-identical to reviewed signed tip
`d8fa6c027d10f5622fd7e2eb6282b3a1bd8a424b`. The
`codex-work-readiness` bookmark was removed locally and remotely, and the
working copy began clean directly above merged `main`.

On 2026-08-19 the complete standard gate passed again through `nix develop` on
aarch64-darwin: all 88 unit tests and every enabled integration test,
formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check`. Nix emitted only the expected incompatible `x86_64-linux`
omission notice. FIP-0001 and FIP-0002 remain partial with explicit gaps;
FIP-0003 through FIP-0011 remain conformant.

## Why this recovery exists

Experiment 0034 identified one real composition defect: this repository's
guest Cargo target was in persistent harness home, which Codex's inner
workspace sandbox made read-only. The merged correction uses ignored,
guest-distinct `target/fortlet-guest`; deterministic fixture evidence and the
model-produced missing-auth shim regression test pass.

Experiment 0035 did not test that correction. Its normal automatic project
preparation stopped after apt could not resolve `deb.debian.org`, before Codex
launched or any provider request occurred. The operator later clarified that a
local firewall prompt was waiting while they were absent. Closing that result
as a Fortlet daily-readiness blocker was an attribution error: the session
should have paused for firewall approval and then tested the corrected path.

The merged Experiment 0035 record remains immutable history and is not
rewritten. This new GOAL supplies a separately numbered successor with the
operator present.

## Selected recovery boundary

Use the public credential-free preparation seam before any model request. One
immutable package runs `fortlet prepare codex --project
/Users/cody/dev/fortlet` while the operator approves only the expected local
MicroSandbox firewall prompt. Preparation must print `codex<TAB>ready`; an
immediate second public prepare must be a quiet verified cache hit. Any failure
after the firewall decision stops without a provider request or retry.

Only after that proof may one packaged non-interactive Codex shim process run.
Its frozen task adds the still-missing invalid-project-environment shim
regression test to `tests/pre_runtime_failures.rs`, reports exactly
`target/fortlet-guest`, and runs the exact focused guest Cargo test without an
override. No product edit, follow-up, repair, alternate command, or second model
process is authorized.

## Next action

Checkpoint the new GOAL, create the single `codex-firewall-recovery` bookmark
and draft PR, then predeclare Experiment 0036 with exact revision/package,
commands, prompt, paths, firewall interaction, preflight, cleanup, and failure
boundary. Do not start preparation until the operator is present for the
firewall prompt. Do not dispatch Codex until preparation and its cache-hit
verification both pass.

Do not reopen or edit Experiments 0034/0035; change product code or accepted
FIPs; inspect credential content; alter firewall configuration; substitute a
warm old environment identity; mutate an unowned capsule; use native Codex;
push `main`; or merge.
