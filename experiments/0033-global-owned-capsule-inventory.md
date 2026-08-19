# Experiment 0033: Global owned-capsule inventory

Status: active — predeclared 2026-08-19
Design: FIP-0001, FIP-0003, FIP-0004, and FIP-0011
Charter scope: `local-foundation/v1`

## Baseline / Control

The treatment implementation is frozen at revision
`866007214df6b45030a4bfae71e89a4083679ff8`. It adds the read-only packaged
`fortlet list` surface without changing project-scoped status, stop, or reset.
The focused deterministic gate passed before declaration: five inventory unit
tests, all five management-failure integrations, conformance, formatting, and
strict all-target/all-feature Clippy.

The complete standard gate passed on merged `main` before goal selection and
will run again against the frozen treatment before live dispatch. The package
store path, clean working copy, public empty result, and raw empty MicroSandbox
inventory must be recorded before the treatment unit begins.

## Hypothesis and Production Mechanism

An immutable package from the frozen treatment will enumerate every page of
MicroSandbox capsules filtered by `fortlet.managed=true`, reconstruct the
project root from each same-path bind, validate its project identity and
effective-user capsule name, and render the complete owned inventory in stable
project/harness order. The project path it reports will select the existing
packaged status, stop, and reset path for exact cleanup.

## Declared Scope

1. Build one immutable aarch64-darwin package from the frozen revision and
   record its store path before dispatch.
2. Require a clean working copy, packaged `fortlet list` output exactly
   `no capsules`, and packaged raw MicroSandbox inventory exactly `[]`.
3. Create one fixed temporary synthetic ChatGPT auth document at
   `/private/tmp/fortlet-exp0033-auth.json`. It contains a locally generated
   non-provider JWT, fake refresh string, and zero account identifier. It must
   never be printed or sent to a provider.
4. From `/private/tmp`, run exactly one packaged credential-free local command:

   ```text
   FORTLET_AUTH_FILE=/private/tmp/fortlet-exp0033-auth.json \
     <package>/bin/fortlet run tact \
     --project /Users/cody/dev/fortlet -- --version
   ```

5. Run packaged `fortlet list` from `/private/tmp` exactly once after launch.
   It must report `/Users/cody/dev/fortlet<TAB>tact<TAB>running`.
6. Use the same packaged public binary with the exact listed project path to
   run status, stop, reset, and final status in that order. Then run packaged
   `fortlet list` and raw packaged MicroSandbox inventory once each.
7. Delete only the fixed synthetic auth document after cleanup.

No Codex launch, real credential read, provider request, model prompt, project
edit, direct capsule mutation, retry, second capsule, unrelated runtime state,
bulk cleanup, remote mutation, or repository mutation by the live unit is in
scope. Existing immutable layers and persistent Tact state are preserved.

## Alternatives

1. Accept deterministic pagination and ownership tests alone. Rejected because
   the product claim includes the pinned SDK list response and real stored
   MicroSandbox configuration shape.
2. Create a capsule directly with `msb`. Rejected because the treatment must
   observe and clean a capsule created through Fortlet's production descriptor.
3. Use Codex. Rejected because its launch requires a real renewable host login;
   local Tact `--version` is the established zero-provider capsule fixture.
4. Add global reset for the experiment. Rejected because existing project-
   scoped cleanup is the accepted narrow mutation boundary.

## Risks

1. The pinned SDK may return stored configuration or pagination fields that
   differ from deterministic fixtures. One mismatch rejects the unit without a
   fallback parser or retry.
2. A stale capsule would invalidate the empty baseline. Stop before dispatch;
   do not clean it under this experiment identity.
3. Tact preparation may require unexpected network work. Any network or
   preparation diagnostic rejects this zero-network unit before continuation.
4. The synthetic token could reach a provider if argument handling changes.
   Any request diagnostic, unexpected delay, or output beyond local version
   information terminates the unit and triggers only declared public cleanup.

## Acceptance Criteria

1. Frozen revision, immutable package, clean working copy, complete green gate,
   public `no capsules`, and raw `[]` are recorded before dispatch.
2. The sole Tact command emits the pinned local version, exits zero, and creates
   exactly one owned running capsule without provider or network diagnostics.
3. Packaged global inventory from outside the project reports exactly the
   canonical project path, `tact`, and `running` on one line.
4. Packaged project-scoped status, stop, reset, and final status report running,
   stopped, reset, and absent; final global inventory reports `no capsules` and
   raw inventory is `[]`.
5. The repository remains unchanged by the live unit, the synthetic document
   is removed, and any failed criterion settles the experiment without retry.

## Budget and Plan

Zero money, zero paid quota, zero provider requests, one immutable package, one
synthetic auth document, one Tact version launch, one owned capsule, one global
observation while present, one explicit public cleanup sequence, zero retries,
and at most two elapsed minutes after dispatch. Build, complete gate, baseline,
dispatch, observe, clean up, and close in that order.

## Rehearsal

The deterministic implementation consumes multiple fake pages, requires the
exact managed-label query, sorts adversarial order, maps every SDK state,
rejects page and cursor failure without a returned result, reconstructs and
validates complete ownership, escapes control and invalid path bytes, and runs
an empty CLI inventory with no `HOME`, project, credentials, or Fortlet state.
No VM or provider has run under this declaration.

## Results

Pending.

## Terminal Closure

Pending.
