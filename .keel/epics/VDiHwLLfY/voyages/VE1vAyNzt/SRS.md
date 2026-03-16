# Bearing Dependency Primitives - SRS

## Summary

Epic: VDiHwLLfY
Goal: Introduce depends_on field, doctor validation, and dependency-aware sort order for bearings

## Scope

### In Scope

- [SCOPE-01] Add `depends_on` field to BearingFrontmatter and parse during board load.
- [SCOPE-02] Validate dependency references in `keel doctor` (existence, acyclicity, no self-refs).
- [SCOPE-03] Factor dependency resolution state into bearing sort order in `bearing list` and `next`.

### Out of Scope

- [SCOPE-04] Automatic dependency inference from content.
- [SCOPE-05] Cross-entity dependencies or dependency graph visualization.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | BearingFrontmatter must deserialize an optional `depends_on: Vec<String>` field from bearing README.md. | SCOPE-01 | FR-01 | test |
| SRS-02 | `keel doctor` must flag dangling `depends_on` references (IDs that don't match any bearing) as errors. | SCOPE-02 | FR-02 | test |
| SRS-03 | `keel doctor` must flag cyclic dependency chains and self-references as errors. | SCOPE-02 | FR-02 | test |
| SRS-04 | `bearing list` must sort bearings with unresolved dependencies below bearings whose dependencies are all terminal. | SCOPE-03 | FR-03 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Dependency validation must scale linearly with the number of bearings. | SCOPE-02 | NFR-01 | test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
