# Session Handoff — Codex daily work closure at terminal publication

Audience: a fresh agent session. `GOAL.md` is normative. Verify `main`, the
Jujutsu stack, conformance, and the complete `docs/RUNBOOK.md` gate before
relying on this summary. Read FIP-0001 through FIP-0011 in full before any
implementation or experiment dispatch.

## Verified baseline and publication

Merged `main` is `bca9438b2bb97b8f79728627b687e8d1ccc943d3` (PR #10). Work is
stacked above it on bookmark `codex-work-readiness` and draft PR #11. The
operator remains the sole merge authority. The complete standard gate passed
on aarch64-darwin before the successor dispatch: 88 unit tests and every
enabled integration test, formatting, strict all-target/all-feature Clippy,
conformance, and `nix flake check`; Nix emitted only its expected incompatible
`x86_64-linux` omission warning.

The immutable successor treatment is declaration revision
`95fb7607e74dff0aaf4f7e69ee94d1d7661a146d` and package
`/nix/store/68vqsxikz7a12sqyrd1518hkz9w9mxq3-fortlet-0.1.0`. The final
documentation tree passed the same complete gate. Its publishable tip is
signed, the bookmark is published, hosted `Rust verification` passed, and PR
#11 is ready. Only operator review and squash merge remain.

## Product and retained change

Fortlet resolves safe project roots, supplies immutable project tools,
optionally prepares layers, launches Codex and Tact explicitly or through
package-owned shims, renews host-owned Codex credentials, suppresses only the
unsupported Apps client, streams non-interactive output with explicit EOF and
a bounded Codex inactivity ceiling, and exposes project lifecycle controls plus
global owned-capsule inventory.

Experiment 0034's sole model process authenticated, streamed, and added the
correct retained regression test in `tests/pre_runtime_failures.rs`. Its exact
guest Cargo command failed because this repository configured
`CARGO_TARGET_DIR=/home/agent/.cargo/fortlet-target`, outside Codex's inner
workspace-write roots. The smallest correction changes only the repository
fixture to ignored, guest-distinct `target/fortlet-guest` and updates its
deterministic assertion. The retained test and complete gate pass. FIP-0002's
conformance evidence includes `tests/pre_runtime_failures.rs`.

## Terminal successor evidence

Experiment 0035 froze the corrected package and passed a clean preflight in
marker-free Herdr pane `wA:p9`: packaged Codex was absent, `fortlet list`
reported `no capsules`, and raw MicroSandbox inventory was `[]`. Its exact sole
launcher selected the normal new project-layer identity and printed the
first-use preparation notice. After 3 minutes 21 seconds the provisioning
recipe exited 100: apt could not resolve `deb.debian.org`, then could not obtain
`libcap-ng-dev` or the pinned `libcap-ng0:arm64` package.

Codex never launched. There was no provider request, target report, guest Cargo
command, model output, or repository diff. The launcher returned status 1 and
the pane returned to idle fish. Post-failure checks again proved a clean working
copy, public Codex absence, `no capsules`, and raw inventory `[]`. Temporary
launcher/prompt files were deleted; immutable layers and persistent harness
state were not directly mutated.

The GOAL allowed only the initial process and one successor after correction.
Experiment 0035 is terminally rejected and no third process is authorized.
Daily Codex readiness therefore remains unproven at automatic cold project
provisioning, even though the demonstrated Cargo-target defect is corrected and
all deterministic evidence is green.

## Next action

Review ready PR #11 and leave squash merge to the operator. Do not dispatch
Codex again, warm or replace the failed project layer to manufacture
acceptance, mutate `main`, or merge.

After the operator squash-merges, fetch `main`, prove exact reviewed-tree
equality, and remove only this goal's local and remote bookmark. A later GOAL
may investigate cold-provisioning network reliability or deliberately rerun
daily-work evidence, but this GOAL cannot do either.
