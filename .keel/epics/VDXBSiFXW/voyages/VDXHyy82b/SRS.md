# Core Storage Traits - SRS

## Summary

Define the fundamental `BoardStore` and `EntityStore` traits to enable the hexagonal architecture refactor. These traits will serve as the primary abstraction for all Keel persistence operations.

## Scope

### In Scope
- [SCOPE-01] Definition of `BoardStore` trait.
- [SCOPE-02] Definition of `EntityStore` traits for all entity types.
- [SCOPE-03] Migration of `src/domain/model/board.rs` to use these traits for high-level operations.

### Out of Scope
- [SCOPE-04] Implementation of the FileSystem adapter.
- [SCOPE-05] Implementation of an HTTP/Server adapter.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Define `BoardStore` trait with `load` and `save` methods for the aggregate Board. | SCOPE-01 | FR-01 | Unit test (mock) |
| SRS-02 | Define `EntityStore<T>` trait with CRUD methods (`get`, `list`, `put`, `delete`). | SCOPE-02 | FR-02 | Unit test (mock) |
| SRS-03 | Traits must be defined in a new `crate::domain::port` or similar agnostic module. | SCOPE-01, SCOPE-02 | FR-03 | Inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Traits must use abstract identifiers (IDs) rather than filesystem paths (`PathBuf`). | SCOPE-01, SCOPE-02 | NFR-01 | Inspection |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
