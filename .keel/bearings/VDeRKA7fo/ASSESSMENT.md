---
id: VDeRKA7fo
---

# Simulation Kernel Architecture Research — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 4 | A clearer internal execution model could simplify several growing subsystems at once. |
| Confidence | 4 | The repository already contains most of the underlying pieces in partial form. |
| Effort | 3 | The extension can start small, but projection and orchestration refactors will still touch multiple modules. |
| Risk | 3 | The main risk is over-abstracting beyond what the current system actually needs. |

*Scores range from 1-5:*
- 1 = Very Low
- 2 = Low
- 3 = Medium
- 4 = High
- 5 = Very High

## Analysis

## Findings

- Keel already has proto-reactor behavior in the process manager and proto-simulation behavior in temporal read models, so the proposal extends an existing pattern instead of inventing a new one [SRC-01][SRC-02]
- The safest next step is to formalize a small internal vocabulary and migrate a few hot spots instead of renaming the whole architecture [SRC-01][SRC-03]

## Opportunity Cost

Pursuing this architecture line spends time on internal clarity instead of immediately shipping new operator features. That trade is justified only if the resulting abstractions reduce future duplication across orchestration, temporal scheduling, and flow projections [SRC-01][SRC-03].

## Dependencies

- A clear ADR or equivalent architecture decision is needed before broad implementation work starts so the team keeps DDD and hexagonal boundaries intact [SRC-01]
- The first implementation slice should target already-identified hotspots such as process management and temporal read models [SRC-01][SRC-02]

## Alternatives Considered

- Keep the current architecture unchanged and continue adding ad hoc orchestration and time logic where needed. This has lower short-term cost but increases conceptual drift as more automation and temporal behavior lands [SRC-01][SRC-03]
- Recast Keel wholesale as a game engine. This would create more terminology churn than practical value and does not fit the current command-driven runtime [SRC-01][SRC-02]

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-02][SRC-03]
[ ] Park → revisit later [SRC-01]
[ ] Decline → document learnings [SRC-01]
