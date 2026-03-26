# Shared Txt Scene Engine For Rich Terminal Surfaces - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Land a reusable `txt-scene` crate and migrate the first Keel scene surface onto it so rich terminal rendering uses semantic scene state and shared layout primitives instead of command-specific string math. | board: VEzA7KvXB |

## Constraints

- Extract the shared renderer into a workspace crate named `txt-scene` so the scene engine can be reused outside Keel.
- Commands must build semantic scene state, not raw scene strings or manual spacing/alignment.
- Use a retained scene tree with attached constraints, exact-size node measurement, visible-column width accounting, and a fixed-width 2D canvas.
- Apply ANSI styling only after layout is finalized so color and no-color modes share identical geometry.
- Use painter's order with optional explicit `z-index`; connector placement and padding belong to the scene engine, not command code.
- Default to single-cell glyphs; double-width Unicode is forbidden unless a primitive explicitly opts in and handles it safely.
- Support ASCII-rich scenes and `txtplot` through an adapter boundary instead of command-specific plot rendering.
- Ship debug mode with node tree, resolved bounds, clip regions, z-order, ANSI-stripped width audit, and overlay render output.
- Migrate scene surfaces in order: `flow`, `doctor`, `workshop`, `watch`, `health`.

## Halting Rules

- DO NOT halt while `flow --scene` still depends on ad hoc string padding, connector math, or ANSI-aware width fixes in command code.
- HALT when `txt-scene` exists as a workspace crate with semantic primitives, fixed-width layout/styling pipeline, debug mode, `txtplot` adapter support, and `flow --scene` migrated with color/no-color regression coverage.
- YIELD to human if the first migration proves the agreed primitive set or constraint vocabulary is insufficient and the crate would need a larger v1 surface than planned.
