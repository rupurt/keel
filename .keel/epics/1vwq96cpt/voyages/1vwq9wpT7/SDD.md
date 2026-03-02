# Migrate CLI Interfaces And Verification Coverage - Software Design Description

> Refactor CLI handlers to thin adapters and add verification suites that enforce architectural contracts.

**SRS:** [SRS.md](SRS.md)

## Overview

Finish the migration by making CLI dispatch an interface adapter over application services and read models, then lock architecture and behavior with robust verification suites.

## Context & Boundaries

- In scope: CLI adapter refactor, architecture tests, command regression tests, rollout docs
- Out of scope: introducing new command surface area

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Application services | internal API | command orchestration target | voyage 3 outputs |
| Read-model DTO services | internal API | diagnostics/query outputs | voyage 4 outputs |
| CI pipeline | tooling | execute contract and regression suites | existing CI |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| CLI responsibility | parse + delegate + render only | preserves clean layering |
| Verification strategy | architecture contracts + behavior parity | catches both structural and functional regressions |
| Migration execution | staged adapters per command group | reduces risk and review scope |

## Architecture

`interfaces::cli` delegates all business operations to `application` and all query projections to `read_model`. Verification suites enforce this contract.

## Components

- CLI dispatch adapters
- Command-group adapter modules
- Architecture contract test suite
- Command regression/snapshot suite
- Migration checklist document

## Interfaces

- Adapter input mapping from clap args to use-case DTOs
- Adapter output mapping from DTOs to terminal/json rendering

## Data Flow

1. CLI parses input
2. Adapter converts input to use-case query/command DTO
3. Application/read-model returns result DTO
4. Adapter renders output
5. Verification suite validates structure and parity

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Adapter delegates to forbidden module | architecture test failure | block merge | update adapter dependency |
| Behavior parity regression | regression test diff | block merge with diff artifact | adjust adapter mapping |
| Missing migration step | checklist validation | mark incomplete | complete required step and re-run suite |
