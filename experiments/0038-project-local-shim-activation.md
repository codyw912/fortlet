# Experiment 0038: Project-local shim activation

Status: rejected — repeated path-flake evaluation was too slow and the native
command omitted the required argument separator
Design: FIP-0001, FIP-0002, and FIP-0005
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0037

## Baseline / Control

PR #12 is merged and its tree verified. The complete pre-implementation gate
passes through `nix develop`. Native Codex and AGD's existing devenv work
without Fortlet activation, and no global shell or Home Manager shim path is
configured.

## Hypothesis and Production Mechanism

A minimal Fortlet named Nix shell can prepend the existing immutable shim
directory for one directly executed command while preserving the caller's
project and later native harness path. The same activation output can be
composed into a devenv project and applied to the current fish after startup.

## Declared Scope

1. Freeze and build the final aarch64-darwin Fortlet package and activation
   output after the complete deterministic gate passes.
2. Start from the AGD root in the operator's existing devenv environment.
   Record the working directory and the native `codex` resolution before
   activation without inspecting shell startup files or credential content.
3. From the existing fish, use the immutable Fortlet `agents` dev shell to run
   one inspection command directly. Require the working directory and existing
   AGD development-tool resolution to remain usable, and require `fortlet`,
   `codex`, and `tact` to resolve from the declared immutable outputs.
4. Through the same direct shell command boundary, run
   `fortlet native codex --version` once and require the installed native Codex
   version, proving the activated shim directory is skipped without recursion.
5. With public Fortlet status absent and global inventory empty, run the
   activated `codex --version` once. It may create only the ordinary AGD Codex
   capsule and persistent Codex state; it must make no model prompt or AGD
   product edit.
6. Use the same immutable public CLI to stop and reset only that owned capsule.
   Require final AGD Codex absence and `no capsules`. Do not retry any failed
   step or substitute another shell.

## Alternatives

1. Add the shim path globally through Home Manager. Rejected until project-local
   daily use is proven.
2. Modify AGD's flake before the activation output is validated. Rejected
   because the first unit should distinguish Fortlet activation from a second
   repository change.
3. Use a Bash shell hook. Rejected because activation should be represented by
   Nix package paths and work unchanged in fish.

## Risks

1. A nested Nix environment may hide AGD's existing tool paths. Treat this as a
   failed composition result rather than editing shell startup state.
2. Alias-directory packaging could make native resolution recurse. The
   deterministic check must prove canonical directory identity before dispatch.
3. A stale owned capsule would invalidate attribution. Stop before mutation if
   preflight is not empty.

## Acceptance Criteria

1. Outside the child shell, native command resolution is unchanged.
2. Through direct `nix develop ...#agents -c` execution from fish, all three
   Fortlet commands resolve from immutable outputs and the AGD working
   directory and development tools remain available.
3. The native escape returns the native Codex version with no recursion.
4. The activated shim returns the pinned guest Codex version through Fortlet.
5. Public cleanup restores absence and empty inventory with no AGD diff.

## Budget and Plan

At most 15 minutes, one immutable package, one direct inspection command, one
native version command, one credential-free shim version command, one owned
capsule, zero model prompts, zero retries, and zero external money. Preflight,
inspect, exercise native, exercise shim, inspect, clean up, and close terminally
in that order.

## Rehearsal

The flake check must first prove exact shim layout, discovery, and native escape
with a fake later executable. The complete standard gate must pass before the
operator runs the live unit.

The deterministic activation check passes. A read-only composition probe ran
from AGD through its cached devenv and the local Fortlet `agents` shell. It
preserved `/Users/cody/dev/agd`, retained AGD's Nix Rust toolchain, resolved
`fortlet`, `codex`, and `tact` from the Fortlet outputs, and returned native
`codex-cli 0.147.0` through `fortlet native`. AGD's tracked tree remained
clean. No shim, capsule, credential lease, or provider request was started.

A separate normal child-fish probe established an important negative control:
Nix initially placed both shims first, but the operator's fish startup moved
`~/.local/bin` ahead and selected native Tact. The live unit therefore uses
direct `nix develop ...#agents -c <command>` execution. Durable ordinary
commands should use devenv/direnv activation after fish startup.

## Results

The operator began the declared unit from fish. Every command reported copying
the Fortlet checkout and its fileset source into the Nix store, and entering the
named environment took too long to be credible as a per-command daily loop.
The unit reached the native-escape step but the declared command was
`fortlet native codex --version`. Fortlet correctly rejected `--version` as an
argument to its own `native` subcommand because the required separator was
missing. The native harness did not start.

The unit stopped there. It never invoked the managed `codex --version` shim,
created no experiment-attributable capsule, made no model prompt, and did not
retry. The repository instructions contained the same incorrect native command
and are corrected with this record.

## Terminal Closure

Rejected. Copying a local path flake is expected Nix behavior, but putting that
evaluation in front of every harness and management command is not an acceptable
daily interface. The native error was a documentation and experiment-command
defect, not a resolver failure: the supported form is
`fortlet native codex -- --version`.

Experiment 0039 replaces both failed assumptions. It activates the already
built immutable outputs once in the existing fish, runs every Fortlet command
without another Nix evaluation, and uses the correct separator. Do not resume
this experiment.
