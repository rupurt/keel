# Canonical Board Graph Kernel - Product Requirements

## Problem Statement

Keel currently treats relationships between missions, epics, bearings, voyages, stories, and generated artifacts as an implied structure reconstructed in many places. `Board` remains a bag of entity maps while doctor checks, topology, traceability, and artifact generators each rebuild their own partial lineage view. That duplication makes the system harder to reason about, slower to validate holistically, and prone to churn where a local lifecycle change rewrites unrelated board artifacts. This epic introduces a canonical `BoardGraph` projection that models containment and dependency edges once, then reuses that graph for fast integrity checks and scoped regeneration decisions.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make entity relationships explicit and queryable through one canonical graph projection. | Core board consumers can answer parent, child, ancestry, and dependency questions without rebuilding ad hoc scans | `BoardGraph` becomes the shared relationship kernel used by at least one doctor path and one non-doctor consumer |
| GOAL-02 | Add a fast graph-level integrity check that validates the board tree from derived relationships instead of repeated local heuristics. | Doctor reports structural tree problems from one graph pass | A graph integrity check runs in near-linear time over nodes and edges and catches orphan/cycle/terminal-state drift |
| GOAL-03 | Reduce unrelated artifact rewrites by identifying the affected lineage frontier before regeneration. | Local lifecycle work rewrites only the changed entity frontier and required ancestors | At least one generation path uses a graph-scoped sync frontier instead of global board regeneration |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Maintainer | Engineers evolving board storage, doctor, and generation internals. | One explicit relationship model that reduces duplicated scans and drift. |
| Operator | Agents or humans running `keel doctor`, `keel flow`, and lifecycle commands. | Stable board artifacts and faster, clearer structural feedback. |

## Scope

### In Scope

- [SCOPE-01] Introduce a derived `BoardGraph` projection with typed nodes and edges for containment, lineage, and implementation dependencies.
- [SCOPE-02] Add graph query helpers for parent/children, ancestors/descendants, and affected-frontier traversal.
- [SCOPE-03] Implement a doctor check that validates tree integrity and terminal-state coherence from the canonical graph.
- [SCOPE-04] Apply the graph frontier to at least one artifact regeneration path so unrelated entities stop being rewritten.

### Out of Scope

- [SCOPE-05] Persist the graph on disk or introduce a separate runtime graph service.
- [SCOPE-06] Replace all read models in one sweep; this epic only needs the first migrated consumers.
- [SCOPE-07] Redesign board markdown contracts or entity directory layout.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Build a canonical `BoardGraph` projection from `Board` that materializes typed nodes and typed edges for containment and lineage between missions, epics, bearings, ADRs, voyages, and stories. | GOAL-01 | must | The graph has to become the relationship kernel rather than another partial helper. |
| FR-02 | Include implementation dependency edges in the graph so story blockers can be reasoned about alongside containment edges. | GOAL-01 | should | One graph should answer both tree and execution-dependency questions. |
| FR-03 | Expose deterministic graph query helpers for parent, children, descendants, ancestors, and affected-frontier traversal. | GOAL-01, GOAL-03 | must | Doctor and scoped regeneration both need stable graph navigation primitives. |
| FR-04 | Add a doctor check that validates graph integrity, including orphaned nodes, containment cycles, and terminal-parent violations that can be derived from the graph. | GOAL-02 | must | This is the fast structural check that motivated the epic. |
| FR-05 | Move at least one artifact regeneration path from full-board sync to graph-frontier sync using the canonical relationship model. | GOAL-03 | must | The epic needs a real churn-reduction outcome, not only a new abstraction. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | `BoardGraph` construction must be deterministic across equivalent board loads and insertion orders. | GOAL-01 | must | Relationship identity cannot drift across repeated reads. |
| NFR-02 | Graph integrity validation must run from one graph build and avoid repeated whole-board rescans inside individual checks. | GOAL-02 | must | The performance and clarity benefit comes from centralizing relationship derivation. |
| NFR-03 | Migration slices must preserve current CLI semantics and leave one canonical relationship path after each step. | GOAL-01, GOAL-03 | must | Hard-cutover discipline matters more than temporary compatibility glue. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Graph determinism | Unit tests over equivalent board fixtures with different insertion orders | Story proofs showing identical graph topology and query results |
| Graph integrity doctor | Focused doctor and gating tests plus `keel doctor` proof on the repo board | Evidence that orphan/cycle/terminal drift is reported from the canonical graph path |
| Scoped regeneration | Unit tests on frontier selection and integration proof from a lifecycle command | Evidence that unrelated voyage or epic artifacts stop being rewritten on local updates |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A derived in-memory graph is sufficient for the first churn-reduction wins; persisted graph storage is unnecessary. | The epic could drift into storage design work too early. | Keep the first slices read-model scoped and measure the payoff before considering persistence. |
| Existing lineage fields and path conventions are rich enough to build the first canonical graph without changing board storage shape. | The epic may become blocked on markdown/schema migration work. | Validate during the first voyage and escalate only if the graph cannot represent the current board cleanly. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which artifact sync path should migrate first to produce the clearest churn reduction with low risk? | Epic owner | Open |
| Should the first doctor graph check enforce only containment integrity or also story dependency cycles once both edges live in the graph? | Epic owner | Open |
| Could frontier-scoped sync hide necessary ancestor updates if ownership rules are not made explicit enough? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `BoardGraph` becomes the canonical relationship projection for at least one doctor path and one non-doctor consumer.
- [ ] `keel doctor` can report orphaned or structurally invalid tree state from a single graph-derived integrity pass.
- [ ] At least one artifact generation path uses a graph-scoped frontier and avoids rewriting unrelated entities during local lifecycle work.
<!-- END SUCCESS_CRITERIA -->
