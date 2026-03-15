# Board Coherence Restoration - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Formalize implementation logic for diagnostics-first steering. | board: VDZcE0Uo5 |
| MG-02 | Formalize mapping of doctor problems to actionable next steps. | board: VDZcE46pb |
| MG-03 | Formalize structural coherence in entity creation templates. | board: VDZcE7gsS |

## Constraints

- Use canonical CLI commands for all transitions.
- No temporary scripts in the repository.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
