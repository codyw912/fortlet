# Experiment 0001: Packaged transparent shim smoke

Status: completed — rejected
Design: FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

The treatment revision is Jujutsu commit `29d42a82` (`implement optional
transparent harness shims`) on `aarch64-darwin`. Its immutable package output is
`/nix/store/6282j3n2vxhmyq1q1pnhskfvkar7k2m7-fortlet-0.1.0`.

Before dispatch on 2026-08-11, the complete Rust gates, exact-tree
`nix flake check`, and `nix run . -- doctor` passed. Doctor identified this
Jujutsu project, MicroSandbox SDK 0.6.8, a valid host ChatGPT credential, a
credential file outside guest mounts, and registered `codex` and `tact`
harnesses. The explicit `fortlet run` path is the established control; this
experiment changes only command entry through the package shim directory.

## Hypothesis and Production Mechanism

With the immutable shim directory first on a minimal `PATH`, its `codex` and
`tact` symlinks enter the Nix wrapper. The wrapper preserves `argv[0]`, so
Fortlet's multicall dispatcher selects the matching registered harness and
forwards `--version` through the existing fail-closed capsule-session
transaction. Each command should report its pinned Linux tool version and exit
zero without resolving or executing a host harness.

## Declared Scope

Two units will run, in fixed order: Codex, then Tact. Each receives only
`--version`, runs from `/Users/cody/dev/fortlet`, and uses this `PATH`:

```text
/nix/store/6282j3n2vxhmyq1q1pnhskfvkar7k2m7-fortlet-0.1.0/libexec/fortlet/shims:/usr/bin:/bin
```

The package output, project, host credentials, MicroSandbox version, harness
versions, arguments, and order stay frozen. The commands may create or attach
only Fortlet-managed project-and-harness capsules and Fortlet-owned local state.
No prompt, repository mutation, publication, remote execution, user shell
mutation, or native escape invocation is in scope.

After each unit, a read-only `msb list --running` query filtered by
`fortlet.managed=true` and the harness tool label will record whether a Fortlet
capsule is running. The query will not inspect credential or secret values.

## Alternatives

1. Rely only on the package install check. Rejected because it proves multicall
   dispatch reaches Fortlet but stops deliberately at missing test credentials;
   it does not contact the real local capsule boundary.
2. Invoke `fortlet run <harness> -- --version`. Rejected because that is the
   established control and does not exercise `PATH`, the package shim, or
   invocation-name dispatch.
3. Start an interactive harness session. Rejected because `--version` is the
   smaller deterministic screen and avoids prompts or agent work.

## Risks

1. First-use provisioning may download pinned tool layers and consume time or
   fail on network availability. A failed unit remains failed and is not
   retried.
2. A shim could recurse or resolve a host harness. Package checks already guard
   `argv[0]` dispatch; any output or process behavior inconsistent with the
   pinned guest version rejects that unit.
3. A credential or mount anomaly would violate a hard invariant. Any indication
   of secret output, guest-readable host credentials, unexpected mount scope,
   or non-Fortlet process execution terminates the experiment immediately.
4. A command could leave a Fortlet-managed capsule running under its declared
   idle timeout. That state is allowed by the charter and will be reported; no
   unowned process will be terminated.

## Acceptance Criteria

1. All pre-dispatch verification gates remain green.
2. The Codex unit exits zero and reports pinned version `0.147.0`.
3. The Tact unit exits zero and reports pinned version `0.3.7`.
4. After each unit, the read-only label query returns a running
   `fortlet.managed=true` capsule for the matching harness.
5. Neither unit prints a Fortlet startup banner, executes a host harness, or
   exposes a credential value.
6. Both units must pass to accept the treatment. A failed unit is recorded
   without retry; unless a hard invariant fires, the other unit still runs.
7. The experiment terminates on a hard-invariant anomaly, the two declared
   units completing, or the 30-minute elapsed-time budget expiring.

