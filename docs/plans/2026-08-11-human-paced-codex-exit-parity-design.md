# Human-paced Codex exit parity

Status: validated with the operator on 2026-08-11.

## Outcome

Verify whether Fortlet's packaged Codex path preserves native Codex's exact
termination result when both receive the same source-established, human-paced
`/exit` interaction. The comparison changes only the repository PTY observer.
It does not change Fortlet product behavior, public commands, package contracts,
harness configuration, or accepted architecture.

## Source finding

The official Codex `rust-v0.147.0` source at peeled commit
`be6e8eac029b183056b7e4402879f15d2c85f61b` explains the two matching timeouts
in Experiment 0007. Codex treats plain characters arriving no more than 8
milliseconds apart as a paste burst. While that burst is active, Enter is added
as a newline instead of submitting the composer. The observer's atomic
`/exit\r` write therefore inserted pasted text and left no later Enter event to
dispatch the local command.

Codex's own composer regression test covers this exact rapid-characters-plus-
Enter behavior. Its human-like test helper sends characters separately, waits
the paste burst's recommended flush delay between them, and flushes pending
input. This is evidence for changing the observer protocol, not evidence for a
Fortlet defect.

## Observer extension

Add a typed-exit action beside the existing signal and atomic exit actions.
After the unchanged initial activity, startup settle, resize activity, and
fixed hold, it performs these writes in order:

1. one-byte `/`, then wait 20 milliseconds;
2. one-byte `e`, then wait 20 milliseconds;
3. one-byte `x`, then wait 20 milliseconds;
4. one-byte `i`, then wait 20 milliseconds;
5. one-byte `t`, then wait 20 milliseconds;
6. one-byte carriage return.

Twenty milliseconds conservatively exceeds Codex's 8-millisecond paste-burst
threshold and 9-millisecond recommended flush delay. The final wait after `t`
lets the last character leave burst consideration before Enter arrives. The
observer emits `typed_exit_command`, discards raw screen content, waits within
its existing exit bound, and reports the exact exit code or signal.

The atomic `observe-exit` mode remains unchanged so Experiment 0007 stays
reproducible. A deterministic typed fixture accepts the same final byte stream
and returns code 23. Unit evidence additionally records the individual write
chunks and requested delays, because a concatenated-byte assertion alone
cannot distinguish typing from an atomic paste.

## Live comparison

One new predeclared experiment contains two ordered, zero-retry units. The
operator launches native Codex 0.147.0 and then the immutable packaged Codex
shim from the same marker-free external Fish shell. Both use the same observer
build, working directory, dimensions, settle behavior, resize, 20-second hold,
typed-exit action, and 15-second exit bound.

The slash command is local control input, not a model task. The observer does
not submit prompt text, inspect screen wording, retain raw UI bytes, or request
intentional inference. Ordinary Codex-managed and Fortlet-managed state writes
are allowed; shell, Fish, Nix, PATH-manager, and Codex-configuration changes are
outside scope.

## Decisions and stopping rule

Both units returning exact code 0 with no signal accepts packaged typed-exit
parity. Any termination or status mismatch records a packaged-path discrepancy
without authorizing repair. If native Codex does not terminate normally, the
native interaction premise is rejected; the packaged unit is still recorded,
and neither unit is retried or adapted.

Atomic input, EOF, signals, manual interaction, adaptive pacing, and raw screen
matching are excluded. After both results, the experiment closes terminally,
conformance is updated only with proven evidence, the mission closes, and work
stops.
