# Speccy Foundation And Keel Integration Pilot - SRS

## Summary

Epic: VEzIwU3fh
Goal: Land a reusable speccy crate boundary with generic markdown template rendering hooks and cut Keel over to it without keeping Keel-specific logic in the reusable crate.

## Scope

### In Scope

- [SCOPE-01] Add a new workspace crate named `speccy` that owns generic placeholder rendering and markdown document helpers equivalent to the current reusable behavior in `template_rendering.rs`.
- [SCOPE-02] Expose host integration hooks for template lookup and optional post-render processing so consumers can provide their own catalogs and transforms without importing Keel modules.
- [SCOPE-03] Migrate Keel's current template rendering call sites onto `speccy`, keeping embedded template inventory and project-specific markdown mutations at the adapter boundary unless they prove generic enough to extract.
- [SCOPE-04] Document the reusable boundary, Keel-owned concerns, and deferred extraction candidates after the pilot cutover.

### Out of Scope

- [SCOPE-05] Adding a richer template DSL beyond today's double-curly token substitution contract.
- [SCOPE-06] Shipping a second production consumer outside Keel during this voyage.
- [SCOPE-07] Extracting all board-specific markdown processing into `speccy` without proving it is generic.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The current pure rendering helpers, `render` and `render_body`, are the right minimal extraction seam for `speccy`. | Design assumption | The voyage may need to widen scope into adjacent markdown utilities before Keel can cut over cleanly. |
| Host-specific behavior such as embedded template catalogs and frontmatter mutation can be modeled as integration hooks or adapter composition without polluting the public `speccy` API. | Architectural assumption | The crate boundary may remain too Keel-specific or force a wider public API. |
| Existing Keel tests over scaffold generation are sufficient to prove migration parity for the first cutover. | Internal dependency | Additional command-level regression tests would be needed before landing the migration. |

## Constraints

- `speccy` must not depend on `keel-core`, `keel-cli`, or `.keel` file layout assumptions.
- The hard-cutover policy applies to the migrated Keel paths: once callers adopt `speccy`, the old generic renderer should not remain active in parallel.
- Placeholder syntax remains the current double-curly token form for this voyage unless a deliberate design decision expands it.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `speccy` must expose deterministic double-curly token rendering plus markdown document helpers equivalent to the current generic Keel behavior for full-document rendering and body-only rendering. | SCOPE-01 | FR-01 | `cargo test -p speccy` |
| SRS-02 | `speccy` must expose host integration hooks for template lookup and optional post-render processing without importing Keel-specific types, file paths, or board concepts. | SCOPE-02 | FR-02 | `cargo test -p speccy` |
| SRS-03 | Keel call sites that currently use `template_rendering::{render, render_body, render_with_mutations}` must consume `speccy` for the generic rendering portion and keep only host-specific adapter composition outside the crate. | SCOPE-03 | FR-03 | `cargo test -p keel` |
| SRS-04 | Voyage artifacts and story scopes must document the final reusable boundary, Keel-owned responsibilities, and any deferred extraction candidates such as generic frontmatter mutation. | SCOPE-04 | FR-04 | docs inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | `speccy` must remain independent of Keel crates and keep its public API free of Keel-specific concepts. | SCOPE-01 SCOPE-02 | NFR-03 | `cargo test -p speccy` |
| SRS-NFR-02 | Representative Keel scaffold-generation paths must retain deterministic output after the cutover to `speccy`. | SCOPE-03 | NFR-02 | `cargo test -p keel` |
| SRS-NFR-03 | Host integration hooks must support embedded or caller-managed template catalogs without hard-coding filesystem assumptions into `speccy`. | SCOPE-02 | NFR-03 | `cargo test -p speccy` |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
