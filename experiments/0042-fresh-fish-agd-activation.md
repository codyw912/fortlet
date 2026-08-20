# Experiment 0042: Fresh-fish AGD activation

Status: accepted
Design: FIP-0001, FIP-0002, and FIP-0005
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0041

## Baseline / Control

Experiment 0040 failed in the operator's long-lived fish after earlier manual
PATH mutation. Experiment 0041 used the exact generated Home Manager fish
configuration in a new process and preserved AGD Cargo, Fortlet, Codex, and
Tact across two real prompt events without any compatibility treatment.

## Hypothesis and Production Mechanism

If the operator starts a genuinely new terminal process outside AGD and then
enters the project, ordinary mise initialization followed by direnv activation
will reproduce the passing control. The production mechanism remains AGD's
unchanged devenv input and packages; no shell-specific Fortlet mechanism is
added.

## Declared Scope

1. Use a new terminal tab or window, not `direnv reload` in the existing shell.
   Enter AGD once and inspect Cargo, Fortlet, Codex, and Tact twice across
   ordinary prompts.
2. Require Nix-resolved Cargo and Fortlet plus activation-output Codex and Tact
   both times. Stop immediately if any identity changes or is absent.
3. Run native Codex version once, then require an absent Codex capsule and empty
   global inventory before one managed `codex --version`.
4. Observe the capsule, then use only public stop/reset commands to restore
   absence and empty inventory. Do not retry a failed command.
5. Keep AGD's diff limited to `flake.nix`, `flake.lock`, and `devenv.nix`; do
   not edit, commit, branch, push, or publish AGD.

## Alternatives

1. Repair the existing shell state. Rejected because it would test recovery
   from experimental mutation rather than normal daily entry.
2. Add a mise-specific devenv workaround. Rejected because Experiment 0041
   showed the untreated fresh configuration is stable.
3. Use a per-command `direnv exec`. Rejected because it omits the operator's
   intended interactive daily path.

## Risks

1. A terminal application may restore process environment rather than create a
   clean login shell. Start the tab or window from the terminal application,
   not as a child of the contaminated shell.
2. The managed version command can create one owned capsule. Verify the absent
   baseline first and clean it publicly afterward.

## Acceptance Criteria

1. Two successive interactive inspections resolve the same four expected Nix
   commands.
2. Native escape returns Codex 0.147.0 without recursion.
3. One managed version command succeeds and creates at most one AGD Codex
   capsule without a model prompt.
4. Public stop/reset restores Codex absence and `no capsules`.
5. AGD retains only its three environment-file changes.

## Budget and Plan

At most 10 minutes, one fresh terminal process, one project entry, two path
inspections, one native version, one managed version, one owned capsule, zero
model prompts, zero retries, zero repository edits, and zero external money.
Enter, inspect twice, exercise native, verify baseline, exercise managed,
observe, clean up, and inspect the diff in that order.

## Rehearsal

Experiment 0041 rehearsed the exact generated fish configuration, cached
direnv activation, and two prompt cycles in a new process. It resolved all four
commands consistently without invoking Fortlet.

## Results

The operator opened a genuinely new terminal process and entered AGD once.
Two successive inspections returned identical paths:

- Cargo from Nix Rust 1.94.0;
- Fortlet from the pinned `fortlet-0.1.0` output; and
- Codex and Tact from `fortlet-shim-activation-0.1.0`.

The native escape returned `codex-cli 0.147.0`. Public status and inventory
then established `codex` absent and `no capsules`. The sole managed
`codex --version` returned `codex-cli 0.147.0` in approximately nine seconds,
after which public status and inventory showed exactly one running AGD Codex
capsule. No model prompt ran.

Public stop reported `stopped`, reset reported `reset`, and final status and
inventory returned to `codex` absent and `no capsules`. AGD's final diff
contains only the three declared environment files: `devenv.nix`,
`flake.lock`, and `flake.nix`. No retry, repository edit during the unit,
commit, branch, push, publication, credential output, or external money was
used.

Accept the production mechanism. A fresh normal project entry survives the
operator's mise-plus-direnv prompt cycle without shell-specific Fortlet code.
The approximately nine-second absent-to-running managed version command is an
observed first-capsule startup cost. This unit did not collect component timing
or measure reattachment to the running capsule, so it makes no narrower latency
claim.
