# Session Handoff — Firewall recovery terminally rejected

Audience: a fresh agent session. `GOAL.md` is normative and complete pending
operator merge. Verify `main`, the Jujutsu stack, conformance, and the complete
`docs/RUNBOOK.md` gate before relying on this summary. Read the FIPs named by a
successor GOAL in full before implementation or experiment dispatch.

## Baseline and publication

Merged `main` is `be43d5909b6a65e9ca6984859b85b6d27c2601dc` (PR #11), whose
tree was proved byte-identical to reviewed signed tip
`d8fa6c027d10f5622fd7e2eb6282b3a1bd8a424b`. The prior goal bookmark was
removed locally and remotely. This recovery uses bookmark
`codex-firewall-recovery` and draft PR #12; the operator remains the sole merge
authority.

The complete standard gate passed on aarch64-darwin on merged `main`, the exact
Experiment 0036 declaration tree, and the terminal closure tree with the
retained test: all 88 unit tests and every enabled integration test,
formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check`. Nix emitted only the expected incompatible `x86_64-linux`
omission warning. FIP-0001 and FIP-0002 remain partial with explicit gaps;
FIP-0003 through FIP-0011 remain conformant.

## Immutable treatment and preparation

Experiment 0036 froze declaration revision
`a1dade963584803e0affa6fd9dfdc874bf74efc5` as package
`/nix/store/is3sfw30rzr58w29q8rnsdcdidwxqgzn-fortlet-0.1.0`. Clean preflight
proved packaged Codex absent, `no capsules`, and raw inventory `[]`.

With the operator present for the expected firewall path, one public cold
`fortlet prepare codex` printed its first-use line and then exact
`codex<TAB>ready`. The identical immediate command returned exact ready in 2.6
seconds with no provisioning diagnostic, proving a verified immutable cache
hit. This corrects Experiment 0035's invalid attribution: the earlier apt DNS
failure occurred while the operator was absent for firewall approval and was
not evidence about the corrected Codex path.

## Sole model-unit result

Marker-free Herdr pane `wA:p9` dispatched one immutable packaged Codex 0.147.0
process. It used the OpenAI provider and workspace-write sandbox, authenticated,
streamed without an Apps warning, and confirmed
`cargo_target=target/fortlet-guest`.

Codex changed only `tests/pre_runtime_failures.rs`: it extracted two small
file-local helpers, preserved the existing tests, and added
`codex_shim_invalid_project_environment_fails_before_credentials_or_runtime_artifacts`.
The new test exercises the compiled binary through a `codex` symlink, asserts
the exact invalid normalized project-environment failure, and proves project,
tool, and environment roots remain absent.

The exact guest Cargo command exited 101 before test execution. The
`microsandbox-filesystem` 0.6.8 build script tried to download pinned
`agentd-aarch64` from GitHub and failed DNS lookup with `Temporary failure in
name resolution`. Codex did not override the target, retry, or substitute a
command; it summarized the failure and exited zero. Because the guest test did
not pass, Experiment 0036 is rejected even though the model process itself
closed normally.

Independent host evidence accepted the retained diff: the exact focused test
passed, all 14 `pre_runtime_failures` tests passed, and formatting passed. Public
inventory identified one owned running Codex capsule; packaged stop/reset
returned `stopped` then `reset`, and final status was absent with `no capsules`
and raw inventory `[]`. Temporary dispatch files were deleted. Immutable layers
and persistent harness state remain.

The project recipe successfully downloaded Jujutsu from GitHub during cold
preparation, so the later observation is not a demonstrated general firewall
or package denial. Runtime-capsule DNS attribution remains unresolved. No
second model process or retry is authorized by this GOAL.

## Next action

Run the final complete gate through `nix develop`, checkpoint the terminal
records, inspect `jj log -r 'main..@'`, sign and push the final
`codex-firewall-recovery` tip, update PR #12 with the exact rejected outcome,
require hosted `Rust verification`, and mark it ready for the operator. Do not
retry Experiment 0036, dispatch another model process, change product code,
alter network/firewall configuration, mutate `main`, or merge.

After operator squash merge, fetch `main`, prove exact reviewed-tree equality,
and remove only this GOAL's local and remote bookmark. A successor GOAL should
diagnose provisioning-versus-runtime DNS deterministically without provider
traffic before considering another daily-work model request.
