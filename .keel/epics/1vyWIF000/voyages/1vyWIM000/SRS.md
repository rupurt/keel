# Epic Topology Terminal View - Software Requirements Specification

> Render an epic-scoped topology map in the terminal that shows voyages, stories, drift hotspots, knowledge annotations, and forward-looking risks in one operator-friendly view.

**Epic:** [1vyWIF000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-01] Add a human-oriented terminal `keel topology` command for one epic at a time.
- [SCOPE-02] Show epic, voyage, and story topology together with planning and execution relationships.
- [SCOPE-03] Annotate topology nodes with drift signals from lineage, requirement coverage, dependency blockage, and verification or proof gaps.
- [SCOPE-04] Annotate topology flow with scoped knowledge, recently surfaced insights, and pending unapplied knowledge.
- [SCOPE-05] Add horizon and recommendation commentary for approaching risks such as verification debt, throughput or ETA risk, and tech or process debt signals.
- [SCOPE-06] Support a focused default view for planned and in-progress work plus an option to include done entities.

Out of scope:
- [SCOPE-07] Whole-board topology across multiple epics.
- [SCOPE-08] JSON, Mermaid, DOT, or other export formats.
- [SCOPE-09] Automatic remediation, replanning, or lifecycle state changes.
- [SCOPE-10] Editing planning artifacts from the topology surface.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| `src/read_model/planning_show.rs` continues to provide canonical epic and voyage coverage, scope drift, and verification rollups. | Internal dependency | Topology would need duplicate lineage parsing or incomplete coverage output. |
| `src/read_model/traceability.rs` remains the canonical source for story dependency derivation. | Internal dependency | Blockage chains would drift from `keel flow`. |
| `src/read_model/knowledge/**` continues to provide scoped knowledge, pending or applied status, and rising-pattern detection primitives. | Internal dependency | Knowledge and horizon overlays would become ad hoc or lossy. |
| Throughput and verification signals remain available from existing read models and board artifacts. | Internal dependency | Horizon commentary would be weak or unavailable. |

## Constraints

- Terminal output only in this voyage; no JSON or graph export contract is introduced.
- Human readability is primary, but the underlying projection and ordering must remain deterministic for tests and tapes.
- The design must reuse canonical parsing, invariants, and drift logic; no shadow lineage or policy implementations.
- Epic scope is the only supported topology root in this voyage; whole-board aggregation is deferred.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel topology --epic <id>` MUST render an epic-scoped topology that includes the epic, its voyages, and the voyage-scoped stories that make up the current flow. | SCOPE-01 SCOPE-02 | FR-01 | read-model tests + CLI snapshot tests |
| SRS-02 | The topology command MUST default to a focused planned and in-progress view and MUST provide an explicit option to include done voyages and stories when broader context is needed. | SCOPE-06 | FR-05 | command parsing tests + CLI snapshot tests |
| SRS-03 | The topology view MUST annotate drift hotspots from scope lineage drift, uncovered PRD or SRS requirements, blocked story dependencies, and missing proof or verification coverage. | SCOPE-03 | FR-02 | fixture-board tests + CLI snapshot tests |
| SRS-04 | The topology view MUST surface scoped knowledge annotations that highlight recent execution insights and pending unapplied knowledge relevant to the epic flow. | SCOPE-04 | FR-03 | knowledge-scoping tests + CLI snapshot tests |
| SRS-05 | The topology view MUST render horizon commentary and recommendations for approaching risks, including verification debt, throughput or ETA risk, and tech or process debt heuristics derived from board signals. | SCOPE-05 | FR-04 | heuristic rule tests + CLI snapshot tests + qualitative review proof |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent board states MUST render topology output with stable ordering for nodes, annotations, and heuristic tie-breaks. | SCOPE-01 SCOPE-03 SCOPE-04 SCOPE-05 SCOPE-06 | NFR-01 | deterministic projection tests + CLI snapshot tests |
| SRS-NFR-02 | The topology renderer MUST remain readable in common terminal widths and degrade gracefully by summarizing or collapsing detail rather than producing ambiguous layout. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-05 | NFR-02 | renderer tests + VHS proof |
| SRS-NFR-03 | Drift, knowledge, and horizon derivation MUST consume canonical read-model and invariant helpers rather than command-local duplicate parsing. | SCOPE-03 SCOPE-04 SCOPE-05 | NFR-03 | architecture contract tests + unit tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
