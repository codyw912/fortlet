# GOAL: Prove portable local project capability

Status: active — the accepted portable schema-1 permission amendment to
FIP-0006 is deterministically implemented and passes the complete local gate;
the remaining live lifecycle campaign, provider work, and model dispatch
remain blocked

## Outcome

Establish one inspectable project-capability path that lets Fortlet-wrapped
Codex and Tact edit, verify, and create unsigned local Jujutsu checkpoints in
two public projects. First replace the provisional Node/Debian environment
assembly with FIP-0013's Nix-built runtime image, isolated project stores, and
pinned harness closures without regressing prepared startup. Preserve
Fortlet's existing isolated recipe for its own toolchain and add one explicit
Nix dev-shell provider for a pinned public Go project. The evidence must show
useful local work, not claim universal project compatibility.

This goal extends the accepted project-environment architecture. FIP-0012 and
FIP-0013 MUST be accepted before their respective implementation. Before
implementation or live dispatch, read FIP-0001, FIP-0004, FIP-0005, FIP-0006,
FIP-0007, FIP-0008, FIP-0012, and FIP-0013 in full.

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

## Deliverable 1a — Establish the common runtime substrate

1. Produce one Nix-built OCI runtime definition per declared Linux guest
   architecture, a canonical runtime-seed manifest, and locally loadable image
   archives under FIP-0013. Ordinary preparation MUST require no Docker daemon,
   registry login, host Nix, Debian package lifecycle, npm, or harness-specific
   base image.
2. Seed one isolated store per project before mounting it at `/nix`, add only
   the selected harness closure, reuse the seeded Nix runtime for schema 2, and
   mount the completed store read-only in managed capsules. Preserve schema 1
   through its explicit common-tool contract.
3. Deterministic evidence MUST cover artifact identity, no-pull local image
   use, atomic and complete seeding, project isolation, Codex and Tact closure
   selection, schema compatibility, offline cache hits, and failure
   preservation before a live rehearsal.
4. One predeclared credential-free feasibility campaign MUST compare the
   existing mechanism and candidate under the same declared conditions. Record
   cold phase durations and bytes, disk sizes, repeated preparation, and
   prepared absent, stopped, and running launch samples.
5. Repeated preparation MUST remain an offline, non-mutating, capsule-free
   cache hit with a sub-second median. Prepared absent, stopped, and running
   launches MUST retain sub-second medians, and every running sample MUST remain
   below one second. Predeclare a finite cold-preparation ceiling; reject the
   candidate terminally if it or any prepared-launch requirement fails.
6. Stop after the substrate campaign's terminal closure and report its result.
   Do not spend a provider or model unit until the operator authorizes the
   remaining goal after reviewing that evidence.

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
6. FIP-0013's runtime artifact, isolated-store, harness-closure, and prepared
   latency requirements are conformant on every declared guest architecture;
   any deferred cold-path optimization remains explicitly measured and bounded.

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

On 2026-08-21, the operator accepted FIP-0013's Nix-built runtime direction and
authorized its deterministic implementation, complete local verification, and
one predeclared credential-free feasibility campaign. That campaign MAY build
and locally load Nix-produced OCI archives, fetch exact public unauthenticated
build inputs, seed isolated project stores, prepare pinned Codex and Tact
closures, and measure the declared cold and prepared paths. It MUST NOT publish
an image or package, require registry authentication, read a provider or model
credential, prepare the schema-2 public testbed, dispatch a model, edit a
project, or mutate a remote. This amendment supersedes the prior base-image
decision stop only for the named substrate work; provider and model dispatch
remain blocked until the campaign closes and the operator reviews its evidence.

After reviewing Experiment 0054's terminal evidence and MicroSandbox's
documented mount policies, on 2026-08-21 the operator authorized one narrowly
predeclared, credential-free successor experiment. It MAY use the already-built
local runtime image and public-neutral synthetic files to reproduce the default
private-permission failure and test explicit mirrored publication,
canonicalized portable modes, and a read-only relaxed-stat consumer mount. It
MUST use fresh isolated state, create no project checkout or provider state,
dispatch no harness or model, read no credential, and mutate no remote. Stop
after terminally recording and cleaning up this experiment; any production
change, lifecycle-campaign retry, schema-2 work, provider preparation, or model
dispatch requires further operator review.

After reviewing Experiment 0055's terminal evidence, on 2026-08-21 the operator
authorized one fresh, narrowly predeclared, credential-free successor. It MAY
use the already-built local runtime image and a public-neutral synthetic tree
to combine mirrored guest creation and validation with a trusted host step that
removes only write bits, then compare strict and relaxed read-only consumers.
It MUST use fresh isolated state, dispatch no harness or model, read no
credential, prepare no provider or project, contact no network, and mutate no
remote. Stop after terminally recording and cleaning up the experiment. This
does not authorize a production change, lifecycle-campaign retry, schema-2
work, provider preparation, or model dispatch.

On 2026-08-21 the operator accepted FIP-0006's portable sealed layer amendment.
Acceptance authorizes its deterministic production implementation, conformance
updates, documentation, and complete local verification. It does not authorize
a live capsule experiment, lifecycle-campaign retry, schema-2 provider
preparation, harness or model dispatch, or remote mutation beyond the existing
goal publication grant.

On 2026-08-22, after the deterministic implementation passed the complete
local gate and draft PR #15's hosted verification, the operator authorized one
fresh credential-free FIP-0013 lifecycle campaign against signed implementation
tip `dd75b81cf8c7`. It MAY build that exact public package, use fresh isolated
state and a synthetic non-provider auth fixture, contact only the recipe's
frozen public unauthenticated inputs during cold schema-1 preparation, run five
unchanged prepares, and run the predeclared three-cycle Tact `--version`
absent/running/stopped sequence. It MUST retain the existing 600-second cold,
sub-second warm and lifecycle median, and every-running-sample-below-one-second
ceilings. It authorizes no schema-2 preparation, provider or model request,
project edit, additional sample, remote project mutation, or retry of a failed
unit. Declaration, terminal evidence, exact cleanup, verification, and updates
to the existing goal bookmark and draft PR are in scope.

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
