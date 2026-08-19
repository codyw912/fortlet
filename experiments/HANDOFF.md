# Session Handoff — Ordinary Codex work-loop correction complete

Audience: a fresh agent session. `GOAL.md` is normative and complete. Verify
the publication and merged `main` state before relying on this summary, then
choose the next GOAL with the operator. Do not send another provider prompt for
Experiments 0030–0032.

## Current local state

PR #8 previously squash-merged FIP-0010 at
`53378399bfa581d5a7b3db8538b234dc3733bb26`. The current stack above that main
contains the completed daily-work-loop GOAL, Experiments 0030–0032, one useful
test-only model edit, and the FIP-0006 login-shell correction. The single goal
bookmark is `daily-codex-work-loop`; draft PR #9 targets `main`. Verify Jujutsu,
the exact signed tip, and GitHub checks rather than inferring readiness from
this handoff. The operator remains the sole merge authority.

## What the live work proved

Experiment 0030 first used synthetic auth only to create a disposable capsule.
Its one credential-free probe resolved `chatgpt.com` and returned HTTP 200 from
`https://chatgpt.com/robots.txt`; public stop/reset restored absence and empty
inventory. The complete standard gate passed before model spend.

The sole real Codex prompt crossed the prior stdin failure, streamed its work,
showed no Apps warning, changed only test code in `src/runtime.rs`, and exited
host status zero. It renamed the stdin regression and made the fake execution
session prove stdin closes before the first event read. Codex's exact focused
Cargo command itself returned 127 because Cargo was unavailable to its
`/usr/bin/bash -lc` child. The same test passed independently through host
`nix develop`. No prompt edit, follow-up, retry, interactive fallback, or native
fallback occurred.

Experiment 0031 used one credential-free Tact capsule to compare the composed
shell path. Guest Cargo was mode 755 and worked directly and through `bash -c`.
Plain `bash -lc` replaced Fortlet's PATH with Debian's login default and lost
Cargo. A disposable `BASH_ENV` restored the exact managed PATH and Cargo 1.97.1.
This accepted experiment is the causal attribution.

Experiment 0032 exercised the production correction from immutable package
`/nix/store/fls59kf9mwxrs8n1g3whd1g7mgjfk0cv-fortlet-0.1.0`. Its login shell
reported the package-owned hook, resolved
`/opt/fortlet/project/bin/cargo`, and passed the exact focused test. The unit is
still rejected for protocol variance: the Tact-specific Cargo cache was cold,
so Cargo downloaded public crates beyond the experiment's declared network
budget, and capsule configuration was not inspected before cleanup. Do not
rerun it to manufacture acceptance. Cleanup reported running, stopped, reset,
absent, and final inventory `[]`; the repo stayed clean.

## Product change

Base environment `bookworm-2` installs the read-only fragment
`/opt/fortlet/base/etc/fortlet/bash-env`. Capsules set reserved internal
`FORTLET_MANAGED_PATH` and `BASH_ENV` values after project and harness
environment values. The fragment restores the complete project/base/harness
PATH after a Debian non-interactive login profile replaces it, preserving the
pinned MicroSandbox script prefix when present.

Project manifests may no longer set `BASH_ENV`; `FORTLET_*` names were already
reserved. Fortlet writes no `.profile`, `.bashrc`, fish configuration, or other
host/persistent startup file. Both harnesses use the common runtime mechanism;
there is no Codex-specific command rewrite.

## Deterministic evidence

The implementation adds a base-script assertion, reserved-variable coverage,
managed shell-environment assertions for both harnesses, and the strengthened
stdin ordering regression. Before the final publication pass, all 83 unit tests
and all enabled integration tests passed through `nix develop`; formatting,
strict all-target/all-feature Clippy, and conformance were green. Run the full
`docs/RUNBOOK.md` set again on the final documentation tip before readiness,
including `nix flake check` on aarch64-darwin.

## Honest limitations

- The sole model task did not run its guest test; the project-layer correction
  was validated afterward without another model prompt.
- Experiment 0032's mechanism succeeded but the experiment is rejected for its
  undeclared Cargo downloads and omitted configuration observation.
- Native x86_64-linux package verification remains outstanding.
- The ten-minute Codex non-interactive inactivity ceiling and unresolved
  automated interactive idle-exit parity remain unchanged.
- Apps remain deliberately disabled inside managed Codex.

## Next action

Finish only PR #9's existing FIP-0005 publication lifecycle: run the complete
final gate, sign and push the exact publishable tip, require hosted `Rust
verification`, and mark the PR ready. Leave squash merge to the operator. After
merge, fetch main, prove tree equality with the signed reviewed tip, remove only
the landed goal bookmark, and then discuss a successor GOAL.

Do not repeat the provider unit; resume Experiments 0030–0032; weaken the
credential/mount boundary; write shell startup files; hide the rejected unit;
push or force-push main; merge; or start unrelated product work under this
completed GOAL.
