# Experiment 0014: Hosted Linux linker closure

Status: declared
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

Pending local implementation and complete verification. No remote mutation is
authorized by this declaration.

## Results

Pending declared treatment.

## Terminal Closure

Pending.
