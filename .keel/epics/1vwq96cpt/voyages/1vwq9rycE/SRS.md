# Consolidate Read Models And Queue Policies - Software Requirements Specification

> Unify flow, status, next, and capacity projections behind canonical read models and policies.

**Epic:** [1vwq96cpt](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

This voyage creates canonical projection services for operational views and removes duplicated metrics/capacity/policy logic spread across diagnostics and flow modules.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing outputs can be reproduced from canonical projections | assumption | output compatibility shim may be required |
| Projection services can consume current board model without format changes | dependency | additional model adapters needed |

## Constraints

- Command outputs for existing flows must remain semantically equivalent
- Policy thresholds must be centralized and reused by all consumers
- Read models must be side-effect free

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Provide a canonical flow/status projection service consumed by `flow`, `status`, and `next` decisions. | FR-05 | projection unit tests + command parity tests |
| SRS-02 | Provide a canonical capacity projection type and calculation used by both diagnostics and flow rendering. | FR-05 | type-level compile checks + parity tests |
| SRS-03 | Centralize queue policy classification and ensure all consumers use the same policy API. | FR-05 | policy integration tests |
| SRS-04 | Remove duplicate local metric structs/calculations in interface modules. | FR-05 | static analysis + command output tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Projection calculations must be deterministic and pure for repeatable diagnostics. | NFR-03 | property and snapshot tests |
| SRS-NFR-02 | Read-model APIs must be stable and documented for interface consumers. | NFR-04 | API docs + interface tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
