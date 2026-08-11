# Preserve the packaged MicroSandbox runtime

Status: validated by the operator on 2026-08-11.

## Outcome

Fortlet's Nix package will preserve the fixed-output MicroSandbox release
runtime byte-for-byte. The package will continue to wrap Fortlet with explicit
paths to that runtime, but Nix's generic stripping phase will not rewrite the
upstream Mach-O signatures or remove their entitlements.

This repairs the packaged runtime boundary without changing FIP-0001,
FIP-0002, the shim contract, capsule behavior, or credential handling.

## Mechanism

The package disables automatic stripping for its output. This is intentionally
broader than re-signing only `msb`: the runtime archive is an immutable,
hash-pinned input, and preserving it is simpler and safer than manufacturing a
Fortlet-specific signature. The trade-off is that Fortlet's own release binary
also remains unstripped in this slice.

The install check extracts the same fixed-output runtime archive into temporary
build state and compares both the installed `msb` executable and its libkrunfw
library byte-for-byte with the archive. This directly catches stripping,
re-signing, or any other fixup mutation on every declared platform without
depending on macOS-only entitlement inspection tools.

## Evidence

The standard Rust, formatting, Clippy, conformance, Nix flake, and doctor gates
must pass. Only then may a new experiment invoke one packaged shim with
`--version`. Successful VM creation falsifies the diagnosed packaging blocker;
the existing two-harness smoke can then be completed without retrying the
terminally closed Experiment 0001.

No shell configuration, native fallback, runtime abstraction, user-owned
MicroSandbox installation, or unowned capsule state is changed.
