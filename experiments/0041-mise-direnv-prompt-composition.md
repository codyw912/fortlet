# Experiment 0041: mise and direnv prompt composition

Status: rejected — fresh combined hooks did not reproduce the live-shell loss
Design: FIP-0001, FIP-0002, and FIP-0005
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0040

## Baseline / Control

Experiment 0040 proved the AGD flake and cached devenv output outside the
operator's normal fish initialization, then failed when the exact interactive
shell omitted Fortlet after a successful direnv reload. Home Manager activates
mise and direnv as independent `fish_prompt` handlers. Mise retains the path
that predates direnv in `__MISE_ORIG_PATH` and its encoded hook diff.

## Hypothesis and Production Mechanism

If the project environment makes the newly activated devenv PATH mise's input
baseline, later mise prompt refreshes will prepend its tools without discarding
AGD or Fortlet paths. The production mechanism, if the control passes, is one
guarded environment-state adjustment in AGD's existing devenv entry: only an
active mise shell loses its stale encoded diff and adopts the current devenv
PATH as its original path. Direnv will restore the prior values on project
exit.

## Declared Scope

1. Run an isolated fish with the operator's real Home Manager configuration and
   AGD's cached direnv environment. Emit two prompt cycles and require the
   second to reproduce host Cargo, missing Fortlet, and older shims.
2. In a separate isolated process, apply the proposed mise baseline adjustment
   after initial devenv activation, emit two more prompt cycles, and require
   AGD Cargo, Fortlet, Codex, and Tact to remain resolved.
3. Invoke no Fortlet command, managed shim, VM, capsule, model, build, lock
   update, or network fetch in the control phase.
4. Only if both controls pass, change the already-authorized AGD `devenv.nix`
   with the guarded adjustment. Keep the AGD diff limited to its existing three
   environment files and do not commit, branch, push, or publish AGD.
5. Run one cached non-interactive inspection after the edit. A later operator
   unit gets a new shell process and one resolution/native/managed sequence;
   no failed command is retried.

## Alternatives

1. Retry `direnv reload`. Rejected because the unchanged environment is not
   reasserted after mise's later rewrite and Experiment 0040 is terminal.
2. Change global Home Manager fish or mise configuration. Rejected because the
   goal requires project-local opt-in and the operator manages that state with
   Nix.
3. Add static Nix store paths to mise configuration. Rejected because store
   identities change with the locked input and should remain owned by devenv.

## Risks

1. Mise's double-underscore variables are integration internals. Guard the
   adjustment to active mise shells, keep it local to AGD, and record this as a
   compatibility experiment rather than a Fortlet public contract.
2. An isolated fish may not reproduce the operator's terminal. Require the
   known failure signature before trusting the candidate control.
3. Resetting the diff could duplicate paths. Require stable resolution across
   two post-adjustment prompt cycles.

## Acceptance Criteria

1. The untreated control reproduces Experiment 0040 on its second prompt.
2. The treated control preserves all four Nix-resolved commands across two
   later prompts, with the activation output ahead of native Codex and Tact.
3. The AGD edit is guarded, project-local, and confined to `devenv.nix`; the
   total AGD diff remains the three already-authorized environment files.
4. Cached evaluation succeeds without a Fortlet rebuild or local checkout
   copy before operator handoff.

## Budget and Plan

At most 10 minutes, two isolated fish processes, four explicit prompt events,
one AGD file edit, one cached inspection, zero retries, zero harness commands,
zero VMs, zero model prompts, zero network fetches, and zero external money.
Reproduce, treat, compare, edit, inspect, and hand off in that order.

## Rehearsal

The complete pre-edit control is the rehearsal: it includes the real generated
fish configuration, both hooks, cached direnv export, repeated prompt events,
and final command resolution without starting Fortlet.

## Results

The untreated control ran once in 0.79 seconds and used AGD's cached dev shell.
Its first prompt resolved Nix Rust 1.94.0 Cargo, Fortlet, Codex, and Tact from
the expected package outputs. Its second prompt resolved the same four paths.
It therefore did not reproduce Experiment 0040's host Cargo, missing Fortlet,
or older shims.

The experiment stopped at acceptance criterion 1. It did not run the proposed
treatment, edit AGD, invoke Fortlet or a shim, rebuild, fetch, create a capsule,
or send a model prompt. Actual cost was one isolated fish process, two prompt
events, 0.79 seconds of command time, and zero external money.

Reject the hypothesis that the ordinary generated mise-plus-direnv hook order
discards devenv PATH. The difference is the operator's long-lived shell state,
which had undergone Experiment 0039's manual PATH prepend and restoration
before AGD's flake changed. A new shell process is the cheapest falsification
of that state attribution; no compatibility edit is justified.
