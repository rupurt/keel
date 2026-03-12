# Build Knowledge Graph Kernel and Cache - SRS

## Summary

Epic: VDfufyl6w
Goal: Create the first deterministic knowledge-graph projection and cache substrate so higher-level graph rendering and drift analysis can reuse one canonical indexed world model.

## Scope

### In Scope

- [SCOPE-01] Canonical graph nodes for board entities, authored planning artifacts, knowledge units, project documents, and selected source files.
- [SCOPE-02] Deterministic structural edges and provenance relationships needed to compare board intent, documents, and code.
- [SCOPE-03] Deterministic cache manifest and content-addressed blob layout under `.keel/cache/knowledge-graph/`, including embedding metadata/version fields.
- [SCOPE-04] Projection APIs that expose structural drift inputs for later topology/show rendering and doctor-style checks.

### Out of Scope

- [SCOPE-05] Interactive `txtplot` rendering and command UX for `keel knowledge graph`.
- [SCOPE-06] Surfacing the drift coefficient inside topology/show commands.
- [SCOPE-07] Structural drift as a doctor gate or semantic similarity as an invariant.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The voyage must emit deterministic graph nodes for missions, epics, bearings, ADRs, voyages, stories, authored artifact files, knowledge units, project docs, and selected source files. | SCOPE-01 | FR-01 | automated |
| SRS-02 | The voyage must emit deterministic structural edges for containment, lineage, provenance, governance, traceability, and document/source attachment relationships. | SCOPE-01, SCOPE-02, SCOPE-04 | FR-01 | automated |
| SRS-03 | The voyage must persist a deterministic cache manifest and content-addressed blob references under `.keel/cache/knowledge-graph/` keyed by graph inputs. | SCOPE-03 | FR-03 | automated |
| SRS-04 | The cache layout must record embedding/model metadata in a way that supports local semantic reuse without changing structural graph correctness when embeddings are absent or stale. | SCOPE-03 | FR-02 | automated |
| SRS-05 | The projection must expose structural drift inputs that compare board entities to document and source-code attachment coverage so later renderer and doctor work can reuse one canonical signal source. | SCOPE-02, SCOPE-04 | FR-05 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent board states and unchanged content must produce stable node ordering, edge ordering, and cache artifacts across repeated runs. | SCOPE-01, SCOPE-02, SCOPE-03, SCOPE-04 | NFR-01 | automated |
| SRS-NFR-02 | This voyage must stay fully local/offline and must not require remote embedding or indexing services. | SCOPE-03 | NFR-02 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
