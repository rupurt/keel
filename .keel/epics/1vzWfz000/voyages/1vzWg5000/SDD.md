# Lineage Persistence - Software Design Description

> GOAL-01

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds durable lineage metadata to the `keel bearing lay` transition. When a bearing graduates to an epic, the lay command persists the destination epic ID and validated goal references from `BRIEF.md` into the bearing's frontmatter. This creates an explicit, machine-readable link from research output to strategic execution.

## Context & Boundaries

```
┌────────────────────────────────────────────────────┐
│                 bearing lay                        │
│                                                    │
│  ┌──────────┐   ┌─────────────┐   ┌────────────┐  │
│  │ BRIEF.md │──▶│ Goal Parser │──▶│ Frontmatter│  │
│  │ Success  │   │ & Validator │   │ Writer     │  │
│  │ Criteria │   └─────────────┘   └────────────┘  │
│  └──────────┘                          │           │
│                                        ▼           │
│                              ┌──────────────────┐  │
│                              │ bearing README   │  │
│                              │ epic: <epic-id>  │  │
│                              │ goals: [GOAL-*]  │  │
│                              └──────────────────┘  │
└────────────────────────────────────────────────────┘
        ↑                              ↑
   [BRIEF.md]                    [Epic entity]
```

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Lineage storage location | Bearing README frontmatter | Keeps lineage co-located with the bearing entity; avoids a separate lineage artifact. |
| Goal reference format | Array of `GOAL-*` IDs in frontmatter | Machine-readable, consistent with PRD goal ID conventions, easy to validate. |
| Validation timing | Before write during lay | Fail-fast prevents partial/corrupt lineage from being persisted. |

## Components

**Goal parser**: Reads `BRIEF.md` Success Criteria section, extracts `GOAL-*` references, and validates them against the target epic's PRD goals.

**Frontmatter writer**: Extends the existing bearing frontmatter writer to persist `epic` and `goals` fields during the lay transition, preserving all existing fields.

## Data Flow

1. User runs `keel bearing lay <id>`.
2. Lay command resolves the target epic (created or selected).
3. Goal parser reads `BRIEF.md` Success Criteria and extracts `GOAL-*` references.
4. Validator checks extracted goals against the epic's PRD `Goals & Objectives` table.
5. On success, frontmatter writer persists `epic: <id>` and `goals: [GOAL-01, ...]` to the bearing README.
6. On validation failure, the command exits with a deterministic error before any write.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unknown goal reference in BRIEF.md | Goal ID not found in epic PRD | Reject lay with error listing offending ID | Author valid goal references in BRIEF.md |
| Missing BRIEF.md Success Criteria section | Section parser returns empty | Reject lay with missing-section error | Add Success Criteria to BRIEF.md |
| Epic PRD missing Goals & Objectives | PRD parser returns no goals | Reject lay with PRD gap error | Author goals in epic PRD |
