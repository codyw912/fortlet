# Fortlet — Overview

Fortlet is a portable, local-first tool that runs coding-agent CLIs inside
project-scoped capsules without making normal agent use feel different from
running directly on the host.

## Project status

Fortlet is an alpha Rust implementation. The verified baseline resolves safe
project roots, provisions immutable Codex and Tact environments, brokers
ChatGPT credentials through MicroSandbox, creates reusable project-and-harness
capsules, preserves interactive terminal behavior, provides optional
transparent Codex and Tact shims, and exposes project-scoped capsule status,
stop, terminal reset, explicit environment preparation, and opt-in immutable
project-tool layers. A reproducible
Nix package exists for
`aarch64-darwin` and is declared for `x86_64-linux`; native Linux verification
remains outstanding.

Global inventory, cross-project removal, restart, logs, explicit workload
leases, automated publication, service orchestration, automatic harness
discovery, and remote execution remain designed directions rather than
implemented features.

## Core idea

1. The native harness command remains the interface; isolation is an
   implementation detail on successful launches.
2. Isolation fails closed. Fortlet never silently falls back to an unisolated
   host launch.
3. The workspace is shared live, but project-tool provisioning receives only
   its snapshotted recipe and empty output; host credentials and publication
   authority stay outside the guest.
4. Capsules are disposable machines with explicit persistent paths, not
   long-lived opaque development pets.
5. Harnesses share a product model while retaining adapter-owned versions,
   state, credentials, and launch behavior.
6. Local execution comes first. Remote execution must introduce a real
   workspace, terminal, and credential transport rather than a speculative
   generic backend abstraction.
7. Nix is Fortlet's canonical build and development system, not part of the
   end-user contract. Ordinary installation and use must not require Nix
   knowledge or configuration.

## Scope and non-goals

Fortlet owns project discovery, harness dispatch, capsule lifecycle,
declarative tool layers, credential delegation, terminal attachment, and an
explicit host-side publication boundary.

Fortlet does not replace an agent harness, provide a hosted control plane,
require a vendor account, add cross-harness messaging, or promise that
networking is disabled. It does not mount SSH private keys or the 1Password SSH
agent into capsules. MicroSandbox is the only implemented backend until a real
second execution mode demonstrates a useful shared seam.

Fortlet may support Nix and devenv project environments exceptionally well,
but projects are not required to use them. Advanced users may opt into
Nix-backed environments, overlays, and caching. Any opinionated runtime use of
Nix must provide a concrete security, reproducibility, or UX advantage that is
not available through a simpler mechanism.

The initial project environment is intentionally smaller: a fixed JSON
manifest and adjacent POSIX recipe produce one content-verified layer inside a
credential-free provisioning capsule. Public downloads are allowed without
authentication and remain the recipe's responsibility to pin. Services,
private inputs, arbitrary repository inputs, and activation hooks are not part
of that contract.

## Workflow bindings (from ADAPTATION.md)

- B1 Proposal prefix: FIP (Fortlet Improvement Proposal), e.g. FIP-0001.
- B2 Architecture-track threshold: security and credential boundaries,
  project/capsule identity, persistence, public CLI or configuration contracts,
  harness adapter contracts, publication semantics, remote workspace and
  transport models, or a choice whose reversal would cost more than one day.
- B3 Experiment spend unit: declared engineering effort. Any external money,
  quota, public remote mutation, or user-facing test requires an explicit
  operator grant in addition to an experiment record.
- B4 Verification set: see `docs/RUNBOOK.md`.
- B5 Charter thresholds: no external spend or remote mutation by default; one
  experiment in flight; stop after two terminal failures sharing an assumption;
  abort a multi-unit experiment above a predeclared 25% failure rate; report at
  every deliverable and terminal closure.
- B6 Generator-owned artifacts: none currently.
- B7 Reserved holdouts: none currently. Add a reserved integration surface
  before optimizing against benchmark or compatibility suites.

## Repository structure

1. `src/` and `tests/` — the Rust CLI and verification coverage.
2. `package.nix`, `flake.nix`, and `nix/` — reproducible packaging.
3. `arch/` — FIPs and their mutable conformance map.
4. `experiments/` — declared contact-with-reality records and session handoff.
5. `governance/` — standing operator charter and self-correction playbook.
6. `docs/` — operational runbook.

## Open design questions

1. What global inventory and safe capsule-removal contract should follow the
   project-scoped management surface?
2. How should background-process leases be observed and expired?
3. How should standalone installation expose optional shims without shell
   mutation?
4. What workspace transfer and credential delegation model should a future
   self-hosted execution host use?
