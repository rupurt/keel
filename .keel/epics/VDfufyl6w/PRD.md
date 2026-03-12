# Canonical Knowledge Graph World Map - Product Requirements

## Problem Statement

Keel has a rich implicit graph across entities, authored documents, knowledge, and source code, but there is no canonical knowledge-graph projection or cached semantic layer that can render the whole world, explain drift between board intent and project reality, or support a deterministic doctor-grade graph check.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Build one canonical world model for entities, authored artifacts, knowledge units, project documents, and source code. | `keel knowledge graph` and adjacent read models consume one deterministic projection instead of ad hoc scanners. | 100% of graph-facing command paths use the canonical projection |
| GOAL-02 | Add a deterministic repo-local cache and semantic substrate for graph exploration. | Unchanged content reuses `.keel/cache/knowledge-graph/` artifacts without changing structural graph correctness or doctor output. | Stable cache reuse in automated regression coverage |
| GOAL-03 | Surface a structural drift coefficient that shows how far code and documents have moved from board intent. | Topology and show-style read surfaces can display a stable drift signal sourced from the graph kernel. | Drift signal available in automated read-model coverage and CLI output |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Mission Steward | The planner or harness coordinating broad change across the board. | A whole-project graph that makes drift and missing lineage obvious without reading dozens of files. |
| Operator | The person executing stories and reading terminal surfaces. | An interactive map that explains where work, knowledge, and code are clustered and where drift is forming. |
| Architect | The person reviewing system coherence and long-range structure. | Deterministic graph and drift signals that can support doctor checks and architecture discussions. |

## Scope

### In Scope

- [SCOPE-01] Canonical graph nodes and structural edges for board entities, authored planning artifacts, knowledge units, project documents, and selected source code files.
- [SCOPE-02] Semantic proximity metadata and deterministic local embeddings cached under `.keel/cache/knowledge-graph/`.
- [SCOPE-03] Interactive and static `keel knowledge graph` visualization with zoom/focus concepts aligned to the existing `txtplot` world-map experience.
- [SCOPE-04] Structural drift coefficient derivation and surfacing through topology- and show-oriented read models.
- [SCOPE-05] Shared graph projections that adjacent commands can reuse instead of recomputing custom relationship views.

### Out of Scope

- [SCOPE-06] Treating semantic similarity as a doctor gate or workflow-enforcement rule.
- [SCOPE-07] Cloud-hosted embedding APIs or non-local inference services.
- [SCOPE-08] Replacing `keel topology`; this epic extends and cross-feeds existing world-map surfaces rather than deleting them.
- [SCOPE-09] Nondeterministic layout algorithms such as `t-SNE` as the canonical graph source of truth.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Build a canonical knowledge-graph projection that unifies board entities, authored artifacts, knowledge units, project documents, and selected source code into one deterministic node/edge model. | GOAL-01 | must | Keel already has an implied graph, but each surface rebuilds pieces of it independently, which causes drift. |
| FR-02 | Represent structural and semantic relationships separately, with structural edges authoritative and semantic edges advisory only. | GOAL-01, GOAL-02 | must | Doctor-grade checks need deterministic structure, while exploration still benefits from semantic neighborhoods. |
| FR-03 | Add a deterministic repo-local cache under `.keel/cache/knowledge-graph/` with manifest keys, content-addressed blobs, and embedding metadata/versioning suitable for incremental reuse. | GOAL-02 | must | The graph should be fast enough to explore repeatedly without rescanning or recomputing everything every run. |
| FR-04 | Implement `keel knowledge graph` as an interactive terminal world map with a stable `--static` mode for harnesses and snapshots. | GOAL-01, GOAL-03 | must | The graph needs both a human-exploration path and a deterministic non-interactive rendering path. |
| FR-05 | Compute a structural drift coefficient between official board intent, authored documents, and source-code reality, and make that signal available to topology and show-style read surfaces. | GOAL-03 | must | Drift needs to become visible in the daily interface, not buried in one-off diagnostics. |
| FR-06 | Reuse the graph and drift projection from adjacent topology/show experiences instead of introducing duplicate command-local relationship logic. | GOAL-01, GOAL-03 | should | A canonical graph only pays off if other surfaces consume it instead of rebuilding partial views. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Equivalent board states and unchanged inputs must produce stable node ordering, edge ordering, cache manifests, and static graph output. | GOAL-01, GOAL-02, GOAL-03 | must | The graph will be reused by doctor and snapshot-style surfaces, so determinism is non-negotiable. |
| NFR-02 | The knowledge graph and embedding path must run fully offline in local Rust using a lightweight Candle-backed model and repo-local cache. | GOAL-02 | must | The user explicitly wants local execution and build-friendly caching rather than service dependencies. |
| NFR-03 | Semantic proximity must never alter structural doctor outcomes, mission gates, or lifecycle transitions. | GOAL-01, GOAL-03 | must | Advisory similarity is useful for exploration but unsafe as an invariant. |
| NFR-04 | Interactive rendering must degrade gracefully to a concise, deterministic `--static` view suitable for harnesses and snapshots. | GOAL-01, GOAL-03 | must | Terminal exploration should not compromise automation friendliness. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Canonical graph projection | Read-model and cache tests over equivalent board states and repeated runs | Story-level targeted proofs plus `just test` |
| Interactive/static command path | CLI and renderer tests plus terminal smoke runs | `cargo run -- knowledge graph --static` and story-level proofs |
| Drift coefficient derivation | Read-model tests that compare board/doc/code mismatch scenarios | Story-level proofs plus `just test` |
| Repo safety | Full hygiene and doctor checks on the real board | `just quality`, `just test`, `just doctest`, `just keel doctor` |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A lightweight local embedding model is sufficient for advisory neighborhoods and does not need remote quality parity. | Semantic exploration may be weaker than desired, but structural graph work remains valuable. | Start with deterministic local embeddings and evaluate usefulness from the interactive view. |
| Structural drift can be approximated from graph mismatches between board entities, authored documents, and source attachments before deeper semantic analysis exists. | The first drift coefficient might be coarse or noisy. | Keep the coefficient structural-only in this epic and refine its heuristics through tests and UI review. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which source-code granularity is most useful in the graph without overwhelming the terminal view: files only, or files plus selected symbols? | Epic owner | Open |
| Whether the first local embedding model is fast enough on commodity laptops when the repo grows. | Epic owner | Open |
| How aggressively topology/show surfaces should summarize drift without making the metric feel decorative. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel knowledge graph` can render a whole-project graph that includes entities, artifacts, knowledge units, project documents, and source-code nodes.
- [ ] The graph cache under `.keel/cache/knowledge-graph/` reuses deterministic artifacts on unchanged input and keeps semantic metadata local.
- [ ] A structural drift coefficient is available from canonical read models and is surfaced in graph-adjacent interfaces such as topology/show experiences.
- [ ] `just keel doctor`, `just quality`, `just test`, and `just doctest` remain clean after the graph surfaces land.
<!-- END SUCCESS_CRITERIA -->
