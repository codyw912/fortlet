# Experiment 0026: Credential-free non-interactive attribution

Status: accepted — terminal 2026-08-19
Design: active GOAL deliverable 0; FIP-0001, FIP-0002, FIP-0008, FIP-0009
Charter scope: `local-foundation/v1`

## Authorization and Scope

The active operator-accepted GOAL predeclared credential-free local fixtures
before any provider request, exact command/outcome retention, and source-first
attribution across Fortlet, MicroSandbox 0.6.8, and Codex 0.147.0. This record
retains that already-authorized zero-provider diagnostic; it does not
retroactively claim a separate dispatch approval.

The diagnostic used one temporary network-disabled sandbox named
`fortlet-diag-noninteractive-20260819`, the existing prepared Codex 0.147.0 and
base-tool layers mounted read-only, an ephemeral guest home, and only in-guest
loopback fixtures. No host credential, broker secret, external provider,
repository mutation, or paid request entered the sandbox.

## Hypotheses

1. If a short shell command exits with exact output and status, the generic
   MicroSandbox non-TTY completion path is intact.
2. If stock Codex exits against local failure and success fixtures, its
   completed non-interactive shutdown and MicroSandbox's exit event are intact.
3. If a fixture that accepts but never answers keeps Codex alive until an
   explicit timeout, Experiment 0024 observed a live upstream wait rather than
   a lost normal exit event.
4. If collected mode hides progress while streaming mode exposes it, Fortlet's
   output collection explains the otherwise identical silence.

## Commands and Results

After creating the named sandbox from `node:24-bookworm` with `--pull never`,
`--no-net`, the prepared layers, and the Fortlet guest PATH:

1. `msb exec --no-tty ... -- sh -c 'printf control-stdout; printf
   control-stderr >&2; exit 7'` returned the exact two byte strings and host
   exit status 7.
2. `msb exec --no-tty ... -- /opt/fortlet/tool/bin/codex --disable apps
   --version` returned `codex-cli 0.147.0` and status 0.
3. Stock `codex --disable apps exec --ephemeral --ignore-rules
   --skip-git-repo-check ...` against an in-guest HTTP 500 Responses fixture
   printed its configuration and terminal error, and exited status 1.
4. The same command against a minimal successful SSE fixture printed
   `local-fixture-ok` and exited status 0.
5. The same command against a fixture that accepted the request and never
   answered remained alive. Collected `msb exec --no-tty --timeout 6s` returned
   only `error: exec timed out after 6s` and status 1.
6. Repeating that local hang with `msb exec --stream --timeout 6s` exposed the
   Codex startup/configuration transcript and `fixture-hang-stream-request`
   before returning the same timeout and status 1.

The exact temporary sandbox was then stopped and removed. `msb list` had been
empty before creation; no Fortlet-owned capsule was created or changed.

## Source Attribution

Fortlet's non-terminal branch uses MicroSandbox `exec_with`, then writes the
collected buffers only after it returns. MicroSandbox 0.6.8 uses null stdin by
default and its collector returns only on `ExecExited`; an externally stopped
sandbox closes the stream without that event and produces the error recorded
by Experiment 0024. Fortlet's renewal future is selected alongside attachment
and is canceled after attachment returns, so it cannot keep attachment open.

Codex 0.147.0 remains in its exec event loop until the turn completes or fails.
Its provider defaults allow a five-minute stream idle wait and five stream
retries. The local hanging fixture directly observed the Codex process remain
alive while awaiting a response.

## Terminal Closure

Accepted. Normal and failed Codex exec sessions produce explicit MicroSandbox
exit events. Experiment 0024's process was still alive, not exited and lost;
Fortlet's collected-output choice hid all progress while it waited. The owned
correction requires streaming plus a bounded non-interactive policy, so
FIP-0010 must be accepted before production implementation.

Actual cost was one temporary network-disabled sandbox, five local command
controls, zero credentials, zero external requests, zero paid quota, and no
repository production-code change. The sandbox was stopped and removed.
