# Mission Bearing Lineage - Product Requirements

## Problem Statement

Missions cannot explicitly own research bearings, which leaves mission readiness and activation blocked even when the real strategic work is bearing-backed.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Let mission stewards attach and inspect bearings as first-class mission children without editing frontmatter by hand. | Mission-bearing linkage is created and discoverable through canonical CLI and show surfaces. | 100% of mission-linked bearings can be attached and inspected through one explicit path |
| GOAL-02 | Make mission readiness, activation, and diagnostics respect explicit bearing lineage. | Mission gates and doctor agree on whether a bearing-backed mission is ready or incoherent. | 0 mission-bearing coherence mismatches across doctor and lifecycle transitions |
| GOAL-03 | Preserve a deterministic relationship model that downstream flow, graph, and next surfaces can reuse. | Mission-bearing lineage is represented canonically in board projections and read surfaces. | No secondary inference path is required to discover mission-owned bearings |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Mission Steward | The planner or harness coordinating a multi-step objective. | Attach research work to a mission explicitly and see it reflected in readiness and next-step surfaces. |
| Research Lead | The person advancing bearings toward decision readiness. | Preserve the strategic context of a bearing without ad hoc notes or manual mission editing. |
| Automation Harness | A deterministic CLI consumer driving mission loops. | Observe mission-bearing lineage through one canonical contract with actionable errors. |

## Scope

### In Scope

- [SCOPE-01] A canonical mission-bearing lineage field or relationship persisted in board state and discoverable from both mission and bearing surfaces.
- [SCOPE-02] An explicit CLI workflow for attaching bearings to missions without manual file edits.
- [SCOPE-03] Mission readiness, activation, doctor, and show/next surfaces that consume the same mission-bearing lineage rules.
- [SCOPE-04] Deterministic validation and regression coverage for mission-bearing coherence.

### Out of Scope

- [SCOPE-05] A new standalone roadmap or horizon product surface.
- [SCOPE-06] Bearing dependency modeling or sequencing logic beyond direct mission membership.
- [SCOPE-07] Automatic bearing-to-mission attachment based on title similarity, chart text, or heuristic inference.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Persist mission-bearing lineage canonically so a bearing can be recognized as a child of exactly one mission without depending on freeform charter text. | GOAL-01, GOAL-03 | must | The relationship must exist in board state before any command or doctor check can rely on it. |
| FR-02 | Provide an explicit command path for attaching a bearing to a mission and update the relevant board artifacts in the same lifecycle flow. | GOAL-01 | must | Users need a discoverable workflow instead of manual edits or hidden conventions. |
| FR-03 | Use mission-bearing lineage in mission readiness, activation, and doctor coherence checks so bearing-backed missions are treated as valid strategic scope. | GOAL-02, GOAL-03 | must | Lifecycle gates and doctor must agree on whether a mission is actionable and coherent. |
| FR-04 | Surface mission-owned bearings in mission show and related decision surfaces with actionable guidance when linkage is missing. | GOAL-01, GOAL-02 | should | Read views must make the lineage visible enough for humans and harnesses to trust it. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Equivalent board state must produce the same mission-bearing child sets, readiness decisions, and doctor outcomes across repeated runs. | GOAL-02, GOAL-03 | must | Strategic lineage must be deterministic for automation and review. |
| NFR-02 | The attach workflow and diagnostics must fail with explicit recovery guidance instead of leaving stewards to infer the missing linkage model. | GOAL-01, GOAL-02 | should | This closes the workflow gap that triggered the mission in the first place. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Lineage persistence | Unit and command regression tests over mission-bearing attach flows and board reloads | `cargo test` coverage plus story-level proof logs |
| Lifecycle coherence | Mission lifecycle and doctor regression tests | Green `just test` and `just keel doctor` on boards with and without mission-linked bearings |
| Operator guidance | CLI proof of the attach workflow and mission/bearing show surfaces | Story evidence logs and updated guidance text |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Mission-bearing lineage belongs on the same canonical graph as mission-epic lineage instead of a separate advisory layer. | A second relationship model would increase drift and duplicate validation rules. | Validate against board graph and mission doctor expectations during implementation. |
| Bearings should remain singly owned by missions, matching the existing spirit of mission children. | Multi-mission bearings would require a different ownership model and transition policy. | Confirm with validation and CLI contract design before sealing the first voyage. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the explicit attach command support both add and move semantics when a bearing is already mission-linked? | Epic owner | Open |
| How much of the mission-bearing relationship should appear in `flow`, `mission next`, and graph surfaces in the first delivery slice? | Epic owner | Open |
| Existing orphaned bearings may need a migration or adoption workflow once lineage becomes first-class. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Mission stewards can attach a bearing to a mission through a canonical CLI workflow with no manual frontmatter editing.
- [ ] Mission readiness, activation, and doctor treat mission-linked bearings as first-class child entities.
- [ ] Mission and bearing read surfaces expose the lineage clearly enough for a harness to understand the strategic relationship without extra inference.
<!-- END SUCCESS_CRITERIA -->
