# Experiment 0034: Post-PATH Codex work loop

Status: rejected — terminal 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0003, FIP-0004, FIP-0005, FIP-0006,
FIP-0007, FIP-0008, FIP-0009, FIP-0010, and FIP-0011
Charter scope: `local-foundation/v1`

## Baseline / Control

PR #10 squash-merged at
`bca9438b2bb97b8f79728627b687e8d1ccc943d3`; its tree exactly matched reviewed
signed tip `47c6e9575a7cd9d30dc081c7e48a6bf22ac0c81c`. The landed bookmark was
removed locally and remotely, and the next working copy began clean above
`main`.

The complete standard gate passed on that merged baseline through
`nix develop` on aarch64-darwin: 88 unit tests and every enabled integration
test, formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check`. Nix emitted only the known incompatible `x86_64-linux`
omission notice. FIP-0001 and FIP-0002 remain partial with explicit gaps;
FIP-0003 through FIP-0011 are conformant.

Experiment 0030's model-backed non-interactive process streamed progress,
produced its intended test-only edit, and exited zero, but its guest
`bash -lc` could not resolve Cargo. Experiment 0031 attributed that failure to
Debian's login profile replacing Fortlet's managed PATH. Experiment 0032 then
observed the production `BASH_ENV` hook restore
`/opt/fortlet/project/bin/cargo`, report Cargo 1.97.1, and pass the exact focused
test. Experiment 0032 was rejected for undeclared Cargo network traffic and an
omitted read-only inspection, not mechanism failure. No provider-backed unit
has exercised the corrected composition.

The immutable treatment revision is
`5ed68deed08454b9201367f2e3c0ecb06ec5e82f`. Its aarch64-darwin Nix package is
`/nix/store/y2543gbhzbhbsnc4lq1c3a49qbrqphas-fortlet-0.1.0`. No live unit was
dispatched before both values were recorded.

## Hypothesis and Production Mechanism

An immutable Fortlet package invoked through its package-owned `codex` shim
will close streaming stdin explicitly, authenticate with the host-owned
renewable ChatGPT lease, suppress only the unsupported Apps client, preserve
the project-tool PATH beneath Codex's non-interactive login shell, stream model
and tool progress, and return Codex's exact zero exit status after one useful
edit-and-test task.

The task adds compiled-CLI evidence that a transparent Codex shim enters the
same fail-closed transaction as explicit `fortlet run`: missing host auth must
produce the existing credentials-stage correction before project state or tool
roots exist.

## Declared Scope

1. Freeze this declaration as one Jujutsu checkpoint. Build one immutable
   aarch64-darwin Nix package from that exact revision and record its full
   revision and store path before dispatch.
2. Require a clean working copy, absent `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
   `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` markers in the outer shell,
   packaged public Codex status `absent`, packaged `fortlet list` output
   `no capsules`, and packaged raw MicroSandbox inventory `[]`.
3. From `/Users/cody/dev/fortlet`, invoke the immutable package's exact
   `libexec/fortlet/shims/codex` path once in non-interactive mode:

   ```text
   <package>/libexec/fortlet/shims/codex exec --ephemeral \
     --skip-git-repo-check <frozen prompt>
   ```

4. The exact frozen prompt is:

   ```text
   In /Users/cody/dev/fortlet, add one integration regression test to
   tests/pre_runtime_failures.rs named
   `codex_shim_missing_auth_fails_before_capsule_or_environment_artifacts`.
   The test must invoke the compiled Fortlet binary through a temporary symlink
   named `codex`, with a cleared environment, the fixture project as its working
   directory, and the fixture's HOME, XDG_STATE_HOME, XDG_DATA_HOME, and missing
   FORTLET_AUTH_FILE values. It must pass `--version` through the shim, require
   the existing credentials-stage `run `codex login` on the host and retry`
   missing-auth failure, and prove the project-state and tool roots were not
   created. Reuse existing fixture helpers where clear. Change only
   tests/pre_runtime_failures.rs. Run exactly `cargo test --test
   pre_runtime_failures
   codex_shim_missing_auth_fails_before_capsule_or_environment_artifacts`.
   Do not change production code or VCS state, commit, or modify any other path.
   Do not initiate network activity except any unauthenticated public Cargo
   access automatically required by that exact test command. Finish with a
   concise summary and the test result.
   ```

