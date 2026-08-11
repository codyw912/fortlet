# Exact Codex exit parity

Status: validated with the operator on 2026-08-11.

## Outcome

Verify whether Fortlet's packaged Codex path preserves the exact termination
result of native Codex when both receive Codex's documented local `/exit`
command. The comparison uses the existing bounded PTY observer and changes no
Fortlet product behavior, public command, package contract, harness
configuration, or accepted architecture.

## Observer extension

The observer gains an explicit exit-command action alongside its existing
foreground-group `SIGINT` action. After the unchanged initial activity,
startup settling, resize activity, and fixed hold, the new action writes the
exact terminal bytes `/exit\r` to the observer-owned PTY. It emits an
`exit_command` structural event, discards raw screen content, and records the
child's exact exit code or signal within the existing bound.

A deterministic fixture accepts only the exact exit command and returns a
distinctive nonzero code. Focused tests prove exact byte delivery, action event
ordering, exit-status preservation, timeout cleanup, and unchanged signal-mode
behavior before the observer is used against either real UI.

## Live comparison

One predeclared experiment contains two ordered units. The operator launches
native Codex 0.147.0 and then the immutable packaged Codex shim from the same
marker-free external Fish shell. Both use the same observer build, working
directory, PTY dimensions, settling behavior, resize, 20-second hold,
`/exit\r` action, and 15-second exit bound. Each unit runs once without manual
input or retry.

The slash command is local control input rather than a model task. The observer
does not submit prompt text, inspect screen wording, retain raw UI bytes, or
request intentional inference. Ordinary Codex-managed state writes are
allowed, while shell, Fish, Nix, PATH-manager, and Codex-configuration changes
remain outside scope.

## Decisions and stopping rule

Matching exact exit codes and signals accept packaged exit parity. Any
termination or status mismatch records a packaged-path discrepancy without
authorizing repair. If native Codex does not reach or respond to the declared
action, exact native termination remains unestablished; the packaged unit is
still recorded, and neither unit is retried.

EOF, repeated signals, adaptive input, manual interaction, and raw screen-text
matching were rejected because they do not compare the documented explicit
exit action. After both results, the experiment closes terminally, conformance
is updated only with proven evidence, the mission closes, and work stops.
