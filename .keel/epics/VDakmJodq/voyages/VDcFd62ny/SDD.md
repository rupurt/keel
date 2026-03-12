# Automation Guide Authoring - Software Design Description

> Document the routine, gating, pulse, and scheduled-lane workflow for business automation users.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage structures the mission’s user-facing guidance around one canonical
document. The guide should explain the automation workflow in the same order an
operator experiences it: author a routine, review schedule state, run pulse,
and inspect scheduled work in flow.

## Context & Boundaries

The guide is documentation-only, but it must stay close to real command
behavior. It should not speculate about future daemons, migrations, or external
services that are not yet supported.

```
Routine authoring -> temporal review -> pulse execution -> scheduled flow review
                           \____________ GUIDE.md ____________/
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Routine, next, pulse, and flow command surfaces | Internal | Source accurate workflow steps and names | Current CLI behavior |
| Mission charter and epic PRDs | Internal | Anchor the guide to mission intent and scope | Mission `VDakm4zUQ` |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Document shape | One `GUIDE.md` organized by operator workflow | Minimizes hunting across multiple docs |
| Example style | Use one end-to-end recurring-work example | Makes abstract feature interactions concrete |
| Safety framing | Explicitly call out supported vs unsupported automation paths | Prevents the guide from implying unsupported behavior |

## Architecture

The guide is treated as a thin documentation projection over the implemented CLI
surfaces and the mission’s recurring-work model.

## Components

| Component | Responsibility |
|-----------|----------------|
| Workflow overview | Explain how routines, next, pulse, and flow relate |
| Worked example | Walk one recurring process through the system |
| Safety section | Describe cron/systemd expectations, idempotency, and boundaries |

## Interfaces

| Interface | Input | Output |
|-----------|-------|--------|
| `GUIDE.md` | Current mission workflow and command behavior | Human-readable automation guide |
| Review checklist | Guide sections vs supported commands | Alignment signal before submission |

## Data Flow

1. Gather the canonical commands and workflow boundaries from the mission artifacts.
2. Organize them into an operator-first narrative.
3. Validate that the guide’s command names and boundaries match supported CLI behavior.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Guide drifts from CLI behavior | Manual review or judge feedback | Treat as planning/doc defect | Update guide language before delivery starts |
| Example implies unsupported automation | Review against mission constraints | Rewrite example and add boundary note | Keep unsupported paths out of the guide |
