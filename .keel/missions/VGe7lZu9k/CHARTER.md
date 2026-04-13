# Implement Mission Stack Protocol - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Materialize Mission Stack as a repo-local coordination projection sourced from `.keel/stacks` metadata plus current git/worktree state so Keel can reason about active stack membership without redefining the board model. | board: VGe7mCcFW |
| MG-02 | Surface Mission Stack state and protocol gating through the canonical operator adapters so `turn`, `next`, `mission next --status`, and `doctor` can explain whether the current repo may act. | board: VGe7mCcFW |
| MG-03 | Enforce the first foreign-reactor guardrails for managed worktree execution, `stack/<id>` branch provenance, and stack-close cleanup reporting while keeping non-stack repos unchanged. | board: VGe7mCcFW |

## Constraints

- Preserve repo-local heartbeat and pacemaker semantics; Mission Stack must remain a parallel read model rather than a heartbeat replacement.
- Keep the first pushed-receipt contract git-native and avoid introducing mandatory non-git receipt artifacts.
- Default to no-op behavior when `.keel/stacks` contains no active stack metadata.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied and the implementation passes `cargo fmt`, `cargo clippy`, `cargo nextest`, and `keel doctor --status`
- YIELD to human when only `metric:` or `manual:` goals remain
