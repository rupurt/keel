# Realize High-Density TUI Show Surfaces - Charter

**Archetype:** Bridging (Realization)

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Implement high-density show views for `keel story show` and `keel voyage show`. | board: VDmdk1uib |
| MG-02 | Add Archetype labeling to mission charters and the `keel mission show` surface. | board: VDmdk1uib |

## Constraints
- "High-Density" views MUST follow the 3-bullet information density pattern where possible.
- Archetype labels MUST match the four canonical types in `FORMAL_RULES.md`.

## Halting Rules
- HALT when `keel mission show` displays the "Bridging" label for this mission.
- HALT when `keel story show` provides a high-density summary.
