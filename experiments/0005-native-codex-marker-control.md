# Experiment 0005: Native Codex marker control

Status: declared
Design: FIP-0001 and FIP-0002
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0004 directly launched native Codex 0.147.0 under the repository PTY
observer. It emitted `started` and `activity_initial`, then closed the PTY while
startup output was settling, before resize or signal. The environment contained
the names `CODEX_THREAD_ID`, `CODEX_SANDBOX`,
`CODEX_SANDBOX_NETWORK_DISABLED`, and `CODEX_CI`; their causal role was not
tested.

The exact native identity remains frozen:

1. Canonical npm launcher:
   `/Users/cody/.local/share/mise/installs/npm-openai-codex/0.147.0/lib/node_modules/@openai/codex/bin/codex.js`,
   SHA-256 `134063e133f0b4244fa3b251acf973d4fe4b4aeeacbdc135211bf480f59f1477`.
2. Selected `aarch64-apple-darwin` binary, SHA-256
   `19c4f144c5226a9f17c58e6f0fa854843b0f77a6eb420f40e2745a12f10f5d37`.
3. Observer checkpoint `ad7fdd0e`.

## Hypothesis and Production Mechanism

The native Codex process closed early because it inherited outer-session
markers and detected or adapted to an already-running Codex context. Removing
only those four names should let the same native UI remain attached long enough
to reach resize and the fixed signal boundary.

The signal outcome is diagnostic rather than part of that mechanism. A timeout
after signal matches packaged Codex's first-interrupt behavior; an exit exposes
a packaged-path discrepancy. Another pre-signal close rejects the declared
marker mechanism.

## Declared Scope

Run one Codex unit from `/Users/cody/dev/fortlet`. Launch the exact npm entry
point above through the unchanged observer after removing only:

- `CODEX_THREAD_ID`
- `CODEX_SANDBOX`
- `CODEX_SANDBOX_NETWORK_DISABLED`
- `CODEX_CI`

The actual execution sandbox, `CODEX_MANAGED_*` launcher behavior, host
environment, project, observer, harness version, hashes, and protocol stay
frozen. Record only initial presence or absence of marker names, never values.

The observer protocol is:

1. Initial PTY: 80 columns by 24 rows.
2. Startup activity timeout: 90 seconds.
3. Startup output settles for 100 milliseconds before resize.
4. Resized PTY: 120 columns by 40 rows.
5. Post-resize activity timeout: 15 seconds.
6. Unattended hold: 20 seconds.
7. Signal: one `SIGINT` to the verified observer-owned process group.
8. Exit timeout: 15 seconds.

No input, model task, Fortlet invocation, package change, harness configuration
change, shell mutation, remote execution, publication, or cleanup outside the
observer-owned process group is in scope.

## Alternatives

1. Ask the operator to launch from an ordinary external terminal. Deferred
   because the marker-removal control is reproducible here and retains the
   workspace-only write boundary.
2. Invoke the native Mach-O binary without the npm entry point. Rejected because
   it changes the installed command path in addition to the marker environment.
3. Build a deterministic guest signal probe. Deferred because it tests SDK
   transport rather than the recorded native Codex startup blocker.

## Risks

1. The markers may not cause the early close. That rejects the hypothesis; it
   does not authorize a retry or another environment change.
2. Removing `CODEX_SANDBOX` could be mistaken for weakening isolation. The
   actual sandbox is enforced outside the child environment and remains active.
3. Native Codex may attempt metadata or update activity. No input or intentional
   inference is allowed, and the workspace-only write boundary stays enforced.
4. The observer intentionally discards raw PTY bytes, so another early close
   may remain behaviorally precise but textually unexplained.

## Acceptance Criteria

1. Exact hashes match and the four declared names are checked without reading
   values before dispatch.
2. The complete runbook gate, six observer tests, and deterministic fixture
   rehearsal pass with no product or observer change.
3. The marker-removal command contains exactly the four declared names and the
   unit receives no input bytes.
4. Reaching `signal` and timing out rejects a Fortlet-specific attribution for
   packaged Codex's first-interrupt behavior.
5. Reaching `signal` and `exited` records the exact status and establishes a
   packaged-path discrepancy without authorizing repair.
6. Closing before `signal` rejects the marker-removal mechanism and establishes
   no signal comparison.
7. The first matching outcome settles the unit. There is one unit, zero retries,
   and no adaptive environment, signal, input, or timeout change.
8. No raw UI output, environment value, prompt, credential value, paid quota,
   shell mutation, or unowned process control is observed.

## Budget and Plan

Budget: zero money, zero paid quota, zero prompts, zero remote mutation, one
unit, zero retries, and at most 10 minutes after dispatch begins. Total mission
engineering is capped at 30 minutes.

## Rehearsal

Pending before dispatch.

## Results

Pending.

## Terminal Closure

Pending.
