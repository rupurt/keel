# Routine Foundation - Software Design Description

> Establish the routine entity, storage integration, and authoring surfaces as the canonical recurring-work blueprint contract.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds routines as a first-class board artifact. The design keeps one
canonical bundle per routine, teaches the board loader and storage layer to
discover it, and exposes a minimal CLI surface for creation and inspection.

## Context & Boundaries

Routine is the mission-level source of truth for recurring work definitions. The
voyage stops short of temporal evaluation or work materialization; it only
establishes the durable contract that later epics consume.

```
┌──────────────────────────────────────────────────────────────┐
│                     Routine Foundation                       │
│  CLI new/list/show -> template/storage -> loader -> Board   │
│                              ↑                               │
│                    .keel/routines/<id>/README.md            │
└──────────────────────────────────────────────────────────────┘
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Board loader/store abstractions | Internal | Discover and persist routine bundles | Current board model APIs |
| Template rendering/frontmatter mutation | Internal | Scaffold authored routine bundles deterministically | Existing template helpers |
| CLI command tree and show/list formatting | Internal | Expose routine authoring/read surfaces | Current clap + presentation stack |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Routine artifact shape | One bundle under `.keel/routines/<id>/` with canonical authored content | Keeps recurring-work metadata and blueprint narrative together |
| Persistence path | Reuse existing board loader/storage patterns | Minimizes drift from other entity types |
| CLI scope | Start with `new/list/show` only | Enough authoring surface for downstream temporal and pulse epics |

## Architecture

The voyage introduces a routine model/frontmatter type, loader/store support,
and CLI command handlers that all read and write the same bundle contract.

## Components

| Component | Responsibility |
|-----------|----------------|
| Routine model/frontmatter | Represent cadence, target scope, and blueprint metadata |
| Routine loader/storage | Discover bundles, parse frontmatter, and persist changes |
| Routine templates | Generate human-editable routine bundle scaffolds |
| Routine CLI handlers | Create, list, and show routine bundles through canonical APIs |

## Interfaces

| Interface | Input | Output |
|-----------|-------|--------|
| `keel routine new` | Title plus required routine metadata | New bundle scaffold in `.keel/routines/` |
| `keel routine list` | Optional filters | Ordered routine summaries |
| `keel routine show <id>` | Routine id | Full routine detail from canonical storage |

## Data Flow

1. `routine new` renders a template bundle and writes it through storage.
2. Board loading discovers routine bundles and parses them into the board model.
3. `routine list/show` read from the same loaded contract and render operator-facing output.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Malformed routine bundle | Frontmatter parse/load failure | Report actionable validation error | Fix bundle content and rerun command |
| Missing routine id | Lookup failure | Show not-found guidance | Use `routine list` to discover valid ids |
| Duplicate/invalid persistence write | Storage guardrails | Abort write without partial mutation | Correct conflicting routine state, then retry |
