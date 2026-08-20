# GOAL: Provide project-local Nix activation for Fortlet shims

Status: active — locally complete; final revision publication and hosted gate
pending

## Outcome

Make Fortlet's optional `codex` and `tact` shims easy to activate for one
development shell without changing global shell configuration. Provide a
minimal named Nix shell for immediate use and a composable package output for
projects that already use devenv or another Nix dev shell.

This is a packaging and activation refinement of accepted FIP-0001 and
FIP-0002, published under FIP-0005. It does not change the capsule-session,
credential, environment, harness, or lifecycle architecture and needs no new
FIP.

## Deliverables

1. Verify merged `main`, clean Jujutsu state, conformance, and the complete
   `docs/RUNBOOK.md` gate through `nix develop` before implementation.
2. Export an explicit shim-activation package whose `bin` entry resolves to
   Fortlet's existing immutable package-owned shim directory.
3. Add a minimal `devShells.agents` containing Fortlet and that activation
   output. Keep `devShells.default` unchanged and avoid startup-file writes,
   aliases, functions, `eval`, generated activation text, or reliance on an
   interactive shell initializer.
4. Prove deterministic discovery for both shims and recursion-safe
   `fortlet native` resolution, including arguments and a native executable
   later on `PATH`.
5. Document bounded direct execution, explicit CLI use without shims, and
   cached composition into an existing devenv project. Do not present repeated
   local-path flake evaluation or a child interactive shell as the daily loop.
   Predeclare bounded local smoke experiments before invoking a packaged shim.
6. Run the complete local and hosted gates, publish one goal-scoped draft PR,
   and leave merge authority with the operator.
7. In a separately scoped, uncommitted AGD change, pin the exact published
   Fortlet revision and add `fortlet` plus `shim-activation` to AGD's devenv.
   Prove one cold activation and one cached re-entry before another live shim
   unit.

## Definition of Done

1. The named shell and its prebuilt outputs resolve `fortlet`, `codex`, and
   `tact` without mutating user configuration or depending on fish startup
   behavior.
2. Plain `nix develop` does not include the activation output, and explicit
   `fortlet run` remains fully usable without either shim.
3. `fortlet native codex -- <arguments>` skips the activated shim path and
   selects the first later native executable without recursion.
4. A devenv project can opt in by adding the two Fortlet package outputs; no
   Fortlet-specific shell hook is required.
5. A terminal successor smoke begins and ends with no owned capsule, changes
   no AGD product file, avoids repeated Nix evaluation, and records the exact
   command resolution and native escape result from the operator's fish.
6. Conformance and the complete `docs/RUNBOOK.md` verification set pass.
7. AGD's direnv-owned environment resolves its project toolchain, `fortlet`,
   `codex`, and `tact` without a per-command flake wrapper or prompt-time PATH
   loss; unsupported Fortlet host systems retain AGD's existing environment.

## Excluded scope

No global Home Manager or shell configuration, AGD Rust product code, new
harness, project guest environment, credential mechanism, runtime behavior,
standalone installer, release, repository setting, AGD publication, or merge is
in scope.

## Experiment stop

Experiments 0038 and 0039 both stopped before any native or managed harness
launch. The first rejected repeated local-path Nix evaluation and contained a
missing native separator. The second proved that AGD's direnv hook restores its
owned environment at the next fish prompt and therefore removes a manual PATH
prepend. Do not dispatch a third activation workaround under this scope.

The operator then explicitly authorized the durable mechanism as a separate
uncommitted AGD development-environment change. Experiment 0040 governs that
work and must prove cached re-entry before another shim smoke.

Experiment 0040's non-interactive agent phase passed: its cold activation took
406.93 seconds, cached activation took 0.51 seconds, and native Codex returned
0.147.0. The operator's exact interactive fish unit then failed without retry.
After direnv successfully renewed and entered devenv, the next prompt selected
host Cargo, omitted `fortlet`, and retained older shim paths; the native command
therefore could not start. No managed shim or capsule ran. Experiment 0041 then
used the exact Home Manager configuration in a new fish process and preserved
all expected Nix paths across two prompt events without treatment. This
falsifies a general hook-order defect and attributes the operator failure to
stale long-lived shell state after the earlier manual PATH experiment. Do not
add a compatibility mechanism; Experiment 0042 must test a genuinely fresh
operator terminal once.

Experiment 0042 passed. Two successive prompts preserved AGD's Nix Cargo,
Fortlet, Codex, and Tact paths; native and managed Codex returned 0.147.0; and
the one owned capsule moved from absent to running and back to absent through
public status, list, stop, and reset. AGD retained exactly its three authorized
environment-file changes. The absent-to-running managed version command took
approximately nine seconds; running-capsule reattachment latency was not
measured.

## Budget and authority

The engineering ceiling is 90 minutes. The smoke may use one immutable package,
one credential-free shim `--version` invocation, one owned capsule at a time,
zero model prompts, zero retries, and zero external money. Acceptance
authorizes one `project-local-shim-activation` bookmark and one PR targeting
`main` under FIP-0005. It also authorizes local edits to AGD's `flake.nix`,
`flake.lock`, and `devenv.nix` for Experiment 0040, but no AGD commit, branch,
push, PR, or product-code change. The operator remains the sole merge authority.

## Verification

Run the complete standard verification set from `docs/RUNBOOK.md` through
`nix develop` before publication readiness.

The final aarch64-darwin local gate passes: 88 unit tests and every enabled
integration test, formatting, strict all-target/all-feature Clippy,
conformance, the curated package, and the shim-activation Nix check. The two
stock-Codex compatibility tests remain explicitly ignored because they require
an external binary. Nix reports only the expected incompatible x86_64-linux
omission warning.
