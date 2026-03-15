# Canonical Serialization - SRS

## Summary

Epic: VDiHwGwe5
Goal: Ensure all entities serialize to a canonical, deterministic format.

## Scope

### In Scope

- [SCOPE-01] Deterministic YAML serialization for frontmatter.
- [SCOPE-02] Standardized newline and spacing for all `.keel` markdown files.
- [SCOPE-03] Validation of entity serialization order in tests.

### Out of Scope

- [SCOPE-04] Scoped regeneration (handled in a separate voyage).

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Frontmatter keys must be serialized in a fixed, alphabetical or semantic order. | SCOPE-01 | FR-01 | automated |
| SRS-02 | The body of a markdown file must be separated from frontmatter by exactly one blank line. | SCOPE-02 | FR-02 | automated |
| SRS-03 | Terminal newlines must be consistently present in all generated files. | SCOPE-02 | FR-02 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Serialization tests must cover all entity types (Mission, Epic, Voyage, Story, ADR, Routine). | SCOPE-03 | NFR-01 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
