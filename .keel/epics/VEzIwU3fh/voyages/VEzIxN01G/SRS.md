# Speccy Foundation And Keel Integration Pilot - SRS

## Summary

Epic: VEzIwU3fh
Goal: Land a reusable speccy crate boundary with generic markdown template rendering hooks, first-class template catalogs, generic frontmatter mutation, and a Keel cutover without leaking Keel-specific logic into the reusable crate.

## Scope

### In Scope

- [SCOPE-01] Add a new workspace crate named `speccy` that owns generic placeholder rendering, markdown document helpers, and generic frontmatter mutation equivalent to the current reusable behavior in `template_rendering.rs` plus `frontmatter_mutation.rs`.
- [SCOPE-02] Expose first-class template catalog/loading abstractions and fallible host hooks for template lookup or optional post-render processing so consumers can provide their own catalogs and transforms without importing Keel modules.
- [SCOPE-03] Migrate Keel's current template rendering and frontmatter mutation call sites onto `speccy`, keeping embedded template inventory and project-specific board semantics at the adapter boundary.
- [SCOPE-04] Document the reusable boundary and final host-owned concerns after the pilot cutover.

### Out of Scope

- [SCOPE-05] Adding a richer template DSL beyond today's double-curly token substitution contract.
- [SCOPE-06] Shipping a second production consumer outside Keel during this voyage.
- [SCOPE-07] Extracting all board-specific markdown processing into `speccy` beyond generic frontmatter mutation.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The current pure rendering helpers plus frontmatter mutation are the right minimal extraction seam for `speccy`. | Design assumption | The voyage may need to widen scope into adjacent markdown utilities before Keel can cut over cleanly. |
| Embedded template inventory can stay host-owned while generic frontmatter mutation moves into `speccy` without polluting the public API. | Architectural assumption | The crate boundary may remain too Keel-specific or force a wider public API. |
| Existing Keel tests over scaffold generation are sufficient to prove migration parity for the first cutover. | Internal dependency | Additional command-level regression tests would be needed before landing the migration. |

## Constraints

- `speccy` must not depend on `keel-core`, `keel-cli`, or `.keel` file layout assumptions.
- The hard-cutover policy applies to the migrated Keel paths: once callers adopt `speccy`, the old generic renderer should not remain active in parallel.
- Placeholder syntax remains the current double-curly token form for this voyage unless a deliberate design decision expands it.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `speccy` must expose deterministic double-curly token rendering, markdown document helpers, and generic frontmatter mutation equivalent to the current generic Keel behavior for full-document rendering and body-only rendering. | SCOPE-01 | FR-01 | `cargo test -p speccy` |
| SRS-02 | `speccy` must expose first-class template catalog/loading abstractions plus fallible host hooks for template lookup and optional post-render processing without importing Keel-specific types, file paths, or board concepts. | SCOPE-02 | FR-02 | `cargo test -p speccy` |
| SRS-03 | Keel call sites that currently use `template_rendering::{render, render_body, render_with_mutations}` and generic frontmatter mutation must consume `speccy` for the canonical reusable implementation while keeping only host-specific template inventory outside the crate. | SCOPE-03 | FR-03 | `cargo test -p keel` |
| SRS-04 | Voyage artifacts and story scopes must document the final reusable boundary, including the decision that template inventory remains host-owned while generic frontmatter mutation lives in `speccy`. | SCOPE-04 | FR-04 | docs inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | `speccy` must remain independent of Keel crates and keep its public API free of Keel-specific concepts. | SCOPE-01 SCOPE-02 | NFR-03 | `cargo test -p speccy` |
| SRS-NFR-02 | Representative Keel scaffold-generation paths must retain deterministic output after the cutover to `speccy`. | SCOPE-03 | NFR-02 | `cargo test -p keel` |
| SRS-NFR-03 | Template catalogs and host integration hooks must support embedded or caller-managed inventories without hard-coding filesystem assumptions into `speccy`. | SCOPE-02 | NFR-03 | `cargo test -p speccy` |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
