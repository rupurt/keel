# Trusted Consumer Mission Request Scheduling - Product Requirements

## Problem Statement

The janitor transition is currently scoped too close to direct lifecycle
mutation. Keel needs a communication-first path where normalized external work
becomes authored ingress and only trusted consumers schedule or apply board
mutations.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Define a communication-first ingress contract for external work. | The planning slice shows how normalized ingress is persisted as authored demand before scheduling. | Contract accepted |
| GOAL-02 | Define who is allowed to schedule or mutate planning state. | The trusted-consumer boundary is explicit, replayable, and separate from provider acknowledgement or connector policy. | Boundary finalized |
| GOAL-03 | Align janitor and mission-request work under one multiplayer path. | The follow-on execution slice points janitor and connectors at staged ingress plus trusted scheduling instead of direct lane pulls. | Rollout slice ready |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keeper Runtime Owner | Owns provider polling, normalization, and acknowledgement transport. | A native Keel ingress and scheduling contract instead of direct lifecycle calls. |
| Keel Reactor Author | Owns process-manager and read-model automation inside Keel. | Reactor-visible demand and a single trusted scheduling boundary. |
| Human Operator | Oversees multiplayer automation and escalation. | Clear provenance, replay safety, and no hidden board mutation path. |

## Scope

### In Scope

- [SCOPE-01] Define the authored ingress record that carries normalized work, replay identity, and trust metadata before scheduling.
- [SCOPE-02] Define how Keel communication or application-reactor surfaces make staged ingress visible to other reactors.
- [SCOPE-03] Define the trusted-consumer boundary that alone may schedule work or invoke mission-request `apply`.
- [SCOPE-04] Define the first rollout split across `keel` and `spoke`.

### Out of Scope

- [SCOPE-90] Direct provider or connector mutation of `.keel` planning state.
- [SCOPE-91] Replacing conversational `ping` and `poke` with the planning workflow.
- [SCOPE-92] Implementing every provider beyond the GitHub-first example.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Specify the authored ingress contract that persists normalized request data, replay identity, trust metadata, and scheduling status before board mutation. | GOAL-01 | must | External work needs a Keel-native demand record before reactors can reason about it. |
| FR-02 | Specify how Keel-native reactors and read models become aware of staged ingress using communication or application-reactor mechanisms. | GOAL-01, GOAL-02 | must | Awareness must live in Keel rather than in provider-specific runtimes. |
| FR-03 | Specify the trusted-consumer boundary for scheduling and mission-request application. | GOAL-02 | must | Prevents every reactor or connector from mutating planning state. |
| FR-04 | Define the first cross-repo rollout slice for Keel staging and scheduling plus Spoke ingress and acknowledgement transport. | GOAL-03 | must | Makes the correction executable rather than theoretical. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Preserve deterministic replay and deduplication from provider revision through scheduling and mission-request apply. | GOAL-01, GOAL-02 | must | Trusted multiplayer ingress is unsafe if retries can create ambiguous demand or duplicate planning mutations. |
| NFR-02 | Keep conversational comms distinct from structured planning ingress. | GOAL-01, GOAL-03 | must | Free-form chat surfaces should not silently become a board-mutation path. |
| NFR-03 | Keep scheduling and mutation behavior provider-neutral after normalization. | GOAL-02, GOAL-03 | must | GitHub can be first without becoming the core trust model. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Communication boundary | Manual review against `PROTOCOL.md`, `ARCHITECTURE.md`, and prior mission-request contracts | Story evidence linked to the new SDD sections |
| Board readiness | `keel doctor --status`, `keel flow`, and `keel mission next --status` after activating the slice | Clean board status plus an actionable next step |
| Cross-repo split | Manual inspection of named `keel` and `spoke` surfaces | Story evidence covering rollout ownership |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The previously planned mission-request command family remains the right mutation boundary. | This epic would need re-planning around a different native Keel surface. | Re-check against the existing command-contract voyage during decomposition. |
| Keel communication mechanisms can carry typed ingress records without collapsing back into chat-only semantics. | The design would need a sibling queue rather than a comms-backed surface. | Validate in the rollout split and follow-on implementation stories. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the first ingress record live inside `.keel/inbox` with a richer schema or in a sibling typed queue? | Planner | Open |
| What authenticated claim marks a consumer as trusted enough to schedule or call `apply`? | Planner | Open |
| Should trusted consumers schedule only mission requests or also janitor maintenance prompts? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Define the staged ingress record and its trust and replay fields.
- [ ] Define how Keel reactors observe pending ingress.
- [ ] Define the trusted-consumer scheduling and apply boundary.
- [ ] Define the first rollout split across `keel` and `spoke`.
<!-- END SUCCESS_CRITERIA -->
