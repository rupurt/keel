# Lineage And Doctor - Software Design Description

> Mission lineage field on child entities, doctor checks for completion and integrity

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds mission lineage tracing and doctor checks. The lineage field
connects child entities (epics, bearings, ADRs) back to their parent mission.
Doctor checks validate this lineage and detect mission completion, stale missions,
and orphaned references.

## Architecture

### Modified Files

| File | Change |
|------|--------|
| `src/domain/model/epic.rs` | Add `mission: Option<String>` to EpicFrontmatter |
| `src/domain/model/bearing.rs` | Add `mission: Option<String>` to BearingFrontmatter |
| `src/domain/model/adr.rs` | Add `mission: Option<String>` to AdrFrontmatter |
| `src/infrastructure/validation/types.rs` | Add MissionGoalAchieved, MissionActiveNoWork, MissionOrphanedLineage, MissionStale CheckId variants |

### New Files

| File | Purpose |
|------|---------|
| `src/cli/commands/diagnostics/doctor/checks/missions.rs` | Mission doctor checks |
| `src/infrastructure/validation/charter.rs` | CHARTER.md goal parsing utilities |

## Components

### CHARTER.md Goal Parser

Parses the Goals table in CHARTER.md:

```markdown
## Goals
| ID | Description | Verification |
|----|-------------|-------------|
| MG-01 | All epics complete | board: all epics done |
| MG-02 | Latency target met | metric: p95 < 200ms |
```

Extracts: `Vec<MissionGoal { id, description, verification: GoalVerification }>` where
`GoalVerification` is `Board(String)`, `Metric(String)`, or `Manual(String)`.

### Board-Verifiable Goal Evaluation

For `board:` goals, the evaluator interprets common patterns:
- "all epics done" → check all mission-scoped epics have Done status
- "all voyages complete" → check all mission-scoped voyages are Done
- Fallback: if pattern is unrecognized, treat as unmet (safe default)

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| `mission` field is optional on all child entities | Entities can exist without missions | Backward compatible |
| Board goal evaluation uses pattern matching on description | Avoids inventing a DSL | Simple, extensible |
| Unrecognized board goal patterns treated as unmet | Safe default prevents false completion | Harness must use known patterns |
| MissionStale check is Warning not Error | Informational, not blocking | Avoids false halting |
