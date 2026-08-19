# Experiment 0027: Packaged non-interactive streaming rehearsal

Status: declared — 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0008, FIP-0009, and FIP-0010
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0026 accepted that MicroSandbox 0.6.8 delivers explicit exit events
for normal commands and stock Codex 0.147.0, while collected execution hides
progress from a still-running request. FIP-0010 changes Fortlet's generic
non-terminal attachment from collected to streaming execution and assigns only
Codex a ten-minute inactivity ceiling.

The treatment is checkpoint `7d15cddefcff`, with focused streaming, deadline,
renewal-cancellation, conformance, formatting, and strict Clippy checks already
green. The exact immutable Nix package and full revision will be frozen and
recorded before dispatch.

## Hypothesis and Production Mechanism

The packaged public `fortlet run tact -- --version` path will acquire a
non-PTY MicroSandbox streaming handle with null stdin, forward the guest's
version output, observe its explicit zero exit event, and return zero. A
synthetic host auth document satisfies the existing Tact launch precondition
without containing or contacting provider credentials.

## Declared Scope

1. Build one immutable package from the frozen treatment and record its store
   path and full revision.
2. Create one fixed, temporary synthetic auth document under
   `/private/tmp/fortlet-exp0027-auth`; it contains no real token, refresh
   token, or account.
3. Require public Tact absence and an empty MicroSandbox inventory, then run
   exactly once from `/Users/cody/dev/fortlet`:

   ```text
   FORTLET_AUTH_FILE=<synthetic-auth> <package>/bin/fortlet run tact \
     --project /Users/cody/dev/fortlet -- --version
   ```

4. Record stdout, stderr, and exact host status. Use only the same packaged
   public status, stop, and reset commands for cleanup; require final absence
   and an empty inventory.

No Codex launch, provider request, network fixture, real credential read,
repository edit by the guest, retry, second treatment unit, native fallback,
private capsule mutation, remote mutation, release, or publication is in
scope.

## Alternatives

1. Use packaged Codex `--version`. Rejected because Fortlet's Codex path must
   renew a real host login before attachment, violating the credential-free
   rehearsal requirement.
2. Repeat direct `msb exec`. Rejected because Experiment 0026 already proved
   the SDK behavior and this unit must exercise packaged Fortlet production
   code.
3. Use a shell-only guest command. Rejected because the public Fortlet surface
   launches registered harnesses, and Tact's local version command is the
   smallest faithful harness process.

## Risks

1. Existing Tact capsule state could invalidate the clean baseline. If it is
   not absent, terminate the experiment before launch and clean up only through
   a separately authorized successor.
2. Preparation could require an unavailable cached artifact. Any network or
   provisioning need rejects this zero-network rehearsal without retry.
3. A synthetic token could accidentally reach a provider if argument handling
   changes. `--version` must terminate locally; any request diagnostic or
   unexpected delay rejects the unit and triggers public cleanup.

## Acceptance Criteria

1. The immutable package and revision are recorded, Tact begins absent, and
   MicroSandbox inventory is empty.
2. The sole packaged launch emits the pinned Tact version and returns exact
   status zero without a provider, credential, or network diagnostic.
3. Public status reports the reusable Tact capsule running, public stop/reset
   restore absence, and the final inventory is empty.
4. The first launch failure, invariant anomaly, cleanup failure, or two elapsed
   minutes after dispatch terminates the experiment. There is no retry.

## Budget and Plan

One local packaged Tact `--version` unit, one owned capsule, at most two elapsed
minutes after dispatch, zero real credentials, zero provider requests, zero
paid quota, and no retry. Build and read-only baseline checks do not consume
the treatment unit.

## Rehearsal

The focused deterministic suite passed before declaration: five
non-interactive forwarding/exit/timeout tests, one output-renewal test, two
attachment-cancellation tests, conformance, formatting, and strict Clippy.
Experiment 0026 separately proved stock command success and failure events
against local fixtures.

## Results

Pending.

## Terminal Closure

Pending.
