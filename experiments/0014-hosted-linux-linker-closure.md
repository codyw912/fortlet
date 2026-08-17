# Experiment 0014: Hosted Linux linker closure

Status: rejected
Design: FIP-0001, FIP-0005
Charter scope: `local-foundation/v1` plus explicit operator successor
authorization on 2026-08-17

## Baseline / Control

Experiment 0013 terminally rejected its only hosted unit. Draft pull request
`https://github.com/codyw912/fortlet/pull/1` targets `main` from
`goal/project-capsule-reset` at signed SHA
`650ead13e926086f10f4453da83a7704fdad6bad`. Run `32039976577`, job
`95417454658`, checked out that exact SHA and installed Rust `1.97.1`, then
failed its first `cargo test` while linking with
`rust-lld: error: unable to find library -lcap-ng`. No later gate ran and no
retry, correction push, readiness, merge, or settings mutation occurred.

## Hypothesis and Production Mechanism

Installing Ubuntu Noble's `libcap-ng-dev` without recommended packages before
the unchanged Rust gates will supply the `libcap-ng.so` linker file required by
the Linux MicroSandbox dependency. The same product and tests should then link,
allowing tests, formatting, strict Clippy, and conformance to run normally.

## Declared Scope

1. Add one `ubuntu-24.04` package-install step before the Rust gates using
   `sudo apt-get update` and
   `sudo apt-get install --yes --no-install-recommends libcap-ng-dev`.
2. Extend deterministic workflow evidence to require that exact package and
   its position before every Cargo gate.
3. Run the focused contract test and complete local verification set.
4. After separate exact operator approval, push one correction to the existing
   `goal/project-capsule-reset` bookmark. Observe the one automatically created
   replacement pull-request run to a terminal result.

The runner image, Rust and action pins, Cargo commands, permissions, PR
metadata, base branch, product code, Nix policy, and every unrelated GitHub
resource remain frozen. No hosted Nix, custom image, cache, VM, runtime,
harness, second PR, readiness, merge, settings change, or second replacement
run is in scope.

## Alternatives

1. Run hosted Nix. Rejected because the current GOAL explicitly defers it and
   it adds installer, cache, build-time, and supply-chain scope to a one-library
   linker defect.
2. Publish a custom CI image. Rejected because image ownership, patching, and
   registry lifecycle cost more than installing one distribution package.
3. Feature-gate MicroSandbox out of hosted tests. Rejected because it would
   avoid compiling and linking the actual product dependency instead of
   closing its Linux build requirements.
4. Cache APT state. Rejected because the cache key and maintenance surface are
   larger than the expected one-package installation cost.

## Risks

1. The package may not close the full Linux dependency set. Any linker or gate
   failure rejects the single replacement unit and stops without retry.
2. An unpinned distribution package may change within Ubuntu Noble. The runner
   release is pinned, the package name is explicit, and a future version issue
   must become a new experiment rather than an improvised pin.
3. The correction could alter more than the workflow contract and evidence.
   Exact diff inspection before publication prevents that push.
4. A push could produce an unintended run or ref change. Post-push inventory
   must show only the existing bookmark update and one replacement PR run.

## Acceptance Criteria

1. The deterministic test proves the exact no-recommends package installation
   occurs after checkout and before every Cargo command.
2. The complete standard local verification set passes on the correction tip.
3. A separately approved push updates only the existing goal bookmark with
   valid SSH-signed revisions and preserves the reviewed tree.
4. Exactly one new `pull_request` run checks the corrected signed tip and its
   tests, formatting, strict Clippy, and conformance all pass.
5. Any local gate, signature, ref, run-count, package-install, linker, or hosted
   gate failure rejects the unit and stops without bypass or retry.

## Budget and Plan

Budget: zero money, one workflow dependency, one deterministic-test change,
one separately approved correction push, one replacement hosted run, and zero
other remote mutations. Declare first; implement and verify locally; inspect
the exact stack, diff, remote baseline, and failed control; request push
approval; verify the signed tip and single run; stop on its terminal result.

## Rehearsal

Completed locally on 2026-08-17 without remote mutation:

1. Checkpoint `b0ecd9f0` changes only `.github/workflows/verify.yml` and
   `tests/publication_workflow.rs`: five workflow lines and the deterministic
   contract evidence. No product, package, permission, runner, action, Rust,
   Cargo-gate, PR-metadata, or remote setting changed.
2. Before treatment, the focused contract test rejected because its observed
   command list lacked both declared APT commands. After treatment and
   formatting, all three workflow-contract tests and focused strict Clippy
   passed.
3. The implementation checkpoint passed 39 unit tests, 20 integration tests,
   formatting, strict all-target/all-feature Clippy, the dedicated conformance
   test, and `nix flake check` on `aarch64-darwin`. Nix emitted the known
   missing app metadata warning and omitted incompatible `x86_64-linux`.
4. Static workflow evidence proves checkout precedes the package step, both
   exact APT commands precede the pinned toolchain and every Cargo command,
   and the successor adds no cache, container, hosted Nix, secret, VM,
   MicroSandbox runtime, or harness use.
5. Ubuntu Noble's package inventory identifies `libcap-ng-dev` as the
   development package and lists the `libcap-ng.so` linker file supplied for
   supported architectures. The hosted replacement run remains the only
   production test of that mechanism.

This rehearsal does not authorize the correction push. The exact qualified
tip, outgoing stack and diff, remote PR/head/run baseline, signatures, and
Jujutsu push dry-run must be refreshed and presented for separate approval.

## Results

1. The operator approved the exact correction push after reviewing its stack,
   diff, destination, local evidence, and rejected control. Jujutsu pushed only
   `goal/project-capsule-reset`; remote `main` remained at
   `e95b0cdb1308f732d3f45db7a85027d45bcd4048` and the existing draft pull
   request remained open and draft.
2. GitHub stored the corrected branch at signed SHA
   `0b75f02dd5422362dd49acff776aecff1e94ad7b`. The four new remote commits had
   valid GitHub SSH-signature verification, and no unrelated ref or remote
   resource changed.
3. Exactly one replacement `pull_request` run was created: run `32042472155`,
   job `95424137789`, for that exact corrected SHA. Checkout, the declared APT
   package installation, Rust installation, `cargo test`, and formatting all
   passed. This accepts the narrow linker hypothesis: `libcap-ng-dev` supplied
   the missing Linux linker input without a cache, image, Nix installation, or
   product feature bypass.
4. Strict all-target/all-feature Clippy then failed at
   `examples/pty_observer.rs:233` under `clippy::unnecessary_mut_passed` because
   Linux's `libc::openpty` does not require a mutable reference for the
   `winsize` argument. Conformance was consequently skipped. The same source
   passed the complete local macOS Clippy gate, so the hosted run exposed a
   platform-specific source-lint gap rather than another CI bootstrap defect.
5. The run was not retried and no second correction, readiness mutation,
   merge, setting change, or other remote mutation occurred.

## Terminal Closure

Rejected on 2026-08-17. The production mechanism closed the missing-linker
control and let the Linux test suite pass, but the declared unit required every
hosted gate to pass. Its Linux-only Clippy failure therefore rejects the unit
under Acceptance Criterion 5 and exhausts the authorized correction push and
replacement run.

Any follow-up must be a separately declared and authorized successor. Its
minimum hypothesis would be that a platform-correct `openpty` winsize argument
can satisfy strict Linux Clippy while preserving Darwin compilation and
behavior. Experiment 0014 grants no authority to implement, push, rerun, mark
the pull request ready, merge, or mutate repository settings.
