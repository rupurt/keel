# Routine Foundation - SRS

## Summary

First implementation voyage for the Routine entity. This voyage establishes the
canonical routine bundle, board/storage integration, and the minimal CLI
surfaces required to author and inspect recurring work blueprints.

## Scope

### In Scope

- [SCOPE-01] Define the routine bundle contract under `.keel/routines/<id>/`
- [SCOPE-02] Load and persist routines through board and filesystem adapters
- [SCOPE-03] Provide `keel routine new`, `keel routine list`, and `keel routine show`
- [SCOPE-04] Scaffold routine bundles with cadence, target scope, and blueprint narrative

### Out of Scope

- [SCOPE-90] Temporal due-state logic in `keel next`
- [SCOPE-91] Pulse materialization or scheduled-lane flow rendering

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Define a canonical routine bundle and entity contract that captures identity, cadence metadata, target scope, and authored blueprint content. | SCOPE-01 | FR-01 | unit test |
| SRS-02 | Extend board loading and filesystem persistence so routines are discovered and stored alongside existing entities. | SCOPE-02 | FR-02 | integration test |
| SRS-03 | Implement `keel routine new`, `keel routine list`, and `keel routine show` for authoring and inspection of routines. | SCOPE-03 | FR-03 | integration test |
| SRS-04 | Scaffold routine bundles so cadence fields, target scope, and blueprint narrative stay in one human-editable artifact. | SCOPE-04 | FR-04 | integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Routine loading and listing remain deterministic and safe when no routines exist on the board. | SCOPE-02 | NFR-01 | unit test |
| SRS-NFR-02 | Routine adoption does not require compatibility changes to existing story frontmatter or lifecycle contracts. | SCOPE-01,SCOPE-02 | NFR-02 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
