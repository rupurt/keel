---
created_at: 2026-03-02T10:07:49
---

# Knowledge - 1vwq9RqCe

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Extract Frontmatter Mutation Service (1vwqCeiHm)

### 1vyDuwJXq: Declarative Frontmatter Patches Reduce Drift Across Commands

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple commands had bespoke line-replacement logic for status/scope/timestamp edits, increasing drift risk and maintenance overhead. |
| **Insight** | A shared mutation service with `set/remove` operations preserves behavior while eliminating duplicated frontmatter edit loops. |
| **Suggested Action** | Route future frontmatter changes through shared mutation primitives and add service-level tests for insertion/replacement/removal semantics. |
| **Applies To** | src/infrastructure/frontmatter_mutation.rs, src/commands/story/{link,unlink}.rs, src/commands/{adr,bearing}/mod.rs, src/application/voyage_epic_lifecycle.rs |
| **Applied** | Migrated status/timestamp/scope mutations to infrastructure::frontmatter_mutation::apply. |



---

## Story: Implement Filesystem Adapter Layer (1vwqCeXD8)

### 1vyDuwPS4: Frontmatter-rewrite adapters preserve markdown parity with low migration risk

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Implementing filesystem repository/document adapters over existing `.keel` markdown files without changing domain/application behavior. |
| **Insight** | Parsing existing frontmatter, serializing updated typed frontmatter, and reattaching the original body provides a practical parity-preserving persistence strategy while introducing port-based boundaries. |
| **Suggested Action** | Reuse this adapter pattern for future repository migrations and add command-level integration points incrementally to avoid broad behavior shifts. |
| **Applies To** | `src/infrastructure/fs_adapters.rs`, `src/application/ports.rs`, markdown-backed aggregate repositories |
| **Applied** | story `1vwqCeXD8` |



---

## Story: Define Repository Port Interfaces (1vwqCe8MK)

### 1vyDuwLCH: Ports Should Mirror Aggregate Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining persistence abstractions before filesystem adapters are extracted from command modules |
| **Insight** | Repository ports are easier to evolve when contracts are grouped by aggregate boundary (story, voyage, epic, bearing, adr) plus one board snapshot port for orchestration use cases. |
| **Suggested Action** | Keep port interfaces in the application layer and defer adapter wiring to subsequent stories to minimize behavior risk during migration. |
| **Applies To** | src/application/ports.rs, upcoming infrastructure adapter stories |
| **Applied** | yes |



---

## Story: Extract Template Rendering Service (1vwqCeX9I)

### 1vyDuwrqB: Shared template rendering reduces cross-command coupling

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple creation paths (story, epic, voyage, bearing, ADR, transitions) performing placeholder substitution |
| **Insight** | Keeping placeholder substitution in command-local helpers increases coupling and makes cross-command refactors noisier than necessary. |
| **Suggested Action** | Route all template substitution through `infrastructure::template_rendering::render` and enforce usage with architecture contract tests. |
| **Applies To** | `src/infrastructure/template_rendering.rs`, `src/commands/*/new.rs`, `src/commands/story/reflect.rs`, `src/transitions/bearing_engine.rs` |
| **Applied** | story `1vwqCeX9I` |



---

## Synthesis

### YerUoeCw5: Declarative Frontmatter Patches Reduce Drift Across Commands

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple commands had bespoke line-replacement logic for status/scope/timestamp edits, increasing drift risk and maintenance overhead. |
| **Insight** | A shared mutation service with `set/remove` operations preserves behavior while eliminating duplicated frontmatter edit loops. |
| **Suggested Action** | Route future frontmatter changes through shared mutation primitives and add service-level tests for insertion/replacement/removal semantics. |
| **Applies To** | src/infrastructure/frontmatter_mutation.rs, src/commands/story/{link,unlink}.rs, src/commands/{adr,bearing}/mod.rs, src/application/voyage_epic_lifecycle.rs |
| **Linked Knowledge IDs** | 1vyDuwJXq |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | Migrated status/timestamp/scope mutations to infrastructure::frontmatter_mutation::apply. |

### hpVUqQ88C: Frontmatter-rewrite adapters preserve markdown parity with low migration risk

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Implementing filesystem repository/document adapters over existing `.keel` markdown files without changing domain/application behavior. |
| **Insight** | Parsing existing frontmatter, serializing updated typed frontmatter, and reattaching the original body provides a practical parity-preserving persistence strategy while introducing port-based boundaries. |
| **Suggested Action** | Reuse this adapter pattern for future repository migrations and add command-level integration points incrementally to avoid broad behavior shifts. |
| **Applies To** | `src/infrastructure/fs_adapters.rs`, `src/application/ports.rs`, markdown-backed aggregate repositories |
| **Linked Knowledge IDs** | 1vyDuwPS4 |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCeXD8` |

### xds3XilMh: Ports Should Mirror Aggregate Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining persistence abstractions before filesystem adapters are extracted from command modules |
| **Insight** | Repository ports are easier to evolve when contracts are grouped by aggregate boundary (story, voyage, epic, bearing, adr) plus one board snapshot port for orchestration use cases. |
| **Suggested Action** | Keep port interfaces in the application layer and defer adapter wiring to subsequent stories to minimize behavior risk during migration. |
| **Applies To** | src/application/ports.rs, upcoming infrastructure adapter stories |
| **Linked Knowledge IDs** | 1vyDuwLCH |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | yes |

### rbij9ueSM: Shared template rendering reduces cross-command coupling

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple creation paths (story, epic, voyage, bearing, ADR, transitions) performing placeholder substitution |
| **Insight** | Keeping placeholder substitution in command-local helpers increases coupling and makes cross-command refactors noisier than necessary. |
| **Suggested Action** | Route all template substitution through `infrastructure::template_rendering::render` and enforce usage with architecture contract tests. |
| **Applies To** | `src/infrastructure/template_rendering.rs`, `src/commands/*/new.rs`, `src/commands/story/reflect.rs`, `src/transitions/bearing_engine.rs` |
| **Linked Knowledge IDs** | 1vyDuwrqB |
| **Score** | 0.88 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCeX9I` |

