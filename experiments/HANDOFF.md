# Session Handoff — validate the portable project-capability implementation

Audience: a fresh agent session. The operator accepted the outcome, public
testbeds, exclusions, evidence budget, and four-hour ceiling in the active
`GOAL.md`. FIP-0012 is Accepted. Verify `main`, the Jujutsu stack,
conformance, and the complete `docs/RUNBOOK.md` gate before relying on this
summary. Implement only the exact FIP-0012 contract within the active goal.

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
its sole preparation attempt lost Docker Hub DNS while fetching the common base
image. It created no capsule and never ran the corrected script or Nix. Every
root was exactly removed and global inventory was unchanged. The operator then
authorized one fresh firewall-clearance successor with new isolated state and
identical frozen inputs. Experiment 0050 remains terminal; only the separately
declared Experiment 0051 may run, with network approval from the outset.

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
2. Run only declared Experiment 0051 with network approval from the outset;
   never reuse Experiment 0050's terminal checkout or state.
3. If 0051 fails, close it and stop. No further provider or model unit is
   authorized without another explicit GOAL amendment. Keep schema 1
   conformant and the accepted FIP boundary fixed.
4. Only after provider rehearsal succeeds may the exact Codex and Tact public
   work units be predeclared before either model process.

## What not to do

Do not solve only AGD. Do not install Nix into every capsule and call the
problem complete. Do not copy the host home or `.gitconfig`, forward SSH or
1Password sockets, expose a general GitHub token through the guest proxy, or
run repository setup or Git hooks on the host. Do not silently guess among
multiple environment definitions. Do not claim full Dev Container support
from a subset. Do not unpark Claude Code merely because its proposal already
exists. Do not broaden FIP-0012 without a successor architecture decision, and
do not put any private project name, path, or personal identity value into a
public artifact.
