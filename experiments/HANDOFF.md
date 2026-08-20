# Session Handoff — capsule launch latency ready for operator merge

Audience: a fresh agent session. `GOAL.md` is normative. Verify `main`, the
Jujutsu stack, conformance, and the complete `docs/RUNBOOK.md` gate before
relying on this summary. Read FIP-0001, FIP-0002, and FIP-0005 in full before
implementation or experiment dispatch.

## Verified baseline

PR #13 was squash-merged as `5c5b6d2fba62f97cd462fe77e090af307483efb3`.
Its tree matched the exact reviewed signed tip. The working copy is a clean
empty child of `main`, and `main..@` contains no work.

The complete successor baseline gate passes on aarch64-darwin through
`nix develop`: 88 unit tests and every enabled integration test, formatting,
strict all-target/all-feature Clippy, conformance, the curated package, and the
shim-activation check. Nix reports only the expected incompatible
`x86_64-linux` omission warning.

## Why this mission exists

Experiment 0042 proved project-local shim activation in a fresh real fish, but
its first managed `codex --version` from an absent capsule took approximately
nine seconds. That was one coarse end-to-end shell observation. It did not
measure attachment to the running capsule and did not separate Fortlet
preparation, MicroSandbox materialization or boot, PTY attachment, and Codex
startup.

MicroSandbox's advertised sub-100-millisecond boundary is guest boot on an M1,
not Fortlet's complete absent-to-command transaction. MicroSandbox 0.6.8 also
reports guest kernel boot, filesystem initialization, and total
boot-to-agent-ready timing internally. Fortlet currently does not expose those
events. The cached `node:24-bookworm` image predates Experiment 0042, native
Codex returns in roughly 40 milliseconds when warm, and AGD had no Fortlet
project-environment recipe, so image download, native Codex startup, and
project provisioning are not credible explanations for most of nine seconds.

## Current mission

The locally knowable mission is complete. Experiment 0043 rejected the strict
materialization hypothesis but established packaged medians of 0.46 seconds
absent, 0.41 seconds stopped, and 0.06 seconds running. Experiment 0044 stopped
honestly when its selected development executable was stale. Experiment 0045
accepted after exact-binary rehearsal: instrumented medians were 610
milliseconds absent, 410 milliseconds stopped, and 55 milliseconds running.

Runtime reconciliation accounted for 76–87 percent of every absent and
stopped total. Running reconciliation took 7–8 milliseconds, and attachment
through Codex exit took 35–40 milliseconds. The approximately nine-second
Experiment 0042 aggregate did not reproduce in 24 valid successor launches and
is not a persistent daily-use result.

Fortlet now exposes exact opt-in `FORTLET_STARTUP_TIMINGS=1` events for
`resolve`, `credentials`, `capsule-state`, `environment`, `runtime`, and
`command`. Normal startup remains silent. The event format is fixed and emits
only delta and total milliseconds, never paths, environment values,
credentials, configuration, or harness output. Do not enable unrestricted SDK
tracing in its place.

The final aarch64-darwin gate passes through `nix develop`: 91 unit tests and
every enabled integration, formatting, strict Clippy, conformance, the curated
package, and shim-activation check. Nix reports only the expected incompatible
x86_64-linux omission. Every experiment ended with AGD Codex absent, global
inventory empty, and AGD retaining exactly its three prior environment-file
changes.

## Next action

PR #14 targets `main` from the authorized `capsule-launch-latency` bookmark.
Its required hosted Rust verification passes. Verify its final signed tip and
mark it ready, then wait for the operator to merge. After merge, fetch `main`,
prove exact tree equality with the reviewed tip, and remove only the landed
goal bookmark locally and remotely. The operator remains the sole merge
authority.

## What not to do

Do not treat the nine-second aggregate as guest boot time or as current product
behavior. Do not optimize the measured sub-second lifecycle speculatively. Do
not compare an Alpine provider benchmark directly with Fortlet's Node image and
mounted, brokered capsule. Do not change root-disk persistence, switch
runtimes, or reduce isolation without a successor proposal. Do not send a model
prompt during latency work.
