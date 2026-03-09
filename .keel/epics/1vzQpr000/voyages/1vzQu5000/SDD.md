# Evidence-Backed Assessment and Surfaces - Software Design Description

> Make bearing assessment and read surfaces evidence-backed and compute EV scores from evidence quality signals.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage consumes the renamed artifact contract and evidence model from the earlier voyages and makes them operational in three places:
- assessment authoring and parsing
- bearing-facing terminal/file surfaces
- readiness, doctor, and board-level scoring projections

The design keeps one canonical scoring path and one canonical read-model path. Assessment text cites evidence IDs, the scoring engine consumes evidence-quality signals, and bearing-facing surfaces summarize evidence provenance without duplicating parsing logic in command handlers.

## Context & Boundaries

```text
┌──────────────────────────────────────────────────────────────┐
│        evidence-backed assessment and bearing surfaces       │
│                                                              │
│  EVIDENCE.md ─┐                                              │
│               ├─> bearing assessment parser ─> EV scoring    │
│ ASSESSMENT.md ┘                    │             │           │
│                                    ├─> doctor    ├─> list    │
│                                    └─> show/file └─> flow    │
└──────────────────────────────────────────────────────────────┘
```

In scope:
- Citation-aware assessment parsing and validation.
- Evidence-quality-aware scoring.
- Evidence-backed rendering for `bearing show`, `bearing file`, and board-level projections that surface EV or readiness.

Out of scope:
- Provider ingestion internals.
- Artifact renaming and command cutover mechanics.
- Non-bearing consumers of the evidence model.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| evidence parser/model | internal service | Supply source metadata, IDs, and quality signals | prior-voyage canonical API |
| assessment scoring module | internal service | Extend factor scoring with evidence-quality inputs | existing crate API |
| bearing read models and renderers | internal service | Surface citations and evidence-backed summaries | existing crate API |
| doctor and flow projections | internal service | Gate readiness and render board-level support quality | existing crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Citation enforcement | Require evidence IDs for assessment conclusions in scope | Keeps recommendation logic grounded in inspectable sources |
| Scoring composition | Blend authored factors with derived evidence-quality factors rather than replacing authored factors outright | Preserves operator intent while grounding it in research quality |
| Surface density | Default to concise provenance summaries with drill-down through `bearing file` or evidence references | Keeps terminal use practical while preserving traceability |
| Canonical score path | Reuse one scoring engine for list, show, doctor, and any generated board summaries | Avoids score drift across surfaces |

## Architecture

The voyage should introduce or extend:
- assessment parser helpers that understand citation references and derived evidence-quality rollups
- scoring inputs that combine authored factor rows with evidence-derived metrics
- bearing read-model projections that summarize cited evidence and support quality
- doctor/readiness checks that fail when assessment claims are uncited, stale, contradictory, or structurally incomplete

Command adapters should stay thin. They call canonical read models or scoring services rather than parsing citations locally.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| Assessment citation parser | Extract cited source IDs and support metrics | parse helper / read model | Must tolerate compact markdown references without ambiguity |
| Evidence-aware score builder | Convert evidence quality into score inputs | scoring service | One canonical formula path |
| Bearing surface projection | Summarize provenance, support quality, and recommendation state | read model DTO | Feeds `show`, `file`, and list surfaces |
| Readiness/doctor evaluator | Gate decision readiness on evidence quality and citation completeness | diagnostics checks | Explicit failure modes, not advisory-only |

## Interfaces

Expected behavior after this voyage:
- `bearing show <id>` summarizes evidence support and cites source IDs for recommendation-facing content.
- `bearing file <id> ASSESSMENT` and `bearing file <id> EVIDENCE` remain the drill-down path for authored detail.
- EV scores visible in list/show or generated board surfaces change when evidence quality changes.
- Doctor explains missing citations, weak support, stale evidence, or contradictory support explicitly.

## Data Flow

1. Load `EVIDENCE.md` and `ASSESSMENT.md` for a bearing.
2. Parse evidence metadata and cited source references.
3. Derive support-quality aggregates such as evidence breadth, freshness profile, authority distribution, and contradiction/gap signals.
4. Feed those aggregates into the EV scoring model together with authored impact/confidence/effort/risk factors.
5. Project the result into `bearing show`, `bearing list`, doctor checks, and any board-level rollups.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Assessment claim lacks citation | assessment parse/doctor validation | Fail readiness and show actionable doctor error | Add canonical evidence references to the assessment |
| Cited evidence IDs do not exist | evidence cross-reference validation | Fail scoring/readiness and report dangling citation IDs | Repair citations or source records |
| Evidence is stale, weak, or contradictory | support-quality aggregation | Lower or block readiness score and surface explicit rationale | Refresh evidence set or narrow recommendation confidence |
| Surface output becomes too noisy | renderer width and summary rules | Collapse to compact summaries while preserving drill-down paths | Use `bearing file` for detailed authored context |
