# Session Handoff — Bounded non-interactive Codex complete

Audience: a fresh agent session. `GOAL.md` is normative and complete; do not
start implementation under it. Verify PR #8 and `main` before relying on this
briefing, then choose a new GOAL with the operator. FIP-0010 is the durable
non-interactive contract.

## Verified branch state

Protected `main` began this GOAL at
`f05da5ec8274e5905d8baf81fe61c48c8e76ecd4`, the squash merge of PR #7. The
single authorized bookmark is `noninteractive-codex-reliability`; PR #8 is the
goal pull request. The operator remains the sole merge authority.

The branch contains accepted FIP-0010 and conformant implementation evidence.
Before the final documentation tip, 81 unit tests, 29 ordinary integration
tests, formatting, strict all-target/all-feature Clippy, conformance, and
`nix flake check` passed through `nix develop` on aarch64-darwin. Both ignored
stock-Codex 0.147.0 compatibility fixtures passed against local fake servers.
The final publishable tip must retain a complete green local gate and hosted
`Rust verification`; verify those exact PR facts rather than assuming them
from this handoff.

## What changed

When host stdin or stdout is not a terminal, Fortlet now:

1. acquires a non-PTY MicroSandbox streaming exec handle;
2. immediately closes an explicit empty stdin pipe;
3. forwards and flushes stdout and stderr events without rewriting them;
4. returns only the explicit guest exit code; and
5. rejects stream closure without an exit event.

The harness adapter owns an optional inactivity policy. Codex 0.147.0 selects
600 seconds renewed by every stdout or stderr event. Tact selects none. On
expiry Fortlet kills only the command, drains for at most five seconds,
preserves the capsule, cancels the credential-renewal task, and reports one
correction: retry interactively from a supported terminal. Interactive
attachment and `fortlet native` are unchanged.

## Paid-for diagnosis

Experiment 0026 and pinned-source inspection established that Codex can remain
alive in an upstream wait while the old collected attachment hides all output.
Normal shell and stock-Codex local success/failure commands still produced
explicit MicroSandbox exit events; Fortlet's renewal task did not keep the
attachment open.

Experiment 0027 accepted the first immutable production streaming path with a
credential-free packaged Tact 0.3.7 launch and exact zero exit. Experiment 0028
then used the one permitted real prompt. It streamed
`Reading additional input from stdin...`, remained there, and failed exactly at
Fortlet's 600-second inactivity ceiling with status 1. Public stop/reset
restored absence. No Apps warning appeared and no retry occurred.

That line is emitted immediately before Codex 0.147.0 calls
`read_to_end(stdin)`. MicroSandbox 0.6.8's streaming `StdinMode::Null` is only a
host-side option: it neither enters `ExecRequest` nor sends the empty
`ExecStdin` EOF frame. The contemporaneous macOS firewall/DNS popup therefore
did not cause this process-level block; Codex had not reached its model request.

The correction uses `StdinMode::Pipe` plus awaited `ExecSink::close`.
Experiment 0029 accepted the resulting immutable package: packaged Codex
advanced immediately beyond its stdin read, displayed its session, attempted
only `http://127.0.0.1:9/v1`, and returned exact status 1 in about 8.7 seconds.
Public cleanup restored Codex absence and an empty MicroSandbox inventory.

## Honest limitations

- The sole authorized model-backed successor was consumed discovering the
  pinned SDK's unclosed streaming stdin. No post-correction provider prompt was
  permitted, so ordinary model success through the corrected non-interactive
  path is not claimed. Interactive model success remains proven by Experiment
  0025.
- The 600-second Codex inactivity ceiling is fixed for this first slice. A
  legitimately silent longer command must be run interactively.
- Native x86_64-linux package verification remains outstanding.
- Automated interactive Codex exit parity remains unresolved; manual evidence
  says one Ctrl-C exits an empty composer.

## What to do next

If PR #8 is not merged, finish only its existing FIP-0005 lifecycle: verify the
signed reviewed tip and hosted check, leave merge to the operator, then fetch
`main`, prove tree equality, and remove only the landed goal bookmark. If it is
merged, verify that closure before discussing a successor GOAL.

A natural product-focused successor is one deliberately authorized
post-correction daily-use Codex session that separates first-run macOS
firewall/DNS readiness from Fortlet behavior and proves an ordinary prompt/edit
loop. Predeclare any provider unit and rehearse its full cleanup path first.
Standalone installation, general logs/restart/inventory, remote execution, and
alternative runtimes remain outside the completed mission.

Do not resume Experiments 0024, 0028, or 0029; silently retry a provider prompt;
restore streaming `stdin_null`; weaken credential or mount boundaries; infer
post-correction model success from the loopback regression; merge PR #8; or
start a new implementation before the operator accepts a new GOAL.
