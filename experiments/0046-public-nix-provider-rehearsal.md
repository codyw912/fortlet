# Experiment 0046: public Nix-provider rehearsal

Status: declared
Design: FIP-0012

## Baseline / Control

Fortlet revision `062a68c69026` passes 101 unit tests, the complete Rust test
suite through the pre-runtime boundary, the isolated `plan` CLI tests, and
strict all-target/all-feature Clippy on `aarch64-darwin`. No real schema-2
provider capsule or project-scoped Nix store has run yet.

The sole public input is `https://github.com/muesli/reflow` at reviewed commit
`83f63799117108248750298720ed34dd55d5201a`. The checkout receives an
experiment-only schema-2 manifest and locked root flake using nixpkgs
`f13ff45afd1bb73e640eaa08a7066dbed07e3238`; no project source is changed for
this rehearsal.

## Hypothesis and Production Mechanism

The development Fortlet will bootstrap its digest-pinned Nix 2.35.2 binary
inside one credential-free MicroSandbox capsule, archive the read-only public
checkout into its isolated project store, resolve a scalar-only dev shell with
Go and Jujutsu, publish one bounded record, and remove the provisioning capsule.
A second `prepare tact` will verify the record and store without VM or network
contact because the declaration and resolved identity are unchanged.

## Declared Scope

Create one disposable checkout and isolated `HOME`, `XDG_STATE_HOME`, and
`XDG_DATA_HOME` beneath an operator-owned temporary directory. Add only:

- `.fortlet/environment.json` selecting `nix-dev-shell` `default`;
- `flake.nix` exposing Go and Jujutsu on `aarch64-linux` and `x86_64-linux`;
- the exact hand-pinned `flake.lock` described above.

Run the complete local verification gate first. Then run, in order:

1. `fortlet plan --project <checkout>`;
2. `fortlet prepare tact --project <checkout>`;
3. the same `prepare` a second time;
4. `fortlet plan --project <checkout>` again;
5. read-only raw MicroSandbox inventory and isolated Fortlet state inspection.

No model process, provider credential, personal identity, harness launch,
managed project capsule, project edit, local checkpoint, remote write, or
external-repository publication is permitted. The repository revision, Nix
version and digest, nixpkgs lock, target, provider, harness, commands, network
policy, and one-unit count are frozen.

## Alternatives

Mock-only evidence was rejected because it cannot validate the Nix installer,
Linux guest user, archive output, or MicroSandbox bind mounts. Running host Nix
was rejected because FIP-0012 requires guest-only evaluation. A model-backed
unit was deferred until this zero-model provider path settles.

## Risks

The installer may reject the numeric guest user, MicroSandbox may not support
the required `/nix` ownership or mount behavior, the Nix CLI shape may differ
from fixtures, or the pinned public input may exceed disk/network bounds. Any
provider-command failure, remaining provisioning capsule, credential access,
host execution of repository text, unexpected network input, or state outside
the isolated roots rejects the unit. One failure closes this experiment; it is
not retried or resumed.

## Acceptance Criteria

1. The complete runbook gate passes before dispatch.
2. Initial plan reports the public schema-2 selection as unprepared without
   creating state.
3. First prepare returns exactly `tact<TAB>ready`; prepared tools and activation
   are bound to the isolated store and no managed project capsule exists.
4. Second prepare returns the same line without a first-use message, VM
   contact, marker rewrite, or network activity.
5. Final plan reports prepared state and activation names without values,
   paths, internal identities, or personal data.
6. Raw inventory contains no experiment provisioning capsule after settlement.
   Exact isolated roots and the disposable checkout are inspected and removed.

Any failed criterion rejects the experiment terminally. This is one screen,
not a universal compatibility claim.

## Budget and Plan

One provider unit, at most 45 minutes of the accepted four-hour engineering
ceiling, zero model calls, and zero external money. Stop immediately on one
unit failure, a hard-invariant anomaly, undeclared remote mutation, or need to
change the accepted provider contract.

## Rehearsal

Deterministic schema parsing, activation filtering, redacted plan rendering,
side-effect-free broad-root handling, provider identity, narrow identity
configuration, stale-capsule detection, schema-1 compatibility, and strict
Clippy passed at revision `062a68c69026`. On 2026-08-21 the complete runbook
gate passed on `aarch64-darwin`: 101 unit tests and every enabled integration
test, formatting, strict all-target/all-feature Clippy, conformance, the Nix
package, and shim activation were green. The two stock-Codex compatibility
tests remained explicitly ignored because they require an external binary;
Nix emitted only the expected incompatible x86_64-linux omission.

## Results

Pending.

## Terminal Closure

Pending.
