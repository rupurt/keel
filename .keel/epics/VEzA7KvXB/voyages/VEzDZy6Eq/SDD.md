# Txt Scene Foundation And Flow Pilot - Software Design Description

> Land a reusable txt-scene crate boundary and migrate keel flow --scene onto it as the first proving surface.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extracts the reusable width-aware scene helpers out of `keel-cli` and into a standalone workspace crate named `txt-scene`, then rewires the `flow --scene` rendering path to consume that crate. The pilot deliberately keeps the command-local semantics of the electrical metaphor in `flow` while moving generic fixed-width rendering primitives into the reusable boundary.

## Context & Boundaries

```text
┌────────────────────────────────────────────────────────────┐
│                txt-scene flow pilot voyage                │
│                                                            │
│  flow scene semantics ─┬─> txt-scene primitives            │
│                        │                                   │
│                        ├─> width / style regression tests  │
│                        │                                   │
│                        └─> migration boundary docs         │
└────────────────────────────────────────────────────────────┘
```

### In Scope

- Move generic scene primitives into `crates/txt-scene`.
- Rewire `flow --scene` to use the new crate.
- Preserve operator-visible geometry and annotations.

### Out of Scope

- New higher-level semantic primitives beyond the pilot seam.
- Non-`flow` scene migrations.
- Plot adapters, debug overlays, and broader Unicode support.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `owo-colors` | library | Optional ANSI styling for palette output | existing workspace dependency |
| `regex` | library | ANSI escape stripping for visible-width measurement | existing workspace dependency |
| `flow` diagnostics command | internal consumer | Proves the crate boundary on a real scene surface | existing crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Crate boundary | Keep `txt-scene` independent of `keel-core` and `keel-cli` | Preserves cross-project reuse instead of exporting another Keel-specific helper module |
| Pilot scope | Move generic width-aware primitives first, leave command semantics local | Minimizes migration risk while proving the boundary on the most drift-prone surface |
| Hard cutover | Delete local duplicate primitive definitions once `flow` imports `txt-scene` | Prevents the new crate and old helper path from diverging immediately |
| Next migration order | Record `doctor`, `workshop`, `watch`, `health` explicitly in planning artifacts | Makes the pilot a deliberate first step instead of an isolated refactor |

## Architecture

The design splits responsibilities into two layers:

- `txt-scene` owns fixed-width rendering mechanics: ANSI-stripped visible-width measurement, line padding, framed rows, and color-optional styling helpers.
- `keel-cli` continues to own scene-specific semantics: selecting tones, translating flow metrics into battery/watch/capacitor indicators, and deciding which scene variant to render.

This keeps the initial crate API narrow while still removing the geometry logic that previously lived in `keel-cli`.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| `txt-scene::width` | Measure visible terminal columns with ANSI removed | pure functions | Keeps the single-cell width policy local to the reusable crate |
| `txt-scene::SceneLine` | Build width-safe lines through push and pad operations | builder API | Replaces the local `SceneLine` helper |
| `txt-scene::SceneFrame` | Wrap fixed-width lines in framed rows | builder API | Replaces the local `SceneFrame` helper |
| `txt-scene::ScenePalette` | Apply optional color styling without changing geometry | styling API | Replaces the local `ScenePalette` helper |
| `flow` scene renderer | Translate flow state into scene content and feed `txt-scene` primitives | command-local functions | Semantic rendering stays local for the pilot |

## Interfaces

The crate should expose a small public API:

- `visible_width(&str) -> usize`
- `SceneLine::new(target_width)`
- `SceneLine::{push, pad_to, finish}`
- `SceneFrame::new(indent, left_border, right_border, inner_width)`
- `SceneFrame::{row, empty_row}`
- `ScenePalette::new(use_color)` plus the palette helpers already used by `flow`

`flow.rs` should import those primitives from `txt_scene` and keep the rest of the scene-building logic unchanged in intent.

## Data Flow

1. `flow` computes readiness, health, autonomy, and watch-capacity metrics.
2. Command-local helpers map those metrics into semantic indicator strings and tone choices.
3. `txt-scene` primitives assemble the final fixed-width rows and apply optional styling.
4. Existing `flow` tests assert width invariants and operator-facing wording.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| `txt-scene` width math changes visible geometry | Flow and crate tests fail width assertions | Reject the change before submit | Adjust padding or ANSI stripping until widths stabilize |
| The new crate accidentally depends on Keel internals | Build graph or imports reveal cross-crate coupling | Refactor measurement/styling helpers into the reusable crate | Remove the dependency and keep the API generic |
| `flow` leaves duplicate local primitives after migration | Review and compile-time imports show dead parallel helpers | Delete the local definitions in the same slice | Re-run tests to confirm the crate path is canonical |
