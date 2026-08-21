# Experiment 0047: short-root public Nix-provider rehearsal

Status: declared
Design: FIP-0012

## Baseline / Control

Experiment 0046 passed the complete `aarch64-darwin` runbook gate and its
side-effect-free initial plan, then failed before VM creation because its
disposable `HOME` produced a 111-byte MicroSandbox relay socket path above the
104-byte host limit. It executed no Nix command and supplied no provider
evidence.

The implementation remains revision `062a68c69026`. The sole public source is
`https://github.com/muesli/reflow` at
`83f63799117108248750298720ed34dd55d5201a`, with the same three declared setup
files and nixpkgs lock `f13ff45afd1bb73e640eaa08a7066dbed07e3238`.

## Hypothesis and Production Mechanism

Using exact short isolated roots eliminates only the pre-provider socket-path
control failure. Fortlet will then bootstrap digest-pinned Nix 2.35.2 in one
credential-free capsule, archive and resolve the locked public dev shell into
its project store, publish a bounded record, and remove provisioning capsules.
The unchanged second prepare will be a verified no-contact cache hit.

## Declared Scope

Use only `/tmp/f47` with checkout `/tmp/f47/r`, `HOME=/tmp/f47/h`,
`XDG_STATE_HOME=/tmp/f47/s`, `XDG_DATA_HOME=/tmp/f47/d`, and
`MSB_HOME=/tmp/f47/m`. Verify the root is absent before creation. Clone the
exact public revision afresh and add only the schema-2 manifest, locked flake,
and lockfile declared in Experiment 0046.

Run initial plan, `prepare tact`, identical second prepare, final plan, raw
inventory, and isolated-state inspection in that order. Freeze every source,
provider, target, harness, command, and network input from Experiment 0046.
No model, credential, harness launch, managed project capsule, project-source
edit, checkpoint, remote write, or publication is permitted.

## Alternatives

Resuming Experiment 0046 was rejected by experiment policy. Changing Fortlet
or the provider was rejected because neither ran far enough to fail. Using the
ordinary personal MicroSandbox root was rejected because the rehearsal must
keep created state exactly attributable and cleanable.

## Risks

The corrected root may expose a distinct installer, guest-user, mount, disk,
network, archive, structured-output, or activation failure. Any unit failure,
credential access, host repository execution, remaining experiment capsule,
undeclared path, or remote mutation rejects the successor. A second failure
sharing the socket-path assumption stops under the charter.

## Acceptance Criteria

1. The already-recorded complete gate remains applicable because no code or
   environment mechanism changed after it.
2. Initial plan is unprepared, redacted, and side-effect-free.
3. First prepare returns exactly `tact<TAB>ready`, creates a verified isolated
   store/record, and leaves no provisioning or managed project capsule.
4. Second prepare returns the same line without first-use output, VM/network
   contact, or record rewrite.
5. Final plan is prepared and prints activation names only.
6. Exact inspection precedes removal of `/tmp/f47`; raw inventory under the
   isolated MicroSandbox root is empty after settlement.

One failed criterion rejects this screen; no retry or resume is permitted.

## Budget and Plan

One provider unit, at most 45 minutes, zero model calls, and $0. This consumes
no additional model-process allowance. Stop on the first terminal failure,
hard-invariant anomaly, repeated socket-path cause, or need to alter FIP-0012.

## Rehearsal

Experiment 0046 records the complete green runbook gate and exact successful
initial plan at the unchanged code revision. The sole correction is shortening
the isolated control paths before any VM or provider operation.

## Results

Pending.

## Terminal Closure

Pending.
