# Experiment 0040: AGD devenv Fortlet activation

Status: rejected — interactive mise refresh replaced direnv's activated PATH
Design: FIP-0001, FIP-0002, and FIP-0005
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0039

## Baseline / Control

Experiments 0038 and 0039 both stopped before any harness launch. They rejected
repeated local-path flake evaluation and manual PATH mutation outside AGD's
direnv-owned environment. The operator restored the saved PATH and explicitly
authorized a separate AGD devenv change.

Fortlet draft revision `803ce20b951863db0ce8152588bef740f6f95647` is public
on the `project-local-shim-activation` branch. Its deterministic activation
check and complete implementation gate pass. AGD begins clean on `main` with an
existing flake-backed devenv and no Fortlet input.

## Hypothesis and Production Mechanism

If AGD pins that exact public Fortlet revision and includes its `fortlet` and
`shim-activation` outputs in devenv's package list, direnv will own the complete
PATH update after fish startup. The first activation may fetch, evaluate, and
build once; an unchanged second activation should use the cached environment
without copying the local Fortlet checkout or rebuilding the package.

## Declared Scope

1. Change only AGD's `flake.nix`, generated `flake.lock`, and `devenv.nix`.
   Add exact GitHub revision
   `803ce20b951863db0ce8152588bef740f6f95647`, follow AGD's `nixpkgs`, and add
   the two Fortlet outputs only on systems Fortlet exposes.
2. Do not change AGD Rust code, tests, project behavior, hooks, services,
   environment variables, remote state, commits, branches, or publication.
3. Regenerate the lock through Nix. Run one cold `direnv exec` inspection from
   AGD and require its root, Rust toolchain, and exact Fortlet command paths.
4. Run the same credential-free inspection again. Require cached activation
   without a Fortlet source copy or package rebuild and record both durations.
5. Through the cached environment, run
   `fortlet native codex -- --version` once and require native Codex 0.147.0.
   Do not invoke the managed shim in this agent-run phase.
6. Require the AGD diff to contain only the three declared environment files.
   Hand the operator one `direnv reload`, command-resolution check, managed
   `codex --version`, and public stop/reset sequence. Do not retry a failed
   build, activation, or command.

## Alternatives

1. A third shell or PATH workaround. Rejected by the terminal predecessor
   evidence and workflow stop.
2. Use a local `path:` Fortlet input. Rejected because official devenv guidance
   confirms it copies the entire directory; the exact public GitHub revision is
   smaller, immutable, and reviewable.
3. Publish or merge either repository first. Rejected because a draft revision
   is sufficient for local integration evidence and merge authority remains
   with the operator.

## Risks

1. Fortlet supports only aarch64-darwin and x86_64-linux while AGD declares four
   host systems. Conditional package inclusion must preserve AGD evaluation on
   unsupported systems.
2. The initial remote package build may remain slow. Distinguish one-time cold
   work from unchanged cached re-entry; reject the daily path if both are slow.
3. The draft branch reference will not be AGD's final durable input after
   Fortlet squash merge. Pinning the exact revision is valid for this unit but
   must be updated before any AGD publication.

## Acceptance Criteria

1. AGD's declared environment files are the only diff and the lock pins the
   exact Fortlet revision with `nixpkgs` following AGD.
2. The cold environment resolves AGD Cargo and all three Fortlet commands from
   Nix outputs without local Fortlet source copying.
3. Unchanged re-entry uses the cache with no Fortlet build and acceptable
   command latency.
4. Native escape returns Codex 0.147.0 without recursion.
5. The operator's later managed version smoke begins from empty inventory,
   creates at most one AGD Codex capsule, cleans up publicly, and leaves AGD's
   declared environment-only diff unchanged.

## Budget and Plan

At most 20 minutes, one exact public flake fetch, one cold environment build,
one cached re-entry, one native version command, zero managed shims in the
agent-run phase, zero model prompts, zero retries, and zero external money.
Edit, lock, inspect cold, inspect cached, exercise native, inspect diff, and
hand off in that order.

## Rehearsal

The Fortlet activation output, directory identity, and native lookup have
deterministic coverage. A prior read-only nested composition probe preserved
AGD's root and Rust toolchain. This experiment changes ownership of activation:
AGD's devenv and direnv apply it directly instead of an outer wrapper.

## Results

AGD now has exactly the three declared uncommitted environment changes. Its
lock pins Fortlet revision
`803ce20b951863db0ce8152588bef740f6f95647`, with Fortlet's `nixpkgs` input
following AGD's. No AGD product file, commit, branch, or remote state changed.

The single cold inspection succeeded in 406.93 seconds. It built Fortlet once
against AGD's followed Nix inputs, without copying the local Fortlet checkout,
then resolved:

- Cargo from AGD's Nix Rust 1.94.0 toolchain;
- Fortlet from its immutable `fortlet-0.1.0` output; and
- Codex and Tact from the immutable `fortlet-shim-activation-0.1.0` output.

The one unchanged re-entry reported `nix-direnv: Using cached dev shell`,
performed no Fortlet copy or build, resolved the same four commands, and
completed in 0.51 seconds. The native escape then returned
`codex-cli 0.147.0` without recursion. The agent invoked no managed shim,
created no capsule, sent no model prompt, and used no retry.

The cold cost is substantial but one-time for this pinned input set. The cached
result met the non-interactive activation criterion but did not prove the
operator's actual interactive fish path.

The operator then ran the declared unit without retry. `direnv reload`
successfully renewed the cache and ran all devenv entry tasks, but the next
prompt resolved host Cargo, omitted `fortlet`, and retained older Fortlet shim
paths. Consequently `fortlet native codex -- --version` failed with
`fish: Unknown command: fortlet`. The unit stopped there: it did not inspect an
absent baseline, invoke the managed shim, create a capsule, send a prompt, or
run cleanup.

Initial read-only inspection found two independently active fish prompt
integrations and suggested that mise reconstructed its earlier baseline after
direnv's successful export. Successor Experiment 0041 falsified that as a
general ordering defect: a new fish with the exact generated configuration
preserved all four Nix paths across two prompt cycles. The narrower attribution
is stale, internally inconsistent direnv/mise state in the long-lived operator
shell after Experiments 0039 and its manual PATH restoration. This still
explains all three observed path identities and distinguishes the failure from
Fortlet packaging or AGD flake evaluation.

Reject this experiment because project-local devenv composition does not yet
survive the operator's real mise-plus-direnv prompt cycle. Preserve the AGD
environment-only diff for diagnosis, but do not retry this record or launch a
managed shim under it.
