# Evidence Capture and Provider Signals - Software Design Description

> Add first-class evidence capture, provider provenance, and configurable research-source weighting for bearing research.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces the canonical evidence model that sits behind bearing research. It defines the `EVIDENCE.md` structure, a provider registry and configuration layer, deterministic ingestion of multiple signal classes, and explicit provider-state reporting when evidence cannot be gathered automatically.

The design separates three concerns:
- source modeling and storage
- provider orchestration and provenance capture
- ranking/weighting configuration that later assessment logic can consume

## Context & Boundaries

```text
┌──────────────────────────────────────────────────────────────┐
│                 bearing evidence capture layer               │
│                                                              │
│  research command ─> provider registry ─> evidence records   │
│          │                    │                 │            │
│          │                    ├─ web            ├─ source IDs│
│          │                    ├─ academic       ├─ metadata  │
│          │                    ├─ social         ├─ provenance│
│          │                    └─ manual         └─ ranking   │
│          └────────────────────────────> `keel.toml` weights  │
└──────────────────────────────────────────────────────────────┘
```

### In Scope
- Structured evidence records and parser/renderer support.
- Provider registry, status reporting, and deterministic fixture-backed adapters.
- Configurable weighting metadata for evidence ordering and downstream scoring.

### Out of Scope
- Hard cutover naming and lifecycle migration.
- Assessment/read-surface scoring logic.
- Broad paid-provider auth flows for every external service.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| bearing lifecycle/read surfaces | internal service | Consume the new evidence artifact once the cutover voyage lands | existing crate API |
| config loading and `keel.toml` schema | internal service | Store provider enablement and weighting heuristics | existing crate API |
| markdown/file adapters | internal service | Persist and load structured evidence documents | existing crate API |
| web-backed or fixture-backed provider clients | external/adapter layer | Supply evidence items for web, academic, social, and manual sources | deterministic adapter contract |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Evidence store | Keep one canonical `EVIDENCE.md` artifact with structured source rows | Makes citations, reviews, and diffs human-visible |
| Provider abstraction | Use a registry with explicit provider status and deterministic adapter outputs | Allows multiple signal classes without coupling command code to each source |
| Manual evidence | Treat manual/internal evidence as a first-class provider class | Keeps all evidence on one schema and one citation model |
| Weighting controls | Store per-provider or per-source-class weights in config | Lets projects bias toward official docs, papers, or social signal explicitly |

## Architecture

Core modules to introduce or extend:
- evidence model/parser module
  Defines source IDs, source classes, metadata, and serialization to `EVIDENCE.md`.
- provider registry
  Resolves enabled providers, executes supported lookups, and reports disabled/unavailable states.
- research command/service
  Orchestrates evidence capture requests and persists normalized source records.
- config integration
  Loads enablement and weighting settings for providers or source classes.

The storage contract should keep normalized records simple and durable. Provider-specific data can live in optional metadata fields or normalized notes, but the top-level schema must remain provider-agnostic enough that later scoring and surface code can reason about evidence consistently.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| Evidence record model | Canonical source schema | parse/render helpers | One format across all signal classes |
| Provider registry | Resolve enabled research providers | `list_enabled`, `run`, `status` | Explicit disabled/unavailable reporting |
| Research capture service | Convert provider outputs into evidence records | `capture(bearing, query/options)` | Thin orchestrator over providers |
| Config weighting layer | Apply source/provider weighting rules | config DTO + ranking helper | Deterministic and inspectable |
| Manual evidence adapter | Record internal or hand-curated sources | same capture service path | Avoids a separate non-canonical workflow |

## Interfaces

Expected operator-facing capabilities after this voyage:
- a research command path that can append or refresh evidence for a bearing
- `EVIDENCE.md` rows with stable IDs and metadata
- config sections that show provider status and weighting rules

Provider result normalization should include:
- source ID
- source class
- provider name
- title or label
- location (`url`, internal path, or manual origin)
- published/observed and retrieved timestamps
- authority/freshness indicators
- summary/claim notes

## Data Flow

1. Operator or agent invokes the research capture path for a bearing.
2. The service resolves enabled providers and applicable source classes from config.
3. Each provider returns normalized evidence candidates or an explicit unavailable/disabled state.
4. The capture service assigns stable source IDs and writes canonical evidence records into `EVIDENCE.md`.
5. Ranking metadata is derived deterministically from config and stored or exposed for downstream surfaces.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Provider disabled in config | registry/config lookup | Report disabled state explicitly and skip capture | Re-enable provider in `keel.toml` if desired |
| Provider unavailable or unsupported | adapter runtime or capability check | Emit explicit unavailable status; do not fabricate evidence | Fall back to another provider or manual evidence entry |
| Provider returns incomplete metadata | normalization validation | Persist only valid fields and mark missing metadata explicitly | Add manual completion or improve provider mapping |
| Duplicate or conflicting evidence IDs | persistence validation | Reject non-deterministic write and regenerate through canonical ID assignment | Rerun through canonical capture path |
