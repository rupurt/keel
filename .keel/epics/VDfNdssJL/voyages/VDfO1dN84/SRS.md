# BoardGraph Integrity Kernel - SRS

## Summary

Epic: VDfNdssJL
Goal: Introduce the canonical BoardGraph projection and first doctor integrity path so tree-state validation stops depending on repeated ad hoc scans.

## Scope

### In Scope

- [SCOPE-01] Build the first canonical `BoardGraph` projection over the existing `Board` entity set.
- [SCOPE-02] Expose deterministic graph query helpers for relationship traversal and dependency lookup.
- [SCOPE-03] Add a graph-level doctor integrity check driven by the canonical graph.
- [SCOPE-04] Migrate one non-doctor consumer to the graph so the new relationship model is not diagnostics-only.

### Out of Scope

- [SCOPE-05] Frontier-scoped artifact regeneration work beyond the graph primitives needed to support it later.
- [SCOPE-06] Persisted graph storage, background indexing, or board directory schema changes.
- [SCOPE-07] Full migration of every read model and doctor check to `BoardGraph` in one slice.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Provide a `BoardGraph` projection that materializes typed nodes and typed containment/lineage edges for missions, epics, bearings, ADRs, voyages, and stories from a loaded `Board`. | SCOPE-01 | FR-01 | test |
| SRS-02 | `BoardGraph` exposes deterministic traversal/query helpers for parent, children, ancestors, descendants, and relationship lookups needed by doctor and read-model consumers. | SCOPE-02 | FR-03 | test |
| SRS-03 | `BoardGraph` includes story dependency edges derived from existing implementation-order logic plus explicit `blocked_by` edges where present. | SCOPE-01, SCOPE-02 | FR-02 | test |
| SRS-04 | Add a doctor check that validates graph integrity from `BoardGraph`, including orphaned nodes, containment cycles, and terminal-parent violations that are derivable from the graph. | SCOPE-03 | FR-04 | test |
| SRS-05 | Migrate at least one non-doctor relationship consumer to `BoardGraph` without changing the existing CLI behavior or output contract. | SCOPE-04 | FR-03 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent board states must produce identical graph node ordering, edge ordering, and traversal results regardless of entity insertion order. | SCOPE-01, SCOPE-02 | NFR-01 | test |
| SRS-NFR-02 | The graph integrity check must operate from one graph build inside a doctor run instead of rebuilding multiple whole-board relationship scans. | SCOPE-03 | NFR-02 | test |
| SRS-NFR-03 | The migration leaves one canonical relationship path in place for the migrated consumer and doctor check, with no compatibility alias or parallel fallback logic. | SCOPE-03, SCOPE-04 | NFR-03 | test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
