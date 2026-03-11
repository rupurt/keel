---
id: VDXBU7W4O
title: Application Service Refactor
mission: VDXqZtRef
created_at: 2026-03-10T22:36:20
---

# Application Service Refactor - PRD

## Problem Statement

The current application services (e.g., `StoryLifecycleService`, `VoyageEpicLifecycleService`) are directly coupled to filesystem operations. This coupling makes it difficult to use Keel in contexts without a traditional local filesystem (like a web server with a database backend) and hinders unit testing of the service logic without hitting the disk.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Decouple application services from `std::fs` and concrete loader functions. | Services can be initialized with any implementation of the Storage Ports. | 100% |
| GOAL-02 | Enable dependency injection for storage in all application services. | Unit tests use mock storage without disk access. | 100% |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keel Maintainer | Developer working on the core logic. | Faster, more reliable unit tests for service logic. |
| System Integrator | Developer embedding Keel into a server environment. | Ability to provide custom storage backends (SQL, Cloud Storage). |

## Scope

### In Scope
- [SCOPE-01] Refactoring of `src/application/*.rs` services to use Storage Ports.
- [SCOPE-02] Introduction of dependency injection patterns for service initialization.
- [SCOPE-03] Updating service methods to operate on abstract stores rather than `board_dir` paths.

### Out of Scope
- [SCOPE-04] Implementing custom non-filesystem backends.
- [SCOPE-05] Refactoring the CLI layer beyond what's needed to pass the new service dependencies.

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| FR-01 | Services must accept store implementations as dependencies (likely via traits). | Strategic | GOAL-01, GOAL-02 |
| FR-02 | Service logic must remain unchanged in terms of outcome while the underlying I/O is abstracted. | Strategic | GOAL-01 |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| NFR-01 | Refactoring must not introduce significant performance overhead compared to direct disk access. | Strategic | GOAL-01 |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Exhaustive unit testing of refactored services using mock stores.
- Integration testing using the existing `FileSystem` adapter to ensure CLI parity.

## Assumptions

| ID | Assumption | Impact |
|----|------------|--------|
| AS-01 | A trait-based approach with `Box<dyn Store>` or generics will be suitable for the DI needs. | High |

## Open Questions & Risks

| ID | Risk / Question | Mitigation |
|----|-----------------|------------|
| R-01 | Complex lifetimes or ownership issues when passing stores to services. | Use `Arc` or carefully design service ownership. |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Application services no longer contain direct `fs::` calls or `loader::load_board` calls.
- [ ] Existing integration tests pass using the refactored services.
<!-- END SUCCESS_CRITERIA -->