5. Permit exactly one Codex process and the one frozen prompt. Cargo may reuse
   the existing persistent public cache or make only unauthenticated crates.io
   index and crate requests required by the exact command. No follow-up,
   prompt edit, retry, second model process, native fallback, direct capsule
   mutation, credential inspection, or unrelated network request is in scope.
6. After the process exits, preserve its first diff unchanged for independent
   review. Run the exact focused test through `nix develop` on the host.
7. Query packaged `fortlet list` and project-scoped Codex status, then use only
   the same package's public stop and reset commands. Require final status
   `absent`, `fortlet list` output `no capsules`, and raw packaged MicroSandbox
   inventory `[]`.

If the active agent process inherits any forbidden Codex marker, it must not
dispatch. The operator may run the one exact command from a marker-free outer
shell and return the unedited output and status. That is the declared operator
boundary, not a second unit.

## Alternatives

1. Repeat Experiment 0030's exact task. Rejected because terminal experiments
   are not retried; this unit has a new useful regression and identity.
2. Use interactive Codex. Rejected because Experiments 0023 and 0025 already
   accepted the interactive model path, while this goal must close the newer
   non-interactive EOF and login-shell composition.
3. Ask Codex to change production behavior. Rejected because no current defect
   justifies a production change; test-only evidence is useful and contains the
   model-produced diff.
4. Run a synthetic local provider. Rejected because Experiments 0026, 0027,
   0029, and 0032 already isolate the individual mechanisms. The remaining
   claim is ordinary provider-backed composition.

## Risks

1. The host login may require bounded refresh or be permanently invalid. One
   failure is terminal and must not be corrected by reading, copying, or
   projecting credential content.
2. Codex may touch another path, production code, or VCS state. Preserve and
   reject that first diff; do not silently repair it as model success.
3. Cargo may require public crate traffic from a cold cache. This is explicitly
   permitted only for the exact test command and must be reported.
4. The model or guest test may remain silent. Fortlet's accepted ten-minute
   inactivity ceiling remains the bound; do not extend it or switch attachment
   modes under this identity.
5. Cleanup may reveal a stale or unexpected capsule. Do not mutate an unowned
   capsule or broaden cleanup to make the experiment pass.

## Acceptance Criteria

1. Exact treatment revision/package and every clean baseline are recorded
   before dispatch; the outer shell has none of the four forbidden Codex
   markers.
2. The sole shimmed process visibly streams progress, emits no Apps startup or
   authentication failure, changes only `tests/pre_runtime_failures.rs`, runs
   the exact focused Cargo test successfully beneath Codex, and exits zero.
3. The retained test invokes the compiled binary as `codex`, asserts the exact
   existing credentials-stage guidance, and proves no project-state or tool
   root creation.
4. Independent diff review accepts the change and the same host test passes
   through `nix develop`.
5. Public inventory and project lifecycle commands observe and remove only the
   owned Codex capsule, ending with public absence and raw inventory `[]` while
   preserving immutable layers and persistent harness state.
6. Any failed criterion settles this experiment terminally without retry. A
   demonstrated Fortlet defect may receive deterministic correction only under
   the GOAL and a separately declared successor experiment.

## Budget and Plan

One immutable aarch64-darwin package, one packaged non-interactive Codex
process, one model prompt, one focused guest test, one independent host test,
one owned capsule, zero retries, no external money, and at most 45 elapsed
minutes within the GOAL's two-hour engineering ceiling. Provider and ordinary
public Cargo traffic are limited exactly as declared above.

