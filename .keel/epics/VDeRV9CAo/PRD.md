# Simulation Kernel Core - Product Requirements

## Problem Statement

Keel already behaves like a deterministic simulation in several subsystems, but the architecture does not name that pattern directly. Cross-aggregate reactions, reference-time evaluation, and projection building are spread across application and read-model modules, which makes the system harder to explain and easier to drift as automation grows. This epic introduces a minimal simulation kernel that clarifies those internals without replacing the existing DDD and hexagonal architecture.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Introduce a minimal simulation vocabulary that makes pulse evaluation, reactors, and projections explicit inside the current architecture. | Core modules use the new vocabulary without changing user-facing workflow concepts | `process_manager`, temporal read models, and planning/read projections share the same architectural language |
| GOAL-02 | Centralize deterministic reference-time evaluation for time-aware behavior. | Temporal features stop reaching for ad hoc clock access and become easier to test | At least one canonical reference-time abstraction is reused by temporal evaluation paths |
| GOAL-03 | Make cross-aggregate automated responses explicit and composable. | Lifecycle chaining is expressed through named reaction units rather than one bespoke branching function | Process-manager behavior can be read and tested as explicit reactors |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Maintainer | Engineers evolving Keel’s workflow engine and board model. | A clearer internal execution model that reduces conceptual drift. |
| Harness Author | Agent or automation authors depending on predictable `next`, `flow`, and mission behavior. | Deterministic semantics that stay aligned across commands. |

## Scope

### In Scope

- [SCOPE-01] Introduce a small simulation-context abstraction for deterministic board evaluation at a reference instant.
- [SCOPE-02] Refactor cross-aggregate lifecycle orchestration into explicit reactor-style components.
- [SCOPE-03] Reuse shared projection inputs where `flow`, `next`, and temporal scheduling currently re-derive similar board state.

### Out of Scope

- [SCOPE-04] Replace DDD and hexagonal architecture with a new top-level doctrine.
- [SCOPE-05] Introduce a continuous event loop, daemon, or ECS-style entity model.
- [SCOPE-06] Rename stable user-facing CLI vocabulary around game terminology.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Provide a simulation-context abstraction that combines board state with an injected reference time for deterministic evaluation. | GOAL-01, GOAL-02 | must | Gives temporal and projection code one canonical frame of reference. |
| FR-02 | Refactor current process-manager lifecycle chaining into explicit reactor-style units that consume domain events and emit concrete actions. | GOAL-01, GOAL-03 | must | Makes cross-aggregate automation easier to extend and test. |
| FR-03 | Ensure at least one shared projection path is reused across `keel flow`, `keel next`, or mission steering instead of duplicating board-derivation logic. | GOAL-01, GOAL-02 | should | Proves the simulation vocabulary simplifies real read paths. |
| FR-04 | Preserve current CLI workflows, entity lifecycles, and mission semantics while introducing the new internal abstractions. | GOAL-01 | must | This epic extends the architecture rather than replacing it. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | New abstractions must preserve current layer boundaries so CLI adapters, domain rules, and infrastructure concerns do not bleed together. | GOAL-01 | must | Architectural clarity is the point of the refactor. |
| NFR-02 | Time-aware behavior must remain deterministic and testable with injected reference times and in-memory board fixtures. | GOAL-02 | must | Prevents temporal logic from becoming flaky or opaque. |
| NFR-03 | Migration slices must leave one canonical execution path in place after each step. | GOAL-01, GOAL-03 | must | Avoids dual-path complexity during the refactor. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Architecture boundaries | PRD/SDD review plus unit and integration tests | Story proofs showing no layer inversion and preserved workflow behavior |
| Reactor orchestration | Unit tests over event-to-action planning plus lifecycle smoke tests | Story-level evidence around process-manager replacement slices |
| Temporal evaluation | Deterministic tests with injected reference times | Proof that shared reference-time abstractions drive time-aware read paths |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The current architecture already contains enough repeated patterns to justify formalizing a simulation kernel. | The refactor may add vocabulary without reducing complexity. | Validate through the research bearing before decomposition. |
| The first implementation slice can be landed incrementally without redesigning the entire command runtime. | The epic could balloon into a sweeping rewrite. | Keep the first voyage narrowly scoped around named hotspots. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should `keel next` consume a simulation context directly or only derived projections? | Epic owner | Open |
| Could reactor terminology create confusion if the actual implementation stays very small? | Epic owner | Open |
| Is a dedicated ADR needed before the first implementation voyage, or can the bearing recommendation serve as the architecture contract? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Maintainers can point to one canonical reference-time abstraction used by time-aware evaluation paths.
- [ ] Cross-aggregate lifecycle automation is implemented through explicit reaction units instead of a single bespoke branching function.
- [ ] At least one shared projection path is used by multiple decision/rendering flows without changing user-facing semantics.
<!-- END SUCCESS_CRITERIA -->
