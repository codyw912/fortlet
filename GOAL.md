# GOAL: Provide project-local Nix activation for Fortlet shims

Status: active

## Outcome

Make Fortlet's optional `codex` and `tact` shims easy to activate for one
development shell without changing global shell configuration. Provide a
minimal named Nix shell for immediate use and a composable package output for
projects that already use devenv or another Nix dev shell.

This is a packaging and activation refinement of accepted FIP-0001 and
FIP-0002, published under FIP-0005. It does not change the capsule-session,
credential, environment, harness, or lifecycle architecture and needs no new
FIP.

## Deliverables

1. Verify merged `main`, clean Jujutsu state, conformance, and the complete
   `docs/RUNBOOK.md` gate through `nix develop` before implementation.
2. Export an explicit shim-activation package whose `bin` entry resolves to
   Fortlet's existing immutable package-owned shim directory.
3. Add a minimal `devShells.agents` containing Fortlet and that activation
   output. Keep `devShells.default` unchanged and avoid startup-file writes,
   aliases, functions, `eval`, generated activation text, or reliance on an
   interactive shell initializer.
4. Prove deterministic discovery for both shims and recursion-safe
   `fortlet native` resolution, including arguments and a native executable
   later on `PATH`.
5. Document fish entry, explicit CLI use without shims, exit behavior, and
   composition into an existing devenv project. Predeclare one bounded local
   smoke experiment for the AGD pilot before invoking a packaged shim.
6. Run the complete local and hosted gates, publish one goal-scoped draft PR,
   and leave merge authority with the operator.

## Definition of Done

1. `nix develop .#agents -c fish` resolves `fortlet`, `codex`, and `tact` from
   immutable Fortlet outputs without mutating user configuration.
2. Plain `nix develop` does not include the activation output, and explicit
   `fortlet run` remains fully usable without either shim.
3. `fortlet native codex -- <arguments>` skips the activated shim path and
   selects the first later native executable without recursion.
4. A devenv project can opt in by adding the two Fortlet package outputs; no
   Fortlet-specific shell hook is required.
5. The bounded smoke begins and ends with no owned capsule, changes no AGD
   product file, and records the exact fish-visible resolution and native
   escape result.
6. Conformance and the complete `docs/RUNBOOK.md` verification set pass.

## Excluded scope

No global Home Manager or shell configuration, AGD product change, new
harness, project guest environment, credential mechanism, runtime behavior,
standalone installer, release, repository setting, or merge is in scope.

## Budget and authority

The engineering ceiling is 90 minutes. The smoke may use one immutable package,
one credential-free shim `--version` invocation, one owned capsule at a time,
zero model prompts, zero retries, and zero external money. Acceptance
authorizes one `project-local-shim-activation` bookmark and one PR targeting
`main` under FIP-0005. The operator remains the sole merge authority.

## Verification

Run the complete standard verification set from `docs/RUNBOOK.md` through
`nix develop` before publication readiness.

