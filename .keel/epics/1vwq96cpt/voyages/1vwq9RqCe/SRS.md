# Extract Shared Infrastructure And Repositories - Software Requirements Specification

> Centralize filesystem persistence, frontmatter mutation, template rendering, and repository abstractions.

**Epic:** [1vwq96cpt](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

This voyage introduces infrastructure ports/adapters to centralize board persistence and reusable markdown/frontmatter services currently scattered across command modules.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing filesystem layout remains authoritative | dependency | repository adapters need broader migration |
| Shared frontmatter/template logic can be lifted without semantic drift | assumption | command behavior regressions may appear |

## Constraints

- Infrastructure services must remain compatible with existing `.keel` markdown format
- Domain and application layers cannot depend on `std::fs` directly
- Migration should preserve current command outputs

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Define repository interfaces for reading and writing board entities through explicit ports. | FR-04 | unit tests against trait contracts |
| SRS-02 | Move frontmatter mutation logic into a shared infrastructure service consumed by use cases. | FR-04 | unit tests + command regression tests |
| SRS-03 | Move template rendering helpers into a shared infrastructure service independent of story commands. | FR-04 | integration test across epic/voyage/story/bearing creation |
| SRS-04 | Provide filesystem adapters that implement repository and document services with parity to current behavior. | FR-04 | adapter integration tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Infrastructure APIs must be deterministic and mockable for application-layer tests. | NFR-03 | mock-based tests |
| SRS-NFR-02 | Adapter failures must return contextual errors (path + operation). | NFR-02 | error-path tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
