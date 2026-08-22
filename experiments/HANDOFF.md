# Session Handoff — establish the Nix-built runtime substrate

Audience: a fresh agent session. The operator accepted the outcome, public
testbeds, exclusions, evidence budget, and four-hour ceiling in the active
`GOAL.md`. FIP-0012 and FIP-0013 are Accepted. The portable schema-1 sealing
mechanism is implemented at signed tip `dd75b81cf8c7`, and its complete local
gate and draft PR #15 hosted checks pass. Experiment 0057 produced no product
lifecycle evidence and completed exact cleanup with unchanged global
inventory. The operator authorized Experiment 0058 as one fresh rerun using
the exact existing package and frozen campaign conditions. Its 200.95-second
cold preparation passed, but five unchanged prepares had a 5.14-second median
because full reuse verification recursively validates modes and rehashes the
1.2-GiB sealed output. The sub-second requirement failed before lifecycle
sampling. Exact cleanup passed and global inventory was unchanged. Authority
has stopped for operator review. The operator accepted a FIP-0006 bounded
verified-reuse amendment that moves complete tree validation and digest
enforcement to cold atomic publication, treats the invoking host user and
Fortlet data as one trusted principal, and limits ordinary reuse to root and
bounded-marker verification. Deterministic implementation is active; schema-2
preparation, provider or model dispatch, and public-project mutation remain
unauthorized.

The base-image review selected one small Nix-built OCI runtime for every
harness, plus one isolated Nix store per project. Fortlet must locally load and
digest-verify the OCI archive with pulling disabled, seed the runtime closure
before `/nix` hides the image store, add only the selected pinned harness
closure, and let schema 2 add its locked project closure. Managed capsules
mount the completed store read-only. npm, Debian package assembly, per-harness
images, registry login, and host Nix are not ordinary preparation contracts.

FIP-0013 makes performance an adoption gate. Repeated preparation must be an
offline, non-mutating, capsule-free cache hit with a sub-second median.
Prepared absent, stopped, and running lifecycle medians must remain below one
second, and every running sample must remain below one second. A predeclared
credential-free feasibility campaign must compare the existing mechanism and
candidate under the same conditions, record cold phase timings, bytes and disk
sizes, and use a finite cold ceiling. The current authority stops after that
campaign; it does not authorize schema-2 provider preparation or a model unit.

The deterministic implementation is checkpointed at `062a68c69026`. It adds
strict schema-2 discovery, side-effect-free `fortlet plan`, a pinned guest-only
Nix provider with project-scoped store and resolved record, common Git/bootstrap
tools, bounded structured activation, and narrow generated Git/Jujutsu identity.
The complete `aarch64-darwin` runbook gate passed after implementation: 101
unit tests and every enabled integration test, formatting, strict Clippy,
conformance, Nix packaging, and shim activation were green.

Do not start another provider rehearsal without operator direction. Experiments
0046 and 0047 stopped on a long MicroSandbox socket path and an escaped relative
clone respectively. Experiment 0048's absolute setup and plan passed, but its
isolated state hid the installed runtime. Experiment 0049 selected byte-pinned
runtime and firmware, reached the guest, and exposed Fortlet's incorrect Debian
tar validation path before Nix. That path was corrected at `95a87bec`, its
focused test and the complete runbook gate passed, and the operator authorized
the GOAL's one fresh successor. Experiment 0050's exact preflight passed, but
its sole preparation attempt stopped while fetching the common base image. It
created no capsule and never ran the corrected script or Nix. Every root was
exactly removed and global inventory was unchanged. The operator then
authorized one fresh successor with new isolated state and identical frozen
inputs. Experiment 0051 entered the base script, which exited 1 before Tact or
Nix. Static inspection proves that
package extraction cannot satisfy its later requirement for the generated CA
bundle because the script never runs `update-ca-certificates`. Its capsule and
exact root were removed, global inventory was unchanged, and the amended retry
unit was closed. The operator authorized a deterministic correction and one
fresh provider-validation successor. Checkpoint `dd72b652` constructs the
bundle from enabled trusted entries, rejects traversal, surfaces the failing
base step, bumps the immutable layer identity, and updates FIP-0012 conformance
to honest partial status. The complete 102-test runbook gate, strict Clippy,
conformance, Nix package, and shim checks passed. The separately declared
Experiment 0052 then proved the diagnostic but rejected the CA fix:
`certificate-bundle` failed because raw package extraction does not create
`/etc/ca-certificates.conf`. Its synthetic fixture had supplied that file and
therefore repeated the production assumption rather than falsifying it. The
capsule and exact root were removed, global inventory was unchanged, and no
Tact, Nix, or model command ran. The GOAL is blocked and all authorized
provider successors are terminal.

