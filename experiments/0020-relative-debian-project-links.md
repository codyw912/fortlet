# Experiment 0020: Relative Debian project links

Status: accepted
Design: FIP-0001 and FIP-0006
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0019 proved that output-backed staging removed the provisioning
capacity failure, then rejected publication because the installed tree contained
an absolute link. Post-closure inspection of the exact pinned Debian arm64 and
amd64 archives identified two `libcap-ng-dev` links to absolute
`/lib/<triplet>/...` targets that `libcap-ng0` supplies inside the same output.

The treatment at `a6670a6ede64` (`Normalize pinned Debian links within project
layers`) adds only the declared GNU triplet, exact-target/existence checks, and
rewrites those two links to equivalent `../../../lib/<triplet>/...` targets.
Fortlet's validator remains unchanged and still rejects every other absolute or
escaping link.

The immutable package is
`/nix/store/ki9llb8dy1zrwxbypmpk4lbzy3p7ax6n-fortlet-0.1.0` on
`aarch64-darwin` with MicroSandbox SDK and CLI 0.6.8 and Codex 0.147.0.
Fetched `main` is `633ea638013ba675e82ea7b06b1d3a287d7aa003`. Exact project inputs:

1. `.fortlet/environment.json` SHA-256
   `3908a64857fb1a0b5cca6c580774a6e15125a86172d5123ecbd610af62f31f1d`.
2. `.fortlet/environment.sh` SHA-256
   `5857604b2ceb0dda2117ee4a54256d342300b3832305ebd9a777cf6ecfe703e0`.
3. Guest image `node:24-bookworm`, Rust 1.97.1, Jujutsu 0.43.0, and Debian
   `libcap-ng` 0.8.3-1+b3 for `linux/aarch64`.

Read-only baseline checks found valid authentication, safe credential boundary,
both existing immutable layer markers, no project environment layer, absent
Codex and Tact status, and no MicroSandbox capsules. No experiment is active
before this declaration.

## Hypothesis and Production Mechanism

Because both package links now retain their in-layer targets without absolute
guest-root resolution, the unchanged validator should accept and digest the
complete output. The public launch should atomically publish that layer, mount
it read-only, and expose the pinned Rust/Jujutsu toolchain to Codex without host
recipe execution.

## Declared Scope

Use the immutable package and `/Users/cody/dev/fortlet` in this fixed order:

1. Reconfirm markers, authentication, safe boundary, public Codex absence,
   environment-layer absence, and an empty capsule list.
2. Run exactly one public non-prompt launch:
   `fortlet run codex --project /Users/cody/dev/fortlet -- --version`.
3. Require exactly one running capsule matching managed, schema, project, tool,
   and project-environment labels. Inspect its read-only
   `/opt/fortlet/project` mount and sole newly published source directory.
4. Verify the published marker, no transient workspace, no absolute or escaping
   link, and the two relative Debian development links.
5. Use `msb exec` only against that exact owned capsule. Record paths and
   versions for `cargo`, `rustc`, and `jj`; verify `CARGO_HOME` and
   `CARGO_TARGET_DIR`; run `cargo test --bin fortlet project_environment` from
   the canonical mounted project.
6. Publicly stop and reset the capsule. Verify final absence, no provisioning
   capsule, and preservation of the project, persistent state, base, harness,
   and verified project layers.

The implementation, package, inputs, project, host, runtime, harness, command
order, and arguments remain frozen. Only declared unauthenticated public
downloads are allowed. No command may inspect or record provider credentials or
fingerprints.

No model prompt, native harness, second harness, registry login, private input,
host recipe execution, source mutation, publication, remote mutation, settings
change, release, package publication, second live unit, or unowned capsule is
in scope.

## Alternatives

1. Weaken the validator to allow `/lib` links. Rejected because their guest-root
   meaning escapes the immutable project layer and violates FIP-0006.
2. Delete the development links. Rejected because retaining a correct
   layer-relative linker view is more complete and directly testable.
3. Resume experiment 0019. Rejected because terminal evidence is immutable and
   this treatment has a new identity.
4. Skip the focused test after version checks. Rejected because only an actual
   Fortlet build detects the required Linux linker closure.

## Risks

1. The assumed relative depth may be wrong. Published link inspection and the
   focused build both reject that result.
2. Another absolute or escaping link may remain. The unchanged validator rejects
   publication.
3. Tool versions may pass while compilation or linking fails. The focused test
   remains mandatory.
4. Any ownership, credential, or unexpected-capsule anomaly prevents dispatch
   or cleanup and closes the unit.
5. Cleanup failure rejects the unit and reports the exact owned artifact.

## Acceptance Criteria

1. Complete verification and immutable package build pass before dispatch,
   including both fixture tests and POSIX recipe syntax.
2. Initial public status is `codex<TAB>absent`, immutable markers are present,
   no environment layer exists, and the capsule list is empty.
