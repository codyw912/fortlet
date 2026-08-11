# FIP-0002: Optional transparent harness shims

Status: Accepted
Recorded: 2026-08-11 from the operator-validated transparent-shim design
Requires: FIP-0001

## Summary

Fortlet packages transparent `codex` and `tact` launchers in an immutable shim
directory that users may explicitly prepend to `PATH`. Shim invocation enters
the same fail-closed capsule-session transaction as the explicit Fortlet CLI.
Users may instead keep using `fortlet run`, or deliberately invoke a host
harness through `fortlet native`; Fortlet never chooses native execution as a
fallback.

## Motivation

FIP-0001 selects transparent harness commands as the ordinary product
experience but leaves their activation and escape-hatch contract open. Shell
aliases and startup-file installers are fragile across interactive,
non-interactive, and declaratively managed environments. Placing shims directly
beside ordinary package binaries would activate them implicitly and can create
package-manager collisions with host harnesses.

Fortlet needs a reversible activation boundary that works in vanilla shells,
supports Nix and Home Manager without imperative file mutation, and preserves
an explicit route to a host-native harness for deliberate troubleshooting or
comparison.

## Decision

The Fortlet package owns a dedicated shim directory outside its ordinary binary
directory. That directory contains launchers for registered transparent
harnesses and becomes active only when the user places it before host harnesses
on `PATH`.

The Fortlet executable uses its invocation name to distinguish the ordinary
CLI from a harness shim. Recognized shims enter the existing capsule-session
transaction. The ordinary CLI remains available without shim activation and
adds `fortlet native <harness> -- <arguments>` as the only host escape hatch in
this design.

## Specification

### Packaging and activation

1. A supported Fortlet package MUST expose an immutable shim directory separate
   from the directory containing the ordinary `fortlet` executable.
2. The shim directory MUST contain launchers named `codex` and `tact`, and their
   identities MUST be validated against Fortlet's registered harnesses.
3. Installing Fortlet MUST NOT activate the shims implicitly. Activation MUST
   be an explicit, user-controlled `PATH` ordering decision.
4. Fortlet MUST NOT write or propose automated edits to shell startup files to
   activate shims.
5. Activation MUST NOT require aliases, shell functions, generated shell text,
   `eval`, or interactive-shell initialization.
6. Omitting the shim directory from `PATH` MUST leave `fortlet doctor` and
   `fortlet run <harness> -- <arguments>` fully usable.

### Transparent invocation

1. Invocation as `codex` or `tact` MUST select the identically named registered
   harness and enter the same fail-closed capsule-session transaction as
   `fortlet run`.
2. A shim MUST forward its argument vector without consuming harness arguments
   and MUST preserve the explicit launch path's current directory, interactive
   terminal behavior, terminal resize, signals, and exit status.
3. Successful shim startup MUST be silent.
4. A recognized shim MUST NOT execute a host harness unless the user separately
   invokes the native escape hatch.
5. An unsupported invocation name MUST fail before capsule creation or host
   process execution.

### Native escape hatch

1. `fortlet native <harness> -- <arguments>` MUST be the explicit host-native
   invocation form for registered harnesses.
2. Native resolution MUST inspect `PATH` in order, skip Fortlet's own shims and
   any candidate that resolves to the running Fortlet executable, and select
   the first remaining executable with the requested harness name.
3. Native execution MUST invoke the selected executable directly without a
   shell and MUST preserve arguments, current directory, environment, signals,
   and exit status according to the host process model.
4. Failure to find a native executable MUST prevent process launch and report
   one actionable correction.
5. Isolated launch failure MUST NOT invoke `fortlet native` or any host harness
   automatically.

### Failure behavior and evidence

1. A shim launch failure MUST identify whether project resolution, credentials,
   environment provisioning, capsule reconciliation, or terminal attachment
   failed and MUST give one actionable next step.
2. Automated evidence MUST cover dispatch for both registered shim names,
   argument forwarding, explicit-CLI compatibility, ordered native lookup,
   recursion rejection, and missing-native failure.
3. Package evidence MUST verify both launchers exist in the shim directory and
   can be discovered without shell initialization in a minimal non-interactive
   environment.
4. Runner-executed local evidence MUST verify both packaged shims enter Fortlet
   without silently falling back to host execution.

## Consequences

Users with declarative shell configuration can activate Fortlet without
allowing it to mutate their setup, while users who prefer explicit commands do
not take on a second command-resolution layer. Keeping shims outside the
ordinary binary directory makes activation visible and avoids package output
collisions, but documentation must teach one `PATH` ordering step.

Multicall dispatch keeps package launchers thin and routes all isolated behavior
through the existing product seam. Native lookup requires careful recursion
checks because the Fortlet shim is intentionally the first matching executable
on an activated `PATH`.

## Alternatives Considered

1. Writing shims into `~/.local/bin` would offer automatic activation but would
   mutate user-owned state and need collision, backup, repair, and uninstall
   semantics outside this slice.
2. Shell aliases or functions would be small but depend on shell initialization
   and behave inconsistently in non-interactive and tool-managed shells.
3. Putting `codex` and `tact` beside `fortlet` in the ordinary package binary
   directory would make installation implicitly change command resolution and
   can collide with separately packaged host harnesses.
4. `FORTLET_NATIVE=1 codex` would avoid a subcommand but makes the security-
   relevant escape less visible and easier to propagate accidentally through an
   environment.
5. Separate `codex-native` and `tact-native` commands would add global command
   names and complicate declarative collision management.

## Open Questions

1. Whether daily use justifies a different explicit native command shape.
2. How a future standalone installer exposes and documents the same optional
   shim directory without making shell mutation a requirement.
