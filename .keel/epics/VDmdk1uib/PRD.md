# TUI Compact Layout Research - Product Requirements

> A three-bullet compact status will significantly reduce operator cognitive load.

## Problem Statement

Current status reports are too verbose for quick checks.

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

- [SCOPE-01] Add Archetype labeling to missions.
- [SCOPE-02] Implement High-Density `story show` layout.
- [SCOPE-03] Implement High-Density `voyage show` layout.

### Out of Scope

- [SCOPE-04] Full TUI dashboard rewrite.

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
- [ ] Bearing-backed workflow can be executed end-to-end in production conditions.
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

### Findings

- Three bullets are sufficient for quick status. [SRC-01]
- The framing of a "Ramping Path" provides a clear onboarding journey for new developers. [SRC-02]

### Opportunity Cost

Slightly higher code complexity in the CLI layer. [SRC-01]

### Dependencies

- Depends on the existing `calculate_next` algorithm. [SRC-01]

### Alternatives Considered

- Multi-page status was rejected as too verbose. [SRC-01]

---

*This PRD was seeded from bearing `VDmdk1uib`. See `bearings/VDmdk1uib/` for original research.*
