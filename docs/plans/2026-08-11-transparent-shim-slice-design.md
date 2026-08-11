# Optional transparent harness shims

Status: validated with the operator on 2026-08-11.

## Outcome

Fortlet will provide an optional package-owned shim directory containing
`codex` and `tact`. When a user declaratively prepends that directory to
`PATH`, ordinary harness invocations enter Fortlet's existing fail-closed
capsule-session transaction. Users who do not activate the directory retain
the explicit `fortlet doctor` and `fortlet run` interfaces unchanged.

This is the smallest daily-use slice of FIP-0001. It does not add lifecycle
management, standalone distribution, or another runtime.

## Architecture and activation

The Fortlet package owns an immutable directory such as
`libexec/fortlet/shims` with launchers named `codex` and `tact`. Each launcher
dispatches to the installed Fortlet binary by invocation name. Harness
registration remains the single source of truth for recognized names.

The shim directory is separate from the package's ordinary `bin` directory.
Installing Fortlet therefore does not automatically shadow host harnesses or
create package-manager collisions. Activation is an explicit `PATH` ordering
decision. A Nix or Home Manager user can express it declaratively, for example:

```nix
home.sessionPath = [
  "${pkgs.fortlet}/libexec/fortlet/shims"
];
```

Fortlet does not edit shell startup files. The activation contract does not
use aliases, functions, `eval`, or interactive-shell initialization, and the
launchers must work in a minimal non-interactive environment.

## Invocation and data flow

Fortlet has three explicit invocation paths:

1. `fortlet ...` uses the existing CLI.
2. A `codex` or `tact` shim selects that registered harness and forwards the
   remaining arguments through the existing capsule-session path.
3. `fortlet native <harness> -- <arguments>` deliberately executes a host
   harness and never enters MicroSandbox.

The transparent path preserves the explicit path's project resolution,
credential checks, environment provisioning, capsule reconciliation, working
directory, terminal attachment, signals, and exit status. Successful startup
remains silent. It never falls back to native execution.

The native resolver searches `PATH` for the requested harness, skips Fortlet's
own shim target to prevent recursion, and replaces the current process without
using a shell. Failure to find a native executable reports how to place one
after the shim directory or use Fortlet's isolated path. The initial command
shape is intentionally revisable by a successor proposal if daily use supplies
better evidence.

## Failure behavior

Errors on the affected session path identify the failed stage—project,
credentials, environment, capsule, or terminal—and give one actionable next
step. This is bounded launch-path work, not a general error-framework rewrite.
An unrecognized invocation name and an unsupported native harness fail before
any capsule or host harness starts.

The credential and isolation boundaries remain those of FIP-0001. Shims never
mount host credential files, signing authority, or the 1Password agent, and no
successful or failed isolated launch silently invokes a host harness.

## Evidence

Automated evidence covers invocation-name dispatch, argument and optional
separator handling, both registered harnesses, recursion-safe native lookup,
failure when no native executable exists, explicit-mode compatibility, and the
installed package layout. Package checks exercise launcher discovery under a
minimal non-interactive environment without sourcing shell initialization.

After the standard verification set is green, a predeclared local smoke
experiment invokes `codex --version` and `tact --version` with the packaged
shim directory first on `PATH`. It may create only Fortlet-owned local test
capsules and state. The record reports argument flow, project selection,
isolation, exit status, actual effort, and cleanup.

Conformance changes ship with the behavior and tests that establish them. The
map records the shim surface that is now implemented while retaining terminal,
lifecycle, distribution, adapter-ownership, and insufficient-coverage gaps
that the slice does not close.

## Explicit non-goals

- Writing or modifying shell startup files.
- Shim install, uninstall, status, or repair commands.
- Lifecycle commands or observable lease implementation.
- Standalone non-Nix distribution.
- New harnesses, a generic runtime abstraction, or remote execution.
- Host-side publication or any remote mutation.
- Complete FIP-0001 conformance.
