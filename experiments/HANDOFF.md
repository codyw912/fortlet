# Session Handoff — capsule launch latency active

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

Run Experiment 0043 exactly as declared. Its lifecycle comparison comes before
instrumentation: absent, stopped, and running results can identify whether the
cost belongs to one-time materialization, VM restart, or every attachment. Do
not make a performance correction until this screen is terminal and attributed.

The absent path currently creates a managed 8 GiB ext4 writable upper, records
MicroSandbox state, starts a detached VM and relay, mounts the project,
persistent home, harness layer, and base layer, installs network-broker policy,
then attaches Codex. A stopped capsule reuses its writable upper. A running
capsule skips VM creation and restart but still performs credential rotation,
touch, and attachment. These lifecycle differences are the first attribution
instrument.

## What not to do

Do not treat the nine-second aggregate as guest boot time. Do not compare an
Alpine provider benchmark directly with Fortlet's Node image and mounted,
brokered capsule. Do not enable unrestricted SDK tracing around credentials,
change root-disk persistence, switch runtimes, or reduce isolation before the
dominant phase is measured. Do not send a model prompt during latency work.
