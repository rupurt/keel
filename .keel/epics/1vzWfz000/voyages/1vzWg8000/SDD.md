# Lineage Validation - Software Design Description

> GOAL-01

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds validation, diagnostics, and migration support for bearing lineage metadata. Doctor checks enforce that laid bearings carry valid lineage. CLI show surfaces render lineage fields. A migration path handles pre-contract bearings that were laid before lineage persistence existed.

## Context & Boundaries

```
┌────────────────────────────────────────────────────┐
│              Lineage Validation                    │
│                                                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │
│  │ Doctor Check │  │ CLI Show     │  │ Migration│ │
│  │ (lineage)    │  │ (lineage     │  │ (legacy  │ │
│  │              │  │  rendering)  │  │  repair) │ │
│  └──────────────┘  └──────────────┘  └──────────┘ │
└────────────────────────────────────────────────────┘
        ↑                    ↑               ↑
  [Bearing entity]    [Bearing entity]  [Board fixtures]
```

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Doctor check severity | Hard error (not warning) | Lineage is a contract; silent drift defeats the purpose. |
| Remediation in diagnostics | Include path + offending token + fix command | Matches existing doctor UX patterns (actionable errors). |
| Migration approach | Explicit repair command, not auto-fix | Keeps migration visible and auditable per the hard-cutover policy. |

## Components

**Lineage doctor check**: Validates that every laid bearing has a non-empty `epic` frontmatter field and that all `goals` entries reference valid epic PRD goals. Reports hard-fail diagnostics with remediation commands.

**CLI lineage rendering**: Extends `keel bearing show` and `keel bearing lay` output to display lineage metadata (epic ID, goal references). Handles unknown/legacy values without truncation.

**Migration flow**: Provides a repair path for bearings laid before the lineage contract. Detects missing metadata and emits correction instructions. Scales linearly with board size.

## Data Flow

1. `keel doctor` iterates all laid bearings.
2. For each bearing, reads `epic` and `goals` frontmatter fields.
3. Validates `epic` exists as a known epic ID on the board.
4. Validates each `goals` entry exists in the target epic's PRD.
5. Missing or invalid fields produce a hard-fail diagnostic with the bearing path, offending value, and suggested fix command.
6. Migration check detects pre-contract bearings (laid status, no lineage fields) and reports repair instructions.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Laid bearing missing `epic` field | Frontmatter read returns None | Doctor hard-fail with bearing path and `keel bearing lay` suggestion | Re-run lay or manually add lineage |
| Invalid goal reference | Goal ID not in epic PRD | Doctor hard-fail with offending token and PRD path | Update bearing goals or epic PRD |
| Pre-contract legacy bearing | Laid status + missing lineage fields | Doctor warning with migration command | Run migration/repair flow |
