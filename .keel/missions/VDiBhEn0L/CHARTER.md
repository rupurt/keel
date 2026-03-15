# Dependency-Aware Bearing Prioritization - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Extend bearing prioritization with explicit dependency signals so Keel can recommend research sequence, not just impact/effort/risk ranking. | board: VDiHwLLfY |

## Constraints

- Model bearing dependencies as first-class, explicit board relationships rather than inferring them only from prose.
- Dependency-aware priority must remain deterministic and explainable.
- Keep impact, effort, and risk in the decision model; do not replace them wholesale with dependency order.
- Surface why a bearing is sequenced where it is, especially when dependency order outweighs raw score.

## Halting Rules

- DO NOT halt while bearing prioritization cannot represent blocked-by chains or explain recommended sequence.
- HALT when Keel can encode bearing dependencies and use them to drive understandable prioritization in flow/list-style surfaces.
- YIELD to human if dependency modeling must wait on a larger roadmap/horizon design decision.
