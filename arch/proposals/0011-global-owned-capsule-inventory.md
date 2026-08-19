# FIP-0011: Global owned-capsule inventory

Status: Accepted
Recorded: 2026-08-19 from the operator-selected lifecycle direction
Requires: FIP-0001, FIP-0003, FIP-0004

## Summary

Add `fortlet list` as a read-only global inventory of Fortlet-owned
project+harness capsules. It enumerates the complete managed MicroSandbox
collection, reconstructs and validates each capsule's ownership from stored
configuration, and reports its project path, harness, and lifecycle state in a
stable safe order.

The listed project path feeds the existing explicit `status`, `stop`, and
`reset --project` workflow. This proposal does not add raw-name management,
bulk removal, automatic cleanup, or workload-lease policy.

## Motivation

Project-scoped status, stop, and reset are safe once an operator already knows
the project and harness. Repeated daily use across repositories makes that
precondition fragile: a stopped or crashed capsule can remain present after the
operator changes directories, and direct MicroSandbox inventory exposes
Fortlet's internal names without applying its ownership checks.

The operator has already needed to find and reset old stopped capsules. More
harnesses will multiply the number and variety of project+harness capsules, so
global visibility is useful before adapter expansion. Visibility should arrive
before automatic lifetime or bulk-deletion semantics because Fortlet does not
yet represent background workload leases.

## Decision

Provide one global observation command, `fortlet list`. It uses
MicroSandbox's label-filtered paginated list API, validates every candidate as
a complete Fortlet-owned capsule, and only then renders the complete result.

The canonical project root is recovered from the same-path project bind mount
required by FIP-0001. Its exact bytes must reproduce the stored project
identity and deterministic capsule name together with the stored harness. This
lets inventory remain useful if the project path later disappears without
adding a second mutable registry or making the identity hash a public selector.

Existing project-scoped management remains the only mutation surface. For a
listed root that still resolves, the operator passes that path to `status`,
`stop`, and `reset`. A missing root is reported honestly but cannot be removed
through a new global escape hatch in this slice.

## Specification

### Command and scope

1. Fortlet MUST provide:

   ```text
   fortlet list
   ```

2. List MUST be global to the current local MicroSandbox backend and effective
   user. It MUST NOT resolve or require a current project, accept a harness or
   project selector, or apply broad-root selection.
3. List MUST NOT read provider credentials, prepare or verify environment
   layers, create Fortlet state or persistent harness directories, create,
   start, stop, connect, modify, or remove a capsule, or attach a terminal.
4. Unrelated MicroSandbox capsules MUST be excluded through the exact
   `fortlet.managed=true` label filter and MUST NOT influence output.
5. List MUST consume every SDK page until the terminal cursor. A repeated or
   invalid cursor, page failure, or implementation inventory bound MUST fail
   the capsule stage rather than return a partial inventory.

### Ownership and project reconstruction

1. Every returned candidate MUST be parsed from its stored `SandboxConfig`
   before any output is written.
2. The stored name and labels MUST identify the effective user,
   `fortlet.managed=true`, the supported Fortlet schema, a syntactically valid
   harness name, and a syntactically valid project identity.
3. The stored mounts MUST contain exactly one writable bind whose canonical
   project host path is mounted at the identical absolute guest path required
   by FIP-0001. Fortlet-owned persistent-home and immutable-layer mounts MUST
   not be mistaken for the project mount.
4. Hashing the recovered project path with the current project-identity
   algorithm MUST reproduce the stored `fortlet.project` value. The effective
   user, stored harness, and reproduced identity MUST in turn reproduce the
   exact stored capsule name.
5. Missing, malformed, duplicate, contradictory, or ambiguous ownership data
   MUST fail the complete command before output. `fortlet.version` and the
   project-environment label remain diagnostic and MUST NOT reject an otherwise
   owned historical capsule.
6. Ownership reconstruction MUST use the stored path without requiring that it
   still exists. List MUST NOT canonicalize, open, or otherwise access the
   project directory.

### Output and ordering

1. Each capsule row MUST contain the recovered project path, one tab, the
   harness, one tab, and the lowercase lifecycle state:

   ```text
   <project><TAB><harness><TAB><state>
   ```

2. Project and harness presentation MUST escape tabs, carriage returns,
   newlines, non-printing bytes, and terminal control characters so one capsule
   always occupies exactly one line. Ordinary printable UTF-8 project paths
   MUST remain recognizable.
