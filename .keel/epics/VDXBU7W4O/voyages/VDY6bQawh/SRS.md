# Dependency Injection for Services - SRS

## Summary

Refactor application services to use dependency injection for storage ports. This will decouple the core business logic from direct filesystem operations, enabling better testability and alternative storage backends.

## Scope

### In Scope
- [SCOPE-01] Refactor `StoryLifecycleService` to use `BoardStore` and `EntityStore<Story>`.
- [SCOPE-02] Refactor `VoyageEpicLifecycleService` to use `BoardStore` and `EntityStore<Voyage>/<Epic>`.
- [SCOPE-03] Update service instantiation in the CLI layer to inject concrete `FileSystem` adapters.

### Out of Scope
- [SCOPE-04] Implementing remote HTTP storage (handled in a future epic).

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `StoryLifecycleService` must accept `Arc<dyn BoardStore>` and `Arc<dyn EntityStore<Story>>` in its constructor or methods. | SCOPE-01 | FR-01 | Unit test (mock) |
| SRS-04 | Service methods should no longer take `board_dir: &Path` as an argument if it's only used for I/O. | SCOPE-01, SCOPE-02 | FR-01 | Inspection |
| SRS-02 | `VoyageEpicLifecycleService` must accept relevant stores as dependencies. | SCOPE-02 | FR-01 | Unit test (mock) |
| SRS-03 | CLI command handlers must initialize and inject the `FileSystemAdapter` into services. | SCOPE-03 | FR-01 | CLI regression test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Refactoring must maintain 100% behavioral parity with the current filesystem-based implementation. | SCOPE-01, SCOPE-02 | NFR-01 | CLI regression test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
