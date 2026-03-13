# Add Keel Play Theater Mode - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Create a theater-style interactive keel play mode with genre-themed TUI sessions and playful puppet narratives, including comedy, drama, action, stand-up, and Shakespeare-inspired personas. | board: VDlzCqxr9 |

## Constraints
- Scope is limited to interaction UX and presentation for `keel play --theater`.
- Do not introduce new external runtime dependencies without explicit approval.
- No external network calls or non-deterministic data sources in theater session rendering.
## Halting Rules
- DO NOT halt while any MG-* goal has unfinished board work.
- HALT when all MG-* goals with `board:` verification are satisfied.
- YIELD to human when only `metric:` or `manual:` goals remain.
