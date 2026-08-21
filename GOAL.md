# GOAL: Prove portable local project capability

Status: active — accepted by the operator on 2026-08-21; FIP-0012 is Accepted,
and one bounded common-base correction plus base-only rehearsal is authorized;
provider and model dispatch remain blocked

## Outcome

Establish one inspectable project-capability path that lets Fortlet-wrapped
Codex and Tact edit, verify, and create unsigned local Jujutsu checkpoints in
two public projects. Preserve Fortlet's existing isolated recipe for its own
toolchain and add one explicit Nix dev-shell provider for a pinned public Go
project. The evidence must show useful local work, not claim universal project
compatibility.

This goal extends the accepted project-environment architecture. FIP-0012 MUST
be accepted before implementation. Before implementation or live dispatch,
read FIP-0001, FIP-0004, FIP-0005, FIP-0006, FIP-0007, FIP-0008, and FIP-0012
in full.

## Deliverable 0 — Freeze the verified baseline

1. Verify fetched `main`, the Jujutsu stack, the conformance map, and the
   complete `docs/RUNBOOK.md` gate through `nix develop` before implementation.
2. Preserve the local-only `parked-claude-code-harness` bookmark without
   rebasing, publishing, or otherwise changing it.
3. Use one `portable-project-capability` bookmark and one draft pull request
   targeting `main` under FIP-0005. The operator remains the sole merge
   authority.

## Deliverable 1 — Accept the provider contract

1. Record one harness-neutral resolved project-capability plan covering the
   selected provider, target, immutable inputs, activation, cache classes,
   projected identity, and unsupported requirements.
2. Preserve FIP-0006 schema 1 as the explicit isolated-recipe provider. Add
   only one new provider: an explicitly selected, locked Nix dev shell evaluated
   and prepared inside a credential-free guest boundary.
3. Make the declared and prepared plan inspectable without reading provider
   credentials, contacting the network, creating a capsule, or exposing host
   state paths, identity values, environment values, or internal hashes.
4. Reject ambiguity, unsupported activation behavior, missing locks,
   authenticated inputs, services, and host-only requirements with one
   actionable correction. Do not silently choose or approximate an
   environment.

## Deliverable 2 — Supply ordinary local project capability

1. Ensure the common base contains the minimum version-control and bootstrap
   tools needed before provider activation. Project compilers and package
   managers remain provider-owned.
2. Apply the same resolved project plan to Codex and Tact without moving
   project behavior into harness adapters.
3. Project only the host's effective `user.name` and `user.email` into narrow,
   generated Git and Jujutsu configuration. Do not copy host configuration,
   signing settings, hooks, aliases, pagers, helpers, includes, URL rewrites,
   SSH commands, or credential material.
4. Keep the project workspace durable, provider and dependency caches distinct
   from the capsule root, and unpromoted root mutation disposable. Stop/start
   MAY reuse the root; public reset MUST retain its existing destructive
   runtime-state meaning without deleting the project or immutable inputs.
5. Preserve FIP-0001's fail-closed credential, publication, mount, capsule, and
   native-escape boundaries.

## Deliverable 3 — Prove two public daily-work units

1. Use Fortlet itself as the schema-1 recipe testbed. Use a clean checkout of
   `https://github.com/muesli/reflow` pinned to an exact reviewed revision as
   the Nix-provider testbed. The external checkout MUST receive no push, pull
   request, issue, tag, release, or other remote mutation.
2. Before each live unit, predeclare its exact public revision, setup, prompt,
   permitted paths, verification command, checkpoint command, capsule state,
   cache state, and cleanup. Never record private project names or personal
   identity values.
3. Run exactly one packaged Codex work unit in one testbed and one packaged
   Tact work unit in the other. Across the two units, require a useful bounded
   edit, the exact in-capsule project check, an unsigned local Jujutsu
   checkpoint, visible harness success, and no undeclared path or remote
   mutation.