3. States MUST retain FIP-0003's exact `created`, `starting`, `running`,
   `draining`, `paused`, `stopped`, and `crashed` names.
4. Rows MUST sort first by the recovered project-path bytes and then by harness
   name. Runtime enumeration order MUST NOT affect output.
5. An empty valid inventory MUST print exactly `no capsules` followed by a
   newline and succeed.
6. Normal output MUST NOT expose internal capsule names, project identity
   hashes, configuration bodies, state paths, layer paths, credentials, secret
   placeholders, process identifiers, or runtime database details.

### Existing cleanup path

1. This FIP MUST NOT change `status`, `stop`, or `reset` syntax, project
   resolution, ownership, state decisions, locking, persistence, or output.
2. A listed project path that still resolves MAY be passed explicitly to the
   existing commands:

   ```text
   fortlet status <harness> --project <project>
   fortlet stop <harness> --project <project>
   fortlet reset <harness> --project <project>
   ```

3. List MUST NOT print generated shell commands or promise that an escaped
   display path can be evaluated as shell input. The operator supplies the
   actual path through the normal argument boundary.
4. A listed root that no longer resolves MUST remain visible. Global removal,
   management by project identity or raw capsule name, and cleanup of missing
   roots require a successor contract.

### Evidence

1. Automated evidence MUST cover all pages, stable ordering, empty inventory,
   every lifecycle state, project-path reconstruction, exact ownership,
   historical version tolerance, unrelated-capsule exclusion, malformed and
   ambiguous candidate rejection, safe rendering, and failure without partial
   output.
2. Isolated CLI evidence MUST prove list does not require `HOME`, a current
   project, credentials, layer state, or persistent harness state and performs
   no lifecycle mutation.
3. One predeclared bounded local experiment MUST use an immutable package to
   create at most one credential-free owned capsule, observe it through
   `fortlet list` from outside its project, and use the listed project and
   harness with packaged public stop/reset until both public status and global
   inventory report absence.
4. The live experiment MUST begin and end with empty MicroSandbox inventory,
   verify exact ownership before mutation, make no provider or model request,
   and never silently retry.

## Consequences

Operators can see Fortlet's complete local capsule footprint without learning
MicroSandbox naming or scanning unrelated runtime state. Existing explicit
project-scoped mutation stays narrow, so visibility does not create a new bulk
destructive authority.

Reading stored mounts becomes part of inventory compatibility. The same-path
project bind is already a FIP-0001 contract, and recomputing identity and name
prevents an arbitrary managed label from being accepted alone. Paths require
safe rendering because Unix path bytes are not necessarily printable UTF-8.

Projects that have moved or been deleted can be identified but not cleaned up
through Fortlet in this slice. That limitation is preferable to exposing raw
capsule names or designing a global selector before real inventory evidence.

## Alternatives Considered

1. Add `status --all`. Rejected because `status` without a harness already
   means all registered harnesses for one resolved project; overloading `all`
   would blur project and global scope.
2. Add global reset or prune together with inventory. Deferred because a bulk
   mutation needs stale-snapshot, partial-failure, concurrent-attachment, and
   workload-lease semantics that are not required for read-only visibility.
3. Expose the internal capsule name or project identity as a selector. Rejected
   because prior lifecycle FIPs deliberately keep runtime identity out of the
   user contract and future execution modes need portable project identity.
4. Maintain a separate Fortlet registry mapping project identities to paths.
   Rejected because it introduces atomic cross-store updates, drift, migration,
   and garbage collection when the stored capsule already contains the
   FIP-0001 same-path project mount.
5. Read the MicroSandbox database directly. Rejected because it bypasses the
   typed SDK and couples Fortlet to private storage details.
6. List only currently registered harnesses. Rejected because inventory must
   remain useful across Fortlet upgrades and future adapter removal; complete
   structural ownership is stronger than current-registry membership.

## Open Questions

1. Whether inventory evidence justifies exact global cleanup for projects that
   no longer exist.
2. How explicit interactive and background workload leases should constrain
   stop, removal, expiry, and grace periods.
3. Whether a future structured-output mode should cover inventory and all
   project-scoped management commands together.
4. Whether harness expansion needs version or adapter-support state in the
   inventory output.