The operator subsequently authorized one mechanism-level correction and one
base-only rehearsal, while explicitly leaving the suitability of
`node:24-bookworm` open for later discussion. The correction may use Debian's
real certificate installation/generation lifecycle inside the disposable,
credential-free capsule. It does not authorize another provider or model run.

Checkpoint `d7edc93d` implements that correction: it reinstalls
`ca-certificates`, invokes Debian's generator, copies the resulting nonempty
bundle, retains command-level diagnostics, and bumps the immutable base identity.
The complete 101-test runbook gate, strict Clippy, conformance, Nix package,
and shim checks passed. Experiment 0053 then accepted the production mechanism:
one base first-use event published a nonempty generated CA bundle after every
declared tool executed in the guest, the preseeded Tact marker prevented
harness work, and the identical second prepare was an unchanged cache hit
without network approval. Both isolated inventories were empty; exact cleanup
left global inventory unchanged. No provider or model work ran.

Checkpoint `d2df7964` implements FIP-0013's Nix-built OCI archive, canonical
runtime seed, per-project named ext4 store, and pinned Codex and Tact closures.
Checkpoint `894d306b` adds bounded opt-in cold-preparation phase timings. The
complete aarch64-darwin gate passed at that candidate: 102 unit tests and every
enabled integration test, formatting, strict Clippy, conformance, the curated
package, and shim activation were green. Nix reported only the expected native
x86_64-linux omission.

Experiment 0054 is terminally rejected. The candidate loaded its 99.5 MiB local
image, seeded the runtime store, imported Tact, and built Fortlet's schema-1
layer in 267.29 seconds, below the predeclared 600-second ceiling. Five offline
warm prepares had a 0.01-second median without marker mutation or capsules.
The first prepared absent launch then failed before Tact: MicroSandbox rejected
the read-only schema-1 project-layer bind with `Permission denied`. Read-only
inspection attributed this to host-side extracted modes normalized to
owner-only, followed by recursive sealing that produced representative `0500`
directories and `0400` commands. Public reset and exact cleanup succeeded; the
public project was unchanged; and no real credential, provider, model, edit,
remote mutation, or publication occurred.

Experiment 0055 is also terminally rejected. Its explicit strict/private
control reproduced the mechanism: guest `0755`/`0644` became literal host
`0700`/`0600`, current sealing became `0500`/`0400`, and the consumer mount
failed with `EACCES`. The strict/mirrored candidate's trusted guest finalizer
reported canonical `0555`/`0444`, but literal host modes remained
`0755`/`0644`. Exact MicroSandbox 0.6.8 source inspection showed this is the
documented owner-access floor: Mirror always ORs `0700` into directories and
`0600` into regular files so the host process cannot lock itself out. The
candidate failed before its consumer. All isolated state was exactly removed,
and global inventory was unchanged.

After Experiment 0055's terminal cleanup, the complete `aarch64-darwin`
standard gate passed through `nix develop`: 102 unit tests and every enabled
integration test, formatting, strict all-target/all-feature Clippy,
conformance, the Nix package, and shim activation were green. The two
stock-Codex compatibility tests remain explicitly ignored because they require
an external binary. Nix reported only the expected incompatible
`x86_64-linux` omission.

Experiment 0056 accepted the smallest successor mechanism. One strict/mirrored
publisher's trusted guest finalizer reported `0555` directories and executable
plus `0444` data; MicroSandbox's intentional floor made the literal host modes
`0755`/`0644`. Exact host validation passed, and removing only write bits made
the host modes canonical without changing the relative link, types, hashes, or
stat-override xattr names. A strict read-only consumer still failed mounting
the sealed root. The otherwise identical relaxed private `ro,nosuid,nodev`
consumer ran as numeric user 1000, executed both file and link, read data,
proved append and chmod failed, and left every recorded property unchanged.
Publisher, strict, and relaxed runs took 0.32, 0.05, and 0.26 seconds. Exact
cleanup restored empty isolated inventory and unchanged global inventory; no
network, provider, harness, model, credential, project, or remote work ran.

