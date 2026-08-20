# Experiment 0039: Prebuilt fish shim activation

Status: rejected — AGD's direnv prompt hook removed the manual PATH activation
before command inspection
Design: FIP-0001, FIP-0002, and FIP-0005
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0038

## Baseline / Control

Experiment 0038 stopped before native or managed Codex launch. It established
that repeated `nix develop path:...` evaluation is too slow for the intended
loop and that the native command omitted Fortlet's required `--` separator.
The complete local gate already produced and verified these immutable outputs:

- Fortlet: `/nix/store/qs1a28yz1nagpx3xjmgfznw0ivvz0glc-fortlet-0.1.0`
- activation: `/nix/store/dg0hyd58fp3ylkw64gdw4hyw0216sn47-fortlet-shim-activation-0.1.0`

Both paths are present. The activation `bin` resolves to the named Fortlet
package's existing `libexec/fortlet/shims` directory.

## Hypothesis and Production Mechanism

Prepending those two immutable outputs once to the current fish `PATH` provides
immediate project-local Fortlet commands without Nix evaluation, shell startup
edits, aliases, functions, or a child shell. The existing native resolver will
skip the canonical shim directory and select native Codex after the required
argument separator.

## Declared Scope

1. Start at the AGD root in the operator's existing fish and devenv. Require
   the four outer-Codex markers absent, AGD Codex status absent, global Fortlet
   inventory `no capsules`, and a clean AGD tracked tree.
2. Save the current fish `PATH` in one non-exported session variable. Prepend
   exactly the activation `bin` and Fortlet `bin` paths above to the current
   exported `PATH`; do not start another shell or evaluate Nix.
3. Require `cargo` to retain AGD's project toolchain resolution and require
   `fortlet`, `codex`, and `tact` to resolve through the exact immutable paths.
4. Run `fortlet native codex -- --version` once and require exact native
   `codex-cli 0.147.0` without recursion.
5. Run managed `codex --version` once. It may create only the ordinary AGD Codex
   capsule and persistent Codex state; it must make no model prompt or AGD edit.
6. Inspect status and inventory, then use the same immutable public CLI to stop
   and reset only the AGD Codex capsule. Require final absence, `no capsules`,
   and a clean AGD tracked tree.
7. Restore the saved fish `PATH` exactly and remove only the experiment's fish
   variables. Do not retry any failed step.

## Alternatives

1. Repeat the local `nix develop` wrapper with the corrected separator.
   Rejected because Experiment 0038 already rejected its per-command latency.
2. Commit Fortlet to AGD's devenv before the smoke. Deferred so this unit proves
   the exact activation output without a second repository diff.
3. Configure Home Manager globally. Rejected until project-local use passes.

## Risks

1. A stale capsule invalidates attribution; stop before activation if preflight
   is not empty.
2. An incorrectly restored `PATH` would alter the operator's shell session.
   Snapshot it before mutation and restore the exact fish list at the end.
3. The unrooted immutable paths could be garbage-collected. Stop if either is
   absent; do not rebuild or substitute another output inside this unit.

## Acceptance Criteria

1. Activation and all commands after it perform zero Nix evaluations or source
   copies.
2. AGD's working directory and Cargo remain intact while all Fortlet commands
   resolve from the exact immutable outputs.
3. Native escape returns Codex 0.147.0 and the managed shim returns the pinned
   guest version.
4. Public cleanup restores absence and empty inventory with no AGD diff.
5. Restoring `PATH` returns native `codex` and `tact` resolution exactly.

## Budget and Plan

At most five minutes, two existing immutable outputs, one PATH activation, one
native version command, one credential-free shim version command, one owned
capsule, zero Nix evaluations, zero retries, zero model prompts, and zero
external money. Preflight, activate, inspect, exercise native, exercise shim,
clean up, restore, and close terminally in that order.

## Rehearsal

The deterministic check already proves directory identity, both shim names,
argument preservation, and native selection after the shim. Experiment 0038
proves the per-command Nix path is unsuitable and did not reach the shim. The
successor changes only activation lifetime and the missing separator.

## Results

The operator established the exact clean baseline: all four outer-Codex markers
were absent, packaged AGD Codex status was absent, global inventory was
`no capsules`, and `git status --short` was empty. Both declared store paths
were assigned, and the operator saved and prepended PATH as declared.

The next fish prompt ran AGD's direnv hook and restored its cached environment,
removing the manual activation before inspection. The project remained
`/Users/cody/dev/agd`, but `type -p` resolved Cargo to
`/Users/cody/.cargo/bin/cargo`, printed no `fortlet`, and found Codex and Tact
under an older Fortlet output already present later on PATH. The corrected
native command then failed immediately with `fish: Unknown command: fortlet`.

No native Codex process, managed shim, capsule, credential lease, model prompt,
or repository edit was started. The operator was directed to restore the exact
saved PATH and remove the experiment variables. The unit did not retry.

## Terminal Closure

Rejected. The immutable package and activation outputs are present and paired
correctly; this was not garbage collection or another package identity. A
manual PATH mutation in the current fish is not durable while direnv owns that
project environment and reapplies it at the next prompt.

Together, Experiments 0038 and 0039 reject both ad-hoc activation paths: repeat
the local flake around every command, or mutate PATH outside the project
environment owner. Stop further live work under this experiment. The next
mechanism should put Fortlet's two outputs inside AGD's devenv so direnv applies
and caches them itself; that is a cross-repository scope decision, not another
shell workaround.
