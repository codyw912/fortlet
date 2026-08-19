# Experiment 0031: Login-shell project PATH preservation

Status: accepted — 2026-08-19
Design: FIP-0001 and FIP-0006
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0030's one model-backed work unit streamed and edited successfully,
but Codex's `/usr/bin/bash -lc` child could not find `cargo`. The independent
host test passed, and public cleanup restored both Codex absence and an empty
MicroSandbox inventory. Experiment 0020 previously proved that the same
published project layer directly exposed and executed Cargo 1.97.1, Rust 1.97.1,
and Jujutsu 0.43.0 inside a managed capsule.

The immutable diagnostic package remains
`/nix/store/1xkx1xk7fd97ba5f1nlddq9bjilfdvbj-fortlet-0.1.0`. The project
environment inputs and published identity
`a0e90418dca950e944e6243c446e93eb977061cd73333da4996a8d560d021252`
are unchanged. Host inspection sees the published tool files as mode 0600, so
host mode display and login-shell PATH replacement are competing mechanisms.

## Hypothesis and Production Mechanism

The mounted tools remain executable in the guest and Fortlet's injected PATH
reaches direct and non-login child commands. Debian's `/usr/bin/bash -lc`
replaces that PATH while processing the login profile, which removes
`/opt/fortlet/project/bin` before Codex's command runs. A Fortlet-owned
`BASH_ENV` file read after login-profile processing can restore the exact
managed PATH without changing any user shell startup file.

## Declared Scope

1. Require a clean working copy, packaged Tact absence, and empty packaged
   MicroSandbox inventory.
2. Launch the immutable package once with the credential-free public command:

   ```text
   fortlet run tact --project /Users/cody/dev/fortlet -- --version
   ```

3. Resolve the sole owned Tact capsule and run one direct non-TTY guest process.
   It records guest mode and resolution for Cargo under the inherited shell,
   `/usr/bin/bash -c`, `/usr/bin/bash -lc`, and `/usr/bin/bash -lc` with a fixed
   disposable `BASH_ENV` file. That file contains only:

   ```sh
   PATH=$FORTLET_MANAGED_PATH
   export PATH
   ```

   The probe records each PATH, `command -v cargo`, and `cargo --version`. A
   missing Cargo result is rendered explicitly rather than terminating before
   the remaining comparison arms.
4. Use only packaged public status/stop/reset for cleanup. Require final Tact
   absence and empty inventory.

The probe may write only `/tmp/fortlet-exp0031-bash-env` inside the disposable
capsule. It sends no provider credential, model request, prompt, repository
content, or network request and makes no repository or host configuration
change. No Codex launch, second capsule, retry, user startup-file write, or
production implementation is in scope.

## Alternatives

1. Treat host mode 0600 as root cause. Rejected because prior direct guest
   execution contradicts that inference and guest mount translation must be
   measured directly.
2. Send Codex another prompt that prints PATH. Rejected because Experiment 0030
   consumed the sole authorized provider unit and a model request is unnecessary.
3. Edit `.bashrc`, `.profile`, or another persistent startup file. Rejected
   because the operator uses declarative shell configuration and Fortlet must
   not take ownership of user startup files.

## Risks

1. Tact may alter the environment before returning its version. The direct
   probe reads the capsule's configured process environment independently.
2. `BASH_ENV` may be ignored for a non-interactive login shell. That rejects the
   proposed mechanism without changing production.
3. Guest mode may be non-executable, making permission translation causal. The
   direct resolution/version arms distinguish that result.

## Acceptance Criteria

1. Baselines are clean and the sole public launch exits zero without credentials
   or network traffic.
2. Guest Cargo has at least one execute bit, resolves below
   `/opt/fortlet/project/bin`, and reports 1.97.x directly and through
   `/usr/bin/bash -c`.
3. Plain `/usr/bin/bash -lc` drops the project path and reports Cargo absent.
4. The fixed `BASH_ENV` arm restores the exact inherited managed PATH, resolves
   the same Cargo path, and reports 1.97.x.
5. Public cleanup restores Tact absence and empty inventory. Any differing arm,
   unexpected mutation, or cleanup failure rejects the experiment without retry.

## Budget and Plan

Zero money, zero paid quota, zero model prompts, one immutable Tact version
launch, one owned capsule, one direct comparison process, zero retries, and at
most 10 elapsed minutes. Diagnose, clean up, then either implement the proven
internal environment mechanism under FIP-0006 or stop.

## Rehearsal

The comparison uses the already published and verified project layer from
Experiment 0020. Experiment 0030 independently proved current public cleanup,
empty final inventory, and host execution of the focused test. No live process
has run under this declaration.

## Results

The final baseline was clean: Tact was absent and packaged MicroSandbox
inventory was `[]`. The sole public launch exited zero in about three seconds
and printed Tact 0.3.7 with Rust 1.97.1. It created exactly one running owned
Tact capsule.

The one direct comparison process exited zero and reported:

```text
direct_path=/.msb/scripts:/opt/fortlet/project/bin:/opt/fortlet/base/usr/bin:/opt/fortlet/tool/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
direct_cargo=/opt/fortlet/project/bin/cargo
direct_mode=755
cargo 1.97.1 (c980f4866 2026-06-30)
nonlogin_path=/.msb/scripts:/opt/fortlet/project/bin:/opt/fortlet/base/usr/bin:/opt/fortlet/tool/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
nonlogin_cargo=/opt/fortlet/project/bin/cargo
cargo 1.97.1 (c980f4866 2026-06-30)
login_path=/.msb/scripts:/usr/local/bin:/usr/bin:/bin:/usr/local/games:/usr/games
login_cargo=absent
restored_path=/.msb/scripts:/opt/fortlet/project/bin:/opt/fortlet/base/usr/bin:/opt/fortlet/tool/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
restored_cargo=/opt/fortlet/project/bin/cargo
cargo 1.97.1 (c980f4866 2026-06-30)
```

This falsifies host permission loss: the guest sees mode 755. It confirms that
Debian's login profile replaces the injected PATH and that a process-scoped
`BASH_ENV` restores the exact value after profile processing. Public cleanup
reported `running`, `stopped`, `reset`, then `absent`; final inventory was `[]`.
No provider credential, prompt, model request, or network request was used.

## Terminal Closure

Accepted. Root cause: `/usr/bin/bash -lc` replaces Fortlet's capsule PATH with
the Debian login default, dropping the read-only project tool layer before
Codex runs repository commands. The fixed `BASH_ENV` mechanism restored the
managed PATH without persistent shell mutation. Actual cost matched the budget:
one Tact capsule, one comparison process, zero retries, zero provider units,
and zero money. Next action: implement the internal environment hook under
FIP-0006, add deterministic coverage, and validate it without another model
prompt.
