# Domain Driven Design Restructure - Product Requirements


## Problem Statement

The current codebase models strong domain concepts, but implementation is still heavily command-centric. Business rules, orchestration, persistence, and read models are mixed across modules, which increases coupling, duplicates behavior, and makes parallel workstreams unsafe.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish explicit domain boundaries | Bounded context map and ownership committed | 100% of core modules mapped |
| GOAL-02 | Enforce layered architecture | Layering contract tests in CI | Zero forbidden imports in main tree |
| GOAL-03 | Enable parallel subsystem execution | Context-scoped workstreams with clear seams | 5 parallel-safe context tracks |
| GOAL-04 | Reduce duplicate projection logic | Canonical read model services adopted | Single source for flow/status/capacity |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Core Maintainer | Maintains architecture and release health | Predictable module ownership and safe refactors |
| Implementer Agent | Executes stories in parallel | Stable contracts and minimal cross-context collisions |
| Human Reviewer | Reviews and accepts completed work | Clear traceability from requirements to implementation |

## Scope

### In Scope

- [SCOPE-01] Bounded context definitions and ownership
- [SCOPE-02] Layered module structure: domain, application, infrastructure, read-model, interface
- [SCOPE-03] Migration of orchestration from command handlers to application services
- [SCOPE-04] Consolidation of duplicated projections and policies
- [SCOPE-05] Verification suites that guard architectural contracts

### Out of Scope

- [SCOPE-06] Net-new product features unrelated to architecture migration
- [SCOPE-07] Runtime behavior changes beyond what is required for boundary enforcement
- [SCOPE-08] Replacing markdown board storage model

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Define bounded contexts and ownership rules for governance, work-management, research, verification, read-models, and interfaces. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Parallel safety depends on clear ownership boundaries. |
| FR-02 | Introduce explicit layered architecture contracts and enforce them with automated tests. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Prevents regression into cross-layer coupling. |
| FR-03 | Move command orchestration into application services and process managers with domain events. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Centralizes business workflows and cross-aggregate coordination. |
| FR-04 | Extract shared infrastructure services for persistence, frontmatter mutation, and template rendering. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Removes duplicated mutation logic and command-level filesystem coupling. |
| FR-05 | Consolidate flow, status, next, and capacity projections behind canonical read-model services. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Eliminates divergent operational views and duplication. |
| FR-06 | Refactor CLI handlers into thin adapters that invoke application use cases only. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Keeps interface layer stable and testable. |
| FR-07 | Deliver migration and regression coverage that verifies both behavior and architectural contracts. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Protects correctness during phased refactor. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Architectural contract checks must run in CI and fail builds on layer violations. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Architectural drift must be caught early. |
| NFR-02 | Refactor must preserve existing command semantics unless explicitly documented and approved. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Reduces rollout risk. |
| NFR-03 | New module seams must support independent testing per bounded context. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Enables safe parallel development throughput. |
| NFR-04 | Traceability from story AC to SRS and PRD requirements must remain intact across migration. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Preserves governance and acceptance model integrity. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Enforce module boundary tests that fail on forbidden imports between domain/application/infrastructure/interface layers.
- Keep command-regression tests for `flow`, `status`, `next`, and lifecycle transitions to ensure behavior parity during refactor.
- Add read-model service tests that prove one canonical projection path is used by management commands.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing board workflows can be migrated incrementally without full freeze | Migration may need a hard cutover | Voyage-level regression suites and dual-path checks |
| Architectural tests can express critical boundaries without excessive maintenance overhead | Tests may become brittle and ignored | Introduce focused rules per boundary and keep them small |
| Current domain behavior is sufficiently captured by tests and doctor checks | Hidden behavior may regress | Add command-level regression and contract tests before deeper moves |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How strict should process-manager event ordering be during transition? | Architecture owner | open |
| Which legacy helpers can be retired first without breaking command UX? | Work-management owner | open |
| Can all projection consumers switch to canonical read-model APIs in one pass? | Read-model owner | open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Bounded context and layer contracts are implemented and enforced by automated tests.
- [ ] Cross-command orchestration paths are replaced by application services/process managers.
- [ ] Flow/status/next/capacity outputs are produced from canonical read-model services.
- [ ] CLI handlers are thin adapters and no longer own business orchestration.
- [ ] Regression and architecture verification suites pass for the migrated flows.
<!-- END SUCCESS_CRITERIA -->