4. Independently inspect each diff and checkpoint, rerun its focused check,
   demonstrate one useful stop/start cache or state reuse, and clean up only
   owned capsules and the disposable external checkout.
5. Deterministic evidence MUST cover both harnesses with both provider classes;
   the two live units need not repeat that full matrix.

## Deliverable 4 — Close and publish once

1. Keep FIP-0012 conformance honest and update it in the same checkpoint as
   the implementation and tests that establish each requirement.
2. Close every experiment terminally, update the experiment index, user
   documentation, runbook, GOAL, and handoff only for established behavior.
3. Run the complete local verification set, sign the final publishable tip,
   require hosted Rust verification, and leave one pull request ready for the
   operator's squash merge.
4. After operator merge, fetch `main`, prove exact reviewed-tree equality, and
   remove only the goal's local and remote bookmarks.

## Definition of Done

1. One explicit Nix dev-shell declaration resolves to a bounded, inspectable,
   reusable Linux project plan without requiring host Nix or executing
   repository-controlled text on the host.
2. `prepare`, plan inspection, and launch agree on provider selection and
   prepared identity; unsupported or stale state fails closed and actionably.
3. Codex and Tact receive equivalent project tools, activation, cache classes,
   and narrow Git/Jujutsu identity without receiving host credentials or
   unrestricted configuration.
4. The two public work units each retain only the declared edit, pass their
   exact project check, and produce an unsigned local Jujutsu checkpoint. At
   least one unit proves useful cache or state reuse after stop/start.
5. FIP-0006 remains conformant, FIP-0012's status is accurate, every experiment
   is terminal, and the complete local and hosted gates pass.

## Excluded scope

No private project name or path, external-project publication, general
provider framework, automatic language or environment detection, devenv- or
Dev-Container-specific compatibility, services, ports, tasks, daemons,
authenticated package inputs, private overlays, universal cache DSL, mutable
system-package contract, publication lease, general credential broker,
additional harness, standalone installer, remote runtime, release, tag,
package, repository setting, or merge is in scope.

## Budget and authority

The engineering ceiling is four hours total, including architecture drafting,
implementation, deterministic evidence, and local experiments but excluding
operator review, hosted CI, and manual merge waits. External money is zero.

Live provider activity is limited to one Codex process and one Tact process.
After a demonstrated Fortlet defect is closed and corrected deterministically,
at most one fresh successor process total MAY validate the correction within
the same time ceiling. No failed unit may be resumed, edited, or silently
retried. Use at most one owned capsule at a time.

On 2026-08-21, after that successor stopped before capsule creation because the
execution boundary could not resolve Docker Hub, the operator authorized one
additional fresh firewall-clearance successor. It MUST use new isolated state,
the identical frozen public inputs, and network approval from the outset. It
does not authorize resuming a terminal unit, another correction, or a further
retry.

That successor cleared the firewall and exposed a deterministic common-base CA
bundle defect before harness or provider preparation. After the defect was
closed, the operator authorized its bounded correction, the complete gate, and
one final fresh provider-validation successor. No terminal unit may be resumed,
and this amendment authorizes no additional correction or retry.

That validation proved the attempted correction retained the false assumption
that raw package extraction provides Debian maintainer-generated state. On
2026-08-21 the operator authorized one mechanism-level correction using the
package's real installation/generation lifecycle inside the disposable,
credential-free capsule, followed by one base-only rehearsal. Retaining
`node:24-bookworm` for this bounded correction does not settle the base-image
choice. No provider or model dispatch is authorized by this amendment.

Acceptance authorizes one `portable-project-capability` bookmark and one draft
pull request targeting `main` under FIP-0005. Stop on a need to broaden the
provider contract, execute repository text on the host, expose a credential or
personal identity value, mutate the public test repository remotely, weaken an
accepted FIP, exceed the time or provider budget, merge, or repeat one
unresolved failure assumption.

## Verification

Run focused deterministic checks while working and the complete standard set
from `docs/RUNBOOK.md` through `nix develop` before live dispatch and before
publication readiness.
