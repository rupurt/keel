---
id: 1vzeJF000
---

# MissionEntity — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | direct:observation | ~/workspace/rupurt/sift/AGENTS.md | 2026-03-09 | 2026-03-09 | high | high | Autonomous Delivery Policy — 7 prose rules for when harnesses should stop |
| SRC-02 | manual | direct:observation | ~/workspace/spoke-sh/port/AGENTS.md | 2026-03-09 | 2026-03-09 | high | high | Same Autonomous Delivery Policy pattern, port running 3+ days |
| SRC-03 | manual | direct:analysis | src/domain/model/ | 2026-03-09 | 2026-03-09 | high | high | Existing entity conventions: frontmatter, state machines, loaders, doctor checks |

## Technical Research

### Feasibility

Fully feasible. Mission follows the same structural pattern as every existing
keel entity (frontmatter + state machine + loader + doctor checks). The main
new surface is CHARTER.md goal parsing, the refinement loop command, and
`keel next` mission-awareness. All of these extend existing infrastructure
rather than requiring new architectural patterns [SRC-03].

## Key Findings

1. Both autonomous projects use identical AGENTS.md prose to prevent halting,
   but harnesses still halt early when the queue empties [SRC-01] [SRC-02]
2. Existing keel entities follow consistent conventions that Mission can
   replicate: YAML frontmatter, typed state machine, directory-based identity,
   doctor checks, transition gating [SRC-03]
3. No current entity captures "the real-world objective" — epics capture
   strategic initiatives but there is no parent linking them to a common
   goal with explicit halting rules [SRC-03]
4. The refinement loop pattern (`keel mission refine`) is new to keel but
   follows the bearing research workflow model — iterative document filling
   with a readiness gate [SRC-03]

## Unknowns

- LOG.md digest threshold — start with entry count, tune from usage
- Multi-mission scheduling — design for it but defer implementation
