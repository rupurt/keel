# Introduce Application Services And Process Managers - Software Design Description

> Move orchestration out of command handlers into application use cases with explicit process managers.

**SRS:** [SRS.md](SRS.md)

## Overview

Replace command-to-command orchestration with use-case services. Cross-aggregate flows are handled by process managers triggered from domain events and executed through repository ports.

## Context & Boundaries

- In scope: use-case services, domain events, process managers, command adapter rewiring
- Out of scope: replacing core state machine semantics

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Transition enforcement services | domain service | validate legal/gated transitions | current state machine contracts |
| Repository ports | infrastructure port | persistent load/save of aggregates | voyage 2 outputs |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Orchestration host | application services | keeps domain pure and interfaces thin |
| Cross-aggregate coordination | process managers + events | explicit, testable workflows |
| Compatibility strategy | behavior-preserving migration | reduces rollout risk |

## Architecture

CLI adapters call use cases. Use cases invoke domain policies and repositories. Process managers subscribe to domain events and trigger secondary use cases.

## Components

- Lifecycle use-case services
- Domain event definitions
- Process manager implementations
- Adapter wiring for CLI commands

## Interfaces

- Use-case input/output DTOs
- Event emission interfaces
- Process manager handler contracts

## Data Flow

1. CLI parses command and invokes use case
2. Use case validates transition and persists state
3. Use case emits event when cross-aggregate action is required
4. Process manager handles event and invokes secondary use case

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Transition policy failure | domain service result | return structured application error | user resolves unmet gate |
| Process manager secondary failure | handler result | log and return partial-failure context | manual replay path |
| Event emission failure | event adapter | fail use case transaction | retry with idempotency guard |
