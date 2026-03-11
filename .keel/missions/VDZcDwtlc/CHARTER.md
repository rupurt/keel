# Integrate Board Diagnostics into Pull-System Steering - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Run doctor checks as the first phase of the `next` algorithm. | board: VDZb97pMO |
| MG-02 | Map doctor problems to actionable next steps in `keel next` output. | board: VDZb9FeSU |
| MG-03 | Orchestrate coherence of doctor checks in entity creation. | board: VDZb9NTVh |

## Constraints

- Diagnostic-based next steps must take priority over new work.
- Support both Error and Warning severity levels in the pull system.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
