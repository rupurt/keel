---
id: 1vzeJF000
---

# MissionEntity — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 5 | Directly addresses the #1 pain point in autonomous workflows: false halting |
| Confidence | 4 | Follows proven entity patterns; CHARTER parsing and refinement loop are new but bounded |
| Effort | 4 | ~4 voyages across domain, CLI, doctor, and flow integration |
| Risk | 2 | Low architectural risk — extends existing patterns. Main risk is getting halting heuristics right |

## Analysis

## Findings

- Autonomous harnesses halt early despite explicit AGENTS.md policy because the
  stopping condition is prose, not board state [SRC-01] [SRC-02]
- Mission entity fills the gap between "board work done" and "objective
  achieved" by encoding both machine-checkable and human-verifiable goals [SRC-03]
- Design follows existing entity conventions, minimizing new infrastructure [SRC-03]

## Opportunity Cost

Time spent building Mission is time not spent on other keel features. However,
Mission directly enables the core use case (autonomous multi-day delivery) and
the false-halting problem is the highest-friction issue observed in production
harness usage [SRC-01] [SRC-02].

## Dependencies

- Existing entity infrastructure is stable and well-tested [SRC-03]
- Lineage field pattern proven by bearing → epic lineage (just shipped) [SRC-03]

## Alternatives Considered

- **Enhanced AGENTS.md only**: More prescriptive prose rules. Rejected — harnesses
  already ignore the existing rules because they're not machine-checkable [SRC-01]
- **External objective tracker**: Track goals outside keel. Rejected — breaks the
  "board is authoritative" principle and adds integration complexity [SRC-01] [SRC-02]
- **Epic-level goals**: Add goal tracking to epics instead. Rejected — objectives
  span multiple epics and need a dedicated lifecycle [SRC-03]

## Recommendation

[x] Proceed — convert to epic [SRC-01] [SRC-02] [SRC-03]
[ ] Park — revisit later
[ ] Decline — document learnings
