# Engine Infrastructure and Standard Work - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Implement Watch time constraint primitive | board: VDseuzIFg |
| MG-02 | Codify Pacemaker stability rules in INSTRUCTIONS.md | board: VDseuzIFg |
| MG-03 | Stabilize system heartbeat and clear Med-Bay failures | board: VDseuzIFg |

## Constraints

- 12hr analog time limit via watch:VE3IAG4jZ

## Halting Rules

- HALT when all implementation stories are done and doctor reports nominal status.
- YIELD to human if watch limit is exceeded.
