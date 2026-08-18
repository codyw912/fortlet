# Experiment 0019: Output-backed project provisioning

Status: rejected
Design: FIP-0001 and FIP-0006
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0018 rejected its sole live unit when the provisioning capsule's
writable root filled while the repository recipe stored and expanded the full
Rust 1.97.1 standalone distribution in `/tmp`. It published no project layer,
created no harness capsule, and left no provisioning capsule. The treatment
changes only the recipe workspace and its fixture assertion at checkpoint
`d0a917dd1084` (`Keep project environment staging off capsule root`):
`mktemp` now creates the transient directory under the host-backed
`$FORTLET_OUTPUT`, and the existing exit trap removes it before validation.

The immutable treatment package is
`/nix/store/c7mj5wfai029g435ll6np9byld2nksx5-fortlet-0.1.0` on
`aarch64-darwin` with MicroSandbox SDK and CLI 0.6.8 and Codex 0.147.0.
Fetched `main` remains `633ea638013ba675e82ea7b06b1d3a287d7aa003`.
The exact inputs are:

1. `.fortlet/environment.json` SHA-256
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh` SHA-256
   `e0d6fe2ef1ec9fea18024d1ec9c2ad464ddca0091fec6e9451633fd18640af5b`.
3. Guest image `node:24-bookworm`, Rust 1.97.1, Jujutsu 0.43.0, and Debian
   `libcap-ng` 0.8.3-1+b3 for `linux/aarch64`.

Read-only preflight found both existing immutable layer markers, no project
environment layer, valid authentication for at least one hour, a safe credential
boundary, public Codex and Tact absence, and no MicroSandbox capsules. No other
experiment is active before this declaration.

## Hypothesis and Production Mechanism

Because the archive, expanded Rust distribution, and final installed output now
share the host-backed `/out` filesystem instead of consuming the capsule root,
one otherwise identical public Codex launch should provision and atomically
publish the content-verified layer. Fortlet should then mount it read-only and
expose the pinned Rust and Jujutsu tools without host recipe execution.

## Declared Scope

Use the immutable package and canonical project `/Users/cody/dev/fortlet` in
this fixed order:

1. Reconfirm the recorded marker, authentication, layer-absence, capsule-absence,
   and public `codex<TAB>absent` preconditions.
2. Run exactly one public non-prompt launch:
   `fortlet run codex --project /Users/cody/dev/fortlet -- --version`.
3. Query MicroSandbox read-only and require exactly one running capsule matching
   the managed, schema, project, tool, and project-environment labels. Inspect
   its configuration for a read-only `/opt/fortlet/project` mount whose source
   is the sole newly published environment directory.
4. Use `msb exec` only against that exact owned capsule. Record `command -v` and
   versions for `cargo`, `rustc`, and `jj`; verify `CARGO_HOME` and
   `CARGO_TARGET_DIR`; run `cargo test --bin fortlet project_environment` from
   the canonical mounted project.
5. Publicly stop and reset the exact capsule. Verify final public absence, no
   provisioning capsule, and preservation of the project, persistent state,
   base layer, harness layer, and verified project layer.

The package, inputs, project, host, runtime, harness, command order, and
arguments are frozen. The recipe may make only its declared unauthenticated
public downloads. No command may inspect or record credential or fingerprint
contents.

No model prompt, native harness, second harness, registry login, private input,
host execution of the recipe, source mutation, publication, remote mutation,
settings change, release, package publication, second live unit, or unowned
capsule is in scope.

## Alternatives

1. Increase the capsule root disk. Rejected because transient bytes already
   have a dedicated host-backed staging mount and should not require a second
   capacity budget.
2. Remove Rust components from the pinned distribution. Deferred because it
   changes the intended toolchain; output-backed staging tests the smaller
   capacity correction first.
3. Resume experiment 0018. Rejected because terminal experiment records are
   immutable and successor evidence requires a fresh identity.
4. Ask Codex to run the test. Rejected because it spends a model prompt and
   tests model behavior rather than the mounted environment.

## Risks

1. The output filesystem may still lack capacity or the installer may not
   tolerate source files below its destination. Either failure rejects the unit.
2. Cleanup may leave `.fortlet-work.*` in output. Validation evidence must show
   the published layer contains no transient workspace.
3. Tool versions may pass while Linux linking fails. The focused Rust test is
   required before acceptance.
4. Any ownership mismatch, credential anomaly, or unexpected capsule prevents
   dispatch or cleanup and triggers terminal reporting.
5. Cleanup failure rejects the complete unit and reports the exact owned
   artifact for operator recovery.

## Acceptance Criteria

1. The complete standard verification set and immutable package build pass
   before dispatch, including the fixture assertion that workspace allocation
   and cleanup are output-backed.
2. Initial public status is `codex<TAB>absent`, both existing layer markers are
   present, no environment layer exists, and the capsule list is empty.
3. The sole launch exits zero, emits one first-use preparation message and
   pinned Codex 0.147.0, and emits no prompt or credential value.
4. Exactly one owned running capsule has all five labels and a read-only project
   layer mount sourced from the sole new environment directory.
5. The published layer has a valid marker and no `.fortlet-work.*` entry.
6. In the capsule, `cargo` and `rustc` report 1.97.x, `jj` reports 0.43.0, all
   three resolve under `/opt/fortlet/project/bin`, and both declared Cargo
   environment values match.
7. `cargo test --bin fortlet project_environment` exits zero from the canonical
   mounted project.
8. Public stop and reset succeed; final public status is absent; no provisioning
   capsule remains; declared durable inputs and layers remain.
9. Every criterion must pass. One failure rejects this unit. The experiment
   terminates on failure, a hard-invariant anomaly, completed cleanup, or 25
   elapsed minutes after dispatch begins.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, zero remote mutation,
one Codex `--version` launch, one owned local capsule, one project-layer build,
unauthenticated public downloads only, no second live unit, and at most 25
elapsed minutes after dispatch. Record exact inputs, command output, labels,
mount, marker, absence of staging residue, tool versions, test result, cleanup,
preservation, and elapsed time.

## Rehearsal

The focused fixture test and POSIX shell syntax check passed. The complete run
then passed 52 unit tests, 22 integration tests, formatting, strict
all-target/all-feature Clippy, conformance, `nix flake check`, and
`nix build .#fortlet`. Nix retained the known app-metadata warning and omitted
the incompatible `x86_64-linux` system while validating `aarch64-darwin`.

