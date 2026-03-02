# Extract Shared Infrastructure And Repositories - Software Design Description

> Centralize filesystem persistence, frontmatter mutation, template rendering, and repository abstractions.

**SRS:** [SRS.md](SRS.md)

## Overview

Define infrastructure ports and implement filesystem adapters so command handlers and application services rely on abstractions instead of direct filesystem operations.

## Context & Boundaries

- In scope: repository traits, frontmatter/template services, fs adapters
- Out of scope: new business rules

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Markdown frontmatter parser | library module | parse/write entity metadata | existing parser module |
| `.keel` directory schema | data contract | repository path resolution | current board schema |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Repository boundary | per-aggregate repositories + board query facade | improves testability and ownership |
| Frontmatter updates | single service with typed mutations | removes duplicated string replacement |
| Template rendering | shared service in infrastructure layer | avoids cross-command helper coupling |

## Architecture

`application` -> `ports` -> `infrastructure::fs_adapters`.

## Components

- `BoardRepository` and aggregate-specific repositories
- `FrontmatterMutationService`
- `TemplateRenderService`
- Filesystem adapter implementations

## Interfaces

- Trait-based interfaces for load/save/find operations
- Structured mutation requests for status/timestamp/scope updates

## Data Flow

1. Use case requests repository/entity mutation
2. Adapter loads markdown and parses frontmatter
3. Mutation service applies typed change
4. Adapter persists update and returns result

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing entity path | repository lookup | return not-found domain error | caller selects fallback flow |
| Frontmatter parse failure | parse service | return validation error | fix invalid markdown and retry |
| Write failure | fs adapter | return contextual IO error | retry or user intervention |
