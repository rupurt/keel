# Filesystem Storage Implementation - SRS

## Summary

Implement the concrete `FileSystemAdapter` that fulfills the `BoardStore` and `EntityStore` traits. This will allow Keel to continue operating on local repos while using the new hexagonal architecture.

## Scope

### In Scope
- [SCOPE-01] Creation of `src/infrastructure/storage/filesystem.rs`.
- [SCOPE-02] Migration of current loading/saving logic into the `FileSystemAdapter` struct.
- [SCOPE-03] Full support for the existing directory structure and frontmatter formats.

### Out of Scope
- [SCOPE-04] Changing the on-disk format.
- [SCOPE-05] Adding features like file locking.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `FileSystemAdapter` must implement `BoardStore` for loading/saving entire board state. | SCOPE-01 | FR-01 | Integration test |
| SRS-02 | `FileSystemAdapter` must implement `EntityStore<T>` for all Keel entities. | SCOPE-01 | FR-01 | Integration test |
| SRS-03 | The adapter must encapsulate existing logic from `loader.rs` and `parser.rs`. | SCOPE-02 | FR-03 | Inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | File operations must remain efficient to avoid slowing down CLI commands. | SCOPE-03 | NFR-01 | Regression test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
