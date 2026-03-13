---
id: VDmdk1uib
---

# TUI Compact Layout Research — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 4 | Higher density is a major goal |
| Confidence | 5 | Already partially implemented |
| Effort | 2 | Low effort due to existing logic |
| Risk | 1 | Minimal risk of regression |

## Analysis

### Findings

- Three bullets are sufficient for quick status. [SRC-01]

### Opportunity Cost

Slightly higher code complexity in the CLI layer. [SRC-01]

### Dependencies

- Depends on the existing `calculate_next` algorithm. [SRC-01]

### Alternatives Considered

- Multi-page status was rejected as too verbose. [SRC-01]

## Recommendation

[x] Proceed → convert to epic [SRC-01]
