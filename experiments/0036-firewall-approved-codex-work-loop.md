# Experiment 0036: Firewall-approved Codex work loop

Status: declared — no preparation or provider process dispatched
Design: FIP-0001, FIP-0002, FIP-0003, FIP-0004, FIP-0005, FIP-0006,
FIP-0007, FIP-0008, FIP-0009, FIP-0010, and FIP-0011
Charter scope: `local-foundation/v1`
Succeeds: Experiment 0035

## Baseline / Control

Experiment 0034 proved the packaged shim authenticated, streamed, and produced
a correct retained test, but its exact Cargo command found the repository's
then-configured persistent-home target read-only inside Codex's workspace
sandbox. Merged PR #11 corrected only that fixture to ignored,
guest-distinct `target/fortlet-guest`; focused and complete deterministic gates
pass.

Experiment 0035 never reached Codex or the corrected target. Automatic
first-use provisioning lost DNS while the operator was absent from a macOS
firewall approval prompt. It remains a terminal historical record, but its DNS
result is not evidence of Fortlet's daily-work behavior under an approved local
firewall path.

Merged `main` is `be43d5909b6a65e9ca6984859b85b6d27c2601dc`, with tree equality
to PR #11's reviewed signed tip established before bookmark cleanup. The clean
merged baseline passed all tests, formatting, strict Clippy, conformance, and
`nix flake check` on aarch64-darwin; Nix emitted only the expected incompatible
`x86_64-linux` omission warning. The frozen declaration revision is
`a1dade963584803e0affa6fd9dfdc874bf74efc5`, packaged as
`/nix/store/is3sfw30rzr58w29q8rnsdcdidwxqgzn-fortlet-0.1.0`. The complete
standard gate passed again on that exact declaration tree, with only the same
expected Nix system-omission warning.

## Hypothesis and Production Mechanism

With the operator present to approve MicroSandbox's expected local firewall
request, the immutable package's public credential-free `prepare codex` path
will publish and verify the corrected project layer. A second prepare will hit
that verified cache without network contact. The same package's Codex shim will
then expose `CARGO_TARGET_DIR=target/fortlet-guest` inside Codex's writable
workspace, allowing one exact useful integration test and its focused Cargo
command to finish with exit status zero.

## Declared Scope

1. Freeze this declaration as a Jujutsu checkpoint, build exactly one immutable
   aarch64-darwin package from it, and record its complete revision and Nix
   store path before runtime contact.
2. Require a clean working copy, packaged project-scoped Codex status
   `absent`, packaged global list output `no capsules`, and packaged raw
   MicroSandbox inventory `[]`.
3. With the operator present, run exactly once:

   ```text
   <package>/bin/fortlet prepare codex --project /Users/cody/dev/fortlet
   ```

   The operator may approve only the expected local MicroSandbox firewall
   prompt. Do not change firewall configuration, package, manifest, recipe,
   DNS, shell environment, network route, or command. Require exact stdout
   `codex<TAB>ready` and exit status zero. A failure after the operator's
   firewall decision is terminal and forbids provider dispatch or a prepare
   retry.
4. Immediately run the identical public prepare command once more as a
   cache-hit verification. Require the same exact stdout and exit zero with no
   first-use provisioning message or network/firewall contact. This second
   invocation verifies immutable reuse; it is not permission to repair a
   failed first invocation.
5. After both preparation checks pass, require a newly clean working copy; a
   marker-free outer shell with `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
   `CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` absent; packaged Codex
   status `absent`; packaged global list output `no capsules`; and raw inventory
   `[]`.
6. From `/Users/cody/dev/fortlet`, invoke the immutable package's exact shim
   once:

   ```text
   <package>/libexec/fortlet/shims/codex exec --ephemeral \
     --skip-git-repo-check <frozen prompt>
   ```

7. The exact frozen prompt is:

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

8. Permit exactly one Codex process and one prompt. No follow-up, prompt edit,
   retry, second process, target override, native fallback, direct capsule
   mutation, credential inspection, or unrelated network request is in scope.
9. Preserve the first diff unchanged, independently review it, and run the same
   focused test through `nix develop` on the host.
10. Query packaged global inventory and project-scoped status, then use only
    the same package's public stop and reset commands. Require final status
    `absent`, `fortlet list` output `no capsules`, and raw inventory `[]`.

## Alternatives

1. Treat Experiment 0035 as product evidence. Rejected because the operator
   identified the missing host firewall approval after merge; Codex never
   launched and the corrected mechanism was never contacted.
2. Launch Codex and let it prepare implicitly again. Rejected because public
   `prepare` can settle firewall and project-layer readiness without spending a
   provider request.
3. Reuse the prior published environment identity. Rejected because the
   corrected manifest identity was not successfully published and substituting
   an older layer would not test the correction.
4. Add a DNS or firewall workaround to Fortlet. Rejected because no product
   defect is established and firewall configuration is operator-owned.
5. Use a synthetic model provider. Rejected because the remaining claim is the
   fully composed ordinary Codex work loop after environment preparation.

## Risks

1. The firewall prompt may not appear, may name an unexpected executable, or
   approval may arrive after apt exits. Any first-prepare failure is terminal;
   do not retry or dispatch Codex.
2. Public preparation may expose an independent recipe, DNS, package, or
   validation failure. Preserve it without product repair under this GOAL.
3. Codex may alter the target, command, or declared path. Reject the exact unit
   without repair or follow-up.
4. Cargo may make ordinary public crates.io requests from the cold
   `target/fortlet-guest`; only requests automatically required by the exact
   command are declared.
5. Cleanup may reveal unexpected or unowned state. Never mutate an unowned
   capsule.

## Acceptance Criteria

1. Exact treatment revision/package, complete deterministic checks, clean
   repository, public absence, and empty inventories are recorded before
   preparation.
2. The sole cold public prepare succeeds after only the expected firewall
   approval, and the immediate second prepare proves a verified cache hit with
   no provisioning contact.
3. The sole shimmed process reports the exact corrected target, authenticates,
   streams without Apps failure, changes only the declared test file, runs the
   exact focused Cargo test without a target override, and exits zero.
4. Independent review accepts the diff and the same host test passes through
   `nix develop`.
5. Public cleanup ends at status `absent`, `no capsules`, and raw inventory
   `[]`, preserving immutable layers and persistent harness state.
6. Any preparation failure after the firewall decision or any model-unit
   failure rejects the experiment terminally. There is no retry or successor
   process inside this GOAL.

## Budget and Plan

One immutable package, one cold prepare, one immediate cache-hit prepare, one
packaged Codex process and prompt, one focused guest test, one independent host
test, one owned capsule, zero retries, no external money, and at most 45 minutes
of engineering time. Network is limited to the recipe's pinned public Debian,
Rust, and Jujutsu traffic, ordinary exact-command Cargo traffic if needed, and
the single normal Codex provider request.

Freeze and record the package, pass the absence preflight, run preparation with
the operator present, prove the cache hit, pass the marker-free launch
preflight, dispatch once, review and independently test, clean up publicly, and
close terminally in that order.

## Rehearsal

The complete deterministic gate passes on merged `main`. Experiment 0034
proved authentication, Apps disablement, streaming, EOF, the intended test-only
edit pattern, and independent host testing. The retained repository fixture
asserts the corrected target lies under Codex's observed workspace-write root.
No preparation or provider process has run under this experiment identity.

## Results

The immutable declaration revision and package are recorded above, and the
complete pre-dispatch deterministic gate passed. No runtime unit has been
dispatched.

## Terminal Closure

Pending.
