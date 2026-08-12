# Session Handoff — Pre-runtime launch failure evidence is complete

Audience: a fresh agent session. `GOAL.md` is normative and complete. No
experiment is active, and Experiments 0001 through 0009 remain terminally
closed.

## Verified result

1. Checkpoint `f5f1d341` adds ten Unix integration cases that invoke the real
   compiled `fortlet run` interface with a cleared subprocess environment.
2. Harness, project, credential loading, credential mount boundary, local
   capsule-state preparation, guest auth projection, and incomplete base and
   harness layer failures all exit nonzero with their stage, one primary
   correction, and a useful cause.
3. Every fixture uses temporary home, project, state, data, and fake-auth paths.
   The fake token is structurally valid where needed but unusable and never
   appears in assertions or diagnostics.
4. The project-mounted credential case exposed a transaction-ordering defect:
   local capsule state was created before a known mount exposure was rejected.
   Known project and data mounts are now validated first; the capsule-state
   mount is validated once its path has been safely prepared.
5. Incomplete layers fail synchronously before provisioning. No test reaches a
   runtime credential fingerprint, MicroSandbox reconciliation, terminal
   attachment, harness execution, or network operation.

## Verification

The focused pre-runtime suite passed with ten cases. The full suite passed with
25 unit tests, one conformance test, two native integration tests, and all ten
pre-runtime integration tests. Formatting, strict all-target/all-feature
Clippy, the standalone conformance gate, and `nix flake check` passed. Nix
emitted the existing missing app metadata warning and omitted incompatible
`x86_64-linux`; native Linux verification remains outstanding.

No real credential, harness, MicroSandbox capsule, provisioning run, network
operation, model prompt, paid quota, external money, or live experiment was
used. Engineering remained within the two-hour ceiling; numeric elapsed time
was not captured and is not backfilled.

## Successor boundary

Do not reopen deterministic pre-runtime failures, project resolution, or
Codex-specific exit work without new evidence. Live capsule reconciliation and
terminal attachment failure behavior remain unproved, but proving them would
require a separately designed injection seam or live fault experiment and
should not be inferred as the automatic next goal.

Other FIP-0001 gaps remain: harness-owned persistent paths and credential
policy, capsule topology and concurrency coverage, lifecycle leases and
management commands, standalone installation, declarative environments, and
native Linux package verification. Preserve fail-closed ordering, optional shim
activation, broad-root protection, credential isolation, package runtime
integrity, and Jujutsu discipline.
