# Lineage Validation - Software Requirements Specification

> GOAL-01

**Epic:** [1vzWfz000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-03] Add doctor validation for missing/invalid lineage references on laid bearings.
- [SCOPE-04] Emit actionable remediation commands and evidence in diagnostics for lineage failures.
- [SCOPE-05] Provide correction/migration support for previously laid or scaffolded legacy bearings.

### Out of Scope

- [SCOPE-01] Writing the initial `epic` lineage token during `keel bearing lay`.
- [SCOPE-02] Capturing brief success-criteria to epic-goal references.

## Assumptions & Dependencies

<!-- What we assume to be true; external systems, services, or conditions we depend on -->

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|

## Constraints

<!-- Technical, business, or regulatory limitations that shape the solution -->

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel doctor` MUST hard-fail when a laid bearing is missing an `epic` lineage field, with the bearing path and a remediation command in the diagnostic. | SCOPE-03 | FR-03 | automated test |
| SRS-02 | `keel doctor` MUST hard-fail when a laid bearing has invalid goal-lineage references, reporting the offending token and suggested fix. | SCOPE-03 | FR-03 | automated test |
| SRS-03 | User-facing CLI output for `keel bearing lay` and `keel bearing show` MUST render the resolved epic lineage token and goal references. | SCOPE-04 | FR-03 | automated test |
| SRS-04 | CLI messaging for invalid/unknown goal references MUST include the offending field and a suggested correction command. | SCOPE-04 | FR-03 | automated test |
| SRS-05 | An explicit migration/repair flow MUST exist for pre-contract bearings missing lineage metadata. | SCOPE-05 | FR-04 | automated test |
| SRS-06 | Show surfaces MUST render lineage-related fields without truncating unknown legacy values. | SCOPE-04 | FR-04 | automated test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Lineage validation failures MUST be deterministic and fail fast without best-effort fallbacks. | SCOPE-03 | NFR-01 | automated test |
| SRS-NFR-02 | Error messages for lineage failures MUST include artifact path, offending token, and correction command. | SCOPE-04 | NFR-03 | automated test |
| SRS-NFR-03 | Migration and doctor checks MUST scale linearly with board size without non-interactive regressions. | SCOPE-05 | NFR-04 | automated test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
