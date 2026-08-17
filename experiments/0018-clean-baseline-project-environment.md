# Experiment 0018: Clean-baseline project environment

Status: declared
Design: FIP-0001 and FIP-0006
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0017 rejected before dispatch because a stopped Codex capsule made
its baseline non-absent. The operator separately authorized cleanup. Fortlet's
public reset removed the stopped Codex and Tact capsules for the canonical
project `/Users/cody/dev/fortlet`; read-only configuration inspection established
ownership before six stopped Fortlet provisioning and predecessor `agent-env`
capsules were removed. Public status then reported both harnesses absent and
`msb list --stopped --format json` returned an empty list.

Fetched `main` is `633ea638013ba675e82ea7b06b1d3a287d7aa003`. It can
launch Codex and Tact with immutable base and harness layers, but activates no
project toolchain. The treatment is the FIP-0006 stack through checkpoint
`ed2e3596e29a` (`Document opt-in project environments`) on
`aarch64-darwin` with MicroSandbox SDK and CLI 0.6.8 and Codex 0.147.0.

The exact project inputs are:

1. `.fortlet/environment.json` SHA-256
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh` SHA-256
   `011339eb574ffaae3d568328b77560d8620c19a20eb7f093d7a95c10c6c99339`.
3. Guest image `node:24-bookworm`, Rust 1.97.1, Jujutsu 0.43.0, and
   Debian `libcap-ng` 0.8.3-1+b3 for `linux/aarch64`.

No experiment is active before this declaration.

## Hypothesis and Production Mechanism

Because Fortlet snapshots and hashes the two fixed files, runs only the recipe
text with an empty output mount in a credential-free provisioning capsule, and
atomically mounts the validated output at `/opt/fortlet/project`, one public
Codex launch should create a content-verified layer. The resulting capsule
should expose the pinned `cargo`, `rustc`, and `jj` through its declared `PATH`
and should run a focused Fortlet test without host Nix activation.

## Declared Scope

Use the immutable package
`/nix/store/l4cdam1i3077z0f7s6rx6abjyfz0vr6d-fortlet-0.1.0` and the
canonical project `/Users/cody/dev/fortlet` in this fixed order:

1. Reconfirm valid host authentication, existing base and Codex layer markers,
   public `codex<TAB>absent` status, and absence of a published FIP-0006 layer.
2. Run exactly one public non-prompt launch:
   `fortlet run codex --project /Users/cody/dev/fortlet -- --version`.
3. Read-only MicroSandbox queries MUST identify exactly one running capsule by
   the managed, schema, project, tool, and project-environment labels. Inspect
   its configuration for the read-only `/opt/fortlet/project` mount.
4. Use `msb exec` only as an experiment probe against that exact owned capsule.
   Record `command -v` and version output for `cargo`, `rustc`, and `jj`;
   verify the declared Cargo environment; then run
   `cargo test --bin fortlet project_environment` from the mounted project.
5. Publicly stop and reset the exact capsule, verify final public absence, and
   verify the immutable project layer and its marker remain reusable. The
   project, persistent Codex state, Cargo cache, base layer, harness layer, and
   successful project layer are intentionally preserved.

The implementation, inputs, package, project, host, runtime, harness, command
order, and arguments remain frozen. The recipe may make only its declared
unauthenticated public downloads from Debian, Rust, and GitHub. No command may
print, copy, inspect, or record provider credential or fingerprint contents.

No model prompt, native harness, second harness, registry login, private input,
host shell execution of the recipe, project source mutation, publication,
remote mutation, settings change, release, package publication, second live
unit, or unowned capsule is in scope.

## Alternatives

1. Resume experiment 0017. Rejected because terminal experiment records are
   immutable; a successor requires a new number and identity.
2. Ask Codex to run the test. Rejected because it spends a model prompt and
   tests model behavior rather than the environment mount.
3. Add a product `exec` command. Rejected because raw project execution is not
   part of FIP-0006; the runtime CLI is sufficient for experiment-only probing.
4. Run both harnesses live. Rejected because deterministic configuration
   evidence covers both, while one capsule falsifies the shared mechanism.

## Risks

1. A new or unowned capsule could appear after the clean baseline. Any
   non-absent public preflight or ownership mismatch prevents dispatch or
   cleanup.
2. A download, checksum, installer, output validation, or capsule operation may
   fail. The unit closes rejected without a second live unit.
3. The pinned tools may exist but fail to build Fortlet because a Linux system
   dependency or environment path is missing. The focused test makes that a
   rejection rather than accepting version output alone.
4. Project recipe output could expose control characters or unexpected data.
   Diagnostics are bounded and sanitized; any credential-looking or anomalous
   output triggers immediate terminal closure without reproduction.
5. Cleanup may fail after behavior passes. That rejects the complete unit and
   leaves the exact owned artifact reported for operator recovery.

## Acceptance Criteria

1. The immutable package remains the output of the complete standard
   verification set recorded by experiment 0017.
2. Initial public status is exactly `codex<TAB>absent`, both existing Fortlet
   layer markers are present, and no FIP-0006 environment layer exists.
3. The sole public launch exits zero, emits one project first-use preparation
   message and pinned Codex 0.147.0, and emits no prompt or credential value.
4. Exactly one running capsule matches all five ownership/configuration labels.
   Its project layer mount is read-only and its source is the sole newly
   published content-verified environment directory.
5. Inside that capsule, `cargo` and `rustc` report 1.97.x, `jj` reports 0.43.0,
   all three resolve below `/opt/fortlet/project/bin`, and the declared Cargo
   home and target directory values are present.
6. `cargo test --bin fortlet project_environment` exits zero inside the capsule
   from the canonical project working directory.
7. No provisioning capsule remains. Public stop and reset succeed, final status
   is exactly `codex<TAB>absent`, and the project, persistent state, base,
   harness, and verified project layers remain.
8. Every criterion must pass to accept this one-unit screen. One failure rejects
   it. The experiment terminates on one failure, a hard-invariant anomaly,
   completed cleanup, or 25 elapsed minutes after dispatch begins.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, zero remote mutation,
one Codex `--version` launch, one owned local capsule, one project-layer build,
unauthenticated public downloads only, no second live unit, and at most 25
elapsed minutes after dispatch begins. Record exact revision, package, inputs,
commands, sanitized output, labels, mount, tool versions, focused-test result,
preservation, cleanup, and elapsed time.

## Rehearsal

The immutable package passed 52 unit tests, 22 integration tests, formatting,
strict all-target/all-feature Clippy, conformance, `nix flake check`, and
`nix build .#fortlet`. Experiment 0017 records the two pre-dispatch source-filter
failures and the operator-authorized correction that preceded this green run.
The base and Codex harness markers are present, the project environment
directory is absent, authentication is valid for at least one hour, public
Codex and Tact status is absent, and the stopped-capsule list is empty.

## Results

Pending dispatch.

## Terminal Closure

Pending terminal outcome.
