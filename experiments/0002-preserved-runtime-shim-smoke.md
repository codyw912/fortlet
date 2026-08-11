# Experiment 0002: Preserved-runtime packaged shim smoke

Status: declared
Design: FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0001 used package output
`/nix/store/6282j3n2vxhmyq1q1pnhskfvkar7k2m7-fortlet-0.1.0`. Both shims
entered Fortlet, but both provisioning capsules failed at VM creation with
`Internal(Vm(VmSetup(VmCreate)))`.

Read-only diagnosis established that Nix's `strip -S` fixup changed the
fixed-output MicroSandbox runtime. The release `msb` and the previously working
user installation had SHA-256
`799c8d50bf8281582fee5aa75613a79101ad7631310dc08b6c4045ae57985c02`
and carried `com.apple.security.hypervisor=true`; Experiment 0001's packaged
copy had SHA-256
`a88aab48442ec416c5bde44332121a31b1edd90376f252433490756cdb97379f`
and no entitlements.

The treatment is Jujutsu commit `85d4766b` (`preserve packaged MicroSandbox
runtime integrity`). Its immutable package output is
`/nix/store/xvhq837ac4fmw6144vl2magmlr8miqgl-fortlet-0.1.0`. Package checks
compare `msb` and libkrunfw byte-for-byte with the fixed-output release archive.
The installed files match, and `codesign` reports the Hypervisor and
library-validation entitlements on the installed `msb`.

## Hypothesis and Production Mechanism

Preventing Nix from stripping the bundled release runtime preserves the
Hypervisor entitlement required by MicroSandbox's macOS VM-creation path.
Therefore the packaged Codex shim will pass the VM-creation point that rejected
Experiment 0001 and report the pinned guest Codex version. Once that
falsification unit passes, the identically packaged Tact shim should report its
pinned guest version through the same fail-closed session path.

## Declared Scope

Run from `/Users/cody/dev/fortlet` with this exact shim-first path:

```text
/nix/store/xvhq837ac4fmw6144vl2magmlr8miqgl-fortlet-0.1.0/libexec/fortlet/shims:/usr/bin:/bin
```

Codex receives only `--version`. If and only if Codex exits zero with its
pinned guest version, Tact receives only `--version`. The immutable package,
project, credentials, MicroSandbox 0.6.8 runtime, harness versions, arguments,
order, and environment stay frozen.

Commands may create or attach only Fortlet-managed project-and-harness
capsules and Fortlet-owned local state. No prompt, repository mutation,
publication, remote execution, user shell mutation, native escape invocation,
or user-owned MicroSandbox state change is in scope.

## Alternatives

1. Run both harnesses regardless of Codex outcome. Rejected because another
   identical VM-creation failure would already reject the diagnosed mechanism;
   a Tact attempt would add no useful falsification.
2. Invoke the unpacked release `msb` directly. Rejected because it would not
   exercise Fortlet's packaged SDK path, shims, environment provisioning, or
   credential boundary.
3. Stop after static entitlement inspection. Rejected because signature
   presence does not itself prove that the host can create and enter a VM.

## Risks

1. VM creation may still fail for an independent host cause. Codex then settles
   the experiment as rejected; Tact will not run and no retry is allowed.
2. First-use provisioning may download pinned tool layers and consume time or
   fail on network availability. The exact failure is recorded without an
   adaptive retry.
3. A credential, mount, fallback, or ownership anomaly violates a hard
   invariant and terminates the experiment immediately.
4. A successful command may leave an allowed Fortlet-managed capsule running
   under its idle timeout. No unowned process will be stopped.

## Acceptance Criteria

1. The complete standard verification set and `nix run . -- doctor` pass before
   dispatch.
2. Codex exits zero and reports pinned guest version `0.147.0`; any other
   outcome rejects the treatment and prevents the Tact unit.
3. Conditional on Codex passing, Tact exits zero and reports pinned guest
   version `0.3.7`.
4. Each executed unit is followed by a read-only query for a running capsule
   with the matching Fortlet-managed harness label.
5. Neither unit prints a startup banner, executes a host harness, or exposes a
   credential value.
6. Both units must pass to accept the experiment. This remains a local release
   screen, not statistical reliability evidence.
7. The experiment terminates on Codex failure, a hard-invariant anomaly, both
   units completing, or the 30-minute elapsed budget expiring.

## Budget and Plan

Budget: zero money, zero paid quota, zero remote mutation, at most two command
units, zero retries, and at most 30 minutes elapsed after dispatch begins. Run
Codex and record its raw exit and output plus the filtered capsule result. Run
Tact only if the declared Codex acceptance criterion passes, then record the
same evidence and close the experiment terminally.

## Rehearsal

On 2026-08-11, before declaration:

1. A package build with the new integrity check but without `dontStrip` failed
   because the release and installed `msb` differed at byte 1794.
2. With `dontStrip`, the package build and its empty-environment entrypoint and
   byte-integrity checks passed.
3. The installed runtime hashes matched the fixed-output archive and macOS
   reported both required `msb` entitlements.
4. `cargo test`, formatting, strict Clippy, the explicit conformance test,
   exact-tree `nix flake check`, and `nix run . -- doctor` passed.
5. Doctor reported a ready SDK 0.6.8 host, valid credential metadata, the
   Jujutsu project, an outside-mount credential file, and both harnesses without
   printing credential values. Doctor does not create a VM.

The remaining contact-with-reality step is the declared packaged shim dispatch.

## Results

Pending.

## Terminal Closure

Pending.
