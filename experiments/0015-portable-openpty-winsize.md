# Experiment 0015: Portable `openpty` winsize pointer

Status: accepted
Design: FIP-0001, FIP-0005
Charter scope: `local-foundation/v1` plus explicit operator successor
authorization on 2026-08-17

## Baseline / Control

Experiment 0014 published signed SHA
`0b75f02dd5422362dd49acff776aecff1e94ad7b` to the existing draft pull
request. Replacement run `32042472155`, job `95424137789`, installed the
declared Linux package, passed `cargo test` and formatting, then failed strict
Clippy at `examples/pty_observer.rs:233` under
`clippy::unnecessary_mut_passed`. The source passes the same strict gate on
`aarch64-darwin` because `libc` 0.2.189 declares `openpty`'s winsize parameter
as `*mut winsize` on Apple and `*const winsize` on glibc Linux.

Before treatment, the current local tree passed 39 unit tests, 20 integration
tests, formatting, strict all-target/all-feature Clippy, the dedicated
conformance test, and `nix flake check` on `aarch64-darwin`. The Nix check
emitted the known missing app metadata warning and omitted incompatible
`x86_64-linux`.

## Hypothesis and Production Mechanism

Constructing one explicit raw mutable pointer from the initialized mutable
`winsize` and passing that pointer to `libc::openpty` will satisfy both platform
bindings: Apple receives the required mutable pointer, while Linux coerces it
to its const parameter without receiving a syntactic mutable reference that
triggers strict Clippy. The C call, dimensions, ownership, and PTY behavior
remain unchanged.

## Declared Scope

1. Change only the winsize argument construction in
   `examples/pty_observer.rs`; retain the existing unsafe call and error,
   descriptor-ownership, fixture, resize, signal, and termination behavior.
2. Run the PTY observer example tests and complete local verification set.
3. After separate exact operator approval, push one correction to the existing
   `goal/project-capsule-reset` bookmark and observe the one automatically
   created replacement pull-request run to a terminal result.

The `libc` version, PTY observer interface and timeouts, workflow, runner,
package installation, action and Rust pins, Cargo commands, permissions, PR
metadata, base branch, product runtime, and every unrelated GitHub resource
remain frozen. No dependency update, lint allowance, conditional compilation,
hosted Nix, custom image, cache, VM, harness, new PR, readiness, merge, settings
change, or second replacement run is in scope.

## Alternatives

1. Add a Linux-only Clippy allowance. Rejected because it suppresses a useful
   API-contract distinction instead of expressing a pointer valid for both
   signatures.
2. Add platform-specific `openpty` call branches. Rejected because the only
   platform difference is pointer constness and one raw pointer represents the
   shared operation without duplicated unsafe calls.
3. Cast an immutable pointer to mutable for Apple. Rejected because the Apple
   binding permits mutation; retaining mutable storage avoids asserting an
   immutability guarantee across that FFI boundary.
4. Update or wrap `libc`. Rejected because the pinned binding accurately
   represents the platform headers and the call site can honor both contracts.

## Risks

1. Strict Linux Clippy may diagnose the explicit pointer construction or a
   later target-specific issue. Any hosted gate failure rejects this unit and
   stops without retry.
2. Pointer refactoring could accidentally change the lifetime or mutability of
   the stack value passed to C. Keeping the initialized value in the same
   function and deriving the pointer immediately before the call limits that
   risk; existing fixture tests exercise PTY creation.
3. Another correction could alter more than the one FFI seam. Exact diff and
   stack inspection before publication prevents that push.
4. A push could create an unexpected ref or run. Post-push inventory must show
   only the existing bookmark update and one replacement PR run.

## Acceptance Criteria

1. The diff changes only the declared winsize-pointer seam and experiment
   evidence; it adds no lint allowance, platform branch, dependency, or
   workflow change.
2. `cargo test --example pty_observer` and the complete standard local
   verification set pass on `aarch64-darwin`.
3. A separately approved push updates only the existing goal bookmark with
   valid SSH-signed revisions and preserves the reviewed tree.
4. Exactly one new `pull_request` run checks the corrected signed tip and its
   tests, formatting, strict Clippy, and conformance all pass.
