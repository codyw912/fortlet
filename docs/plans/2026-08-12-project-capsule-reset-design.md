# Project capsule reset design

Status: validated by the operator on 2026-08-12.
Governing design: FIP-0001, FIP-0003, and FIP-0004.

## Purpose

Add the smallest safe recovery command for a disposable project+harness
capsule. Users should not need raw MicroSandbox names or `msb` commands when a
capsule has stale configuration or needs a fresh disposable root. Reset must
not become a hidden process terminator or a persistent-state purge.

## User contract

1. `fortlet reset <harness>` targets exactly one registered harness for the
   resolved project.
2. It accepts `--project PATH` and `--allow-broad-mount` with the same project
   semantics as run, status, and stop.
3. An absent capsule succeeds with `harness<TAB>absent` and creates no Fortlet
   state or lock.
4. An owned stopped or crashed capsule is removed and reported as
   `harness<TAB>reset`.
5. Created, starting, running, draining, and paused capsules fail without
   mutation and direct the user to `fortlet stop <harness>` before retrying.
6. Reset does not stop, kill, start, or attach a capsule.

## Ownership and deletion boundary

Reset derives the shared capsule descriptor used by launch, status, and stop.
After an initial lookup, it acquires the same per-capsule lock used by launch
and stop, fetches current state again, parses the stored configuration, and
verifies the deterministic name plus managed, schema, project, and tool
ownership labels. Version skew remains acceptable for recovery. Malformed
configuration or any ownership mismatch fails closed.

For an eligible terminal capsule, reset uses the MicroSandbox SDK handle's
remove operation. The SDK deletes the runtime record and disposable VM
directory. Reset does not remove the project, Fortlet's project+harness state
directory, credential projection, credential fingerprint, immutable base
layer, or immutable harness layer. A later run creates a fresh capsule around
the preserved state.

## Errors and recovery guidance

Active or transitional state uses one direct correction:

```text
fortlet: capsule stage failed; run `fortlet stop codex` and retry reset: capsule is running
```

Diagnostics name the public harness, not the internal capsule name or project
identity. Documentation explains that explicit project selection must be
repeated across launch, stop, and reset. Launch's stale-version correction
changes from raw `msb` removal guidance to `fortlet stop <harness>` followed by
`fortlet reset <harness>`. Unknown harness, project, lookup, lock, ownership,
configuration, and SDK removal failures remain nonzero and actionable.

## Implementation shape

Extend the existing local management seam with a reset decision and keep
project resolution, status mapping, ownership validation, and capsule locking
shared. The seam remains a narrow deterministic-test boundary around the sole
local MicroSandbox SDK, not a generic runtime abstraction.

Reset performs lookup before locking so absence remains side-effect free. Any
present target is fetched again under the lock before authorization or state
decision. Output rendering remains pure and tab-delimited.

## Verification

Deterministic tests cover absence without lock/state creation; stopped and
crashed removal; refusal for every other SDK state; ownership and version skew;
malformed stored configuration; lookup, lock, and removal failures; output;
CLI parsing; project and harness failures; and the revised launch diagnostic.
Existing launch, status, stop, shim, and native behavior remain green.

One predeclared local experiment uses a unique temporary project and one Codex
`--version` launch. It proves absent to running, running reset refusal, stop,
terminal reset, final absence, and idempotent second reset. A harmless sentinel
in the owned persistent harness-state directory proves preservation. The
experiment verifies the disposable runtime record is gone while the project,
sentinel, public credential projection, and immutable layers remain. It does
not inspect credential or fingerprint values. Exact ownership is checked
before cleanup, and all owned temporary artifacts are removed afterward.

## Completion boundary

Update recovery documentation and conformance with only reset evidence. Do not
add automatic stop, persistent-state purge, global inventory, raw-name
management, restart, logs, leases, CI, distribution, or publication. Run the
complete standard verification set, close the experiment, rewrite the handoff,
complete the mission, inspect `main..@`, then stop. Remote publication requires
separate operator authorization.
