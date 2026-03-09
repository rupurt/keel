# Lineage Persistence - Software Requirements Specification

> GOAL-01

**Epic:** [1vzWfz000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Persist a durable `epic` lineage token on every laid bearing.
- [SCOPE-02] Capture and persist machine-readable goal links from `BRIEF.md` success criteria during `keel bearing lay`.

### Out of Scope

- [SCOPE-03] Strict doctor diagnostics, remediation messaging, and failure hardening.
- [SCOPE-04] Legacy-bearing correction flow and migration behavior.
- [SCOPE-05] Changes to artifact layout outside lineage payloads and links.

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
| SRS-01 | `keel bearing lay` MUST write the destination epic ID into a durable `epic` frontmatter field on the bearing README. | SCOPE-01 | FR-01 | automated test |
| SRS-02 | The persisted epic lineage value MUST be the canonical epic ID for both newly created and pre-existing epics. | SCOPE-01 | FR-01 | automated test |
| SRS-03 | `keel bearing lay` MUST parse `BRIEF.md` Success Criteria and persist validated goal references as a machine-readable frontmatter field. | SCOPE-02 | FR-02 | automated test |
| SRS-04 | Invalid or unknown goal references during lay MUST produce a deterministic validation error before any write occurs. | SCOPE-02 | FR-02 | automated test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Lineage writes during `keel bearing lay` MUST be idempotent for repeated operations on the same bearing. | SCOPE-01 | NFR-02 | automated test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
