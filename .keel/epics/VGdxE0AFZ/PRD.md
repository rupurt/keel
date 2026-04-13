# Federated Mission Stack Domain Model - Product Requirements

## Problem Statement

Keel can describe mission requests, but it does not yet define a federated Mission Stack contract that links multiple boards, stack modes, reactor negotiation, and push-based handoff receipts.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Define a stable Mission Stack domain contract that links independent Keel boards without collapsing them into one shared planning model. | The protocol names stack identity, member roles, stack modes, and ownership boundaries with no conflicting board-authority rules. | Protocol contract approved |
| GOAL-02 | Define the push-based coordination and negotiation protocol between stack steward and member reactors. | Operators can describe the exact handoff sequence and required receipt fields from local seal through remote acknowledgment. | Handoff contract approved |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Stack Steward Reactor | The reactor coordinating one cross-repo outcome across multiple member repositories. | A canonical way to declare stack membership, route requests, and understand who may act next. |
| Member Reactor | The reactor that owns one participating repository in the stack. | A safe way to accept, materialize, and acknowledge stack-linked work without losing local board authority. |
| Human Operator | The human reviewing or steering a cross-repo integration effort. | Legible protocol stages and clear escalation points instead of hidden coordination state. |

## Scope

### In Scope

- [SCOPE-01] Define the Mission Stack identity model: stack id, shared `stack/<id>` branch convention, steward role, and member role.
- [SCOPE-02] Define the allowed stack modes: `exclusive`, `shared`, and `checkpoint`.
- [SCOPE-03] Define the coordination flow from local work, seal, and push through receipt, negotiation, foreign execution, and acknowledgment.
- [SCOPE-04] Define the minimum pushed-receipt metadata required for inter-reactor handoff.
- [SCOPE-05] Define how target reactors materialize local mission lineage after accepting stack work.

### Out of Scope

- [SCOPE-90] Command-surface rendering for stack state in `turn`, `next`, `mission next`, `doctor`, or `flow`.
- [SCOPE-91] Managed git worktree lifecycle and cleanup implementation details.
- [SCOPE-92] Non-git transport, attestation, or artifact schemes stronger than the initial git-backed receipt contract.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keel SHALL define Mission Stack as a federation of independent Keel boards rather than a shared multi-repo board. | GOAL-01 | must | The protocol must preserve local planning authority. |
| FR-02 | Keel SHALL define the steward/member coordination model, including the rule that cross-repo work begins with a mission request to the target reactor. | GOAL-01, GOAL-02 | must | Negotiation needs an explicit ownership and ingress boundary. |
| FR-03 | Keel SHALL define stack modes for exclusive work, shared work windows, and checkpoint gates. | GOAL-01 | must | Multi-repo coordination requires an explicit flow-control vocabulary. |
| FR-04 | Keel SHALL define the pushed-receipt contract using git-native fields sufficient for stack id, branch, head sha, and handoff identity. | GOAL-02 | must | Push is the handoff boundary and needs a minimal interoperable receipt. |
| FR-05 | Keel SHALL define the canonical stack coordination sequence from local work through remote acknowledgment. | GOAL-02 | must | Operators need one deterministic protocol instead of ad hoc conventions. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The domain model SHALL preserve repo-local board authority and repo-local pacemaker semantics. | GOAL-01 | must | Mission Stack must add coordination without redefining core single-repo invariants. |
| NFR-02 | The first receipt contract SHALL remain git-native unless stronger evidence needs are proven. | GOAL-02 | should | The initial protocol should stay simple enough to implement and adopt. |
| NFR-03 | The protocol SHALL remain compatible with future stronger audit or attestation layers without changing the basic handoff sequence. | GOAL-01, GOAL-02 | should | Future multiplayer guarantees should layer on rather than force a redesign. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Domain contract | ADR and PRD review | Mission Stack ADR plus epic planning artifacts |
| Protocol clarity | Manual walkthrough | A reviewer can trace the stack lifecycle from local seal to remote acknowledgment without ambiguous ownership |
| Regression boundary | Planning review | The contract leaves repo-local heartbeat and local board authority unchanged |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| One reactor can act as the stack steward without preventing member repos from recording local receipts or local mission lineage. | The coordination record may need a mirrored or fully replicated design. | Re-check during voyage planning. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the first pushed receipt live only in git branch/head metadata, or should commit trailers become required immediately? | Epic owner | Open |
| How much local stack state should member repos persist beyond mission lineage and receipt acknowledgment? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Mission Stack is defined as a federated coordination contract with steward/member roles and shared stack modes.
- [ ] The local-to-remote handoff sequence is documented clearly enough to plan voyages without reopening protocol ownership questions.
- [ ] The initial pushed-receipt contract is specified in git-native terms.
<!-- END SUCCESS_CRITERIA -->