After terminal cleanup, the complete `aarch64-darwin` standard gate passed:
102 unit tests and every enabled integration test, formatting, strict
all-target/all-feature Clippy, conformance, the Nix package, and shim activation
were green. The two stock-Codex compatibility tests remain explicitly ignored
because they require an external binary. Nix reported only the expected
incompatible `x86_64-linux` omission.

Stop here for operator review. Do not fix the sealing contract, run a successor
campaign, prepare schema 2, or dispatch Codex/Tact model work without fresh
direction. The next implementation should set mirrored permissions on the
schema-1 provisioning bind, retain trusted guest validation, validate the
literal host tree before removing only write bits, and use relaxed private
`ro,nosuid,nodev` consumption. Deterministic tests belong with that code. The
accepted mechanism must then repeat the remaining Fortlet prepared-lifecycle
evidence under a new experiment identity.

## Verified repository state

On 2026-08-20, `jj git fetch --remote origin` reported no changes. Local
`main`, `main@origin`, and the Git-backed `main` bookmark all resolve to
`29cd952ad2d4c28a1fec23456ca50d792024e6b1`, the squash merge of PR #14,
`Add opt-in startup latency diagnostics (#14)`. The working copy was a clean
empty child of `main` before this handoff-only edit.

On 2026-08-21 the successor session repeated the complete aarch64-darwin gate
through `nix develop`: 91 unit tests and every enabled integration test,
formatting, strict all-target/all-feature Clippy, conformance, the curated
package, and the shim-activation check all passed. The two stock-Codex
compatibility tests remain explicitly ignored because they require an external
binary. Nix reported only the expected incompatible x86_64-linux omission.

The completed latency goal's final aarch64-darwin gate passed through
`nix develop`: 91 unit tests and every enabled integration test, formatting,
strict all-target/all-feature Clippy, conformance, the curated package, and
the shim-activation check. Experiments 0043 and 0045 established a 55–60 ms
running-capsule median and sub-second absent and stopped medians. No runtime
optimization remains justified by the evidence.

The local-only bookmark `parked-claude-code-harness` still points to
`285456a2b82d`. It contains the parked Claude Code harness proposal and GOAL.
It has not been pushed and MUST remain untouched until the operator explicitly
returns to additional-harness work.

## Operator direction

Do not frame the next problem around AGD. AGD merely exposed a universal
daily-use gap. Fortlet-wrapped Codex and Tact must be able to perform real work
in arbitrary projects, including using the project's compilers, package
managers, version-control tools, checks, and selected external services.

The operator wants to prove Codex and Tact as useful daily drivers before
adding Claude Code, OMP, Pi, or another harness. They want current and emerging
sandbox prior art consulted heavily, an ergonomic result, and the smallest
vertical slice that yields real evidence. Avoid a policy framework, universal
package DSL, multi-user RBAC system, or other speculative machinery.

## Existing Fortlet contracts to preserve

Read FIP-0001, FIP-0005, FIP-0006, FIP-0007, and FIP-0008 in full before
designing the successor:

- FIP-0001 keeps host credentials, SSH private keys, the 1Password agent, and
  unrestricted publication authority outside capsules. Agents may create
  unsigned local checkpoints. Publication is a trusted host transaction that
  cannot execute repository hooks, aliases, pagers, credential helpers, or
  generated shell text.
- FIP-0006 currently offers one explicit, immutable project tool layer through
  `.fortlet/environment.json` and `.fortlet/environment.sh`. The recipe runs
  only in a credential-free provisioning capsule. Services, mutable package
  installation, authenticated inputs, and private overlays were deliberately
  deferred.
- FIP-0007 provides explicit, credential-free `fortlet prepare <harness>` and
  automatic first-launch preparation.
- FIP-0008 proves that MicroSandbox can substitute destination-bound host
  credentials without making the real credential guest-readable. That solves
  Codex authentication, not general project credentials or publication
  authority.
