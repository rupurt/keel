---
id: VDXBUAn7a
title: FileSystem Storage Adapter
mission: VDXqZtRef
created_at: 2026-03-10T22:36:20
---

# FileSystem Storage Adapter - PRD

## Problem Statement

As we introduce Storage Ports to decouple the application logic, we must preserve the primary use case of Keel: operating on a local `.keel/` directory. We need a concrete implementation of the new storage traits that encapsulates the existing filesystem logic.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Implement `BoardStore` and `EntityStore` for the local filesystem. | Existing board data is correctly loaded and saved. | 100% |
| GOAL-02 | Encapsulate current `infrastructure/loader.rs` and `infrastructure/parser.rs` logic. | No direct loader calls outside the adapter. | 100% |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| CLI User | Traditional user of Keel on a local repo. | Zero change in experience or performance. |
| Developer | Developer refactoring Keel. | A reference implementation for custom adapters. |

## Scope

### In Scope
- [SCOPE-01] Creation of `src/infrastructure/storage/filesystem.rs`.
- [SCOPE-02] Migration of current loading/saving logic into the `FileSystemAdapter` struct.
- [SCOPE-03] Full support for the existing directory structure and frontmatter formats.

### Out of Scope
- [SCOPE-04] Changing the on-disk format.
- [SCOPE-05] Adding features like file locking or remote synchronization (at this stage).

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| FR-01 | The adapter must implement all Storage Ports defined in `VDXBSiFXW`. | Strategic | GOAL-01 |
| FR-02 | The adapter must handle file-not-found and other I/O errors gracefully. | Strategic | GOAL-01 |
| FR-03 | The adapter must encapsulate all `loader.rs` and `parser.rs` logic. | Strategic | GOAL-02 |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| NFR-01 | File operations must remain efficient to avoid slowing down CLI commands. | Strategic | GOAL-01 |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Integration tests that compare the adapter output with direct file reads.
- Regression testing of all CLI commands using the new adapter.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The existing loader logic is atomic enough to be moved without a total rewrite. | Refactor may take longer. | Incremental migration. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How to handle cross-aggregate transactions in a simple filesystem store? | Architect | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] All `BoardStore` and `EntityStore` trait methods are implemented for the local filesystem.
- [ ] Direct `loader.rs` usages are replaced by the adapter in the CLI wiring.
<!-- END SUCCESS_CRITERIA -->
