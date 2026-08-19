# Experiment 0033: Global owned-capsule inventory

Status: accepted — terminal 2026-08-19
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
passed again against the frozen treatment before live dispatch: all 88 unit
tests and every enabled integration test, formatting, strict Clippy,
conformance, and `nix flake check` on aarch64-darwin. Nix emitted only the known
incompatible `x86_64-linux` omission notice. The exact immutable package is
`/nix/store/72qrzh32iaxcvhvi97p8bi1gsj41gmqr-fortlet-0.1.0`.

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

The exact frozen package built successfully at
`/nix/store/72qrzh32iaxcvhvi97p8bi1gsj41gmqr-fortlet-0.1.0`. Before dispatch,
the complete standard gate passed as recorded above, the Jujutsu working copy
was empty, packaged `fortlet list` printed exactly `no capsules`, and the
packaged raw MicroSandbox inventory printed `[]`.

The sole synthetic-auth packaged Tact command ran from `/private/tmp`, exited
zero in about 7.3 seconds, and printed only its pinned local version evidence:

```text
tact 0.3.7
commit: f03a9e323b7a (unknown, clean)
commit timestamp: 2026-08-07T09:05:20-04:00
build timestamp: 2026-08-07T13:09:28+00:00
target: aarch64-unknown-linux-gnu
profile: release
rustc: rustc 1.97.1 (8bab26f4f 2026-07-14)
```

There was no stderr, preparation, network, credential, provider, or model
diagnostic. The one declared packaged inventory observation, also run from
`/private/tmp`, printed exactly:

```text
/Users/cody/dev/fortlet<TAB>tact<TAB>running
```

Using that listed project and harness, packaged public cleanup then reported:

```text
tact<TAB>running
tact<TAB>stopped
tact<TAB>reset
tact<TAB>absent
no capsules
[]
```

The last two lines are packaged Fortlet inventory and packaged raw
MicroSandbox inventory respectively. The fixed synthetic auth document was
deleted after cleanup, and the Jujutsu working copy remained empty. No retry,
second capsule, real credential read, provider request, model prompt, network
diagnostic, guest repository edit, direct capsule mutation, unrelated cleanup,
or live-unit repository mutation occurred.

## Terminal Closure

Accepted. The immutable production package consumed the real pinned SDK list
shape, reconstructed and validated the Fortlet-owned project+harness capsule,
and rendered its canonical project path and running state from outside the
project. That path selected the unchanged project-scoped public cleanup
surface, which restored exact absence and empty raw inventory.

Actual cost was one exact-revision package build, one synthetic auth document,
one Tact version launch lasting about 7.3 seconds, one owned capsule, one global
present-state observation, one public cleanup sequence, zero retries, zero real
credentials, zero provider requests, zero paid quota, and zero money. FIP-0011
may become conformant; the next action is goal closure, the final complete gate,
and PR #10 readiness.
