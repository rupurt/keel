# Deterministic Artifact Generation - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Make voyage-generated artifacts deterministic so repeated syncs and equivalent board loads do not create content churn. | board: VDfhxO64S |

## Constraints
- Preserve existing artifact filenames and stakeholder-facing report structure; this mission is about determinism, not redesign.
- Canonicalize ordering and normalization in generator paths without compatibility fallbacks or dual rendering paths.
- Keep scope to voyage artifact generation and board sync ordering; frontier-scoped selective regeneration remains follow-on work.

## Halting Rules

- DO NOT halt while epic VDfhxO64S lacks a planned voyage or executable story.
- YIELD to human before changing report contracts or removing existing generated artifact classes.
- HALT when repeated voyage artifact generation is byte-stable across equivalent board states and the board-backed evidence for epic VDfhxO64S is landed.
