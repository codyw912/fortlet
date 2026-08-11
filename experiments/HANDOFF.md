# Session Handoff — Transparent shims implemented; live smoke blocked

Audience: a fresh agent session. `GOAL.md` is normative and currently blocked
at Deliverable 3 by the charter's repeated-failure escalation trigger. Do not
retry or alter MicroSandbox runtime state until the operator responds.

## Program context

1. FIP-0002 is Accepted and defines optional package-owned `codex` and `tact`
   shims plus the explicit `fortlet native` escape hatch.
2. The shim directory is activated only by user-controlled `PATH` ordering;
   Fortlet never writes shell startup files.
3. Explicit `fortlet doctor` and `fortlet run` use remains supported without
   shim activation.

## Verified implementation state

1. Checkpoint `29d42a82` implements invocation-name dispatch, direct
   recursion-safe native execution, staged actionable launch errors, Nix-owned
   shims, empty-environment package checks, focused tests, and partial
   conformance.
2. `cargo test`, formatting, strict Clippy, the explicit conformance test, and
   exact-tree `nix flake check` passed before live dispatch on
   `aarch64-darwin`. Native `x86_64-linux` verification remains outstanding.
3. `nix run . -- doctor` passed immediately before dispatch: MicroSandbox SDK
   0.6.8, host authentication, project resolution, and the credential boundary
   were healthy without secret output.
4. Package output
   `/nix/store/6282j3n2vxhmyq1q1pnhskfvkar7k2m7-fortlet-0.1.0` contains both
   shims. Package checks prove each name enters Fortlet under an empty
   environment and stops at credentials rather than resolving a host harness.

## Experiment 0001 terminal result

1. Both predeclared commands used the immutable shim directory first on a
   minimal `PATH` and passed only `--version`.
2. Both shims entered Fortlet and attempted `_base bookworm-1` provisioning.
3. Both failed before guest harness launch with
   `Internal(Vm(VmSetup(VmCreate)))` while creating a provisioning capsule.
4. Read-only label queries found no running Fortlet Codex or Tact capsule.
5. No retries, credential exposure, host fallback, repository mutation, remote
   effect, or unowned cleanup occurred.
6. The record is terminally rejected in
   `experiments/0001-packaged-transparent-shims.md`.

## Why work stopped

The two declared units are consecutive terminal failures sharing the same
assumption: a healthy MicroSandbox diagnosis implies the host can create a VM.
The charter therefore requires operator escalation. The implementation gates
are green, but the goal's end-to-end guest-launch evidence is not.

## Next action requiring operator direction

Choose whether to resume with a bounded diagnosis of local MicroSandbox VM
creation. A successor runtime attempt must not silently retry Experiment 0001;
if authorized, declare a new experiment or an explicitly diagnostic mission as
appropriate, preserve the credential boundary, and do not mutate unowned
runtime state.

## Conventions that remain binding

1. Use Jujutsu checkpoints and inspect `main..@` before handoff.
2. Keep successful wrapper startup silent and failures stage-specific with one
   actionable next step.
3. Never fall back from isolated launch to a host harness.
4. Do not mount provider credentials, SSH keys, signing agents, or publication
   authority into capsules.
5. Do not broaden this into lifecycle, alternative runtime, remote execution,
   or publication work.
