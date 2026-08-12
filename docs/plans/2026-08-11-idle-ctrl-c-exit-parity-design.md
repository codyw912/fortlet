# Idle Ctrl-C key exit parity

Status: validated with the operator on 2026-08-11.

## Outcome

Verify whether Fortlet's packaged Codex path preserves native Codex's exact
termination result when both receive one Ctrl-C terminal key at an idle, empty
composer. The comparison changes only the repository PTY observer. It does not
change Fortlet product behavior, public commands, package contracts, harness
configuration, or accepted architecture.

## Native interaction and source finding

The operator manually confirmed two contextual behaviors in native Codex
0.147.0: one Ctrl-C exits at an idle, empty composer; during an active
conversation, the first Ctrl-C interrupts work and the next exits.

Official tag `rust-v0.147.0`, peeled commit
`be6e8eac029b183056b7e4402879f15d2c85f61b`, matches that observation. Codex's
formal double-press quit shortcut has a one-second window but is disabled in
this version. The TUI first lets modals, history search, or composer text consume
Ctrl-C. Otherwise, active work is interrupted; with no cancellable work, one
Ctrl-C requests a shutdown-first exit.

This mechanism is different from Experiments 0003, 0004, and 0006, which sent
operating-system `SIGINT` to the process group. A PTY byte `0x03` in the TUI's
raw terminal mode is decoded as the Ctrl-C key handled by Codex.

## Observer extension

Add a Ctrl-C-key action beside the existing process-signal, atomic-exit, and
typed-exit actions. After initial activity, a 100-millisecond quiet settle,
resize activity, and a fixed 5-second unattended hold, it writes exactly one
`0x03` byte to the PTY master. It emits `ctrl_c_key`, discards raw screen
content, waits within the existing 15-second exit bound, and reports the exact
exit code or signal.

A deterministic fixture switches its slave terminal to raw mode, accepts only
one `0x03` byte, and returns code 23. Unit evidence records both the final byte
sequence and the individual write call so an accidental process signal, second
byte, or combined action cannot satisfy the test. Integration evidence proves
event order, code preservation, ignored-key timeout cleanup, and all existing
modes unchanged.

## Live comparison

One new predeclared experiment contains two ordered, zero-retry units. The
operator launches native Codex 0.147.0 and then the immutable packaged Codex
shim from the same marker-free external Fish shell. Both use the same observer
build, working directory, dimensions, settle behavior, resize, 5-second hold,
one Ctrl-C key byte, and 15-second exit bound.

No prompt is submitted and no intentional inference occurs. Ordinary Codex and
Fortlet state writes are allowed; raw UI capture, shell changes, Fish changes,
Nix changes, PATH changes, and Codex-configuration changes are outside scope.

## Decisions and stopping rule

Matching exact exit codes and signals accepts packaged idle Ctrl-C parity. The
source-established expected result is normal code 0 with no signal. Any
termination or status mismatch records a packaged-path discrepancy without
authorizing repair. If native Codex does not terminate normally, the automated
idle-readiness premise is rejected; the packaged unit is still recorded, and
neither unit is retried or adapted.

Double Ctrl-C, process signals, slash commands, EOF, manual live input,
adaptive timing, and raw screen matching are excluded. After both results, the
experiment closes terminally, conformance is updated only with proven evidence,
the mission closes, and work stops.
