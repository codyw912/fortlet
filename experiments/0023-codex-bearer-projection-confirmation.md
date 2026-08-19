# Experiment 0023: Codex bearer projection confirmation

Status: declared
Design: FIP-0001, FIP-0002, FIP-0006, FIP-0007, and FIP-0008
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0022 is terminally rejected and must not be resumed. It proved that
the external-token projection prevents Codex's managed guest refresh, but its
sole prompt received 401 over both WebSocket and HTTPS because Fortlet omitted
the non-secret `last_refresh` field and Codex therefore omitted its bearer
header.

The treatment revision is `adc9b0389ea8d30ad3ac012df58424c59b8284f3`
(`Restore Codex bearer projection metadata`). Its immutable aarch64-darwin
package is `/nix/store/yql3hf31rf5qgf8vccmw75b2lz5z6n23-fortlet-0.1.0`,
with MicroSandbox 0.6.8 and Codex 0.147.0. Exact project inputs remain:

1. `.fortlet/environment.json` SHA-256
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh` SHA-256
   `5857604b2ceb0dda2117ee4a54256d342300b3832305ebd9a777cf6ecfe703e0`.

The complete final-tree gate passed 69 unit tests, 28 non-ignored integration
tests, formatting, strict all-target/all-feature Clippy, conformance, `nix
flake check`, and the package build on aarch64-darwin. The separate stock Codex
0.147.0 fixture now requires both external placeholders in the request headers
and no OAuth refresh request. At `2026-08-19T02:37:13Z`, the working copy was
empty, the exact revision and hashes matched, the immutable public CLI reported
`codex<TAB>absent`, and MicroSandbox reported no sandboxes.

## Hypothesis and Production Mechanism

Because the Codex projection now matches the pinned external-token constructor
by including a valid `last_refresh`, Codex will emit both the bearer and account
placeholders. MicroSandbox will replace them only in the TLS-verified ChatGPT
request headers. The same host-owned renewable lease and live rotation path
from Experiment 0022 can therefore authenticate the ordinary prompt without a
guest-readable provider credential.

## Declared Scope

Use only the immutable package and `/Users/cody/dev/fortlet`, in this order:

1. Reconfirm the exact revision, project hashes, public Codex absence, and empty
   MicroSandbox inventory. Stop before launch on any ambiguity.
2. From `/Users/cody/dev/fortlet/src`, run one non-prompt packaged
   `fortlet run codex --project /Users/cody/dev/fortlet -- --version`. Require
   Codex 0.147.0 and exactly one owned running capsule. Fortlet may perform at
   most one bounded host OAuth refresh if required by its safety window.
3. From the same directory, prepend only the package shim directory and start
   one interactive `codex` session. This attachment must rotate both existing
   broker secrets live. Submit exactly this one prompt:

   ```text
   Work only in /Users/cody/dev/fortlet. First report command -v cargo, cargo --version, command -v jj, jj --version, and jj status. Add one integration test to tests/pre_runtime_failures.rs named missing_refresh_token_fails_before_capsule_or_environment_artifacts. It must create otherwise valid file-backed ChatGPT auth with the refresh_token field absent, run the Codex launch fixture, expect a credentials-stage failure that directs the user to run codex login, and prove no project capsule state or tool layer was created. Change no production files. Run cargo test --test pre_runtime_failures. Then run jj diff --stat and summarize. Do not commit, push, access or print credentials, or run other tests.
   ```

4. Exit normally. Record the tool paths and versions, exact diff, focused test
   result, and every visible startup or authentication warning. The known
   `codex_apps` HTTP 451 biscuit warning remains separate from model auth.
5. Require public status to report the same owned running capsule, then stop and
   reset through the packaged public commands. Require final absence, no
   MicroSandbox sandbox, and preservation of the project edit, persistent state
   and Cargo cache, and immutable layers.

The package, inputs, commands, prompt, requested edit, and cleanup are frozen.
No retry, replacement prompt, second interactive session, Tact launch, native
harness, raw credential inspection, remote mutation, publication, settings
change, release, or package publication is in scope.

## Alternatives

1. Treat the strengthened local fixture as sufficient. Rejected because it
   proves Codex emits placeholders but not real MicroSandbox substitution or
   provider acceptance.
2. Resume Experiment 0022. Rejected because terminal experiment identities are
   immutable and may not be retried.
3. Send only a trivial model prompt. Rejected because the active GOAL still
   requires one ordinary project edit/test loop through the pinned tool layer.

## Risks

1. Host authentication may have changed independently. Stop on any credential
   failure, do not inspect values, and do not run `codex login` in this unit.
2. The provider may still reject the substituted credential for a distinct
   reason. Record the first failure and clean up without retry.
3. The interactive agent may exceed the exact task or touch credentials. Stop
   immediately, preserve the diff for review, and reject the unit.
4. Stop/reset may fail. Report the exact owned artifact and do not remove an
   uncertain capsule through private means.

## Acceptance Criteria

1. The exact treatment revision, package, inputs, and complete local gates
   remain verified before dispatch.
2. Initial public status is `codex<TAB>absent`, with no MicroSandbox sandbox.
3. The one non-prompt launch reports Codex 0.147.0 and creates exactly one owned
   running capsule; the shim attaches to that same capsule with live rotation.
4. The sole prompt reaches a model response without refresh failure or 401.
5. Cargo and Jujutsu resolve below `/opt/fortlet/project/bin`; Cargo/Rust are
   1.97.x and Jujutsu is 0.43.0.
6. The requested integration test is the only source change, and `cargo test
   --test pre_runtime_failures` passes inside the guest.
7. Public stop/reset restores absence while preserving the project edit,
   persistent state and cache, and immutable layers.
8. Every criterion must pass. The first failure, invariant anomaly, budget
   breach, cleanup completion, or 15 elapsed minutes terminates the experiment.

## Budget and Plan

Proposed dispatch budget: zero observed money beyond at most one paid-model
prompt, zero remote mutation, at most one bounded host OAuth refresh, one
non-prompt version launch, one interactive session, one owned reusable capsule,
one focused Cargo test command, no layer build expected, no silent retry, and
at most 15 elapsed minutes after dispatch. The operator runs the interactive
commands; the agent records and reviews results and performs only declared
read-only checks plus public owned cleanup.

## Rehearsal

The new unit test first failed because the projection omitted `last_refresh`.
After the repair, it passed with a valid RFC 3339 value while Tact's managed
projection stayed unchanged. Exact Codex 0.147.0 source shows the bearer
accessor requires both `tokens` and `last_refresh`; its external-token
constructor supplies both. The strengthened local-only stock fixture observed
both placeholder headers, observed no refresh request, and reached the fake
backend. MicroSandbox 0.6.8 source supports placeholder replacement in HTTP/1
WebSocket-upgrade and HTTP/2 request headers. The complete final-tree gate and
immutable package build passed without a VM, provider request, model prompt, or
remote mutation.

## Results

Not dispatched.

## Terminal Closure

Pending.
