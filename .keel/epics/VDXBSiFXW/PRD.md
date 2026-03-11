---
id: VDXBSiFXW
title: Storage Port Definitions
mission: VDXqZtRef
created_at: 2026-03-10T22:36:20
---

# Storage Port Definitions - PRD

## Problem Statement

Current application services are tightly coupled to filesystem paths and direct file loading. This makes it impossible to swap the storage backend (e.g., for a server-side Keel) or embed Keel into other projects without bringing the entire filesystem dependency. We need a clean abstraction layer (traits) that decouples the domain and application logic from the persistence implementation.

## Goals & Objectives

| ID | Description |
|----|-------------|
| GOAL-01 | Define a `BoardStore` trait for aggregate board operations (load, save). |
| GOAL-02 | Define an `EntityStore<T>` trait for specific entity operations (stories, voyages, epics). |
| GOAL-03 | Ensure ports support both synchronous and asynchronous operations (if needed) or stay generic. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Developer | Rust developer embedding Keel as a library. | Stable traits to implement custom backends. |
| Keel Core | Maintenance of existing filesystem logic. | Clean separation of concerns. |

## Scope

### In Scope
- Definition of `BoardStore` trait.
- Definition of `EntityStore` traits for all entity types.
- Migration of `src/domain/model/board.rs` to use these traits for high-level operations.

### Out of Scope
- Implementation of the FileSystem adapter (handled in a separate epic).
- Implementation of an HTTP/Server adapter.

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| FR-01 | The `BoardStore` trait must provide methods to load a complete board snapshot. | Strategic | GOAL-01 |
| FR-02 | The `EntityStore` trait must provide CRUD operations for individual entity files. | Strategic | GOAL-02 |
| FR-03 | Traits must be defined in the `domain` or `application` layer, agnostic of `std::fs`. | Strategic | GOAL-01, GOAL-02 |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| NFR-01 | Traits must not leak implementation details like `PathBuf` where abstract identifiers are sufficient. | Strategic | GOAL-03 |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Unit tests for the trait definitions (compilation checks).
- Mock implementations to verify application service compatibility.

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Application services can be unit tested using a mock storage port.
- [ ] No `std::fs` calls remain in the core application logic for `VDXBSiFXW` scope.
<!-- END SUCCESS_CRITERIA -->
