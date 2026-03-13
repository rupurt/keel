# Render Knowledge Graph Command and Drift Surfaces - SRS

## Summary

Epic: VDg0dAPVS
Goal: Ship the first interactive/static keel knowledge graph experience and thread the structural drift coefficient into graph-adjacent read surfaces.

## Scope

### In Scope

- [SCOPE-01] Interactive `keel knowledge graph` rendering backed by the canonical knowledge-graph projection.
- [SCOPE-02] Deterministic `keel knowledge graph --static` output suitable for harnesses and snapshots.
- [SCOPE-03] Structural drift coefficient rendering and supporting graph summary context in graph-adjacent read surfaces.
- [SCOPE-04] Shared graph/drift projection reuse in topology and show-style commands instead of command-local relationship rebuilding.

### Out of Scope

- [SCOPE-05] Semantic similarity as a doctor invariant, lifecycle gate, or required render input.
- [SCOPE-06] Remote embedding providers or nondeterministic online graph enrichment.
- [SCOPE-07] Symbol-level source graphs or advanced clustering beyond file-level world-map rendering.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The voyage must add `keel knowledge graph` as a whole-project renderer that reads exclusively from the canonical knowledge-graph projection and supports interactive navigation when a TTY is available. | SCOPE-01 | FR-01 | automated |
| SRS-02 | The command must provide a deterministic `--static` mode that renders a stable whole-project graph summary suitable for harnesses, snapshots, and low-context review. | SCOPE-02 | FR-02 | automated |
| SRS-03 | The voyage must surface the structural drift coefficient and supporting mismatch context in graph-adjacent surfaces, including the knowledge-graph command, a distinct topology radar treatment, and reused show-style experiences. | SCOPE-03, SCOPE-04 | FR-03 | automated |
| SRS-04 | Topology/show-style commands touched by this work must reuse shared graph/drift read models rather than adding command-local relationship scanners, with mission, epic, and voyage show commands in the first delivery set. | SCOPE-04 | FR-04 | automated |
| SRS-05 | Interactive and static graph rendering must remain structurally correct when semantic cache artifacts are absent, stale, or intentionally omitted. | SCOPE-01, SCOPE-02 | FR-05 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent inputs must produce stable static graph output, drift values, and supporting summaries across repeated runs. | SCOPE-02, SCOPE-03, SCOPE-04 | NFR-01 | automated |
| SRS-NFR-02 | Interactive rendering must fit the live terminal viewport and fall back cleanly when a TTY is unavailable. | SCOPE-01, SCOPE-02 | NFR-03 | automated |
| SRS-NFR-03 | All command paths in this voyage must stay fully local/offline and must not require semantic cache presence for structural rendering correctness. | SCOPE-01, SCOPE-02, SCOPE-03, SCOPE-04 | NFR-02 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
