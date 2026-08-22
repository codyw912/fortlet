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
- `0018-clean-baseline-project-environment.md` — rejected; the sole clean-baseline unit exhausted the provisioning root disk while unpacking the full Rust distribution, before layer publication or Codex launch.
- `0019-output-backed-project-provisioning.md` — rejected; output-backed extraction cleared the capacity failure, but validation rejected an absolute Debian development link before publication.
- `0020-relative-debian-project-links.md` — accepted; one public launch published the verified tool layer, exposed pinned Rust and Jujutsu, passed 10 focused Linux tests, and cleaned up the owned capsule.
- `0021-first-daily-fortlet-session.md` — rejected; packaged prepare and shim attachment passed, but Codex required token refresh before the first prompt and no edit/test loop began.
- `0022-renewable-codex-daily-session.md` — rejected; external-token mode avoided guest refresh, but a missing `last_refresh` field omitted the bearer header and both model transports returned 401 before a response or edit.
- `0023-codex-bearer-projection-confirmation.md` — accepted; the repaired bearer projection authenticated one bounded model-backed edit/test loop, and public stop/reset restored absence while preserving durable layers and cache.
- `0024-managed-codex-apps-disable.md` — rejected; the Apps warning was absent, but the sole packaged non-interactive model attempt produced no response before its ten-minute bound and public cleanup.
- `0025-interactive-codex-apps-disable.md` — accepted; the Apps-disabled packaged interactive path returned the exact model response without the known warning, then exited and cleaned up publicly.
- `0026-credential-free-noninteractive-attribution.md` — accepted; local-only controls proved Codex remained alive on an unanswered request while collected execution hid progress, whereas normal success and failure produced exact output and exit events.
- `0027-packaged-noninteractive-streaming.md` — accepted; one credential-free immutable-package Tact launch streamed the pinned version output, returned exact status zero, and cleaned up publicly before any model-backed unit.
- `0028-managed-noninteractive-codex-successor.md` — rejected; streaming exposed Codex blocked reading an unclosed SDK stdin, then Fortlet enforced the 600-second bound and cleaned up publicly without retry.
- `0029-packaged-codex-explicit-eof.md` — accepted; explicit streaming EOF advanced packaged Codex past its stdin read and returned exact status 1 after guest-loopback-only transport failures.
- `0030-first-post-eof-daily-work-loop.md` — rejected; network readiness and streaming passed, but the sole Codex edit/test task could not resolve Cargo beneath its login-shell command path; cleanup completed without retry.
- `0031-login-shell-project-path.md` — accepted; guest tools remained executable, Debian `bash -lc` alone dropped the managed PATH, and a disposable `BASH_ENV` restored the exact Cargo path without startup-file mutation.
- `0032-packaged-login-shell-project-tools.md` — rejected for protocol variance; the packaged hook restored Cargo and passed the focused login-shell test, but a cold Tact Cargo cache made undeclared public crate downloads and configuration inspection was omitted before cleanup.
- `0033-global-owned-capsule-inventory.md` — accepted; immutable packaged inventory reported one real owned capsule from outside its project, and existing public stop/reset restored absence and empty raw inventory.
- `0034-post-path-codex-work-loop.md` — rejected; the shimmed model produced the correct test, but the exact guest Cargo command found its persistent target read-only inside Codex's workspace sandbox and the altered in-process retry never settled.
- `0035-workspace-cargo-codex-loop.md` — rejected; the sole successor stopped during automatic first-use provisioning before Codex launch, leaving the corrected workspace Cargo target without provider-backed proof.
- `0036-firewall-approved-codex-work-loop.md` — rejected; cold preparation and the corrected target passed, but the exact guest test could not fetch MicroSandbox's build dependency.
- `0037-runtime-capsule-dns-clearance.md` — accepted; the exact focused test passed under the corrected target in an ordinary owned capsule, classifying the earlier interruption as environment-specific.
- `0038-project-local-shim-activation.md` — rejected; repeated local-path Nix evaluation was too slow for the daily loop, and the declared native command omitted Fortlet's required separator before any harness launch.
- `0039-prebuilt-fish-shim-activation.md` — rejected; AGD's direnv prompt hook removed the manual PATH prepend before inspection, so Fortlet was unavailable and no harness launched.
- `0040-agd-devenv-fortlet-activation.md` — rejected; non-interactive activation passed, but mise reconstructed an older PATH after the operator's successful direnv reload, omitting Fortlet before any managed launch.
- `0041-mise-direnv-prompt-composition.md` — rejected; a fresh fish with the exact generated mise and direnv hooks preserved every Nix path, falsifying a general prompt-order defect before any treatment or edit.
- `0042-fresh-fish-agd-activation.md` — accepted; a fresh real fish preserved every Nix path, native and managed Codex returned 0.147.0, and public cleanup restored empty inventory; first managed startup took about nine seconds.
- `0043-capsule-lifecycle-latency-screen.md` — rejected; absent and stopped launches overlapped around 0.4–0.5 seconds while all six running launches completed in 0.06 seconds, so the prior nine-second result did not reproduce and lifecycle state alone did not attribute it.
- `0044-managed-startup-phase-attribution.md` — terminal failure; the selected development executable was stale because the rehearsal built only its test harness, so three lifecycle commands produced no timing events before public cleanup.
- `0045-built-startup-phase-attribution.md` — accepted; runtime reconciliation dominated absent and stopped totals, while six running launches had a 55-millisecond median and the earlier nine-second observation did not reproduce.
- `0046-public-nix-provider-rehearsal.md` — rejected; a long isolated HOME exceeded MicroSandbox's Unix-socket path limit before any Nix provider operation or VM creation.
- `0047-short-root-nix-provider-rehearsal.md` — rejected before dispatch; a relative clone destination escaped the declared short root, so exact cleanup occurred before any Fortlet, VM, Nix, or model command.
- `0048-absolute-root-nix-provider-rehearsal.md` — rejected; absolute setup and plan passed, but isolated MSB_HOME hid the installed runtime binary before any guest or Nix command; exact capsule and root cleanup completed.
- `0049-pinned-runtime-nix-provider-rehearsal.md` — rejected with a Fortlet defect; the pinned runtime reached guest provisioning, where common-base validation used `/usr/bin/tar` instead of Debian's `/bin/tar`; cleanup completed before Nix.
- `0050-corrected-base-nix-provider-rehearsal.md` — rejected before provider preparation; the sole post-fix attempt stopped while fetching the common base image, created no capsule, and exhausted the active GOAL's successor budget.
- `0051-firewall-cleared-nix-provider-rehearsal.md` — rejected with a Fortlet defect; the successor reached common-base extraction, which required a generated CA bundle that it never generated; cleanup completed before Tact or Nix.
- `0052-ca-corrected-nix-provider-rehearsal.md` — rejected; command-level diagnostics identified that raw ca-certificates extraction provides neither the generated selection config nor bundle, invalidating the attempted fix before Tact or Nix.
- `0053-package-lifecycle-base-rehearsal.md` — accepted; Debian's real package lifecycle produced a nonempty CA bundle, every common tool passed guest validation, the second prepare was an unchanged cache hit, and no harness, provider, or model work ran.
- `0054-nix-runtime-substrate-feasibility.md` — rejected; the candidate passed its 267.29-second cold ceiling and 0.01-second warm median, but its sealed owner-only schema-1 tree failed the first prepared project-layer mount before Tact launched.
- `0055-explicit-bind-permission-policy.md` — rejected; the control reproduced, but MicroSandbox's deliberate owner-access floor kept mirrored guest-finalized `0555`/`0444` entries writable on the host, so the consumer was not run.
- `0056-trusted-host-canonical-seal.md` — accepted; mirrored guest validation plus exact host write-bit removal produced canonical `0555`/`0444` modes, and a relaxed private read-only consumer executed and read the layer while rejecting mutation.
- `0057-sealed-schema-one-lifecycle.md` — superseded without product lifecycle evidence; the local run stopped before schema-1 publication, no warm or lifecycle sample ran, and exact cleanup completed.
- `0058-sealed-schema-one-lifecycle-rerun.md` — rejected; cold preparation passed in 200.95 seconds, but five unchanged prepares had a 5.14-second median because full sealed-layer reuse verification traversed the 1.2-GiB output, so lifecycle sampling did not run and exact cleanup completed.
- `0059-bounded-schema-one-lifecycle.md` — rejected; cold preparation passed in 186.27 seconds and five unchanged prepares had a 0.02-second median, but the first prepared absent launch stopped before Tact when the guest agent encountered a read-only filesystem during initialization.
- `0060-root-ca-bundle-lifecycle.md` — authorized; a real root-level CA bundle must first pass a manifest-free guest-agent boot and stop/start control before a fresh schema-1 lifecycle campaign may run.
