# Keeper Provider Mission Request Ingress - Product Requirements

> Keeper should own provider polling, activation detection, normalization, and
acknowledgement for formal mission requests while delegating request semantics
and planning mutation to native Keel commands.

## Problem Statement

The Keeper architecture already defines reactor inboxes, connector ingress, and
provider-facing routing, but it does not yet define a formal mission-request
intake flow. Without that split, provider-specific parsing will leak into Keel
or Keeper workers will drift on how requests are normalized and replayed.

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

- [SCOPE-01] Define GitHub issue activation detection for formal mission requests.
- [SCOPE-02] Define the normalization path from provider artifact to canonical mission request envelope.
- [SCOPE-03] Define Keeper acknowledgement, retry, and replay semantics around native Keel mission-request commands.

### Out of Scope

- [SCOPE-90] Direct provider mutation of Keel planning state without native mission-request commands.
- [SCOPE-91] Unrelated Keeper runtime refactors outside provider mission-request ingress.

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
| Should GitHub issue edits create new normalized revisions or supersede prior drafts? | Planner | Open |
| What evidence should Keeper persist locally versus refer to by provider reference? | Planner | Open |
| Which acknowledgements belong in provider comments versus reactor-private audit streams? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Define the GitHub issue activation rule and its normalization path into a canonical mission request envelope.
- [ ] Define the boundary between Keeper provider polling and Keel mission-request commands.
- [ ] Define how provider revisions, retries, and acknowledgements remain replayable.
- [ ] Define the first ingress worker responsibilities for GitHub issues.
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

## Findings


- Keeper is the correct owner for provider polling, normalization, and acknowledgement in the Keel/Keeper boundary. [SRC-01][SRC-02]
- GitHub issues are a strong first ingress provider, but the normalization path must stay provider-neutral and lower into native Keel commands. [SRC-01][SRC-03]


## Opportunity Cost


- Delaying this work leaves external mission intake informal and prevents Keeper from acting as a controlled multiplayer ingress boundary. [SRC-01][SRC-03]


## Dependencies


- The mission-request command surface needs to exist so Keeper can target a native Keel contract instead of mutating board state directly. [SRC-02][SRC-03]
- The ingress path should align with Keeper’s existing architecture for provider routing and envelope handling. [SRC-01]


## Alternatives Considered


- Let each provider mutate planning state directly. This was rejected because it bypasses a stable Keel contract and makes auditability and provider parity weaker. [SRC-01][SRC-03]

## Research Provenance

*Source records from bearing evidence:*

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/docs/architecture/keeper.md | 2026-04-07 | 2026-04-07 | high | high | Keeper architecture already assigns provider ingress, connector routing, inbox/outbox handling, and GitHub-first external integration to Keeper. |
| SRC-02 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper/src/lib.rs | 2026-04-07 | 2026-04-07 | medium | high | The current Keeper service surface does not yet expose mission-request ingestion endpoints or workflows. |
| SRC-03 | manual | workspace | /home/alex/workspace/spoke-sh/keel/.keel/bearings/VDupml7OG/MISSION_REQUESTS.md | 2026-04-07 | 2026-04-07 | high | high | The existing research package already defines a GitHub issue activation prefix and a provider-neutral request envelope. |

---

*This PRD was seeded from bearing `VG6ggSPFR`. See `bearings/VG6ggSPFR/` for original research.*