3. The sole launch exits zero with one preparation message and Codex 0.147.0,
   without a prompt or credential value.
4. One owned capsule has all five labels and the sole new verified project layer
   mounted read-only.
5. The layer marker is valid; no transient workspace, absolute link, or escaping
   link exists; both Debian development links resolve within the layer.
6. In-capsule tools resolve below `/opt/fortlet/project/bin`; Rust/Cargo report
   1.97.x, Jujutsu reports 0.43.0, and both Cargo environment values match.
7. `cargo test --bin fortlet project_environment` exits zero from the canonical
   mounted project.
8. Stop/reset and final absence succeed; no provisioning capsule remains; all
   declared persistent inputs and immutable layers remain.
9. Every criterion must pass. One failure rejects the unit. It terminates on
   failure, a hard-invariant anomaly, completed cleanup, or 25 elapsed minutes.

## Budget and Plan

Budget: zero money, zero paid quota, zero model prompts, zero remote mutation,
one Codex `--version` launch, one owned capsule, one project-layer build,
unauthenticated public downloads only, no second live unit, and at most 25
minutes after dispatch. Record inputs, output, labels, mount, marker, links,
versions, environment, focused test, cleanup, preservation, and elapsed time.

## Rehearsal

Exact arm64 and amd64 Debian archive inspection confirmed both link shapes and
in-layer runtime targets. POSIX syntax and two fixture tests passed. The complete
run passed 53 unit tests, 22 integration tests, formatting, strict
all-target/all-feature Clippy, conformance, `nix flake check`, and
`nix build .#fortlet`. Nix retained the known app-metadata warning and omitted
incompatible `x86_64-linux` while validating `aarch64-darwin`.

## Results

Accepted. The sole immutable public launch emitted one first-use preparation
message, provisioned and published environment identity
`a0e90418dca950e944e6243c446e93eb977061cd73333da4996a8d560d021252`,
then exited zero with `codex-cli 0.147.0`. Exactly one running capsule existed:
`fortlet-501-codex-6652b8ca5f1b9273`. Its labels were:

- `fortlet.managed=true`
- `fortlet.schema=1`
- `fortlet.project=6652b8ca5f1b9273`
- `fortlet.tool=codex`
- `fortlet.version=0.147.0`
- `fortlet.environment=a0e90418...021252`

The capsule mounted the sole published directory read-only at
`/opt/fortlet/project`. Its marker bound schema 1, the complete environment
identity, output digest
`3a9aab8358bee98478de1a57e2c7c5ed16c64979f839cd3c9e5cbb75e1d4f4e2`,
`node:24-bookworm`, and `linux/aarch64`. No `.fortlet-work.*` residue existed.
The complete layer had four links, all relative. The development links were:

```text
usr/lib/aarch64-linux-gnu/libcap-ng.so -> ../../../lib/aarch64-linux-gnu/libcap-ng.so.0.0.0
usr/lib/aarch64-linux-gnu/libdrop_ambient.so -> ../../../lib/aarch64-linux-gnu/libdrop_ambient.so.0.0.0
```

The exact owned-capsule probe reported:

```text
cargo_path=/opt/fortlet/project/bin/cargo
cargo 1.97.1 (c980f4866 2026-06-30)
rustc_path=/opt/fortlet/project/bin/rustc
rustc 1.97.1 (8bab26f4f 2026-07-14)
jj_path=/opt/fortlet/project/bin/jj
jj 0.43.0-89f62ede8c1c611eaf134c0c49252efd65c7945d
CARGO_HOME=/home/agent/.cargo
CARGO_TARGET_DIR=/home/agent/.cargo/fortlet-target
```

It ran from `/Users/cody/dev/fortlet`; the focused filter passed all 10 selected
tests. The first Linux build populated the declared persistent Cargo cache and
reported a 3m24s Cargo build. The public launch took about 1m54s by command
timings; an independent end-to-end experiment timer was not captured. Actual
cost was one project-layer build, one owned capsule, public toolchain and Cargo
downloads, zero money, zero paid quota, zero model prompts, zero remote
mutations, and no second live unit.

## Terminal Closure

Accepted. Root cause of success: output-backed staging avoided the capsule-root
capacity limit, and exact normalization preserved both Debian development links
inside the immutable layer without weakening Fortlet's absolute-link rejection.
The real public path then proved publication, activation, pinned tools, Cargo
environment, and focused Linux compilation.

Public `stop` produced `codex<TAB>stopped`; public `reset` removed the exact
owned capsule; final status was `codex<TAB>absent` and the complete MicroSandbox
list was empty. The canonical project, persistent Codex state, base marker,
Codex marker, and verified project layer were all preserved. Next action: mark
FIP-0006 conformant, finish the mission handoff and complete local verification,
then prepare the single FIP-0005 publication packet.
