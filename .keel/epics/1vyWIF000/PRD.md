# Topology Maps for Planning Drift - Product Requirements

## Problem Statement

Operators can inspect planning lineage, flow queues, and knowledge surfaces independently, but they cannot see a single terminal topology that explains how a planned epic will execute, where drift is forming, which stories are blocking progress, what knowledge has surfaced, or which risks are approaching.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make epic execution flow legible from one terminal command. | An operator can review one epic's current plan and flow from a single `keel topology` run instead of hopping across multiple read surfaces. | 1 command per epic review |
| GOAL-02 | Surface drift before it compounds into execution churn. | Every supported drift signal family is visible when present in fixture boards. | 100% of supported signal categories surfaced |
| GOAL-03 | Bring execution learning and emerging risk into planning review. | Scoped knowledge and horizon commentary appear whenever matching pending or recent signals exist. | Knowledge and risk surfaced for every matching epic |
| GOAL-04 | Keep the topology view trustworthy and readable in the terminal. | Equivalent board states render deterministically and remain readable at common terminal widths. | Deterministic output plus readable 100+ column layout |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | Human reviewing an epic before planning, during execution, or ahead of verification. | See the whole flow, hotspots, and near-term risks without opening multiple commands. |
| Planner | Human decomposing or recalibrating voyage and story work. | Understand where coverage, drift, and institutional knowledge should change the plan. |

## Scope

### In Scope

- [SCOPE-01] Add a human-oriented terminal `keel topology` command for one epic at a time.
- [SCOPE-02] Show epic, voyage, and story topology together with planning and execution relationships.
- [SCOPE-03] Annotate topology nodes with drift signals from lineage, requirement coverage, dependency blockage, and verification or proof gaps.
- [SCOPE-04] Annotate topology flow with scoped knowledge, recently surfaced insights, and pending unapplied knowledge.
- [SCOPE-05] Add horizon and recommendation commentary for approaching risks such as verification debt, throughput or ETA risk, and tech or process debt signals.
- [SCOPE-06] Support a focused default view for planned and in-progress work plus an option to include done entities.

### Out of Scope

- [SCOPE-07] Whole-board topology across multiple epics.
- [SCOPE-08] JSON, Mermaid, DOT, or other export formats.
- [SCOPE-09] Automatic remediation, replanning, or lifecycle state changes.
- [SCOPE-10] Editing planning artifacts from the topology surface.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `keel topology` MUST render an epic-scoped terminal topology that connects the epic, its voyages, and its scoped stories in one operator-facing view. | GOAL-01 GOAL-04 | must | The command needs a single canonical flow view instead of forcing context hops across `show`, `flow`, and audit surfaces. |
| FR-02 | The topology view MUST annotate drift hotspots, including scope lineage drift, uncovered requirements, blocked story dependencies, and verification or proof gaps. | GOAL-02 GOAL-04 | must | The primary value is not the tree alone; it is seeing where the plan is drifting or execution is stuck. |
| FR-03 | The topology view MUST surface scoped knowledge annotations, including newly surfaced execution insights and pending unapplied knowledge relevant to the epic flow. | GOAL-02 GOAL-03 | must | Operators need execution learning in context so plans can be adjusted before drift compounds. |
| FR-04 | The topology view MUST provide horizon commentary and recommendations for approaching risks such as verification debt, throughput or ETA risk, and tech or process debt heuristics. | GOAL-02 GOAL-03 | must | The user explicitly wants forward-looking warnings, not only current-state reporting. |
| FR-05 | The command MUST default to a focused planned and in-progress view and MUST support including done entities when the operator needs full context. | GOAL-01 GOAL-04 | must | Done work can add useful context, but it should not overwhelm the default operational view. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Topology output MUST be deterministic for equivalent board states, including ordering of nodes, annotations, and recommendations. | GOAL-04 | must | Humans must be able to trust diffs, screenshots, and repeated runs. |
| NFR-02 | Topology rendering MUST remain readable in normal terminal widths and degrade gracefully when the board is dense. | GOAL-04 | must | A topology map that collapses into noise will fail the primary operator workflow. |
| NFR-03 | Drift, knowledge, and horizon derivation MUST reuse canonical read models and invariants instead of introducing parallel parsing or policy logic. | GOAL-02 GOAL-04 | must | Re-implementing existing logic would create the same drift this feature is trying to expose. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Projection correctness | Rust unit tests and fixture-board integration tests | `cargo test` coverage for topology read model, drift annotations, and heuristic tie-breaks |
| Terminal readability | CLI snapshot tests plus VHS capture for representative boards | Stable terminal output proofs and one or more tape recordings |
| Heuristic commentary quality | Deterministic rule tests plus qualitative `llm-judge` spot review when commentary changes materially | Story-level proof artifacts showing rule output and qualitative review notes |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing planning, traceability, throughput, and knowledge read surfaces expose enough canonical inputs to compose a topology view. | A large amount of new parsing infrastructure would be required and scope would expand. | Validate during SDD and the first implementation story. |
| Human operators primarily want terminal review rather than machine-readable export in this slice. | The command shape and renderer could be wrong for the real workflow. | Re-check after the epic-scoped terminal view lands. |
| Whole-board topology can be deferred without weakening the first user outcome. | Epic scope may be incomplete for adoption. | Keep whole-board aggregation explicit as deferred work instead of implicit debt. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Dense epics may produce topology output that is hard to scan even with grouping and summaries. | Epic owner | Open |
| Heuristic horizon commentary could become noisy if thresholds are too eager or too opaque. | Epic owner | Open |
| Existing done-heavy epics may need careful default filtering to avoid overwhelming the main view. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A human operator can review one planned or in-progress epic with a single `keel topology --epic <id>` run and understand its flow, drift hotspots, knowledge annotations, and near-term risks.
- [ ] Every supported drift signal family appears in the topology view when present in deterministic fixture boards.
- [ ] Scoped pending or recent knowledge and horizon commentary appear when relevant signals exist, without requiring a separate knowledge command during normal epic review.
- [ ] Equivalent board states render stable topology output suitable for snapshots, tapes, and repeatable review.
<!-- END SUCCESS_CRITERIA -->
