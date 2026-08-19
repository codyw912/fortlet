# Experiment 0030: First post-EOF daily Codex work loop

Status: declared — 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0005, FIP-0006, FIP-0007, FIP-0008,
FIP-0009, and FIP-0010
Charter scope: `local-foundation/v1`

## Baseline / Control

PR #8 squash-merged the accepted and conformant FIP-0010 implementation to
`main` at `53378399bfa581d5a7b3db8538b234dc3733bb26`. Its tree matched signed
reviewed tip `82df49b7eee2d0d25f9f064518deae67c3cf544b`; the landed bookmark was
removed locally and remotely. The immutable treatment revision is
`02e9c148e7a9e6aceb92993586375e0ab763035d`, including GOAL checkpoint
`92f5de37b496`, above that clean main. Its aarch64-darwin package is
`/nix/store/1xkx1xk7fd97ba5f1nlddq9bjilfdvbj-fortlet-0.1.0`.

Before declaration, `jj status` was clean, conformance selected FIP-0010 as
conformant with no gaps, `cargo test --test conformance` passed through
`nix develop`, packaged public status reported `codex<TAB>absent`, and the
packaged MicroSandbox inventory was `[]`.

Experiment 0028's sole model unit never passed Codex's stdin read. Experiment
0029 proved the explicit-EOF correction in an immutable package against only
guest loopback. No post-correction provider-backed non-interactive task has run.

## Hypothesis and Production Mechanism

First, a synthetic-auth capsule using the same packaged runtime will resolve
`chatgpt.com` and complete one unauthenticated TLS request after any ordinary
operator-approved macOS firewall prompt. This separates host/guest network
readiness from credential and model behavior.

Then the corrected explicit EOF will let packaged Codex 0.147.0 consume one
frozen repository-maintenance prompt, stream progress, strengthen the existing
Rust regression, run its focused Cargo test through the mounted project tool
layer, and exit zero. Fortlet will preserve the intended diff and reusable
capsule for public cleanup.

## Declared Scope

### Immutable treatment and credential-free preflight

1. Build one immutable aarch64-darwin package from this declaration, then
   record its exact full revision and Nix store path before dispatch.
2. Create `/private/tmp/fortlet-exp0030-auth/auth.json` as a mode-0600 regular
   file outside every guest mount. It contains only a far-future unsigned test
   JWT, fixed zero account ID, and dummy refresh string; none is a provider
   credential.
3. Require public Codex absence and empty MicroSandbox inventory. With only
   `FORTLET_AUTH_FILE` pointing to that synthetic document, run the immutable
   packaged public command once:

   ```text
   fortlet run codex --project /Users/cody/dev/fortlet -- --version
   ```

4. Resolve the sole running Fortlet-owned Codex capsule from the packaged
   MicroSandbox inventory and run exactly one direct, non-TTY guest probe:

   ```text
   sh -c 'set -eu
   getent ahostsv4 chatgpt.com | sed -n "1p"
   curl --proto "=https" --tlsv1.2 --silent --show-error --head \
     --max-time 15 --output /dev/null \
     --write-out "tls_http=%{http_code}\n" \
     https://chatgpt.com/robots.txt'
   ```

   It sends no Authorization header, cookie, prompt, model request, repository
   content, or provider credential. The operator may manually approve one
   macOS firewall popup for the already-running `msb` process; Fortlet and the
   experiment do not inspect or mutate firewall settings.
5. Record the DNS line, HTTP status, stderr, exact host status, and whether the
   operator reports a popup. Use only packaged public status/stop/reset for
   cleanup, require final absence and empty inventory, then delete the
   synthetic auth document and directory.
6. Any failure or `tls_http=000` terminates the experiment before model spend.
   There is no second preflight process.

### Frozen repository work unit

After the preflight and complete standard gate pass, launch exactly once:

```text
fortlet run codex --project /Users/cody/dev/fortlet -- \
  exec --ephemeral --skip-git-repo-check <frozen prompt>
```

