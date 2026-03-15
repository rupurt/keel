# Implement Interactive Inquiry for Theater Mode - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Implement the "Student" persona to ask clarification questions about formal rules. | board: VDo4OmYow |
| MG-02 | Implement the "Interrogator" persona to challenge evidence and assumptions. | board: VDo4OmYow |

## Constraints
- Must use existing marionette-style theater runtime.
- Persona prompts MUST be explicitly grounded in `FORMAL_RULES.md` and `CONSTITUTION.md`.
- Inquiry mode must be selectable via `--mood inquiry`.

## Halting Rules
- HALT when both personas are verified with CLI proofs in a theater session.
- DO NOT halt if any PRD requirement in VDo4OmYow is uncovered.
