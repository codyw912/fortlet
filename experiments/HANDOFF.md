# Session Handoff — Transparent shim mission complete

Audience: a fresh agent session. `GOAL.md` is normative and complete. Do not
start follow-on implementation until the operator and agent choose a new goal.

## Completed outcome

1. FIP-0002 is Accepted and defines optional package-owned `codex` and `tact`
   shims plus the explicit recursion-safe `fortlet native` escape hatch.
2. Shim activation is only a user-controlled `PATH` decision. Fortlet never
   writes shell startup files, uses shell aliases or functions, or silently
   falls back to a host harness.
3. Checkpoint `29d42a82` implements invocation-name dispatch, native execution,
   staged failures, package shims, and focused tests.
4. Checkpoint `85d4766b` preserves the fixed-output MicroSandbox runtime and
   adds byte-for-byte package checks for `msb` and libkrunfw.

## Packaging failure and repair

Experiment 0001 rejected the original package after both shims reached Fortlet
but failed with `Internal(Vm(VmSetup(VmCreate)))`. Read-only diagnosis found
that Nix's `strip -S` fixup changed the release runtime and removed `msb`'s
`com.apple.security.hypervisor` entitlement even though `doctor` remained
green. The release archive was byte-identical to the previously working user
installation.

The repair disables stripping for the Fortlet output and compares both runtime
files with the immutable archive during install checks. Repaired package output
`/nix/store/xvhq837ac4fmw6144vl2magmlr8miqgl-fortlet-0.1.0` retains the exact
release hashes and Hypervisor entitlement.

## Accepted runtime evidence

Experiment 0002 is terminally accepted:

1. Packaged `codex --version` crossed VM creation, provisioned `_base` and Codex,
   returned `codex-cli 0.147.0`, and left a running Fortlet-managed Codex
   capsule.
2. Conditional packaged `tact --version` returned `tact 0.3.7` for
   `aarch64-unknown-linux-gnu` and left a running Fortlet-managed Tact capsule.
3. Both commands exited zero, used the immutable shim directory, emitted no
   credential values, and did not resolve a host harness.
4. Actual experiment cost was zero money, two units, zero retries, and 80
   seconds versus the 30-minute ceiling.

## Verification and honest gaps

The complete standard verification set and `nix run . -- doctor` passed on
`aarch64-darwin`. Package install checks exercise empty-environment discovery,
fail-closed dispatch, and runtime byte integrity. Native `x86_64-linux` package
verification remains outstanding.

FIP-0002 remains partial because interactive TTY, resize, and signal behavior
is inherited from the explicit session path but has not been independently
exercised through the shims. FIP-0001 retains its broader lifecycle,
standalone-distribution, adapter-ownership, project/broad-root, terminal, and
publication gaps in `arch/conformance.json`.

## Next action

Choose a new `GOAL.md` with the operator. Preserve the optional activation,
credential boundary, no-fallback rule, package runtime-integrity check, and
Jujutsu checkpoint discipline. Do not turn the remaining conformance list into
an inferred mission; prioritize the next daily-use bottleneck together.
