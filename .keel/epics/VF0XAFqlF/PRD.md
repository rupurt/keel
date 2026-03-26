# Simplify Speccy Architecture And Public API - Product Requirements

## Problem Statement

`speccy` already owns the reusable markdown template engine Keel extracted, but its public surface is broader than the stable boundary we actually need. The crate currently mixes catalog loading, token rendering, document shaping, frontmatter mutation, and tests in a single file, while also exposing multiple top-level helper combinations for hooks, mutations, catalog loading, and body-only rendering.

That structure works, but it makes the reusable boundary harder to reason about and raises the cost of future evolution. Every new rendering concern risks multiplying the top-level function set, and the single-file layout hides where catalog abstractions stop and document transforms begin.

We need a two-pass refactor that first makes the internal module boundaries explicit without changing behavior, then reduces the public API to a smaller core surface that other projects can adopt while Keel cuts over cleanly.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make `speccy`'s internal structure explicit and readable. | The crate is split into focused modules with `lib.rs` acting as the public re-export layer. | `speccy` module split landed |
| GOAL-02 | Reduce `speccy`'s rendering surface to a smaller stable API. | Rendering is exposed through core entrypoints plus options rather than a growing matrix of top-level helper combinations. | Keel uses the reduced surface |
| GOAL-03 | Preserve current Keel behavior while clarifying external extension points. | Existing Keel rendering and mutation flows still pass verification, and the reusable-vs-host boundary is documented. | Refactor lands without scaffold regressions |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keel Maintainer | Maintains `speccy` and the Keel adapters layered on top of it. | A crate layout and public API that are easier to change without accidental coupling. |
| Workspace Adopter | Wants to use `speccy` from another Rust project in the workspace. | A small stable surface for loading, rendering, and optional document transforms without Keel-specific assumptions. |
| Tooling Integrator | Needs to plug catalog loaders, token hooks, or post-processing into the renderer. | Explicit extension points that do not require depending on a large matrix of helper functions. |

## Scope

### In Scope

- [SCOPE-01] Split `speccy` into focused modules for catalog loading, hook definitions, rendering, and frontmatter operations while preserving current supported behavior.
- [SCOPE-02] Replace the current combinatorial render helper surface with a smaller public API built around core render entrypoints plus options.
- [SCOPE-03] Update Keel to consume the reduced `speccy` API while keeping Keel-specific inventory and adapters out of the reusable crate.
- [SCOPE-04] Document the module boundaries and the intended stable extension points for future non-Keel adopters.

### Out of Scope

- [SCOPE-05] Expanding the template language beyond the current double-curly placeholder model.
- [SCOPE-06] Replacing the frontmatter mutation algorithm with a full YAML parser.
- [SCOPE-07] Publishing `speccy` outside the workspace or adding a second production consumer in this voyage.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `speccy` must separate catalog loading, hooks, rendering, and frontmatter behavior into explicit modules with a thin `lib.rs` re-export boundary. | GOAL-01 | must | The first pass is only complete if the internal architecture becomes legible without changing semantics. |
| FR-02 | `speccy` must expose a reduced public rendering surface centered on core render entrypoints and option-driven transforms rather than multiple top-level helper combinations. | GOAL-02 | must | This prevents public API growth from tracking every combination of source, hooks, mutation, and body-only behavior. |
| FR-03 | Keel must consume the reduced `speccy` API for its existing render and frontmatter mutation paths without changing expected scaffold behavior. | GOAL-02 GOAL-03 | must | The new surface is only proven if the first consumer, Keel, actually depends on it. |
| FR-04 | The final docs must identify which extension points are intended to remain stable for host projects and which behavior remains an internal implementation detail. | GOAL-03 | should | Reuse depends on a clear public boundary, not just code movement. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The refactor must preserve deterministic output for current supported render and mutation workflows. | GOAL-01 GOAL-03 | must | Simplifying structure cannot change authored artifact behavior. |
| NFR-02 | The reduced public API must stay free of Keel-specific filesystem assumptions, board semantics, or crate dependencies. | GOAL-02 GOAL-03 | must | The crate remains reusable only if the boundary stays generic. |
| NFR-03 | `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and the relevant test suites must pass after the refactor. | GOAL-01 GOAL-02 GOAL-03 | must | The module split and API reduction must land with full verification. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Module split | `speccy` tests and code inspection | Story evidence for pass 1 |
| API reduction and Keel cutover | `speccy` and Keel tests plus compile verification | Story evidence for pass 2 |
| Boundary documentation | Planning artifacts and voyage report review | Story evidence for docs slice |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Keel is the authoritative first consumer for the reduced `speccy` surface. | The API could still be too broad or too narrow for broader reuse. | Validate by cutting Keel over completely during the second pass. |
| Current frontmatter mutation behavior is acceptable to keep as-is while the surface is simplified around it. | The voyage could uncover pressure to change semantics while refactoring architecture. | Cover representative mutation paths with tests and inspect current callers. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Is the current borrowed-callback hook model good enough for first external reuse, or does it need a follow-on ergonomics pass? | Epic owner | Open |
| Are there hidden callers relying on niche frontmatter mutation behavior such as dotted nested keys? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `speccy` is split into focused modules with a thin `lib.rs` re-export layer.
- [ ] Keel depends on a smaller `speccy` API centered on core render entrypoints plus options.
- [ ] The reusable boundary and stable extension points are documented clearly enough for another project to adopt `speccy`.
<!-- END SUCCESS_CRITERIA -->
