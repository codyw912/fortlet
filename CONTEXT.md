# Fortlet domain language

## Project

A host workspace selected from a Jujutsu root, Git root, development
environment marker, explicit path, or safe scratch directory. A project has a
portable identity whose local-path derivation is an implementation detail.

## Harness

A supported coding-agent command such as Codex or Tact, together with its
pinned Linux installation, launch environment, persistent state, and
credential requirements.

## Environment layer

An immutable, versioned host directory prepared inside a temporary capsule and
mounted read-only into working capsules. Harness tools and shared Linux support
tools occupy separate layers.

## Capsule

A named MicroSandbox microVM assigned to one project and one harness. Multiple
sessions of that harness may attach concurrently and share its processes and
persistent state.

## Capsule session

The fail-closed launch transaction that resolves a project, validates
credentials, prepares environment layers, reconciles a capsule, and attaches a
harness to the terminal.

## Execution host

The trusted machine running MicroSandbox and its credential broker. It is the
laptop for local execution and could later be a self-hosted NixOS machine.

## Terminal client

The machine and terminal from which a user launches and interacts with a
harness. It is initially the same machine as the execution host.

## Workspace mode

The rule for where project files are canonical and how the execution host sees
them. Local execution uses a live bind mount; future remote execution must
choose an explicit synchronized or remote-canonical mode.
