# Mission Request Command Surface - Product Requirements

> Keel should expose a native `mission request` command family so Keeper and other
programs can compose mission-request parsing, validation, drafting, application,
and acknowledgement without embedding provider-specific logic in Keel core.

## Problem Statement

The current Keeper and keeper-cli surfaces are too thin to ingest formal mission
requests from external providers. Without a native Keel CLI surface, each
provider worker would need to reimplement normalization, validation, and
application behavior, which would fracture the planning contract and weaken
replayability.

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

- [SCOPE-01] Define the canonical `keel mission request template|parse|validate|draft|apply|ack` command family.
- [SCOPE-02] Define the provider-neutral mission request envelope and its stdin/stdout contract.
- [SCOPE-03] Define the validation and acknowledgement semantics automation callers rely on.

### Out of Scope

- [SCOPE-90] Provider-specific polling and ingress workers in Keeper.
- [SCOPE-91] Unrelated workflow or planning refactors outside the mission-request contract.

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
| How should `keel mission request apply` behave when a request is exploratory rather than implementation-ready? | Planner | Open |
| Which fields should be required on stdin versus derivable from provider metadata? | Planner | Open |
| Should `ack` emit only provider-facing content or also a canonical audit record payload? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Define the canonical `keel mission request template|parse|validate|draft|apply|ack` command family.
- [ ] Define a provider-neutral request envelope that can be piped over stdin/stdout.
- [ ] Define the minimum required inputs for GitHub issue activation and later provider expansion.
- [ ] Define how Keeper and non-Keeper automation invoke the same Keel surface.
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

## Findings


- A canonical mission-request CLI surface is already specified strongly enough to promote from research into strategic delivery work. [SRC-01]
- Keeper and other automation need a scriptable command boundary instead of embedding provider parsing and mutation rules ad hoc. [SRC-01][SRC-02]


## Opportunity Cost


- Delaying this work keeps mission intake coupled to manual operator steps and blocks consistent provider composition in Keeper. [SRC-01][SRC-02]


## Dependencies


- The command surface should stay aligned with the provider-neutral mission request envelope already captured in the foundational bearing package. [SRC-01]
- Keeper’s current CLI and runtime surface provide the execution context, but not yet the native request commands this mission is defining. [SRC-02]


## Alternatives Considered


- Keep mission-request handling inside Keeper-specific provider code. This was rejected because it would make GitHub-first ingress harder to generalize and would weaken the native Keel contract. [SRC-01][SRC-02]

## Research Provenance

*Source records from bearing evidence:*

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | workspace | /home/alex/workspace/spoke-sh/keel/.keel/bearings/VDupml7OG/MISSION_REQUESTS.md | 2026-04-07 | 2026-04-07 | high | high | Existing research package already defines the candidate command family and normalized mission request envelope. |
| SRC-02 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper-cli/src/main.rs | 2026-04-07 | 2026-04-07 | medium | high | Current keeper-cli exposes only missions, start, and status commands, which leaves mission-request composition unimplemented. |

---

*This PRD was seeded from bearing `VG6ggE3ud`. See `bearings/VG6ggE3ud/` for original research.*
