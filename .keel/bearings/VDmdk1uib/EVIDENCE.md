---
id: VDmdk1uib
---

# TUI Compact Layout Research — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | manual:source | src/cli/commands/management/mission/next.rs | 2026-03-13 | 2026-03-13 | high | high | Analysis of current next-step logic |
| SRC-02 | manual | manual:source | README.md | 2026-03-13 | 2026-03-13 | high | high | Implicit Ramping Path framing |

## Technical Research

### Feasibility
Verified: adding the --status flag and a compact renderer is feasible with the current CLI and domain model.

## Key Findings

1. Deduplication and ranking are key to high-density status reports.

## Unknowns

- How will users respond to the 3-bullet limit?
