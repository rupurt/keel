# Implement CLI and Status Rendering - Software Design Description

> Provide high-density compact status summary

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds a `--status` flag to the `keel mission next` command. When this flag is present, the system calculates the next steps for all active roles in the mission's topology, deduplicates them, and ranks them by priority. It then renders exactly three high-density, action-oriented bullets.

## Context & Boundaries

- In-scope: `keel mission next --status` flag and rendering.
- Out-of-scope: Modifying the default `keel mission next` or other command outputs.

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