This is a two-unit local release screen, not statistical evidence about
reliability or performance.

## Budget and Plan

Budget: zero money, zero paid quota, zero remote mutation, two command units,
and at most 30 minutes elapsed after dispatch begins. Run Codex, record its raw
exit and output plus the filtered capsule name, then do the same for Tact. No
retry or adaptive argument change is allowed.

## Rehearsal

On 2026-08-11:

1. `cargo test`, formatting, strict Clippy, and the explicit conformance test
   passed in `nix develop`.
2. The package build passed its release tests and empty-environment entrypoint
   checks for `fortlet`, `codex`, and `tact`.
3. One earlier package check rejected a textual absolute-symlink assertion
   after Nix correctly normalized the link to relative form; target-equivalence
   replaced that invalid assertion and passed.
4. One exact-tree flake check found that filtered package sources omitted the
   newly cited `package.nix` conformance evidence; including that file made the
   corrected exact-tree `nix flake check` pass.
5. `nix run . -- doctor` passed without printing credential values.

This rehearses build, tests, installed entrypoint discovery, fail-closed shim
dispatch without credentials, conformance, and host-boundary diagnosis. The two
declared runner commands are the remaining contact-with-reality step.

## Results

Dispatch began after checkpoint `98b4b639` at 2026-08-11 12:09:43 EDT.

### Codex unit

Command:

```bash
env PATH=/nix/store/6282j3n2vxhmyq1q1pnhskfvkar7k2m7-fortlet-0.1.0/libexec/fortlet/shims:/usr/bin:/bin codex --version
```

Exit: `1`.

Output:

```text
fortlet: preparing _base bookworm-1 (first use)
fortlet: environment stage failed; check network access or remove the reported incomplete layer and retry: cannot create provisioning capsule fortlet-provision-20263-18cacc18083fdf20: failed to start "fortlet-provision-20263-18cacc18083fdf20": VM enter: build error: start: build_microvm: Internal(Vm(VmSetup(VmCreate)))
```

The declared running-capsule query exited `0` with no names. No Codex version
was produced and no retry was made.

### Tact unit

Command:

```bash
env PATH=/nix/store/6282j3n2vxhmyq1q1pnhskfvkar7k2m7-fortlet-0.1.0/libexec/fortlet/shims:/usr/bin:/bin tact --version
```

Exit: `1`.

Output:

```text
fortlet: preparing _base bookworm-1 (first use)
fortlet: environment stage failed; check network access or remove the reported incomplete layer and retry: cannot create provisioning capsule fortlet-provision-20270-18cacc1e2c6579e0: failed to start "fortlet-provision-20270-18cacc1e2c6579e0": VM enter: build error: start: build_microvm: Internal(Vm(VmSetup(VmCreate)))
```

The declared running-capsule query exited `0` with no names. No Tact version
was produced and no retry was made.

Both immutable packaged shims demonstrably entered Fortlet: each emitted the
Fortlet first-use message and the staged environment error, and neither emitted
a host harness version. Both stopped before a project-and-harness capsule or
guest harness existed. No credential value or unexpected mount information was
printed.

## Terminal Closure

1. Outcome: rejected — both packaged shims entered Fortlet, but neither could
   pass base-layer provisioning to execute its guest harness.
2. Root cause: MicroSandbox SDK diagnosis passed, but actual provisioning VM
   creation failed twice with `Internal(Vm(VmSetup(VmCreate)))`. The repeated
   failure invalidates the assumption that doctor health establishes a usable
   VM-creation boundary and activates the charter escalation trigger.
3. Actual total cost: zero money, zero paid quota, zero remote mutation, two of
   two declared units, zero retries, and 70 seconds elapsed from declaration to
   terminal observation versus the 30-minute ceiling.
4. Next action: stop. The operator must choose whether to resume with a bounded
   MicroSandbox VM-creation diagnosis before any successor smoke experiment.
