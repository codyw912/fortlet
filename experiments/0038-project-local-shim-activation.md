# Experiment 0038: Project-local shim activation

Status: planned
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
directory for one child fish process while preserving the caller's project and
later native harness path. The same activation output can be composed into a
devenv project without shell-specific initialization.

## Declared Scope

1. Freeze and build the final aarch64-darwin Fortlet package and activation
   output after the complete deterministic gate passes.
2. Start from the AGD root in the operator's existing devenv environment.
   Record the working directory and the native `codex` resolution before
   activation without inspecting shell startup files or credential content.
3. Enter exactly one child fish with the immutable Fortlet `agents` dev shell.
   Require the working directory and existing AGD development-tool resolution
   to remain usable, and require `fortlet`, `codex`, and `tact` to resolve from
   the declared immutable outputs.
4. Run `fortlet native codex --version` once and require the installed native
   Codex version, proving the activated shim directory is skipped without
   recursion.
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

1. A nested Nix shell may hide AGD's existing tool paths. Treat this as a
   failed composition result rather than editing shell startup state.
2. Alias-directory packaging could make native resolution recurse. The
   deterministic check must prove canonical directory identity before dispatch.
3. A stale owned capsule would invalidate attribution. Stop before mutation if
   preflight is not empty.

## Acceptance Criteria

1. Outside the child shell, native command resolution is unchanged.
2. Inside fish, all three Fortlet commands resolve from immutable outputs and
   the AGD working directory and development tools remain available.
3. The native escape returns the native Codex version with no recursion.
4. The activated shim returns the pinned guest Codex version through Fortlet.
5. Public cleanup restores absence and empty inventory with no AGD diff.

## Budget and Plan

At most 15 minutes, one immutable package, one child fish, one native version
command, one credential-free shim version command, one owned capsule, zero
model prompts, zero retries, and zero external money. Preflight, enter, inspect,
exercise native, exercise shim, inspect, clean up, and close terminally in that
order.

## Rehearsal

The flake check must first prove exact shim layout, discovery, and native escape
with a fake later executable. The complete standard gate must pass before the
operator runs the live unit.

