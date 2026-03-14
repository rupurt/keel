# Realize High-Density Views - Software Design Description

> Translate research findings into high-density TUI show surfaces

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage bridges the gap between research findings and product reality. It implements high-density layouts for the `show` command group and introduces formal "Mission Archetypes" to the TUI to improve strategic orientation.

## Context & Boundaries

- In-scope: `keel mission show`, `keel story show`, and `keel voyage show` rendering logic.
- Out-of-scope: Modifying the underlying data models (other than adding archetype metadata to charters).

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
