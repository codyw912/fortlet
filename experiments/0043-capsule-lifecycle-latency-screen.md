# Experiment 0043: Capsule lifecycle latency screen

Status: completed — rejected
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

All twelve packaged shim invocations returned `codex-cli 0.147.0` with status
zero. `/usr/bin/time -p` recorded these end-to-end real durations in seconds:

| Cycle | Absent | Running 1 | Stopped | Running 2 |
| --- | ---: | ---: | ---: | ---: |
| 1 | 0.71 | 0.06 | 0.48 | 0.06 |
| 2 | 0.46 | 0.06 | 0.41 | 0.06 |
| 3 | 0.44 | 0.06 | 0.40 | 0.06 |

Absent ranged from 0.44 to 0.71 seconds with median 0.46. Stopped ranged
from 0.40 to 0.48 seconds with median 0.41. All six running samples were 0.06
seconds. The strict materialization rule failed because the fastest absent
sample, 0.44 seconds, was faster than the slowest stopped sample, 0.48
seconds. The running-capsule daily-use ceiling passed with substantial margin.

Every cycle began and ended absent. Final public status reported `absent`,
global inventory reported `no capsules`, and AGD retained exactly its three
pre-existing modified environment files. No model request, prompt, credential
output, file edit, unexpected output, or failed unit occurred.

## Terminal Closure

1. Outcome: rejected; absent materialization did not dominate stopped restart,
   while running attachment passed the daily-use screen at 0.06 seconds.
2. Root cause: the prior approximately nine-second aggregate did not reproduce;
   current absent and stopped costs overlap around 0.4–0.5 seconds, so lifecycle
   state alone cannot attribute either the ordinary restart cost or the earlier
   outlier.
3. Actual cost: 11 minutes, three absent capsules, twelve version commands,
   zero model prompts, and zero external money versus the 30-minute ceiling.
4. Next action: add opt-in secret-safe phase timing before any performance
   treatment, then run a separately declared successor against the same three
   lifecycle states.