- FIP-0005 and FIP-0001's publication amendment already demonstrate the
  operator's preferred workflow: one goal-scoped branch and draft PR may be
  iterated without repetitive approval, while the operator retains merge.

FIP-0001 remains partial. FIP-0006, FIP-0007, and FIP-0008 are conformant as
of the handoff. Check the map rather than trusting these labels blindly.

## Prior-art research completed in the closing session

Three independent research tracks covered whole-environment construction,
credential/publication boundaries, and agent-facing authorization UX. The
central result is that project environments and external authority are related
but distinct product surfaces.

### Project environments

The strongest recurring pattern is:

```text
prepared base or template
    + thin repository-owned setup
    + mutable per-project session state
```

No mature system found in the survey makes arbitrary repositories usable
solely through automatic language detection.

- Docker Sandboxes uses broad Ubuntu base templates, thin experimental kits,
  persistent sandbox mutation across stop/start, and explicit template
  capture. Its template/kit separation and resolved inspection surface are
  highly relevant, but its formats are young and harness-coupled.
  <https://docs.docker.com/ai/sandboxes/customize/>
- Dev Containers is the strongest existing repository-owned environment
  standard: image or Dockerfile, Features, lifecycle commands, mounts,
  services, and ports. Full compatibility is large. Fortlet MUST NOT execute
  `initializeCommand` or other repository-controlled text on the real host,
  and an importer should reject unsupported fields rather than ignore them.
  <https://github.com/devcontainers/spec/blob/main/docs/specs/devcontainerjson-reference.md>
- Coder Envbuilder is useful implementation evidence that a safe Dev Container
  subset can be valuable while remaining explicitly incomplete.
  <https://github.com/coder/envbuilder>
- Nix and devenv are the best first provider for this operator and can express
  pinned project tools, tasks, and services, but they are one provider rather
  than Fortlet's universal public contract.
  <https://nix.dev/tutorials/first-steps/declarative-shell.html>
- E2B, Daytona, DevPod, Sprites, Cloudflare Sandbox, and Kubernetes Agent
  Sandbox reinforce the separation between a prepared base, session root,
  persistent project/cache data, and declared services. Warm pools, memory
  snapshots, remote backends, and Kubernetes are not justified for the next
  local slice.

### Credentials and publication

A proxy can guarantee that the guest cannot read a credential. It does not by
itself guarantee that the guest cannot abuse the credential's authority.

- Docker Sandboxes injects phantom credentials through a host proxy so normal
  `gh` and registry commands work, but an injected GitHub token retains its
  upstream authority. Its SSH-agent forwarding is explicitly unsuitable for
  Fortlet's invariant. Clone mode, where the host fetches the agent's branch
  and publishes it separately, is a safer useful reference.
  <https://docs.docker.com/ai/sandboxes/workflows/>
- Claude's self-hosted guidance recommends ephemeral per-session containers,
  short-lived least-scoped credentials, or a per-session Git proxy; it also
  treats Git identity separately from push credentials.
  <https://code.claude.com/docs/en/self-hosted-environments-deploy>
- GitHub Copilot cloud gives an agent one writable branch, prevents raw Git
  pushes, creates draft PRs, and structurally prevents the agent from marking
  ready, approving, or merging. This is the strongest publication boundary in
  the survey and closely matches the operator's desired authority split.
  <https://docs.github.com/en/copilot/concepts/agents/cloud-agent/risks-and-mitigations>
- Daytona's Pi integration assigns one branch, tells the agent to commit but
  not push, and has a trusted orchestrator synchronize commits after each
  turn. It is the simplest counterpoint to transparent `git push` brokering.
  <https://www.daytona.io/docs/en/guides/pi/pi-extension/>
- Iron Proxy and Infisical Agent Proxy are strong credential and network
  primitives, but neither supplies Git-ref-aware publication semantics.
  Infisical's newer Agent Vault proposal flow is valuable UX prior art: a
  denied operation produces an inspectable proposal, one human decision, and
  automatic retry.
  <https://github.com/Infisical/agent-vault/blob/main/docs/learn/proposals.mdx>
