# Semantic Search and Ranking in Keel - Product Requirements

> Implementing semantic search with a pure-Rust, in-process embedding and vector
search stack will materially improve discovery on large Keel boards without
breaking the standalone, statically linked distribution model.

## Problem Statement

Current search in Keel is limited to simple case-insensitive substring matching
on IDs and titles. That misses semantic context, fails to rank near matches by
relevance, and makes large boards progressively harder to navigate as the
artifact graph grows.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Validate bearing recommendation in delivery flow | Adoption signal | Initial rollout complete |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Product/Delivery Owner | Coordinates planning and execution | Reliable strategic direction |

## Scope

### In Scope

- [SCOPE-01] Core semantic search and ranking slice that improves discovery for large Keel boards.

### Out of Scope

- [SCOPE-02] General search UI redesign outside the existing discovery flow.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Implement the core user workflow identified in bearing research. | GOAL-01 | must | Converts research recommendation into executable product capability. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Ensure deterministic behavior and operational visibility for the delivered workflow. | GOAL-01 | must | Keeps delivery safe and auditable during rollout. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Prove functional behavior through story-level verification evidence mapped to voyage requirements.
- Validate non-functional posture with operational checks and documented artifacts.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Bearing findings reflect current user needs | Scope may need re-planning | Re-check feedback during first voyage |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which rollout constraints should gate broader adoption? | Product | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Search queries can return semantically related results, not just literal
- [ ] Relevance ranking improves operator discovery without introducing an
- [ ] The chosen approach remains compatible with a statically linked Keel
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

### Findings
- Local semantic ranking is feasible without external infrastructure when we pair embedding generation with in-process indexing [SRC-01][SRC-02][SRC-03]
- The workflow direction matches current operator expectations for lightweight, local-first developer tooling [SRC-04]

### Opportunity Cost
Developing semantic search delays other features like graph visualization improvements or better ADR management, but the ROI in knowledge discovery is high [SRC-02][SRC-04].

### Dependencies
- **Model weights**: Reliable mechanism for model weight distribution (e.g., downloading to `.keel/cache/models/`) [SRC-02].
- **Rust Toolchain**: Statically linked binary requires careful dependency management to avoid dynamic libraries (especially for `fastembed-rs` + `candle`) [SRC-02][SRC-03].

### Alternatives Considered
- **Standard fuzzy search (current)**: Simple but misses semantic context (e.g., searching for "bug" doesn't find "crash") [SRC-04].
- **External database (Qdrant/Milvus)**: Overkill for Keel's standalone philosophy and introduces infrastructure overhead [SRC-03].

---

*This PRD was seeded from bearing `1w5H2Bq9L`. See `bearings/1w5H2Bq9L/` for original research.*
