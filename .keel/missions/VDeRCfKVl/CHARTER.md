# Simulation Kernel and Reactive Architecture - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Introduce a simulation-kernel extension path for Keel that keeps DDD and hexagonal architecture as the outer structure while providing explicit internal abstractions for pulse evaluation, reactors, and projections. | board: VDeRV9CAo |
| MG-02 | Validate the architecture direction through mission-scoped research before decomposing implementation so the team does not drift into a full game-engine rewrite. | manual: bearing VDeRKA7fo is assessed and the recommendation is captured in the mission log |
| MG-03 | Produce an incremental migration path that identifies the first refactor slices for orchestration, temporal evaluation, and shared projections. | manual: mission log records the approved rollout sequence and owning modules |

## Constraints

- Extend the existing DDD and hexagonal architecture; do not replace the layer model with game-engine terminology or ownership.
- Keep the system command-driven and deterministic at an injected reference instant; do not introduce a continuous runtime loop or background daemon as part of this mission.
- Prefer small internal abstractions that reduce duplicated board scanning and orchestration branching over a broad framework rewrite.
- Preserve hard-cutover discipline: each migration slice should leave one canonical execution path behind it.

## Halting Rules

- DO NOT halt while epic VDeRV9CAo lacks either an assessed enabling bearing or a planned voyage that makes the first implementation slice executable.
- YIELD to human before introducing a continuous event loop, persistent runtime service, or an ECS-style rewrite that would replace the current aggregate model.
- HALT when the simulation-kernel direction is documented, the first implementation epic is decomposed into board-ready work, and only human prioritization or sign-off remains.
