# Fortlet Experiment Records

This directory records contact-with-reality attempts and their outcomes.
Proposals hold durable design decisions; these files hold mutable
experimental status, declared scope, outcomes, attribution, rejection
reasons, and rollout decisions.

Every experiment must state its production mechanism, scope, control,
alternatives, risks, acceptance criteria, and budget BEFORE any expensive
dispatch. Record losses and invalid evidence as carefully as wins. Never
edit raw run artifacts to improve a result. Never resume a terminally
closed experiment — a successor gets a new number and identity.

Also in this directory:

- `HANDOFF.md` — the descriptive briefing for the next session (see
  `WORKFLOW.md` §1.4). Rewritten at mission completion; history in git.

## Index

Keep one line per record, newest last, outcome included — this list is the
scannable failure/win history of the project:

- `0000-template.md` — the record template.
- `0001-packaged-transparent-shims.md` — rejected; both shims entered Fortlet, but MicroSandbox VM creation failed before guest launch.
- `0002-preserved-runtime-shim-smoke.md` — accepted; preserved runtime entitlements restored VM creation and both packaged shims returned pinned guest versions.
- `0003-interactive-packaged-sessions.md` — rejected; both UIs passed PTY, resize, concurrency, and responsiveness checks, but neither exited after one bounded `SIGINT`.
- `0004-native-ui-signal-control.md` — rejected as a two-harness control; native Tact reproduced the packaged timeout, while native Codex closed before signal.
- `0005-native-codex-marker-control.md` — rejected; removing inherited outer-Codex markers did not prevent native Codex from closing before signal.
- `0006-external-native-codex-control.md` — accepted; external native Codex reached signal and reproduced the packaged first-interrupt timeout.
- `0007-exact-codex-exit-parity.md` — rejected; exact `/exit\r` input reached both UIs, but neither terminated within the fixed bound.
- `0008-human-paced-codex-exit-parity.md` — rejected; fixed paced `/exit` input reached both UIs, but neither terminated within the fixed bound.
- `0009-idle-ctrl-c-key-exit-parity.md` — rejected; one Ctrl-C PTY key reached both UIs, but neither terminated within the fixed bound.
- `0010-project-capsule-status-and-stop.md` — accepted; the public CLI proved absent, running, stopped, and absent around bounded stop and exact owned cleanup.
- `0011-public-alpha-repository.md` — accepted; the complete signed history is public on GitHub with only `main` and no extra remote resources.
- `0012-project-capsule-reset.md` — accepted; the public CLI refused active reset, removed the owned terminal capsule, preserved durable state, reported absence idempotently, and cleaned up exactly.
- `0013-pull-request-workflow-bootstrap.md` — rejected; the exact draft PR opened, but its sole initial hosted unit could not link Linux `libcap-ng`.
- `0014-hosted-linux-linker-closure.md` — rejected; the Ubuntu package closed linking and tests passed, but strict hosted Linux Clippy found a platform-specific `openpty` mutability error.
- `0015-portable-openpty-winsize.md` — accepted; one raw winsize pointer passed the complete PTY suite and every hosted Linux Rust gate across the Apple and Linux `openpty` bindings.
- `0016-bootstrap-publication-closure.md` — accepted; PR #1's exact historical exception is bounded by squash-only settings, protected `main`, unchanged refs, and locally qualified closure evidence.
- `0017-opt-in-project-environment.md` — rejected before dispatch; a pre-existing stopped Codex capsule violated the required absent baseline, so no VM, recipe, or model prompt ran.
- `0018-clean-baseline-project-environment.md` — declared; a fresh bounded unit will test the pinned project environment from a verified absent capsule baseline.
