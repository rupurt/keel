---
# system-managed
id: VGdx8jTUm
index: 5
status: accepted
decided_at: 2026-04-12T21:50:00
supersedes: []
superseded-by: null
# authored
title: Mission Stack Coordination Protocol
context: "multiplayer"
applies-to: ["crates/keel-core/src/read_model", "crates/keel-cli/src/cli/commands"]
mission: VGdx8iyUo
---

# Mission Stack Coordination Protocol

## Status

**Accepted** — Mission Stack is a federated, multi-board coordination contract with git-backed handoff and managed foreign-worktree execution.

## Context

Keel's existing turn loop, pacemaker, and hook boundaries are repo-local. That
works for ordinary single-board operation, but it does not define how work
should move when one outcome spans multiple repositories with their own Keel
boards and their own reactors.

Without a coordination contract, an operator can write directly into another
workspace checkout, bypass the target repo's board, blur branch provenance, and
leave multiple repos carrying ungoverned open-loop work at the same time.

Mission requests solve ingress into another board, but they do not yet define
the execution protocol after a request is accepted. We need a stack-level rule
set that preserves local board authority while still allowing cross-repo work to
flow under Keel turn discipline.

## Decision

Keel formalizes **Mission Stack** as a federated execution contract across
multiple Keel boards.

The protocol is:

1. A stack has a stable stack id and a shared branch name `stack/<id>` in each
   participating repository.
2. Each participating repository keeps its own Keel board and remains the only
   authority allowed to mutate that board directly.
3. Cross-repo work begins with a mission request to the target reactor rather
   than direct `.keel` mutation from another repo.
4. One reactor acts as the stack steward for coordination, while member repos
   may record local receipts and local mission lineage after negotiation.
5. Stack coordination uses explicit modes:
   - `exclusive(<repo>)` for one active member at a time
   - `shared([repos...])` for parallel work windows
   - `checkpoint(<name>, required_members...)` for integration gates that must
     be acknowledged before progress continues
6. Push is the inter-reactor handoff boundary. Local work is sealed by commit,
   then handed off to other reactors through a pushed receipt tied to stack id,
   branch, and head sha.
7. A pushed receipt may remain git metadata unless and until stronger artifact
   requirements prove necessary.
8. Foreign reactor execution in another member repo MUST happen inside a managed
   git worktree on that member's `stack/<id>` branch. Foreign work MUST NOT run
   inside the member repo's primary checkout.
9. Foreign worktrees persist for the stack lifecycle and are garbage-collected
   when the stack closes.
10. Repo-local heartbeat semantics remain repo-local. Aggregated Mission Stack
    readiness and gating are surfaced through stack-aware read models and CLI
    commands rather than by redefining `keel heartbeat`.

The coordination flow is therefore:

`local work -> seal -> push -> issue receipt -> negotiate -> foreign worktree turn -> seal -> push -> acknowledge/integrate`

## Constraints

- **MUST:** Treat Mission Stack as a federation of independent boards rather
  than a single shared board.
- **MUST:** Require mission-request plus reactor negotiation before another repo
  can take on stack-linked work.
- **MUST:** Use `stack/<id>` branches for stack member work in every
  participating repository.
- **MUST:** Treat commit as the local closure boundary and push as the
  inter-reactor handoff boundary.
- **MUST:** Keep repo-local heartbeat and pacemaker surfaces local to one repo.
- **MUST NOT:** Mutate another repository's `.keel` state directly from outside
  that repository's reactor.
- **MUST NOT:** Allow foreign reactor work to run in a member repo's primary
  checkout when a managed worktree is required.
- **SHOULD:** Surface stack mode, member state, receipts, and checkpoint gates
  in `turn`, `next`, `mission next`, and `doctor`.
- **SHOULD:** Garbage-collect managed foreign worktrees when the stack closes.
- **SHOULD:** Keep the first pushed-receipt contract git-native until stronger
  evidence needs are proven.

## Consequences

### Positive

- Cross-repo work keeps local board authority intact instead of bypassing the
  target reactor.
- Operators get a clear separation between local closure and remote handoff.
- Shared and checkpointed integration windows become an explicit policy surface
  instead of an improvised social convention.
- Managed worktrees isolate foreign execution from a member repo's primary
  checkout and make stack provenance auditable.

### Negative

- Stack execution becomes more governed and therefore more operationally complex
  than ad hoc cross-repo editing.
- Member repos need new read models, checks, and lifecycle plumbing before the
  protocol becomes fully enforceable.

### Neutral

- Repo-local heartbeat semantics do not change; Mission Stack adds a parallel
  coordination model rather than replacing the pacemaker contract.
- Stronger attestation or richer receipt artifacts remain future extensions,
  not day-one requirements.

## Verification

| Check | Type | Description |
|-------|------|-------------|
| Mission lineage | automated | `just keel doctor --status` reports nominal mission and ADR integrity with the new Mission Stack planning artifacts attached. |
| ADR compliance | manual | Mission Stack planning and implementation artifacts preserve local-board authority, `stack/<id>` branch naming, and the managed foreign-worktree rule. |
| Surface contract | manual | Future `turn`, `next`, `mission next`, and `doctor` work shows stack gating without changing repo-local `keel heartbeat` semantics. |

## References

- Mission: `VGdx8iyUo`
- Epic: `VGdxE0AFZ` — Federated Mission Stack Domain Model
- Epic: `VGdxDziFF` — Stack-Aware Turn And Queue Surfaces
- Epic: `VGdxE0lFe` — Foreign Reactor Worktree Execution Lifecycle
- Existing boundary docs: `PROTOCOL.md`, `ARCHITECTURE.md`, and `website/docs/foundations/keeper-boundaries-and-mission-requests.mdx`
