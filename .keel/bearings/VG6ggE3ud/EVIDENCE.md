---
id: VG6ggE3ud
---

# Mission Request Command Surface Research — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | workspace | /home/alex/workspace/spoke-sh/keel/.keel/bearings/VDupml7OG/MISSION_REQUESTS.md | 2026-04-07 | 2026-04-07 | high | high | Existing research package already defines the candidate command family and normalized mission request envelope. |
| SRC-02 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper-cli/src/main.rs | 2026-04-07 | 2026-04-07 | medium | high | Current keeper-cli exposes only missions, start, and status commands, which leaves mission-request composition unimplemented. |

## Technical Research

## Key Findings

1. The command family is already sketched clearly enough to promote into a dedicated mission boundary. [SRC-01]
2. Keeper and other automation need a scriptable CLI contract rather than ad hoc embedded parsing logic. [SRC-01][SRC-02]
3. GitHub issues can be the first provider without making the command surface GitHub-specific. [SRC-01]

## Feasibility

This bearing is feasible as a focused command-surface mission because the work
is primarily interface design, normalization semantics, and command behavior
rather than backend runtime implementation.

## Unknowns

- How much response shaping `ack` should own versus leaving to Keeper
- Whether `draft` should surface mutation previews only or also recommended next commands