Build and record the immutable treatment, verify the complete preflight, run
the sole process, preserve and inspect its diff, independently test it, clean
up through the public package, and close the experiment in that order.

## Rehearsal

The complete deterministic gate is green on merged `main`. Experiment 0029
proved packaged explicit EOF against stock Codex without provider traffic;
Experiment 0030 proved the provider-backed process reaches model editing and
streamed output; Experiment 0032 proved the production login-shell hook exposes
and runs the selected Cargo. No live process has run under this declaration.

## Results

The immutable package build exited zero and produced the recorded store path.
The final outer-shell working copy was clean. The neighboring Herdr fish pane
reported all four forbidden Codex markers absent. Packaged public status reported
`codex<TAB>absent`, packaged `fortlet list` reported `no capsules`, and the raw
packaged MicroSandbox inventory was `[]`.

The active agent shell contained all four forbidden markers, so it did not
dispatch directly. Using the operator-authorized Herdr boundary, the exact
frozen script ran once in the verified marker-free neighboring pane. The sole
packaged shim process started Codex 0.147.0 with the OpenAI provider and
workspace-write sandbox, authenticated, streamed inspection and edit progress,
and emitted no Apps startup or authentication failure.

Codex added only the requested 27-line test in
`tests/pre_runtime_failures.rs`. The test invokes the compiled binary through a
temporary `codex` symlink with the declared cleared environment and working
directory, asserts the existing missing-auth credentials error, and verifies
that project state and tool roots remain absent. Independent review accepted
the diff.

The exact guest Cargo command then exited 101 in about 1.4 seconds:

```text
error: failed to open: /home/agent/.cargo/fortlet-target/debug/.cargo-build-lock

Caused by:
  Read-only file system (os error 30)
```

Instead of stopping after that failure, Codex inspected its environment and
changed `CARGO_TARGET_DIR` to retry the test. It also attempted to remove its
temporary target with `rm -rf`, which the Codex sandbox rejected, then used a
bounded `find ... -delete` cleanup. The altered build did not return within the
final observation window. After 17 minutes 34 seconds total, packaged public
status still reported `running`; packaged public stop returned `stopped` and
the waiting wrapper ended with `codex_exit=255`.

The same exact focused test passed independently through `nix develop`: one
passed, zero failed, with 12 filtered out. Packaged public reset then returned
`reset`; final status was `absent`, `fortlet list` reported `no capsules`, and
raw MicroSandbox inventory was `[]`. The temporary host dispatch script was
deleted. Immutable layers and persistent harness state were preserved.

## Terminal Closure

Rejected. The intended model edit and independent test passed, and the composed
shim, authentication, Apps-disablement, EOF, and streaming paths all advanced.
The frozen in-capsule test did not pass: Fortlet's project fixture exports
`CARGO_TARGET_DIR=/home/agent/.cargo/fortlet-target`, but managed Codex runs tool
commands in a workspace-write sandbox whose writable roots include the live
project and `/tmp`, not that persistent harness-home cache. Cargo could read the
tool environment but could not create its build lock there. Experiment 0032's
direct Tact process did not exercise this inner Codex sandbox boundary.

The unit also violated its exact-command protocol when Codex changed the target
directory after the first failure. That altered attempt cannot rescue the
acceptance result and was not treated as a successor. No follow-up prompt,
second Codex process, native fallback, credential inspection, or unowned
capsule mutation occurred.

Actual cost was one immutable package, one model-backed Codex process, one
prompt, one correct 27-line test diff, one failed exact guest test, altered
in-process Cargo attempts, one successful independent host test, 17 minutes 34
seconds of process time, zero retries, zero external money, and exact public
cleanup. Retain the independently verified test. Next action: deterministically
correct the Fortlet repository's guest Cargo target to a Codex-writable,
host-build-distinct workspace path, then declare at most one successor
experiment under the active GOAL.
