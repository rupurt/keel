# Canonical Board Graph and Scoped Regeneration - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Introduce a canonical BoardGraph relationship layer that Keel can use to validate lineage quickly and constrain generated artifact rewrites to the affected graph frontier. | board: VDfNdssJL |

## Constraints
- Keep the change incremental and deterministic; preserve DDD and hexagonal layering.
- Use one canonical relationship path instead of compatibility bridges or parallel graph implementations.
- Do not introduce a database, daemon, or background graph service as part of this mission.

## Halting Rules
- DO NOT halt while epic VDfNdssJL lacks a planned voyage or executable story for the graph kernel.
- YIELD to human before changing on-disk board shape or introducing persisted graph storage.
- HALT when the canonical graph, graph-level doctor check, and first frontier-scoped regeneration path are landed with board-backed evidence and remaining work is prioritization only.
