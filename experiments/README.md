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
- `0011-public-alpha-repository.md` — declared; public repository creation and complete-history publication await rehearsal.
