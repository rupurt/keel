# Txt Scene Foundation And Flow Pilot - Software Requirements Specification

> Land a reusable txt-scene crate boundary and migrate keel flow --scene onto it as the first proving surface.

**Epic:** [VEzA7KvXB](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Add a new workspace crate named `txt-scene` that owns visible-width measurement, fixed-width line assembly, framed rows, and color-optional palette primitives without depending on Keel command modules.
- [SCOPE-02] Cut `keel flow --scene` over to `txt-scene` primitives for all rendered scene variants while preserving the existing electrical metaphor and operator-facing annotations.
- [SCOPE-03] Record the pilot boundary and explicit next migration order for scene-capable commands after `flow`: `doctor`, `workshop`, `watch`, `health`.

### Out of Scope

- [SCOPE-04] Full retained-tree 2D composition, general connector routing, overlay layering, or debug overlays beyond what the extracted line/frame primitives need for the pilot.
- [SCOPE-05] Migrating `doctor`, `workshop`, `watch`, `health`, or any non-`flow` scene surface in this voyage.
- [SCOPE-06] `txtplot` adapter work or broader Unicode layout support beyond the existing single-cell width policy.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The existing `flow` scene tests are sufficient regression guards for geometry and operator-facing wording during the pilot migration. | Internal dependency | Additional snapshot coverage would be needed before the migration can land safely. |
| The current `SceneLine`, `SceneFrame`, and `ScenePalette` helpers capture the minimum reusable seam needed to prove the crate boundary. | Design assumption | The voyage may need to expand its primitive surface before `flow` can move cleanly. |
| `txt-scene` can own ANSI-aware visible-width measurement directly instead of reusing `keel-core` utilities. | Architectural assumption | Reuse would remain coupled to Keel internals and violate the epic's crate-boundary goal. |

## Constraints

- `txt-scene` must remain reusable and must not depend on `keel-cli` or `keel-core`.
- The hard-cutover policy applies to the `flow --scene` path in scope: once migrated, command-local width primitives should not remain as active duplicates.
- Color and no-color output must preserve identical visible geometry after ANSI stripping for every emitted `flow --scene` line.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The workspace must expose a reusable `txt-scene` crate that owns visible-width measurement, `SceneLine`, `SceneFrame`, and `ScenePalette` primitives needed by the pilot migration. | SCOPE-01 | FR-05 | `cargo test -p txt-scene` |
| SRS-02 | `keel flow --scene` must render its open-circuit, unhealthy, unplugged, and powered scene variants through `txt-scene` primitives instead of locally defined width-aware scene helpers. | SCOPE-02 | FR-07 | `cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_has_stable_width_without_color` |
| SRS-03 | Voyage artifacts and in-scope documentation must record the `txt-scene` pilot boundary and the next migration order `doctor`, `workshop`, `watch`, `health`. | SCOPE-03 | FR-05 | docs inspection + `cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_surfaces_watch_pressure_without_color` |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Color and no-color `flow --scene` renders must preserve fixed visible width across all lines after ANSI stripping. | SCOPE-02 | NFR-01 | `cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_has_stable_width_with_color` |
| SRS-NFR-02 | `txt-scene` must enforce the existing single-cell visible-width policy internally and remain independent of Keel command modules. | SCOPE-01 | NFR-05 | `cargo test -p txt-scene` |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
