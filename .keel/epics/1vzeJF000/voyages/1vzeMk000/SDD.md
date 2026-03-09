# Domain Foundation - Software Design Description

> Mission model, state machine, frontmatter, loader, and Board integration

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces the Mission entity as the sixth keel domain primitive.
It follows the same structural pattern as Epic, Voyage, Story, Bearing, and ADR:
YAML frontmatter for metadata, a typed state machine for lifecycle, directory-based
identity, and Board loader integration.

## Architecture

### New Files

| File | Purpose |
|------|---------|
| `src/domain/model/mission.rs` | Mission struct, MissionFrontmatter, Entity impl |
| `src/domain/state_machine/mission.rs` | MissionStatus enum, transition validation |
| `src/infrastructure/templates/mission/` | README.md, CHARTER.md, LOG.md templates |

### Modified Files

| File | Change |
|------|--------|
| `src/domain/model/mod.rs` | Re-export Mission, MissionFrontmatter |
| `src/domain/model/board.rs` | Add `missions: HashMap<String, Mission>` |
| `src/domain/state_machine/mod.rs` | Re-export MissionStatus |
| `src/infrastructure/loader.rs` | Add `load_missions()`, call from `load_board()` |

## Components

### MissionStatus State Machine

```
Defining ──activate──→ Active ──achieve──→ Achieved ──verify──→ Verified
                         │                                       (terminal)
                         ├──pause──→ Paused ──resume──→ Active
                         │
                         └──abandon──→ Abandoned (terminal)
                                         ↑
                   Paused ──abandon──────┘
```

### CHARTER.md Structure

```markdown
## Goals
| ID | Description | Verification |
|----|-------------|-------------|
| MG-01 | ... | board: all epics done |
| MG-02 | ... | metric: p95 latency < 200ms |

## Constraints
- ...

## Halting Rules
- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
```

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Directory under `.keel/missions/` | Consistent with bearings, epics | Path-based identity pattern |
| MG-XX goal IDs (not GOAL-XX) | Avoid collision with epic-level GOAL-XX IDs | Distinct namespace |
| Verification types: board, metric, manual | Minimal set covering observed use cases | Extensible later |
| Paused as explicit state (not Icebox) | Missions are resumed, not thawed | Clearer intent for long-running work |
