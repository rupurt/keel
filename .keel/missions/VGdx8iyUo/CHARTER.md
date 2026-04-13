# Federated Mission Stack Coordination Protocol - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define the federated Mission Stack coordination contract so multiple Keel boards can participate in one stack without collapsing into a shared board or bypassing mission-request boundaries. | board: VGdxE0AFZ |
| MG-02 | Define how stack membership, gating mode, and handoff state surface through the canonical turn and queue commands so operators know when they may act or must wait. | board: VGdxDziFF |
| MG-03 | Define the foreign-reactor execution contract that requires managed git worktrees on `stack/<id>` branches and garbage-collects those worktrees when the stack closes. | board: VGdxE0lFe |

## Constraints

- Each member repository keeps its own Keel board and remains authoritative for its own planning state.
- Cross-repo work starts with mission requests and reactor negotiation; one repo MUST NOT mutate another repo's `.keel` state directly.
- Foreign reactor execution in another repository MUST use a managed git worktree on that member's `stack/<id>` branch.
- Repo-local heartbeat and pacemaker semantics remain repo-local; aggregate coordination belongs to separate Mission Stack surfaces.
- Pushed handoff receipts may remain git metadata until stronger artifacts are justified by implementation needs.

## Halting Rules

- DO NOT halt while any MG-* goal lacks an attached epic with an authored PRD that preserves the agreed protocol direction.
- HALT when the Mission Stack ADR is accepted and the attached epics are planned cleanly enough to decompose into voyages without reopening the core protocol.
- YIELD to human before changing repo-local heartbeat semantics or requiring handoff evidence stronger than git-backed receipts by default.
