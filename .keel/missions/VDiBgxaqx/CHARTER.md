# First-Class Mission Bearing Lineage - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Make mission-to-bearing lineage first-class so missions can own research bearings through an explicit, discoverable workflow and mission readiness can recognize bearing-backed scope without hidden conventions. | board: VDiDhXVKy |

## Constraints

- Keep mission-bearing lineage canonical in board state and graph projections instead of inferring it from loose prose or title similarity.
- Provide a discoverable management path for attachment, such as `keel mission add-child <mission> <bearing>` or an equally explicit equivalent.
- Mission readiness, activation gates, doctor, and show surfaces must all use the same lineage rules.
- Do not silently attach or reparent bearings based on heuristic matching.

## Halting Rules

- DO NOT halt while missions still cannot explicitly own child bearings or mission readiness ignores attached research scope.
- HALT when mission-bearing lineage is explicit, discoverable, and consistently honored by activation, doctor, and mission show/next surfaces.
- YIELD to human if mission-bearing lineage conflicts with the current mission/epic ownership model and requires a larger structural decision.
