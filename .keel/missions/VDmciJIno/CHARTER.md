# Implement Compact Status for Mission Next - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Implement `keel mission next --status` returning exactly three action-oriented bullets. | board: VDm4ld6EX |

## Constraints
- Output MUST be exactly three bullets when using `--status`.
- Bullets MUST contain a concrete next command or direct action.
- Ensure no regressions in default `keel mission next` output.

## Halting Rules
- HALT when `keel mission next --status` is verified with CLI proofs.
- DO NOT halt if any PRD goal in VDm4ld6EX is unmet.
