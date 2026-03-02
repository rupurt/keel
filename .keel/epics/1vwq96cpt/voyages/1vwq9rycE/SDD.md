# Consolidate Read Models And Queue Policies - Software Design Description

> Unify flow, status, next, and capacity projections behind canonical read models and policies.

**SRS:** [SRS.md](SRS.md)

## Overview

Move all projection logic into canonical read-model services and consume those services from command-facing renderers. This removes duplicated structs and divergent calculations.

## Context & Boundaries

- In scope: projection services, policy API reuse, adapter updates
- Out of scope: mutation/write-model workflows

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Board aggregate view | model | projection input | current board model |
| Queue policy module | domain policy | shared classification | existing policy contracts |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Projection ownership | read-model layer | single source of operational truth |
| Command integration | adapter-only formatting in command layer | avoids business logic duplication |
| Parity strategy | snapshot/parity tests before cleanup | safe migration |

## Architecture

`read_model::projections` provides flow/status/next/capacity DTOs. `interfaces::cli` commands consume DTOs and format output.

## Components

- Flow/status projection service
- Capacity projection service
- Shared queue-policy facade
- Command-format adapters

## Interfaces

- Read-model query APIs returning typed DTOs
- Formatter functions that accept DTOs only

## Data Flow

1. Command requests projection DTO from read-model service
2. Service computes deterministic metrics from board snapshot
3. Command formatter renders DTO to terminal/json

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Projection mismatch during migration | parity test failure | block merge and report diff | update projection or formatter |
| Missing policy reuse | import contract failure | fail architecture test | migrate consumer to shared policy |
| DTO contract drift | compile/test failure | adjust adapter contract | version API and migrate callers |
