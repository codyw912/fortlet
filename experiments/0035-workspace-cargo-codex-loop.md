# Experiment 0035: Workspace-backed Cargo Codex loop

Status: declared — no live unit dispatched
Design: FIP-0001, FIP-0002, FIP-0003, FIP-0004, FIP-0005, FIP-0006,
FIP-0007, FIP-0008, FIP-0009, FIP-0010, and FIP-0011
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0034

## Baseline / Control

Experiment 0034 is terminally rejected. Its sole packaged shim process
authenticated, streamed, and produced a correct independently passing test,
but the exact guest Cargo command failed because the repository fixture set
`CARGO_TARGET_DIR=/home/agent/.cargo/fortlet-target`. That persistent
harness-home path is outside managed Codex's workspace-write roots. Codex then
altered the target in-process and failed to settle before public stop; that
attempt is not acceptance evidence.

The independently retained test passes through `nix develop`. Checkpoint
`028be464` changes only the Fortlet repository fixture and its deterministic
assertion: guest Cargo now uses relative `target/fortlet-guest`, beneath the
live project workspace and separate from the host's ordinary `target/debug`.
The repository already ignores `/target/`. The fixture test, retained compiled
shim test, formatting, and conformance are green.

The immutable treatment revision and Nix store path will be frozen from this
declaration checkpoint and recorded before dispatch.

## Hypothesis and Production Mechanism

An immutable package using the corrected project manifest will give Codex's
workspace-sandboxed tool subprocess a writable, guest-distinct Cargo target at
`target/fortlet-guest`. Therefore one new packaged non-interactive shim task
will report that exact target, make one useful test-only edit, run the exact
focused Cargo command without an environment override, and exit zero.

## Declared Scope

1. Freeze this declaration as one Jujutsu checkpoint, build one immutable
   aarch64-darwin package from that revision, and record its full revision and
   store path before dispatch.
2. The changed manifest selects a new project-environment identity. Automatic
   first-launch provisioning may make only the recipe's pinned unauthenticated
   Debian, Rust, and Jujutsu requests. Do not copy, mutate, or remove an older
   immutable layer to avoid this normal identity change.
3. Require a clean working copy; a marker-free outer Herdr fish pane; packaged
   Codex status `absent`; packaged `fortlet list` output `no capsules`; and raw
   packaged MicroSandbox inventory `[]`.
4. From `/Users/cody/dev/fortlet`, invoke the immutable package's exact
   `libexec/fortlet/shims/codex` path once:

   ```text
   <package>/libexec/fortlet/shims/codex exec --ephemeral \
     --skip-git-repo-check <frozen prompt>
   ```

5. The exact frozen prompt is:

   ```text
   In /Users/cody/dev/fortlet, first run `printf 'cargo_target=%s\n'
   "$CARGO_TARGET_DIR"` and require it to report exactly
   `cargo_target=target/fortlet-guest`; do not override or change that variable.
   Then add one integration regression test to tests/pre_runtime_failures.rs
   named
   `codex_shim_invalid_project_environment_fails_before_credentials_or_runtime_artifacts`.
   The test must create the existing invalid `../outside` project-environment
   fixture, invoke the compiled Fortlet binary through a temporary symlink
   named `codex`, use the fixture project as its working directory with a
   cleared environment and the fixture's HOME, XDG_STATE_HOME, XDG_DATA_HOME,
   and missing FORTLET_AUTH_FILE values, and pass `--version` through the shim.
   Require the existing project-environment-stage normalized-relative-path
   failure and prove that project state, tool roots, and environment roots were
   not created. Reuse or extract a small test helper only inside
   tests/pre_runtime_failures.rs when it improves clarity. Change only that
   file. Run exactly `cargo test --test pre_runtime_failures
   codex_shim_invalid_project_environment_fails_before_credentials_or_runtime_artifacts`.
   Do not alter CARGO_TARGET_DIR, change production code or VCS state, commit,
   or modify any other path. Do not initiate network activity except any
   unauthenticated public Cargo access automatically required by that exact test
   command. Finish with a concise summary and the test result.
   ```

6. Permit exactly one Codex process and one prompt. No follow-up, prompt edit,
   retry, second process, target override, native fallback, direct capsule
   mutation, credential inspection, or unrelated network request is in scope.
7. Preserve the first diff unchanged, independently review it, and run the same
   focused test through `nix develop` on the host.
8. Query packaged global inventory and project-scoped status, then use only the
   same package's public stop and reset commands. Require final status `absent`,
   `fortlet list` output `no capsules`, and raw inventory `[]`.

## Alternatives

1. Resume Experiment 0034 or accept its altered target. Rejected because it is
   terminal and the altered command did not test the declared product setup.
2. Add Codex `--add-dir /home/agent/.cargo`. Rejected because it broadens the
   agent's writable persistent-home authority and changes the adapter contract
   when this repository needs only an isolated workspace build target.
3. Use the ordinary host `target/debug`. Rejected because native macOS and
   guest Linux artifacts must not collide. The nested guest target remains
   ignored but distinct.
4. Use a synthetic provider. Rejected because the remaining claim is the full
   provider-backed Codex edit/test composition after the concrete correction.

## Risks

1. New project-layer provisioning may fail or consume the experiment window.
   One failure is terminal; do not reuse an incompatible capsule or layer.
2. Codex may override the target, touch an undeclared path, or run a different
   command. Preserve and reject the first result without repair.
3. Cargo may require declared public crates.io traffic from a cold target even
   when its source cache is warm.
4. A repeated read-only target failure would share Experiment 0034's resolved
   assumption and stop the GOAL under the charter.
5. Cleanup may reveal unexpected state. Never mutate an unowned capsule.

## Acceptance Criteria

1. Exact treatment revision/package, complete deterministic checks, absent
   markers, clean working copy, public Codex absence, and empty inventories are
   recorded before dispatch.
2. The sole shimmed process reports exactly
   `cargo_target=target/fortlet-guest`, streams without Apps or authentication
   failure, changes only `tests/pre_runtime_failures.rs`, runs the exact Cargo
   test without overriding the target, and exits zero.
3. The retained test exercises the actual `codex` shim, exact invalid project
   environment failure, and absence of project, tool, and environment state.
4. Independent review accepts the diff and the same host test passes through
   `nix develop`.
5. Public cleanup ends at status `absent`, `no capsules`, and raw inventory
   `[]`, preserving immutable layers and persistent harness state.
6. Any failure is terminal. No third model process is authorized by the GOAL.

## Budget and Plan

One immutable package, at most one automatic pinned project-layer provisioning,
one packaged Codex process, one model prompt, one focused guest test, one host
test, one owned capsule, zero retries, no external money, and the remainder of
the GOAL's two-hour engineering ceiling. Provider, recipe, and Cargo traffic are
limited exactly as declared.

Build and record the package, pass the complete preflight, dispatch once from
the verified marker-free Herdr pane, preserve and review the diff, independently
test, clean up publicly, and close terminally in that order.

## Rehearsal

The corrected repository fixture assertion and retained compiled-shim test pass
through `nix develop`. The target is inside the same live project root Codex
reported as workspace-writable in Experiment 0034, and `/target/` is ignored.
No provider or live runtime process has run under this declaration.

## Results

Pending. No live unit has been dispatched.

## Terminal Closure

Pending.
