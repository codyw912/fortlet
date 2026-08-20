# Experiment 0043: Capsule lifecycle latency screen

Status: declared
Design: FIP-0001, FIP-0002

## Baseline / Control

Experiment 0042 observed one approximately nine-second shimmed
`codex --version` from an absent AGD Codex capsule on Apple silicon. It then
verified the capsule running and restored absence through public stop and
reset. It did not measure a stopped or already-running launch.

The exact product baseline is merged `main`
`5c5b6d2fba62f97cd462fe77e090af307483efb3`, packaged Fortlet 0.1.0,
MicroSandbox 0.6.8, Codex 0.147.0, cached `node:24-bookworm` digest prefix
`sha256:7e4b29530885`, and the operator's aarch64-darwin host. The complete
pre-dispatch repository gate passes through `nix develop`. The Fortlet working
copy is clean and `main..@` is empty.

## Hypothesis and Production Mechanism

The nine-second observation is dominated by absent-capsule materialization,
not guest kernel boot or repeated shim attachment. MicroSandbox formats a new
managed 8 GiB ext4 upper and establishes persistent runtime state only for an
absent capsule. A stopped capsule reuses that state, while a running capsule
also skips VM boot. Therefore absent launch should be materially slower than
stopped launch, and both should be materially slower than running attachment.

This is an attribution screen, not an optimization claim. If the three states
do not separate, the result rejects this hypothesis and requires phase-level
instrumentation before treatment.

## Declared Scope

Run three ordered cycles against the exact AGD project using the already-built
package and public Fortlet lifecycle commands. Each cycle starts absent and
performs:

1. one timed shimmed `codex --version` from absent;
2. one timed shimmed `codex --version` while running;
3. public stop;
4. one timed shimmed `codex --version` from stopped;
5. one second timed shimmed `codex --version` while running; and
6. public stop and reset, followed by absent status and empty owned inventory.

The image, package, project, host, credentials, layers, command, terminal mode,
and ordering stay frozen. The experiment may renew the host token through the
existing bounded Fortlet mechanism if required, but it never prints or
inspects credential contents. It makes zero model requests and changes no AGD
file.

## Alternatives

Permanent instrumentation was deferred because lifecycle state is a cheaper
existing discriminator. A direct `msb` benchmark was deferred because it would
not include Fortlet's mounts, credential broker, or orchestration and its
capsule would not carry Fortlet ownership metadata. Profiling and runtime
replacement are premature before basic attribution.

## Risks

Token renewal, host load, filesystem cache warming, terminal differences, or a
firewall prompt could contaminate a sample. Any prompt, refresh failure,
unexpected output, capsule ownership mismatch, or cleanup failure settles the
current unit as failed; it is recorded and not silently retried. Three cycles
are a screen only and cannot establish a durable performance distribution.

## Acceptance Criteria

1. Pre-dispatch status is absent and the complete repository gate is green.
2. All twelve timed commands return Codex 0.147.0 with exit status zero, and
   the observed state matches absent, running, or stopped before each launch.
3. All three cycles end absent with no Fortlet-owned capsule and no AGD file
   change beyond its pre-existing three development-environment files.
4. Accept the materialization hypothesis only if every absent sample is slower
   than every stopped sample and every stopped sample is slower than the median
   running sample by at least 250 milliseconds. Otherwise reject it and add
   phase-level timing before changing production behavior.
5. Running samples above one second are a blocking daily-use result regardless
   of the cold-start attribution.
6. Any failed unit is retained. Two failures sharing an unresolved assumption,
   any ownership or credential anomaly, or inability to restore absence
   terminates the experiment.

## Budget and Plan

Budget: 30 minutes, three absent capsules, twelve credential-free version
commands, zero model prompts, zero external money, and one owned capsule at a
time. Use the fixed order above for all three cycles. Do not adapt the order or
add samples after seeing results.

## Rehearsal

On 2026-08-20, read-only inspection confirmed the packaged image cache contains
`node:24-bookworm`, raw MicroSandbox inventory is empty, the Fortlet working
copy is clean, and the complete lifecycle cleanup commands are already proven
by Experiment 0042. Native Codex 0.147.0 returned in 0.04 seconds on two warm
read-only invocations. No capsule or model request was created during rehearsal.

## Results

Pending.

## Terminal Closure

Pending.
