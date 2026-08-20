# Session Handoff — Project-local shim activation active

Audience: a fresh agent session. `GOAL.md` is normative. Verify `main`, the
Jujutsu stack, conformance, and the complete `docs/RUNBOOK.md` gate before
relying on this summary. Read FIP-0001, FIP-0002, and FIP-0005 in full before
implementation or experiment dispatch.

## Verified baseline

PR #12 was squash-merged as
`6336815763aa64222ad43342f8b6e1e9fca9e953`. Its tree matches the reviewed
result. The working copy began as a clean empty child of `main`, and
`main..@` contained no work.

The complete baseline gate passes on aarch64-darwin through `nix develop`: all
88 unit tests and every enabled integration test, formatting, strict
all-target/all-feature Clippy, conformance, and `nix flake check`. Nix emits
only the expected incompatible `x86_64-linux` omission warning. FIP-0001 and
FIP-0002 remain partial with explicit gaps; FIP-0003 through FIP-0011 remain
conformant.

## Current mission

Add one explicit activation output whose `bin` resolves to the existing
package-owned `libexec/fortlet/shims` directory, then use it in a minimal named
`agents` dev shell. The directory-level link is important: native resolution
canonicalizes the `PATH` directory to the existing `FORTLET_SHIM_DIR`, so
`fortlet native` skips it without a second shim identity or resolver change.

The default development shell must remain unchanged. The named shell must not
depend on Bash or fish initialization and must compose with an existing project
environment. A real probe showed Nix initially placed both shims first, but the
operator's normal fish startup then moved `~/.local/bin` ahead and selected
native Tact. Do not recommend a normal child fish as authoritative activation.
The first operator unit rejected repeated `nix develop path:...` invocation as
too slow and exposed a missing separator in the documented native command. The
second began from a fully clean baseline but proved AGD's direnv prompt hook
removes a manual PATH prepend before the next command. Neither unit launched a
native or managed harness. Do not resume Experiment 0038 or 0039 and do not
dispatch another shell workaround.

The operator confirmed PATH restoration and authorized a separate uncommitted
AGD devenv integration. Experiment 0040 pinned exact public Fortlet revision
`803ce20b951863db0ce8152588bef740f6f95647` and conditionally included the two
Fortlet outputs. Its non-interactive agent phase passed: cold build 406.93
seconds, cached entry 0.51 seconds, correct command paths, and native Codex
0.147.0. The exact interactive fish unit then failed without retry: direnv
successfully renewed and entered devenv, but the next prompt selected host
Cargo, omitted `fortlet`, and retained older shim paths. The native command
failed before any managed shim or capsule. Static inspection found both mise
and direnv rewriting PATH from independent `fish_prompt` state; the observed
paths show mise's earlier baseline won after direnv's export. The AGD diff
remains exactly `flake.nix`, `flake.lock`, and `devenv.nix`. No AGD publication
or product-code change is authorized.

The deterministic activation check passes. A read-only AGD composition probe
preserved its root and Nix Rust toolchain, resolved all three Fortlet commands
from the expected outputs, and returned native Codex 0.147.0 through the escape
hatch. AGD's tracked tree remained clean. No shim or capsule was launched.

## Next action

Experiment 0042 passed from a new operator terminal: two prompts preserved all
four expected Nix paths, native and managed Codex returned 0.147.0, exactly one
AGD capsule became running, and public stop/reset restored absence and empty
inventory. The first absent-to-running managed command took about nine seconds;
reattachment was not measured. Do not retry terminal Experiments 0040 or 0041
or add a mise-specific workaround. The final aarch64-darwin local gate passes:
88 unit tests, every enabled integration, formatting, strict Clippy,
conformance, the curated package, and the activation check; only the expected
incompatible x86_64-linux warning remains. Checkpoint and sign the exact final
revision, update draft PR #13 with the terminal evidence, and mark it ready
only after hosted checks pass. Preserve the three-file uncommitted AGD diff and
keep both repositories' merge authority with the operator.
