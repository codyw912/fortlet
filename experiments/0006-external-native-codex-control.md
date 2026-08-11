# Experiment 0006: External native Codex control

Status: declared
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiments 0004 and 0005 both launched native Codex 0.147.0 inside the active
Codex runner. Each emitted `started` and `activity_initial`, then closed during
startup settling before resize or signal. Removing four inherited runner marker
names in Experiment 0005 did not change the result.

The frozen native identity is:

1. Canonical npm launcher:
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js`,
   SHA-256 `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477`.
2. Selected `aarch64-apple-darwin` binary, SHA-256
   `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37`.
3. Observer checkpoint `ad7fdd0e`.

## Hypothesis and Production Mechanism

The active Codex runner context, rather than the four known marker names alone,
caused the native Codex UI to close before signal. Launching the same command
from an ordinary external terminal should let it remain attached through resize
and reach the fixed first-interrupt boundary.

## Declared Scope

The operator runs one unit from `/Users/cody/dev/fortlet` in a separate Fish
shell. A name-only check reported `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
`CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI` all absent. The exact live
command is:

```fish
target/debug/examples/pty_observer observe \
    /Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js \
    /Users/cody/dev/fortlet \
    20
```

The observer protocol remains 80 by 24, 100-millisecond settle, resize to 120
by 40, 15-second resize-activity bound, 20-second unattended hold, one
observer-owned foreground-group `SIGINT`, and 15-second exit bound.

The operator authorizes ordinary Codex-managed state writes for this one
native launch. No shell startup-file, Fish configuration, Nix configuration,
PATH-manager, Fortlet, package, observer, harness configuration, remote,
publication, or unowned-process mutation is in scope. No raw PTY bytes, input,
prompt, adaptive action, or retry is allowed.

## Alternatives

1. Add another sandbox around the control. Rejected because it changes the
   external native context being tested and can reintroduce sandbox markers.
2. Redirect Codex state to a temporary home. Rejected because it changes the
   installed native configuration and authentication context.
3. Build a deterministic guest signal probe. Deferred because it answers SDK
   transport, not native Codex first-interrupt semantics.

## Risks

1. Normal native startup may write Codex-managed session, history, log, helper,
   or state files. The operator explicitly authorizes those ordinary writes for
   this unit; configuration outside Codex-managed state remains unauthorized.
2. Native Codex may still close before signal. That rejects the external-context
   mechanism and does not authorize a retry.
3. Automatic metadata activity may occur. No prompt or intentional inference is
   submitted, and any credential output or paid activity terminates the unit.
4. The operator might press a key or interrupt. Any such input invalidates the
   unit and still consumes it; there is no replacement run.

## Acceptance Criteria

1. Both hashes, unchanged product and observer paths, the complete gate, and
   fixture rehearsal pass before the command is issued.
2. The operator's external shell has the four declared marker names absent.
3. The operator runs the exact command once, supplies no input, and returns its
   complete structural output verbatim.
4. Reaching `signal` and timing out rejects a Fortlet-specific attribution for
   packaged Codex's first-interrupt behavior.
5. Reaching `signal` and `exited` records exact status and establishes a
   packaged-path discrepancy without authorizing repair.
6. Closing before `signal` rejects the external-context mechanism and supplies
   no signal comparison.
7. No raw UI output, environment value, prompt, credential value, paid quota,
   shell or declarative-environment mutation, or unowned process control occurs.
8. The first result settles the sole unit; there are zero retries or adaptive
   changes.

## Budget and Plan

Budget: zero money, zero paid quota, zero prompts, one operator unit, zero
retries, and at most 10 minutes after dispatch begins. Mission engineering is
capped at 30 minutes.

## Rehearsal

Completed on 2026-08-11 before issuing the operator command:

1. The operator reported all four declared runner marker names absent from the
   external Fish shell using the name-only check.
2. The launcher and selected native-binary SHA-256 hashes matched their frozen
   values exactly.
3. A Jujutsu path diff from `ad7fdd0e` showed no product, observer, fixture,
   package, manifest, or lockfile change, and Experiment 0006 was the only
   declared experiment.
4. All six observer tests passed. The deterministic fixture emitted the full
   sequence with signal 2, 7 initial bytes, and 13 resized bytes.
5. The full Rust suite, formatting, strict all-target Clippy, explicit
   conformance, exact-tree `nix flake check`, and `nix run . -- doctor` passed.
   The flake retained the known app-`meta` warning and incompatible
   `x86_64-linux` omission; doctor printed no credential values.

## Results

Pending operator output.

## Terminal Closure

Pending.
