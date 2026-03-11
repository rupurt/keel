# Public Library Surface - SRS

## Summary

Refactor `src/lib.rs` to provide a stable public API for Keel. This will allow other crates to use Keel's domain and application logic without relying on the CLI.

## Scope

### In Scope
- [SCOPE-01] Refactoring `src/lib.rs` to export the necessary services and types.
- [SCOPE-02] Cleaning up public structs to remove CLI-specific fields.
- [SCOPE-03] Providing a "facade" or high-level API for common workflows.

### Out of Scope
- [SCOPE-04] Rewriting the CLI logic.
- [SCOPE-05] Providing bindings for other languages.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `lib.rs` must export the core application and domain modules. | SCOPE-01 | FR-01 | Compilation |
| SRS-02 | The library API must allow providing a custom `StoragePort` implementation. | SCOPE-01 | FR-02 | Compilation |
| SRS-03 | Core types exported in `lib.rs` must be decoupled from CLI-specific dependencies. | SCOPE-02 | FR-01 | Inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Public API must be sufficiently documented via doc comments. | SCOPE-01 | NFR-01 | `cargo doc` |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