5. Any local gate, signature, ref, run-count, PTY test, or hosted gate failure
   rejects the unit and stops without bypass or retry.

## Budget and Plan

Budget: zero money, one FFI call-site correction, no new test apparatus, one
separately approved correction push, one replacement hosted run, and zero
other remote mutations. Declare and checkpoint authority first; implement the
pointer correction; run focused and complete local verification; inspect the
exact stack, diff, remote baseline, and rejected control; request push
approval; verify the signed tip and single run; stop on its terminal result.

## Rehearsal

Completed locally on 2026-08-17 without remote mutation:

1. Stable change `nqzsqukqvzkwuwlrmwmsotnzqzlnsxux` (then commit
   `dca8c70f565e945094a2124849dc035a0cb3e882`) changes only
   `examples/pty_observer.rs`: it derives a named raw pointer from the existing
   mutable winsize and passes that pointer to the unchanged `openpty` call. The
   diff is three insertions and one deletion, including the platform-contract
   rationale.
2. The source and locked `libc` 0.2.189 definitions confirm the control:
   Apple's binding accepts `*mut winsize`, glibc Linux accepts
   `*const winsize`, and a Rust raw mutable pointer coerces to either required
   parameter without an immutable-to-mutable cast.
3. `cargo test --example pty_observer` passed all 15 tests, exercising PTY
   creation, activity, resize, signal, command and key input, exit status,
   timeout, and owned child cleanup. Focused strict Clippy also passed.
4. The implementation checkpoint passed 39 unit tests, 20 integration tests,
   formatting, strict all-target/all-feature Clippy, the dedicated conformance
   test, and `nix flake check` on `aarch64-darwin`. Nix emitted the known
   missing app metadata warning and omitted incompatible `x86_64-linux`.
5. Simplification review retained the named pointer and one non-obvious comment
   because both make the cross-platform FFI reason explicit. No dependency,
   lint allowance, conditional compilation, workflow, runner, package, test
   apparatus, product runtime, or remote resource changed.

This rehearsal does not authorize the correction push. The exact qualified
tip, outgoing stack and diff, remote PR/head/run baseline, signatures, and
Jujutsu push dry-run must be refreshed and presented for separate approval.

## Results

1. The operator approved the exact correction push after reviewing its four
   checkpoints, complete diff, destination, local evidence, signing plan,
   remote baseline, and rejected control. Jujutsu pushed only
   `goal/project-capsule-reset`; remote `main` remained at
   `e95b0cdb1308f732d3f45db7a85027d45bcd4048` and PR #1 remained open and
   draft with unchanged title, base, head name, and metadata.
2. Signing on push rewrote the four local Git commit IDs and stored the
   corrected branch at signed SHA
   `e2e4a6d8b24e974add01a720ddaf73c71de7963a`. GitHub reports all 18 commits
   attached to PR #1 as valid SSH signatures. Remote inventory contained only
   `main` and the updated goal branch.
3. Exactly one replacement `pull_request` run was created: run `32052509798`,
   job `95455045523`, for that exact signed SHA. Checkout, the unchanged Linux
   package installation, Rust installation, `cargo test`, formatting, strict
   Clippy, conformance, and cleanup all passed. The job completed successfully
   in 3 minutes 19 seconds.
4. This accepts the declared mechanism: the same explicit raw mutable pointer
   satisfies Apple's mutable `openpty` binding and Linux's const binding while
   preserving PTY behavior and eliminating the Linux-only lint. No lint
   allowance, platform branch, dependency, workflow, or test apparatus was
   needed.
5. No rerun, second correction, PR metadata or readiness mutation, merge,
   settings change, or other remote mutation occurred.

## Terminal Closure

Accepted on 2026-08-17. The platform bindings differ only in winsize-pointer
constness, and one pointer derived from mutable storage honors both contracts.

Actual cost was zero money, one of one call-site corrections, one of one
correction pushes, one of one replacement hosted runs, and zero other remote
mutations. The experiment is terminal and must not be resumed. Its local
terminal record may travel only through a separately authorized publication
transaction. The next action is to refresh the primary PR's reviewed revision
and verification evidence and consider bootstrap readiness under separate
exact approvals; no additional source correction or branch push is authorized.
