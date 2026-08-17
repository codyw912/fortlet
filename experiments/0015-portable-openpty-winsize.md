# Experiment 0015: Portable `openpty` winsize pointer

Status: declared
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

Pending local treatment.

## Results

Pending declared treatment.

## Terminal Closure

Pending.
