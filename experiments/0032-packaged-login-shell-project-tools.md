# Experiment 0032: Packaged login-shell project tools

Status: declared — 2026-08-19
Design: FIP-0001, FIP-0006, and FIP-0007
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0031 accepted the causal comparison: the existing capsule PATH
resolved executable Cargo 1.97.1 directly and through `bash -c`, Debian
`bash -lc` replaced the PATH and lost Cargo, and a disposable `BASH_ENV`
restored the exact tool path. Experiment 0030's model-produced test-only diff
passes independently on the host but its sole guest Cargo invocation could not
start. No second model prompt is permitted.

Checkpoint `70aff643` implements the correction under accepted FIP-0006. Base
layer `bookworm-2` installs a read-only Fortlet-owned Bash environment fragment;
capsules reserve `BASH_ENV`, provide the exact managed PATH through
`FORTLET_MANAGED_PATH`, and never write a user startup file. Before declaration,
all 83 unit tests and all enabled integration tests passed through `nix
develop`; formatting, strict Clippy, and conformance were green.

## Hypothesis and Production Mechanism

An immutable package from this declaration will provision or reuse base layer
`bookworm-2`, mount its Bash environment fragment read-only, and expose the
managed PATH to a non-interactive login shell. Therefore `bash -lc` will resolve
project-layer Cargo and complete the focused Fortlet test that failed to start
in Experiment 0030.

## Declared Scope

1. Build one immutable aarch64-darwin package from this declaration and record
   its full revision and store path before dispatch.
2. Require a clean working copy, public Tact absence, and empty packaged
   MicroSandbox inventory. Record whether base layer `bookworm-2` already
   exists; preserve every prior immutable layer.
3. Run exactly one public credential-free launch:

   ```text
   fortlet run tact --project /Users/cody/dev/fortlet -- --version
   ```

   First use may make the existing pinned unauthenticated Debian request needed
   to provision base layer `bookworm-2`. No provider or model traffic is allowed.
4. Resolve the sole owned Tact capsule and run exactly one direct non-TTY guest
   process:

   ```text
   /usr/bin/bash -lc 'set -eu
   printf "bash_env=%s\n" "$BASH_ENV"
   printf "login_path=%s\n" "$PATH"
   printf "cargo_path=%s\n" "$(command -v cargo)"
   cargo --version
   cargo test --bin fortlet runtime::tests::non_interactive_stream_closes_stdin_before_reading_events'
   ```

5. Inspect the selected capsule configuration and new base marker read-only.
   Use only packaged public status/stop/reset for cleanup; require final Tact
   absence and empty inventory.

No Codex launch, model request, prompt, retry, second capsule, repository edit,
host startup-file write, persistent-home hook, deletion, publication, or remote
mutation is in scope. The focused Cargo command may update only the existing
persistent guest Cargo target/cache owned by this project and harness.

## Alternatives

1. Accept deterministic string tests alone. Rejected because the defect existed
   only in a composed real login-shell path that prior unit coverage missed.
2. Send the same Codex task again. Rejected because it would violate the one-unit
   provider budget and is unnecessary for validating the environment mechanism.
3. Reuse base layer `bookworm-1`. Rejected because immutable layer identity must
   change when its content gains the Bash environment fragment.

## Risks

1. Base provisioning may fail because its pinned Debian artifact is unavailable.
   One failure rejects the unit without fallback or retry.
2. MicroSandbox may not propagate `BASH_ENV` or the managed variable into the
   login shell. Missing output or Cargo status 127 rejects the mechanism.
3. The PATH may resolve an unintended Cargo. The exact path must remain beneath
   `/opt/fortlet/project/bin`.

## Acceptance Criteria

1. Immutable identity and clean baselines are recorded before dispatch.
2. The sole Tact launch exits zero and creates exactly one owned running capsule
   using base layer `bookworm-2` with the package-owned hook.
3. The sole login-shell process reports
   `/opt/fortlet/base/etc/fortlet/bash-env`, resolves Cargo beneath
   `/opt/fortlet/project/bin`, reports Cargo 1.97.x, and exits zero after the
   exact focused test passes.
4. No repository diff is added by the live unit. Public cleanup reports running,
   stopped, reset, absent, and final inventory `[]`.
5. Any failed criterion settles the unit and experiment without retry.

## Budget and Plan

Zero money, zero paid quota, zero model prompts, one immutable package, at most
one pinned unauthenticated Debian preparation request, one Tact version launch,
one login-shell process, one focused test, one owned capsule, zero retries, and
at most 20 elapsed minutes. Build, baseline, dispatch, inspect, clean up, and
record in that order.

## Rehearsal

Experiment 0031 ran the complete shell comparison in the same pinned
MicroSandbox 0.6.8 and `node:24-bookworm` environment, including a successful
disposable `BASH_ENV` treatment arm. The current deterministic implementation
gate is green. No live process has run under this declaration.

## Results

Pending.

## Terminal Closure

Pending.
