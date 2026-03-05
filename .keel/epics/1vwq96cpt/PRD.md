# Domain Driven Design Restructure - Product Requirements

> Restructure keel into explicit DDD bounded contexts and layered architecture so subsystems can evolve independently and in parallel with verifiable contracts.

## Problem Statement

The current codebase models strong domain concepts, but implementation is still heavily command-centric. Business rules, orchestration, persistence, and read models are mixed across modules, which increases coupling, duplicates behavior, and makes parallel workstreams unsafe.

## Goals & Objectives

| Goal | Success Metric | Target |
|------|----------------|--------|
| Establish explicit domain boundaries | Bounded context map and ownership committed | 100% of core modules mapped |
| Enforce layered architecture | Layering contract tests in CI | Zero forbidden imports in main tree |
| Enable parallel subsystem execution | Context-scoped workstreams with clear seams | 5 parallel-safe context tracks |
| Reduce duplicate projection logic | Canonical read model services adopted | Single source for flow/status/capacity |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Core Maintainer | Maintains architecture and release health | Predictable module ownership and safe refactors |
| Implementer Agent | Executes stories in parallel | Stable contracts and minimal cross-context collisions |
| Human Reviewer | Reviews and accepts completed work | Clear traceability from requirements to implementation |

## Scope

### In Scope

- Bounded context definitions and ownership
- Layered module structure: domain, application, infrastructure, read-model, interface
- Migration of orchestration from command handlers to application services
- Consolidation of duplicated projections and policies
- Verification suites that guard architectural contracts

### Out of Scope

- Net-new product features unrelated to architecture migration
- Runtime behavior changes beyond what is required for boundary enforcement
- Replacing markdown board storage model

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Define bounded contexts and ownership rules for governance, work-management, research, verification, read-models, and interfaces. | must | Parallel safety depends on clear ownership boundaries. |
| FR-02 | Introduce explicit layered architecture contracts and enforce them with automated tests. | must | Prevents regression into cross-layer coupling. |
| FR-03 | Move command orchestration into application services and process managers with domain events. | must | Centralizes business workflows and cross-aggregate coordination. |
| FR-04 | Extract shared infrastructure services for persistence, frontmatter mutation, and template rendering. | must | Removes duplicated mutation logic and command-level filesystem coupling. |
| FR-05 | Consolidate flow, status, next, and capacity projections behind canonical read-model services. | must | Eliminates divergent operational views and duplication. |
| FR-06 | Refactor CLI handlers into thin adapters that invoke application use cases only. | should | Keeps interface layer stable and testable. |
| FR-07 | Deliver migration and regression coverage that verifies both behavior and architectural contracts. | must | Protects correctness during phased refactor. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Architectural contract checks must run in CI and fail builds on layer violations. | must | Architectural drift must be caught early. |
| NFR-02 | Refactor must preserve existing command semantics unless explicitly documented and approved. | must | Reduces rollout risk. |
| NFR-03 | New module seams must support independent testing per bounded context. | should | Enables safe parallel development throughput. |
| NFR-04 | Traceability from story AC to SRS and PRD requirements must remain intact across migration. | must | Preserves governance and acceptance model integrity. |
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

