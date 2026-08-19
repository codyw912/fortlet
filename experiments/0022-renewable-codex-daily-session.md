# Experiment 0022: Renewable Codex daily session

Status: rejected — terminal
Design: FIP-0001, FIP-0002, FIP-0006, FIP-0007, and FIP-0008
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0021 proved packaged preparation, one owned capsule, and shim
attachment, but rejected the first prompt because the managed guest projection
caused Codex 0.147.0 to refresh a fake guest token. Its public stop/reset
cleanup completed, and the operator later removed the possible guest login
residue.

The treatment revision is `e7b1d5a26693` (`Implement renewable host-owned
Codex credentials`). Its immutable aarch64-darwin package is
`/nix/store/kym4by5g33jw1z3wdg9p70manajzmnx0-fortlet-0.1.0`, with
MicroSandbox 0.6.8 and Codex 0.147.0. Exact project inputs remain:

1. `.fortlet/environment.json` SHA-256
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh` SHA-256
   `5857604b2ceb0dda2117ee4a54256d342300b3832305ebd9a777cf6ecfe703e0`.

The complete pre-dispatch gate passed 69 unit tests and 28 non-ignored
integration tests, formatting, strict all-target/all-feature Clippy,
conformance, `nix flake check`, and the package build on aarch64-darwin. The
separate stock Codex 0.147.0 fixture loaded the exact placeholder-only external
token projection, reached a local fake backend, and made no request to the
local OAuth refresh tripwire. The immutable public CLI reported
`codex<TAB>absent` after the build.

## Hypothesis and Production Mechanism

Because Fortlet now projects `chatgptAuthTokens` with no usable guest refresh
credential, Codex will treat model authentication as externally owned instead
of entering its managed refresh path. The host-bounded lease will reuse or
refresh the file-backed login before launch, and attachment to the already
running capsule will apply both current broker secrets through MicroSandbox's
verified live modification path without restarting it. The ordinary shimmed
prompt can therefore reach model work while the refresh token remains on the
host.

## Declared Scope

Use only the immutable package and `/Users/cody/dev/fortlet`, in this order:

1. Reconfirm exact revision and project-input hashes, public Codex absence, and
   no unexpected MicroSandbox capsule. Stop before launch on any ambiguity.
2. From a nested Fortlet directory, run one non-prompt packaged
   `fortlet run codex --project /Users/cody/dev/fortlet -- --version`. Require
   Codex 0.147.0 and exactly one owned running capsule. Fortlet may perform at
   most one bounded host OAuth refresh and atomic host `auth.json` replacement
   if the existing access token is inside the one-hour safety window.
3. From the same nested directory, prepend only the package shim directory and
   start one interactive `codex` session. This attachment must exercise live
   rotation of both existing broker secrets. Submit exactly this one prompt:

   ```text
   Work only in /Users/cody/dev/fortlet. First report command -v cargo, cargo --version, command -v jj, jj --version, and jj status. Add one integration test to tests/pre_runtime_failures.rs named missing_refresh_token_fails_before_capsule_or_environment_artifacts. It must create otherwise valid file-backed ChatGPT auth with the refresh_token field absent, run the Codex launch fixture, expect a credentials-stage failure that directs the user to run codex login, and prove no project capsule state or tool layer was created. Change no production files. Run cargo test --test pre_runtime_failures. Then run jj diff --stat and summarize. Do not commit, push, access or print credentials, or run other tests.
   ```

4. The operator exits the interactive harness normally. Record the tool paths
   and versions, exact source diff, focused test result, and visible startup or
   authentication friction. The known `codex_apps` HTTP 451 biscuit warning is
   recorded separately and does not reject model authentication unless it
   prevents the declared prompt.
5. Verify public status still reports the same owned running capsule, then stop
   and reset through the packaged public commands. Require final absence, no
   provisioning capsule, and preservation of immutable layers, persistent
   project cache, and the project edit.

The package, project inputs, harness, prompt, test name, requested edit, command
order, and cleanup stay frozen. The source edit is retained only if local review
finds it correct and in scope. No retry or replacement prompt is allowed.

No second interactive session, second model prompt, Tact launch, native
harness, unowned capsule, private input, raw credential inspection, credential
fingerprint, remote mutation, publication, settings change, release, or package
publication is in scope.

## Alternatives

1. Stop at deterministic tests and the stock-Codex fake backend. Rejected
   because they do not prove MicroSandbox attachment, live broker rotation, or
   an ordinary model-backed edit/test loop.
2. Force an expired host token. Rejected because deliberately mutating or
   delaying a real login is unnecessary; deterministic local servers already
   prove refresh, persistence, locking, and failure behavior.
3. Inspect guest or host token values. Rejected because credential secrecy is
   an invariant, and structural tests already prove the projection contents.
4. Allow a second prompt after failure. Rejected because it would hide the
   first treatment outcome; a failure closes this experiment honestly.

## Risks

1. The host login may be missing, keyring-backed, malformed, account-changing,
   or permanently rejected. Stop before capsule mutation and run `codex login`
   on the host only outside this dispatch; do not retry the experiment.
2. The one allowed refresh may fail after bounded network delay. Preserve the
   existing source and close the experiment without another request.
3. Live secret modification may report restart-backed or unsupported. Fortlet
   must fail rather than restart the shared capsule; close and clean up only
   through public owned commands.
4. The interactive agent may exceed the exact task or touch credentials. Stop
   immediately, preserve the diff for review, and reject the unit.
5. Terminal exit or stop/reset may fail. Report the exact owned artifact and do
   not remove an uncertain capsule.

## Acceptance Criteria

1. The exact treatment revision, package, project inputs, and complete local
   gates remain verified before dispatch.
2. Initial public status is `codex<TAB>absent`, with no unexpected capsule.
3. The sole non-prompt launch reports Codex 0.147.0 and creates exactly one
   owned running capsule. Any host refresh is single, bounded, and completes
   before capsule reconciliation.
4. The sole shim session attaches to that same capsule through live secret
   rotation, reaches a model response without a Codex access-token refresh
   error, and spends exactly one model prompt.
5. Cargo and Jujutsu resolve below `/opt/fortlet/project/bin`; Cargo/Rust are
   1.97.x and Jujutsu is 0.43.0.
6. The exact requested integration test is the only source change, and
   `cargo test --test pre_runtime_failures` passes inside the guest.
7. Public stop/reset produces final absence while preserving the project edit,
   persistent state and cache, and every immutable layer.
8. Every criterion must pass. One failure, invariant anomaly, budget breach,
   cleanup completion, or 15 elapsed minutes terminates the experiment with no
   retry.

## Budget and Plan

Proposed dispatch budget: zero observed money beyond at most one paid-model
prompt, zero remote mutation, at most one bounded host OAuth refresh, one
non-prompt Codex version launch, one interactive Codex session, one owned
reusable capsule, one focused Cargo test command, no layer build expected, no
silent retry, and at most 15 elapsed minutes after dispatch. The operator runs
the interactive commands; the agent records and reviews results and performs
only declared read-only checks plus public owned cleanup.

## Rehearsal

The deterministic refresh suite passed source validation, timing, no-op reuse,
Codex-compatible request parameters, bounded response handling, atomic mode-0600
replacement, unrelated-field preservation, account mismatch, malformed and
redacted failures, source-identity races, concurrent single refresh, live
disposition enforcement, and bounded cancellation. Isolated CLI tests passed
credential failures before capsule or tool-layer creation. The exact stock
Codex 0.147.0 local-only fixture passed the projected-auth request and refresh
tripwire. The complete runbook gate and immutable package build passed without
a VM, provider request, model prompt, or remote mutation.

## Results

The operator accepted the exact declared budget on 2026-08-18. Immediately
before dispatch, the agent froze this baseline at `2026-08-19T02:03:14Z`:

1. `jj status` reported no working-copy changes, with the empty working copy
   at `0665141a` and the experiment declaration at parent `33d3c4a2`.
2. Revision `e7b1d5a26693` resolved exactly to
   `e7b1d5a2669300aa2e81f55971a6211db8a383df`.
3. Both project-input hashes matched the declared values.
4. The immutable public CLI reported `codex<TAB>absent`.
5. The pinned MicroSandbox CLI reported `No sandboxes found.`

The operator proceeded through the declared non-prompt launch and opened the
sole shimmed interactive session. The available transcript does not include
the version command's stdout, but the immutable package and adapter remained
pinned to Codex 0.147.0. On the sole submitted prompt, Codex first reported a
WebSocket 401 and fell back to HTTPS; the HTTPS request then failed with:

```text
unexpected status 401 Unauthorized: We got your request, but your ChatGPT login did not make it to this service.
```

No model response, repository inspection, edit, or Cargo test occurred. This
differs from Experiment 0021: Codex did not attempt its managed guest refresh,
so the external-token projection changed the lifecycle as designed, but the
brokered authentication was not accepted by either model transport. This
black-box observation does not distinguish a broker substitution failure from
a provider-invalid or otherwise incomplete host access credential.

After ordinary interactive exit, the packaged public commands reported:

```text
codex  running
codex  stopped
codex  reset
codex  absent
```

Independent closure checks found an empty Jujutsu working copy, final public
absence, and no MicroSandbox sandboxes. Metadata-only checks, without reading
host or guest authentication files, found the base marker, Codex 0.147.0
marker, exact project-environment marker, persistent Codex state, and
persistent Cargo target cache still present.

## Terminal Closure

Rejected at the first model request. The accepted implementation successfully
prevented Codex's old guest-owned refresh path, created and reused one owned
capsule, and cleaned it up entirely through the public surface. It did not
deliver usable model authentication: both the WebSocket request and its HTTPS
fallback received 401 before model work.

Actual cost was zero observed money, one prompt submission with no model
response, one non-prompt version launch, one interactive session, one owned
capsule, no source edit, no Cargo test, zero remote mutation, and no retry. The
black-box output did not reveal whether the one-hour safety window caused a
host refresh; Fortlet bounded any such refresh to the declared maximum of one.
The interactive command returned after approximately 2 minutes 19 seconds;
exact end-to-end experiment time was not captured, and no individual command
observation indicated a timeout.

Next action: do not retry this treatment or weaken credential containment.
First obtain deterministic evidence that separates MicroSandbox substitution
on Codex's WebSocket and HTTPS authorization requests from host-token provider
validity and request-shape/account-binding behavior. Any change to the accepted
credential mechanism requires a successor FIP and a newly declared experiment.

Post-closure source attribution found the exact failure chain without another
provider request. Codex 0.147.0's external-token constructor records
`last_refresh`, and its bearer accessor refuses to return `tokens.access_token`
unless that field is present. Fortlet's handwritten projection omitted it.
Codex could therefore derive and send `ChatGPT-Account-ID` while its bearer
provider silently omitted `Authorization`, matching the backend message that
the ChatGPT login did not reach the service. MicroSandbox supports placeholder
replacement in both the HTTP/1 WebSocket upgrade headers and HTTP/2 HTTPS
headers, so substitution never had an access-token placeholder to replace.

The repaired local-only stock-Codex fixture now requires both
`Authorization: Bearer $MSB_FORTLET_CHATGPT_ACCESS_TOKEN` and
`ChatGPT-Account-ID: $MSB_FORTLET_CHATGPT_ACCOUNT_ID`, plus absence of an OAuth
refresh request. The treatment remains terminal; a live confirmation requires
a newly declared experiment, not a resumed prompt.
