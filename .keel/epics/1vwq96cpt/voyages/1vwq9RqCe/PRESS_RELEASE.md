# PRESS RELEASE: Extract Shared Infrastructure And Repositories

## Overview

## Narrative Summary
### Extract Template Rendering Service
Move template rendering into a shared infrastructure rendering service.

### Define Repository Port Interfaces
Define repository ports that abstract board entity persistence operations.

### Implement Filesystem Adapter Layer
Implement filesystem adapters for repository and document services.

### Extract Frontmatter Mutation Service
Extract frontmatter mutation behavior into a shared infrastructure service.

## Key Insights
### Insights from Extract Template Rendering Service
# Reflection - Extract Template Rendering Service

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Shared template rendering reduces cross-command coupling

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple creation paths (story, epic, voyage, bearing, ADR, transitions) performing placeholder substitution |
| **Insight** | Keeping placeholder substitution in command-local helpers increases coupling and makes cross-command refactors noisier than necessary. |
| **Suggested Action** | Route all template substitution through `infrastructure::template_rendering::render` and enforce usage with architecture contract tests. |
| **Applies To** | `src/infrastructure/template_rendering.rs`, `src/commands/*/new.rs`, `src/commands/story/reflect.rs`, `src/transitions/bearing_engine.rs` |
| **Observed At** | 2026-03-02T17:22:22Z |
| **Score** | 0.88 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCeX9I` |

## Observations

The migration was low risk because template rendering had a tiny, deterministic surface and could be moved without changing frontmatter mutation behavior.
Adding a contract test for creation paths prevented regression back to `story::new::render_template` and gave concrete verification for the AC.

### Insights from Define Repository Port Interfaces
# Reflection - Define Repository Port Interfaces

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Ports Should Mirror Aggregate Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining persistence abstractions before filesystem adapters are extracted from command modules |
| **Insight** | Repository ports are easier to evolve when contracts are grouped by aggregate boundary (story, voyage, epic, bearing, adr) plus one board snapshot port for orchestration use cases. |
| **Suggested Action** | Keep port interfaces in the application layer and defer adapter wiring to subsequent stories to minimize behavior risk during migration. |
| **Applies To** | src/application/ports.rs, upcoming infrastructure adapter stories |
| **Observed At** | 2026-03-02T01:29:45Z |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | yes |

## Observations

The ports were straightforward once aggregate boundaries were explicit in the voyage SRS. The main design tradeoff was including both per-aggregate repositories and a board-level snapshot port to support existing orchestration flows.

### Insights from Implement Filesystem Adapter Layer
# Reflection - Implement Filesystem Adapter Layer

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Frontmatter-rewrite adapters preserve markdown parity with low migration risk

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Implementing filesystem repository/document adapters over existing `.keel` markdown files without changing domain/application behavior. |
| **Insight** | Parsing existing frontmatter, serializing updated typed frontmatter, and reattaching the original body provides a practical parity-preserving persistence strategy while introducing port-based boundaries. |
| **Suggested Action** | Reuse this adapter pattern for future repository migrations and add command-level integration points incrementally to avoid broad behavior shifts. |
| **Applies To** | `src/infrastructure/fs_adapters.rs`, `src/application/ports.rs`, markdown-backed aggregate repositories |
| **Observed At** | 2026-03-02T18:05:22Z |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCeXD8` |

## Observations

Adding adapter-level tests against `TestBoardBuilder` made behavior parity verification fast and deterministic.
The main friction was strict `dead_code` quality gates in a staged migration; targeted allowances were needed until application services consume the new adapter directly.

### Insights from Extract Frontmatter Mutation Service
# Reflection - Extract Frontmatter Mutation Service

## Knowledge

### L001: Declarative Frontmatter Patches Reduce Drift Across Commands

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple commands had bespoke line-replacement logic for status/scope/timestamp edits, increasing drift risk and maintenance overhead. |
| **Insight** | A shared mutation service with `set/remove` operations preserves behavior while eliminating duplicated frontmatter edit loops. |
| **Suggested Action** | Route future frontmatter changes through shared mutation primitives and add service-level tests for insertion/replacement/removal semantics. |
| **Applies To** | src/infrastructure/frontmatter_mutation.rs, src/commands/story/{link,unlink}.rs, src/commands/{adr,bearing}/mod.rs, src/application/voyage_epic_lifecycle.rs |
| **Observed At** | 2026-03-02T15:56:05Z |
| **Score** | 0.82 |
| **Confidence** | 0.9 |
| **Applied** | Migrated status/timestamp/scope mutations to infrastructure::frontmatter_mutation::apply. |

## Observations

Migration was straightforward once mutation semantics were encoded as reusable `set/remove` operations.
The highest-risk area was preserving existing ADR supersede behavior; expressing list updates as explicit field replacement avoided brittle substring replacements.
Full-suite tests provided confidence that existing command behavior stayed intact after centralizing mutation logic.

## Verification Proof
### Proof for Extract Template Rendering Service
- [ac-1.log](../../../../stories/1vwqCeX9I/EVIDENCE/ac-1.log)

### Proof for Define Repository Port Interfaces
- [ac-1.log](../../../../stories/1vwqCe8MK/EVIDENCE/ac-1.log)

### Proof for Implement Filesystem Adapter Layer
- [ac-1.log](../../../../stories/1vwqCeXD8/EVIDENCE/ac-1.log)

### Proof for Extract Frontmatter Mutation Service
- [ac-1.log](../../../../stories/1vwqCeiHm/EVIDENCE/ac-1.log)

