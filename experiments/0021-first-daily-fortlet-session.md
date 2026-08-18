# Experiment 0021: First daily Fortlet session

Status: declared — awaiting operator budget acceptance
Design: FIP-0001, FIP-0002, FIP-0006, and FIP-0007
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0020 accepted the immutable project layer and closed with the Codex
capsule absent while preserving the base, Codex, project, persistent Codex, and
Cargo-cache state. That observation is historical; this experiment must
reconfirm current public absence and exact ownership before launch.

The treatment revision is
`6ee4fede84f51cd6ca05e986f64ceb1bce56d96f` (`Add credential-free environment
preparation`). Its immutable aarch64-darwin package is
`/nix/store/v1zaa33klcx0ra7s1acsb6d01bgmhpms-fortlet-0.1.0`, with
MicroSandbox 0.6.8 and Codex 0.147.0. Exact project inputs remain:

1. `.fortlet/environment.json` SHA-256
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh` SHA-256
   `5857604b2ceb0dda2117ee4a54256d342300b3832305ebd9a777cf6ecfe703e0`.

The complete pre-dispatch gate passed 57 unit tests and 28 integration tests,
formatting, strict all-target/all-feature Clippy, conformance, `nix flake
check`, and the package build on aarch64-darwin. FIP-0007 is partial only for
this live daily-session evidence.

## Hypothesis and Production Mechanism

Because `prepare` resolves the same immutable environment inputs as launch but
stops before credentials and runtime state, an explicit packaged preparation
will report readiness without creating a reusable capsule. The next ordinary
launch will reuse those verified layers, and a subsequent shimmed interactive
session will attach to the same owned project+harness capsule and reuse the
preserved project Cargo cache for a real edit/test loop.

## Declared Scope

Use only the immutable package and `/Users/cody/dev/fortlet`, in this order:

1. Reconfirm exact revision and input hashes, public Codex absence, and no
   unexpected MicroSandbox capsule. Stop before dispatch if the baseline is not
   clean or ownership is uncertain.
2. From a nested Fortlet directory, run exactly one packaged
   `fortlet prepare codex --project /Users/cody/dev/fortlet`. Record stdout,
   stderr, elapsed time, and absence of a reusable Codex capsule afterward.
3. Run one non-prompt packaged
   `fortlet run codex --project /Users/cody/dev/fortlet -- --version`. Require
   Codex 0.147.0 and exactly one owned running capsule.
4. From the same nested directory, prepend only the package shim directory and
   start one interactive `codex` session. Submit exactly this one model prompt:

   ```text
   Work only in /Users/cody/dev/fortlet. First report command -v cargo, cargo --version, command -v jj, jj --version, jj status, and whether $CARGO_TARGET_DIR/debug/deps already exists. Add one integration test to tests/prepare.rs named invalid_base_marker_fails_without_runtime_state. It must seed {} as the base marker, run prepare codex against the fixture project, expect an environment-stage failure containing "published base environment marker does not match its inputs", and prove no runtime state was created. Change no production files. Run cargo test --test prepare. Then run jj diff --stat and summarize. Do not commit, push, access credentials, or run other tests.
   ```

5. The operator exits the interactive harness normally. Record the agent's tool
   paths and versions, pre-existing Cargo-cache observation, exact source diff,
   focused test result, and user-visible preparation/session friction.
6. Verify public status still reports the same running capsule, then stop and
   reset through the packaged public commands. Require final public absence,
   no provisioning capsule, and preservation of immutable layers, persistent
   Codex state, Cargo cache, and the project edit.

The package, project inputs, harness, model prompt, test name, requested edit,
command order, and cleanup stay frozen. The source edit is retained only if
local review finds it correct and in scope. No retry or replacement prompt is
allowed inside this experiment.

No second interactive session, second model prompt, Tact launch, native
harness, unowned capsule, private input, host recipe execution, credential or
fingerprint inspection, remote mutation, publication, settings change, release,
or package publication is in scope.

## Alternatives

1. Test only `prepare --help` or deterministic fixtures. Rejected because they
   cannot establish package behavior, capsule reuse, interactive UX, or the
   persistent project toolchain in ordinary work.
2. Force a fresh project-layer build. Rejected because deleting the accepted
   layer is unnecessary and risks replacing useful evidence; deterministic
   tests already cover cache misses and publication behavior.
3. Use `msb exec` for the edit/test loop. Rejected because the goal is the
   ordinary shimmed harness surface, not an administrative back door.
4. Allow a second prompt if the edit fails. Rejected because it would hide the
   first daily-session outcome; failure closes this unit honestly.

## Risks

1. The supposedly absent baseline may contain a capsule. Any unexpected or
   unowned capsule closes the unit before launch.
2. A stricter marker check may reject a previously published layer. That is a
   valid preparation failure, not authority to rewrite or delete the layer.
3. The interactive agent may exceed the exact task or touch credentials. Stop
   immediately, preserve the diff for review, and reject the unit.
4. Terminal exit may reproduce the earlier observer ambiguity. The operator
   may use ordinary idle Ctrl-C behavior, but no automated retry or second
   session is allowed.
5. Stop/reset may fail. Report the exact owned artifact and do not remove an
   uncertain capsule.

## Acceptance Criteria

1. The immutable package and complete deterministic gate remain exact and
   green before dispatch.
2. Initial public status is `codex<TAB>absent`, and there is no unexpected
   capsule.
3. Packaged preparation exits zero with exactly `codex<TAB>ready` on stdout,
   emits no first-use message on the expected cache hit, and leaves Codex
   absent.
4. The non-prompt launch reports Codex 0.147.0 and creates exactly one owned
   running capsule using the prepared project environment.
5. The sole interactive shim session attaches to that same capsule and spends
   exactly one model prompt.
6. Cargo and Jujutsu resolve below `/opt/fortlet/project/bin`; Cargo/Rust are
   1.97.x, Jujutsu is 0.43.0, and the persistent Cargo target cache exists
   before the focused test.
7. The exact requested integration test is the only source change, and
   `cargo test --test prepare` passes inside the guest.
8. Public stop/reset produces final absence while preserving the project edit,
   persistent state and cache, and every immutable layer.
9. Every criterion must pass. One failure, invariant anomaly, budget breach,
   cleanup completion, or 20 elapsed minutes terminates the experiment with no
   retry.

## Budget and Plan

Proposed dispatch budget: zero money, at most one paid-model prompt, zero remote
mutation, one packaged prepare invocation, one non-prompt Codex version launch,
one interactive Codex session, one owned reusable capsule, one focused Cargo
test command, no layer build expected, no silent retry, and at most 20 elapsed
minutes after dispatch. The operator performs the interactive terminal steps;
the agent records and reviews results, performs only declared read-only checks
and public owned cleanup, and stops on any anomaly.

## Rehearsal

Deterministic preparation evidence used isolated homes and state roots with an
invalid credential path. It passed exact CLI output, both adapters, repeated
cache hits, nested discovery, configured project digest verification, invalid
request ordering, prior-layer preservation, and absence of reusable capsule or
harness state. The complete runbook gate and immutable package build then
passed without a VM, harness, model prompt, or remote mutation.

## Results

Not dispatched.

## Terminal Closure

Pending dispatch and evidence.
