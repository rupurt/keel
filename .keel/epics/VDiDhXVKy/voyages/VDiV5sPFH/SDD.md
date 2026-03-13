# AttachMissionBearingCommand - Software Design Description

> GOAL-01

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a dedicated attach command path so mission stewards can explicitly assign a bearing to a mission and record lineage in board state during the command flow. It will produce deterministic updates in both mission and bearing metadata and route failures to actionable guidance.

## Context & Boundaries

Scope is limited to mission and bearing board metadata. Inputs are existing `mission` and `bearing` entities; no external services are introduced.

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │         │  │         │  │         │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑
   [External]      [External]
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Board loader/store | Internal | Persist mission-bearing lineage updates during transitions | Current repository interfaces |

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Command shape | Add/extend mission/bearing command handling in one explicit attach flow | Removes ambiguity from freeform edits and keeps command transitions deterministic |
| Error handling | Use explicit guardrails with recovery guidance before any partial write | Prevents inconsistent board state and improves operator observability |

## Story Mapping

| Story | Requirement Mapping |
|-------|--------------------|
| Attach command implementation | SRS-01 |
| Readiness/activation wiring | SRS-02 |
| Guidance UX and failure handling | SRS-NFR-01 |

## Architecture

- Command layer parses mission-bearing attach input and validates identifiers.
- Domain service updates mission/bearing lineage and status transitions.
- Board projection updates mission and bearing summaries for `mission show`, `flow`, and doctor checks.

### Interfaces

- Mission command accepts mission id + bearing id and optional confirmation flags.

## Components

## Data Flow

- Validate command input.
- Resolve mission and bearing entities from board.
- Enforce valid states and ownership policy.
- Write lineage update in mission and bearing records.
- Regenerate affected artifacts and caches where required.

## Error Handling


| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Bearing already attached | Board lookup + existing lineage check | Report no-op semantics with explicit current owner | Skip write and present resolution command |
| Missing/invalid ids | Entity lookup failure | Return actionable error with valid-id hints | Re-run command with corrected identifiers |