- `agent-sandbox` and `nono` are useful newer open-source references for
  repository-aware proxy policy and authority attached to controlled tool
  invocation. Neither removes the need for Fortlet to validate branch and PR
  semantics at a trusted boundary.
  <https://github.com/mattolson/agent-sandbox>
  <https://github.com/nolabs-ai/nono>

### Authorization UX

Command-prefix approval rules in Codex, Claude, and Copilot reduce prompt
volume, but `allow git push` cannot express repository, remote, ref,
force-update, deletion, tag, release, or merge restrictions. Fortlet's durable
boundary must be semantic and independently enforced. The useful UX pattern is
one visible, revocable lease with many ordinary in-scope operations beneath it.

Official Codex documentation explicitly separates sandbox enforcement from
approval policy and recommends fixing the common-case boundary rather than
teaching a reviewer to approve noisy escalations indefinitely:
<https://learn.chatgpt.com/docs/agent-approvals-security>.

## Selected successor and review boundary

The active goal proves **complete local project work** without combining it
with publication authority:

1. Define a harness-neutral resolved project-capability plan and a safe
   `fortlet plan` inspection surface.
2. Keep the current FIP-0006 recipe as schema 1 and add only an explicitly
   selected, locked Nix dev-shell schema 2. Direct devenv and Dev Container
   compatibility remain outside the goal.
3. Put Git and bootstrap necessities in the common base. Project compilers and
   package managers belong to the selected provider rather than harness
   adapters.
4. Project a narrowly sanitized Git identity such as `user.name` and
   `user.email`. Do not copy the host `.gitconfig`, credential helpers, signing
   configuration, URL rewrites, aliases, filters, hook paths, SSH commands, or
   includes.
5. Allow useful mutable scratch state across capsule stop/start while reset
   remains destructive. Keep reproducible environment inputs and dependency
   caches distinct from unpromoted root mutation.
6. Prove edit, project verification, and unsigned local checkpoint through one
   Codex unit and one Tact unit using only public testbeds: Fortlet and a pinned
   disposable checkout of `https://github.com/muesli/reflow`. Never mutate the
   external repository remotely or record personal identity values.

FIP-0012 is deliberately narrower than general Nix activation. It requires a
project-scoped store, credential-free archiving/evaluation/realization,
scalar-only captured activation, rejection of executable hooks and required
functions or services, discarding inert Nix/stdenv function and array metadata,
and narrow generated Git and Jujutsu identity. The operator accepted this
architecture contract and its activation clarification on 2026-08-21.

After that succeeds, use a separate architecture slice to test one external
publication lease:

```text
one capsule + one repository + one assigned branch
    -> fetch/read repository and PR state
    -> create or fast-forward that branch
    -> create/update one draft PR
    -> never merge, approve, mark ready, tag, release, delete, or touch another repo
```

The operator should approve that lease once. The research has not yet chosen
between a transparent `git`/`gh` command bridge and Daytona-style trusted
orchestrator synchronization. Compare those two concrete interactions before
designing a generic broker. Do not build a policy DSL, RBAC system, general
OAuth framework, Git protocol parser, or required external secrets control
plane for the first experiment.

## Next session

1. Follow the repository startup order and confirm the verified baseline,
   implementation checkpoint, goal bookmark, and parked Claude bookmark.
2. Implement FIP-0013 without changing FIP-0012's provider selection,
   activation, inspection, identity, credential, workspace, or reset boundary.
3. Run deterministic evidence and the complete runbook gate before declaring
   one credential-free control-and-candidate substrate campaign.
4. Close that campaign terminally and stop. Provider preparation and Codex or
   Tact model work remain unauthorized pending operator review.

## What not to do

Do not solve only AGD. Do not retain Node or npm merely for Codex, install Nix
again inside every capsule, publish or require a registry image, share a
writable store across projects, or mount the image's store over an empty
project store. Do not copy the host home or `.gitconfig`, forward SSH or
1Password sockets, expose a general GitHub token through the guest proxy, or
run repository setup or Git hooks on the host. Do not silently guess among
multiple environment definitions. Do not claim full Dev Container support
from a subset. Do not unpark Claude Code merely because its proposal already
exists. Do not broaden FIP-0012 without a successor architecture decision, and
do not put any private project name, path, or personal identity value into a
public artifact.
