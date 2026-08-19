# Experiment 0025: Interactive Codex Apps disable confirmation

Status: accepted — terminal 2026-08-19
Design: FIP-0001, FIP-0002, FIP-0008, and FIP-0009
Charter scope: `local-foundation/v1`

## Baseline / Control

Experiment 0024 is terminally rejected and must not be resumed. Its immutable
package emitted none of the known Apps diagnostics, but its sole
non-interactive `codex exec` attempt produced no model response before the
ten-minute bound. Experiment 0023 already proved the packaged interactive
attachment and ordinary model path.

This successor keeps Experiment 0024's exact treatment revision
`a98288dff08db53e34127e51b27c817078cf20b5` and immutable aarch64-darwin
package `/nix/store/d2qnnmsfvbchf4vz1j9j8iwngbiy97q8-fortlet-0.1.0`.
Only the attachment mode changes.

## Hypothesis and Production Mechanism

The adapter-owned `--disable apps` argument will suppress only the reserved
Apps client during the already-proven interactive packaged path. The ordinary
ChatGPT model request will still authenticate and answer, with no Apps 451 or
MCP-startup-incomplete warning.

## Declared Scope

1. Reconfirm the empty working copy, exact treatment revision and package,
   unchanged project-input hashes, public Codex absence, and empty
   MicroSandbox inventory.
2. From `/Users/cody/dev/fortlet`, start exactly one interactive packaged
   launch:

   ```text
   fortlet run codex --project /Users/cody/dev/fortlet --
   ```

3. Submit exactly one prompt: `Return exactly fortlet-apps-disabled-ok without
   using tools.` Record the response and every startup diagnostic.
4. Attempt normal exit only after the response. Regardless of UI exit behavior,
   use the packaged public status, stop, and reset commands to require final
   absence and an empty MicroSandbox inventory.

No App invocation, tool call, repository edit, replacement prompt, retry,
second session, credential-content read, native launch, remote mutation,
settings change, release, or package publication is in scope.

## Alternatives

1. Repeat non-interactive `codex exec`. Rejected because Experiment 0024
   already settled that exact unit negatively.
2. Treat the missing warning alone as success. Rejected because the GOAL also
   requires preserved ordinary model behavior.
3. Ask for an edit/test loop. Rejected because Experiment 0023 already supplied
   that evidence and this experiment changes only Apps startup handling.

## Risks

1. Host authentication or the provider may fail independently. Record the
   first result and clean up without retry or credential inspection.
2. Automated UI exit may remain unreliable. Treat model completion as separate
   from exit mechanics and use the public lifecycle commands for cleanup.
3. Stop/reset may fail. Report the exact owned artifact and do not remove an
   uncertain capsule through private means.

## Acceptance Criteria

1. The frozen revision, package, inputs, public absence, and empty inventory
   match before launch.
2. The interactive UI starts without `codex_apps`, HTTP 451,
   `no_biscuit_no_service`, or MCP-startup-incomplete diagnostics.
3. The sole prompt returns exactly `fortlet-apps-disabled-ok` through ordinary
   ChatGPT model authentication.
4. Public stop/reset restores absence and MicroSandbox reports no sandbox.
5. The first failure, invariant anomaly, cleanup completion, or ten elapsed
   minutes after dispatch terminates the experiment.

## Budget and Plan

One newly operator-authorized model prompt, at most one bounded host OAuth
refresh, one interactive session, one owned capsule, no retry, and at most ten
elapsed minutes after dispatch. No other external spend or paid quota is
authorized.

## Rehearsal

Experiment 0023 accepted the same interactive package/shim path with ordinary
model authentication. Experiment 0024 froze the current treatment, passed the
complete deterministic and package gates, and observed the Apps warning absent
from the non-interactive startup. The only unrehearsed combination is the
current Apps-disabled package with the interactive prompt.

## Results

Immediately before dispatch, the working copy was empty, treatment revision
`a98288dff08db53e34127e51b27c817078cf20b5` and the immutable package matched,
both project-input hashes retained their declared values, the packaged public
CLI reported `codex<TAB>absent`, and the pinned MicroSandbox CLI reported no
sandboxes.

The one interactive packaged launch opened Codex 0.147.0 in
`/Users/cody/dev/fortlet` with no `codex_apps`, HTTP 451,
`no_biscuit_no_service`, or MCP-startup-incomplete diagnostic. The frozen
prompt was submitted once and returned exactly:

```text
fortlet-apps-disabled-ok
```

No tool was invoked and no repository file changed. One Ctrl-C at the idle
composer shut Codex down normally. The packaged public CLI then reported
`codex<TAB>running`, `codex<TAB>stopped`, `codex<TAB>reset`, and finally
`codex<TAB>absent`; the pinned MicroSandbox CLI reported no sandboxes.

## Terminal Closure

Accepted. The adapter-owned Apps disable removed the known startup warning
while preserving the ordinary interactive ChatGPT model path. The rejected
Experiment 0024 was an attachment-mode failure: changing only to the previously
proven interactive path produced the exact model response and normal UI exit.

Actual cost was one operator-authorized model prompt, one interactive session,
one owned reusable capsule, no tools, no repository edit, no retry, and public
stop/reset cleanup. The attempt completed within the declared ten-minute bound;
exact elapsed time and whether a host refresh occurred were not separately
observed. Mark FIP-0009 conformant and complete the GOAL's single-PR publication
path.