The exact prompt is:

```text
In /Users/cody/dev/fortlet, strengthen the existing Rust unit test
`non_interactive_stream_closes_stdin_before_returning` in `src/runtime.rs` so
it proves stdin is closed before the first exec event is read, not merely
before `forward_non_interactive` returns. Rename the test to
`non_interactive_stream_closes_stdin_before_reading_events`. Change only
`src/runtime.rs`, and do not change production code. Run exactly
`cargo test --bin fortlet
runtime::tests::non_interactive_stream_closes_stdin_before_reading_events`.
Do not use network, commit, or modify VCS state. Finish with a concise summary
and the test result.
```

Before dispatch the working copy MUST be clean. The allowed model-produced
diff is test-only code in `src/runtime.rs`; no production hunk or other path is
permitted. After Codex exits, inspect the exact diff and independently run the
same focused test through `nix develop` on the host.

Regardless of outcome, query packaged public status, then stop/reset Codex and
require final public absence and empty MicroSandbox inventory. No prompt edit,
follow-up, retry, interactive/native fallback, or second model process is in
scope.

## Alternatives

1. Send another exact-response prompt. Rejected because it would prove only
   transport, not the ordinary repository edit/test loop named by the GOAL.
2. Ask Codex to implement a new product feature. Rejected because daily-path
   validation does not justify unrelated scope or an architecture decision.
3. Use a disposable toy repository. Rejected because Fortlet's actual project
   environment, mount, Rust toolchain, and current code are the product path
   being evaluated.
4. Modify firewall settings or add `fortlet doctor` network checks. Rejected
   because the credential-free probe can attribute readiness without creating
   a public contract or mutating host security policy.

## Risks

1. macOS may block the first guest DNS/TLS attempt pending a firewall decision.
   A manual approval may let that same process continue; a failed process is
   terminal and is not relaunched.
2. Synthetic auth could unexpectedly trigger refresh or provider traffic.
   The token covers the safety window and `--version` is local; any refresh,
   bearer use, model diagnostic, or provider-auth request is an invariant
   anomaly and stops the experiment.
3. Codex may edit production code, another file, or VCS state. Preserve and
   record the first diff as a rejected unit; do not rationalize or silently
   repair it as model success.
4. The focused guest Cargo test could depend on cached crates or exceed the
   inactivity ceiling. No network is permitted; streamed build output renews
   the accepted ceiling, while a silent expiry is a terminal result.

## Acceptance Criteria

1. Exact package/revision and clean baselines are recorded before preflight.
2. The sole preflight emits one resolved `chatgpt.com` address, a nonzero HTTP
   code with curl status zero, no credential/model traffic, and completes
   public cleanup to absence and empty inventory.
3. The complete standard gate passes after preflight and before model dispatch.
4. The sole model task streams visible progress without an Apps warning,
   changes only test code in `src/runtime.rs`, strengthens ordering as prompted,
   runs the exact focused test successfully, and exits with host status zero.
5. The independently repeated host test passes, the final diff contains only
   the intended maintenance change, and public cleanup restores absence and
   empty inventory.
6. The first failure or invariant anomaly settles its phase and terminates the
   experiment. There is no retry.

## Budget and Plan

One synthetic-auth public version launch, one guest DNS lookup, one
unauthenticated TLS HEAD to `https://chatgpt.com/robots.txt`, one owned capsule
per phase, one short operator-authorized model-backed Codex task, zero retries,
zero additional paid quota, and at most 90 engineering minutes. Complete the
preflight and cleanup first, run the full gate, then dispatch the model unit and
clean up publicly.

## Rehearsal

Experiment 0029 exercised the exact immutable packaged Codex attachment after
the EOF correction, advanced past stdin, streamed local-provider output, and
returned exact status 1 in about 8.7 seconds. Its public lifecycle cleanup
restored absence and empty inventory. The current conformance gate is green.

## Results

Pending.

## Terminal Closure

Pending.
