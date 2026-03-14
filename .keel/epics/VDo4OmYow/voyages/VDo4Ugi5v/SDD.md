# Implement Inquiry Personas - Software Design Description

> Add Student and Interrogator personas to theater mode

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage implements the "Student" and "Interrogator" personas within the `keel play --theater` runtime. These personas are designed to use the `FORMAL_RULES.md` and `CONSTITUTION.md` as their primary source of truth, shifting the theater from a narrative performance to an interactive inquiry session.

## Context & Boundaries

- In-scope: Theater persona definitions and prompt grounding.
- Out-of-scope: Modifying the core TUI renderer or marionette-orchestration.

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

<!-- External systems, libraries, services this design relies on -->

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|

## Architecture

<!-- Component relationships, layers, modules -->

## Components

<!-- For each major component: purpose, interface, behavior -->

## Interfaces

<!-- API contracts, message formats, protocols (if this voyage exposes/consumes APIs) -->

## Data Flow

<!-- How data moves through the system; sequence diagrams if helpful -->

## Error Handling

<!-- What can go wrong, how we detect it, how we recover -->

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
