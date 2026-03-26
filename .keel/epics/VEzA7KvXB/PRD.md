# Extract Shared Txt Scene Engine - Product Requirements

## Problem Statement

Rich terminal surfaces in Keel are still hand-rendered with command-specific spacing, connector math, ANSI-aware width fixes, and one-off rendering helpers. That duplication has already produced repeated alignment regressions and forces command authors to reason about terminal geometry instead of scene intent.

We need a reusable workspace crate named `txt-scene` that lets commands declare semantic scene state and render it through one retained-tree layout engine. The engine must own fixed-width 2D composition, visible-column measurement, connector routing, clipping, and post-layout styling so scene-capable commands stop doing manual string geometry.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish `txt-scene` as the canonical rendering engine for rich terminal surfaces. | Shared scene crate exists in the workspace and can render fixed-width styled scenes deterministically. | Crate landed with retained-tree layout, semantic primitives, and debug tooling. |
| GOAL-02 | Prove the engine on the highest-risk existing surface first. | `keel flow --scene` renders through `txt-scene` in both color modes with stable geometry. | Flow migration complete with regression coverage. |
| GOAL-03 | Make future migrations lower-cognitive-load rather than bespoke rewrites. | Commands describe semantic scene state and approved primitives instead of manual spacing rules. | Migration contract documented and ready for `doctor`, `workshop`, `watch`, and `health`. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| CLI Maintainer | Extends scene-capable commands and fixes rendering bugs. | A shared scene engine that removes command-local layout math. |
| Operator | Runs Keel scene surfaces in color and no-color modes. | Stable, legible scene output without alignment drift. |
| Reuse Adopter | Wants to reuse the terminal scene engine in another project. | A crate boundary that is not tied to Keel command internals. |

## Scope

### In Scope

- [SCOPE-01] New workspace crate `txt-scene` with retained scene tree, attached constraints, exact-size measurement, fixed-width 2D canvas, clipping, and post-layout style emission.
- [SCOPE-02] Semantic primitives sufficient for the first migration: `Frame`, `Line`, `Columns`, `Label`, `Meter`, `Connector`, `Spacer`, `Callout`, plus overlay and anchor support.
- [SCOPE-03] Painter's-order overlap with optional explicit `z-index`.
- [SCOPE-04] Visible-column measurement with single-cell glyphs by default and explicit opt-in handling for wider Unicode.
- [SCOPE-05] Debug mode that exposes node tree, bounds, clip regions, z-order, ANSI-stripped width audit, and an overlay render.
- [SCOPE-06] `txtplot` adapter integration point.
- [SCOPE-07] Migration of `flow --scene` onto the new crate, with documented next migration order: `doctor`, `workshop`, `watch`, `health`.

### Out of Scope

- [SCOPE-08] Full migration of every existing scene-capable command in the first delivery slice.
- [SCOPE-09] Adaptive-to-terminal-width layout; v1 is fixed-width only.
- [SCOPE-10] Freeform command-side raw line emission inside diagrams.
- [SCOPE-11] Broad Unicode layout support beyond explicit primitive opt-ins.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Scene-capable commands must describe semantic scene state through a retained scene tree instead of emitting raw scene strings. | GOAL-01 GOAL-03 | must | Removes command-local geometry logic and keeps layout responsibility in one engine. |
| FR-02 | The engine must resolve attached constraints into fixed-width 2D bounds using visible terminal column width rather than byte length. | GOAL-01 GOAL-02 | must | Prevents ANSI/style drift and layout regressions across color modes. |
| FR-03 | The engine must support 2D positioning with painter's order and optional explicit `z-index`. | GOAL-01 GOAL-02 | must | Needed for connectors, overlays, and non-trivial scene composition. |
| FR-04 | The engine must hard-clip diagrams and support optional ellipsis behavior for labels. | GOAL-01 GOAL-02 | must | Keeps fixed-width scenes deterministic while preserving readable labels. |
| FR-05 | The engine must expose approved scene primitives and own padding, alignment, and connector routing. | GOAL-01 GOAL-03 | must | Prevents command code from reintroducing manual layout logic. |
| FR-06 | The crate must provide an adapter boundary for `txtplot` output that can be positioned and clipped inside the shared scene canvas. | GOAL-01 GOAL-03 | should | Allows immediate reuse for plot-backed surfaces without designing plot primitives first. |
| FR-07 | `keel flow --scene` must render through `txt-scene` as the first migration surface. | GOAL-02 | must | Proves the engine against the surface where regressions have already occurred. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Scene output must be deterministic in color and no-color modes, with identical visible geometry after ANSI stripping. | GOAL-01 GOAL-02 | must | Geometry drift across style modes is one of the primary failure cases this epic must eliminate. |
| NFR-02 | The engine must default to single-cell glyph assumptions and reject double-width Unicode unless a primitive explicitly opts in. | GOAL-01 | must | Constrains Unicode complexity to known-safe surfaces. |
| NFR-03 | Every scene line emitted from a finalized scene must satisfy a fixed-width visible-column contract. | GOAL-01 GOAL-02 | must | Enforces the core layout invariant at the engine boundary. |
| NFR-04 | The crate must include debug tooling that exposes layout decisions without requiring command-specific instrumentation. | GOAL-01 GOAL-03 | must | Rendering bugs are otherwise too expensive to diagnose. |
| NFR-05 | The crate boundary must be reusable by other projects without depending on Keel command modules. | GOAL-01 GOAL-03 | should | This work is explicitly intended for cross-project reuse. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Primitive layer: unit tests for measurement, clipping, overlap order, connector routing, style-free geometry, and Unicode policy.
- Adapter layer: `txtplot` integration tests proving fixed-width clipping and positioning.
- Command migration: regression tests for `flow --scene` in color and no-color modes, plus width-invariant assertions.
- Debug tooling: tests that debug views expose node tree, resolved bounds, clip regions, z-order, and ANSI-stripped width audit deterministically.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A retained scene tree with exact-size measurement is sufficient for v1 without a more general constraint solver. | The crate surface may need to expand before the first migration lands. | Validate during the `flow` migration and debug tooling pass. |
| `txtplot` can be safely integrated through an adapter rather than immediate native plot primitives. | Plot-backed surfaces may force a larger primitive set earlier than planned. | Prove the adapter on one plot-backed surface after the initial engine lands. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which minimal primitive set is sufficient after the `flow` migration without causing primitive churn in later command migrations? | Epic owner | Open |
| Does any required scene surface need broader Unicode support earlier than the explicit-opt-in policy allows? | Epic owner | Open |
| How much of the current `keel-cli` presentation helper layer should move into `txt-scene` versus stay command-local? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `txt-scene` exists as a workspace crate and does not depend on Keel command modules.
- [ ] Scene-capable commands build semantic scene state rather than raw diagram strings for the migrated path.
- [ ] The engine renders fixed-width 2D scenes using visible-column measurement and post-layout styling.
- [ ] Painter's-order overlap, optional `z-index`, clipping, and connector routing are engine-owned behaviors.
- [ ] Default Unicode policy rejects double-width glyph use unless a primitive explicitly opts in.
- [ ] Debug mode exposes node tree, bounds, clip regions, z-order, ANSI-stripped width audit, and overlay render deterministically.
- [ ] `flow --scene` is migrated onto `txt-scene` and regression-tested in color and no-color modes.
- [ ] The next migration order is explicit: `doctor`, `workshop`, `watch`, `health`.
<!-- END SUCCESS_CRITERIA -->
