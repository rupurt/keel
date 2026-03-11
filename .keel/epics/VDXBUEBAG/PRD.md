---
id: VDXBUEBAG
title: Public Library API
mission: VDXqZtRef
created_at: 2026-03-10T22:36:20
---

# Public Library API - PRD

## Problem Statement

Keel is currently primarily a CLI tool. While the code is organized into layers, it is not optimized for use as a library. Public interfaces are often coupled to CLI-specific types (like `clap` types or raw paths) or are not sufficiently exported in `lib.rs`. We need a stable, well-documented public API that allows Keel's core engine to be embedded in other Rust projects.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Define a stable public interface in `lib.rs`. | An external crate can perform a full story lifecycle without using CLI commands. | 100% |
| GOAL-02 | Decouple core types from CLI dependencies. | Types used in the public API do not require `clap` or other CLI-only crates. | 100% |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Rust Developer | Developer building on top of Keel. | Predictable, type-safe API for board operations. |
| Harness Developer | Developer building agentic CI/CD tools. | Headless Keel engine for automated SDLC. |

## Scope

### In Scope
- [SCOPE-01] Refactoring `src/lib.rs` to export the necessary services and types.
- [SCOPE-02] Cleaning up public structs to remove CLI-specific fields.
- [SCOPE-03] Providing a "facade" or high-level API for common workflows (e.g., `Keel::load(store).story_start(id)`).

### Out of Scope
- [SCOPE-04] Rewriting the CLI logic (the CLI should become a thin client of the library).
- [SCOPE-05] Providing bindings for other languages (C, Python, etc.).

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| FR-01 | `lib.rs` must export application services and their required domain models. | Strategic | GOAL-01 |
| FR-02 | The library API must allow providing a custom `StoragePort` implementation. | Strategic | GOAL-01, GOAL-02 |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| NFR-01 | Public types must have consistent naming and documentation. | Strategic | GOAL-01 |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Creation of an `examples/` directory with a standalone Rust program using the library.
- Unit tests focusing on the stability and correctness of the public API surface.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The current layer separation is clean enough that `lib.rs` refactor is mostly about visibility. | May require deeper refactoring. | Architecture review. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How to handle global configuration (e.g. `keel.toml`) in library mode? | Architect | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A sample program can load a board, create a story, and submit it using only the library API.
- [ ] `cargo doc` produces clear documentation for the public API surface.
<!-- END SUCCESS_CRITERIA -->
