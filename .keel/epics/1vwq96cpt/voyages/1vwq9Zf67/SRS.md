# Introduce Application Services And Process Managers - Software Requirements Specification

> Move orchestration out of command handlers into application use cases with explicit process managers.

**Epic:** [1vwq96cpt](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

This voyage centralizes command orchestration into application services and introduces process managers for cross-aggregate transitions (story->voyage->epic).

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing transition rules in state machines remain valid | assumption | use case APIs may need redesign |
| Domain events can be introduced incrementally | dependency | migration may require temporary dual orchestration |

## Constraints

- Interface layer (CLI) must not directly call other command handlers
- Cross-aggregate effects must be explicit and testable
- Use cases must preserve current user-facing behavior

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Implement application services for primary lifecycle actions (story, voyage, epic, bearing, adr) that own orchestration. | FR-03 | service unit tests |
| SRS-02 | Introduce domain events and process managers for cross-aggregate flows such as auto-start and auto-complete. | FR-03 | process-manager integration tests |
| SRS-03 | Refactor command handlers to delegate orchestration to application services only. | FR-06 | adapter tests + import contract checks |
| SRS-04 | Ensure transition enforcement policies are invoked through use-case orchestration paths. | FR-03 | policy-path tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Use-case APIs must support deterministic tests through dependency injection. | NFR-03 | mock-driven tests |
| SRS-NFR-02 | Process managers must emit observable outcomes for auditability. | NFR-04 | event/audit tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