## Results

Rejected by the sole live unit. The immutable command emitted the expected
first-use preparation message and progressed beyond experiment 0018's root-disk
failure, but exited 1 after about 70 seconds with:

```text
fortlet: environment stage failed; check network access or remove the reported incomplete layer and retry: project environment output contains an absolute link
```

This shows that output-backed download and extraction removed the prior capacity
blocker, while Fortlet's post-recipe validator correctly rejected an absolute
symbolic link in the installed output. Codex never launched, no
version output appeared, and no provider value was emitted. The unit used one
project-layer build, zero model prompts, zero paid quota, zero remote mutations,
and no second live unit.

## Terminal Closure

Rejected after the sole dispatch. Root cause: the installed output contained an
absolute link beneath `/out`, conflicting with FIP-0006's accepted requirement
to reject every absolute or escaping output link. The capacity correction
itself passed its falsification point, but the repository recipe did not yet
normalize the installed toolchain into a layer-relative tree.

Bounded cleanup completed automatically. `msb list --format json` returned an
empty list, public status remained `codex<TAB>absent`, and the environment store
contained no published layer or staging residue. The project, persistent Codex
state, base layer, and harness layer remain unchanged. Live elapsed time was
about 70 seconds. Post-closure inspection of the exact pinned arm64 and amd64
Debian archives identified the source more precisely: `libcap-ng-dev`, not
Rust, provides `libcap-ng.so` and `libdrop_ambient.so` links to absolute
`/lib/<triplet>/...` targets, while `libcap-ng0` provides those targets inside
the layer. Next action: rewrite only those verified output-internal targets into
relative links, while continuing to reject other absolute targets; cover the
normalization deterministically before declaring a fresh experiment. Never
resume this terminal record.
