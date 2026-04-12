# Keeper Managed Janitor Stewardship Boundary - Product Requirements

> Define how Keeper takes custody of a Keel board in janitor posture without
> collapsing provider logic, planning truth, or board-role routing into one
> surface.

## Problem Statement

Keel's multiplayer direction needs a concrete transition from local CLI
stewardship to Keeper-managed custody. Spoke already has a Keeper posture model
(`janitor`, `driver`, `navigator`) and a documented connector architecture, but
Keel still treats authenticated execution as a generic `system` actor and
Keeper does not yet have a GitHub connector contract that can safely drive
janitor-grade maintenance loops against a bound board. Without one shared
contract, janitor behavior will drift across repos, GitHub connector work will
guess at Keel semantics, and Keeper will not have a clear limit on what it may
do autonomously.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Define the custody model that lets Keeper act as janitor over a Keel board. | Architecture readiness | Posture, provenance, and escalation model captured |
| GOAL-02 | Define the GitHub connector bridge janitor uses for inbound stimuli and outbound acknowledgements. | Interface readiness | Connector event and acknowledgement contract captured |
| GOAL-03 | Decompose the first cross-repo rollout slice. | Execution readiness | Planned voyage and seed story exist |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keel Maintainer | Owns board semantics and lifecycle contracts. | A custody model that keeps Keel authoritative without embedding Keeper logic. |
| Keeper Operator | Runs repository stewardship through Keeper. | A constrained janitor posture with clear autonomous limits and provenance. |
| Repository Steward | Watches GitHub-facing work and escalations. | Visible acknowledgements and handoffs that stay attributable to Keeper janitor actions. |

## Scope

### In Scope

- [SCOPE-01] A structured custody context that records Keeper identity, reactor/project provenance, janitor posture, and the specific Keel board role selected for an action.
- [SCOPE-02] The janitor policy envelope: permitted autonomous turn-loop actions, required native Keel command surfaces, and escalation boundaries.
- [SCOPE-03] The GitHub connector ingress/egress contract for janitor signals such as maintenance prompts, review requests, workflow failures, acknowledgements, and handoff summaries.
- [SCOPE-04] The first rollout split between `keel` and `spoke`.

### Out of Scope

- [SCOPE-90] Driver/navigator posture semantics or fleet-wide multi-reactor coordination policy.
- [SCOPE-91] Non-GitHub connectors or production webhook infrastructure.
- [SCOPE-92] Direct connector mutation of `.keel` state or bypassing native Keel commands.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Specify a structured Keeper custody context for Keel that separates Keeper posture from board-role selection and records stable provenance for lifecycle logs. | GOAL-01 | must | Prevents posture semantics from drifting into lane routing and keeps Keeper actions attributable. |
| FR-02 | Specify the janitor automation envelope across Orient/Inspect/Pull/Ship/Close, including which actions stay autonomous and which require human escalation. | GOAL-01 | must | Makes janitor safe enough to run unattended without turning it into an unconstrained agent. |
| FR-03 | Specify the GitHub connector event model and acknowledgement surfaces used by janitor posture without direct board mutation. | GOAL-02 | must | Drives the first real connector boundary in Spoke without leaking provider semantics into Keel. |
| FR-04 | Define the first rollout split across `keel` and `spoke`, including the minimum crates/docs each repo must own. | GOAL-03 | must | Keeps the transition slice executable instead of ending as abstract architecture. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Preserve deterministic, replayable evidence from GitHub event through Keeper routing to Keel lifecycle change and back to provider acknowledgement. | GOAL-01, GOAL-02 | must | Janitor custody is unsafe if retries or replays can produce ambiguous provenance. |
| NFR-02 | Preserve explicit separation between Keeper posture and Keel board role so queue routing and operator personas do not drift. | GOAL-01 | must | Prevents one repo from redefining the other's control vocabulary. |
| NFR-03 | Keep the bridge GitHub-first but provider-extensible. | GOAL-02 | should | Lets the first connector land cleanly without making GitHub a permanent special case. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Custody model | Planning review against mission-request and Keeper architecture sources | Story-level acceptance criteria and authored planning docs |
| Connector boundary | Manual inspection of ingress/egress flow and ownership split | Story-level acceptance criteria and rollout tables |
| Board readiness | `keel doctor` / `keel flow` after planning transitions | Board state and commit-boundary diagnostics |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Spoke's Keeper posture model (`janitor`/`driver`/`navigator`) remains the runtime control vocabulary. | Keel custody fields and janitor policy would need redesign. | Re-check against Spoke architecture and Hub surfaces during implementation. |
| Keel will continue to own planning truth through native lifecycle commands rather than connector-side mutation. | The entire custody boundary would move and this epic would mis-scope the split. | Re-check against Keeper architecture and any new ADRs. |
| GitHub is the first connector worth hardening for janitor posture. | The first rollout slice may target the wrong provider. | Re-check at story start if connector priorities change. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should janitor posture be allowed to pull both management and delivery lanes, or only a narrower maintenance subset? | Planner | Open |
| What is the minimum Keeper provenance Keel must retain in lifecycle logs: keeper id, reactor id, project ref, or all three? | Planner | Open |
| Which janitor acknowledgements belong in public GitHub comments versus reactor-private audit streams? | Planner | Open |
| Does janitor need a dedicated Keel command family, or can the first rollout compose existing commands with structured custody claims? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] The custody model captures Keeper janitor posture, Keel board-role selection, and provenance as separate concepts.
- [ ] The janitor automation envelope and escalation boundary are defined.
- [ ] The GitHub connector ingress/egress contract is defined without direct board mutation.
- [ ] The first cross-repo rollout slice is decomposed into planned work.
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*Grounded in existing repo and sibling-repo architecture work:*

## Findings

- Keeper already owns provider ingress, routing, execution, and replay while
  Keel stays authoritative for planning truth. [SRC-01]
- Spoke already exposes `janitor` as a first-class Keeper posture, so the
  transition should reuse that vocabulary instead of inventing a second runtime
  posture model. [SRC-02]
- Keel's current authenticated execution context still collapses automated work
  into a generic `system` role, so there is no explicit custody contract for
  Keeper provenance or janitor posture yet. [SRC-03]
- Prior mission-request work already established the safe connector boundary:
  provider artifact -> Keeper normalization/routing -> native Keel commands ->
  provider acknowledgement. [SRC-04][SRC-05]

## Opportunity Cost

If this work is delayed, the first GitHub connector in Spoke will be forced to
guess at Keel custody semantics, and multiplayer janitor behavior will emerge
implicitly instead of through one explicit, auditable boundary.

## Dependencies

- Keeper architecture and connector model in `spoke` [SRC-01]
- Existing Keeper posture vocabulary in Spoke Hub [SRC-02]
- A richer authenticated execution context in Keel [SRC-03]
- Prior mission-request and connector-boundary research in Keel [SRC-04][SRC-05]

## Alternatives Considered

- Treat `janitor` as a new Keel board role. Rejected because it collapses
  Keeper posture into queue routing and duplicates Keel's existing role
  topology. [SRC-02][SRC-03]
- Let the GitHub connector mutate board state directly. Rejected because it
  bypasses Keel lifecycle surfaces and weakens provenance. [SRC-01][SRC-05]
- Keep authenticated automation as a generic `system` actor. Rejected because it
  hides who acted, under what posture, and why the action was permitted. [SRC-03]

## Research Provenance

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/docs/architecture/keeper.md | 2026-04-11 | 2026-04-11 | high | high | Keeper architecture defines Keel/Keepers boundaries, reactor duties, and GitHub connector direction. |
| SRC-02 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/hub/src/main.rs | 2026-04-11 | 2026-04-11 | medium | high | Spoke Hub already exposes janitor/driver/navigator as Keeper posture IDs and recommendation language. |
| SRC-03 | manual | workspace | /home/alex/workspace/spoke-sh/keel/crates/spoke-auth/src/lib.rs | 2026-04-11 | 2026-04-11 | medium | high | Keel authenticated execution currently records only generic identity + role and defaults to `system`. |
| SRC-04 | manual | workspace | /home/alex/workspace/spoke-sh/keel/.keel/bearings/VDupml7OG/MISSION_REQUESTS.md | 2026-04-11 | 2026-04-11 | high | high | Prior research defines provider-neutral request lowering and the native Keel command boundary. |
| SRC-05 | manual | workspace | /home/alex/workspace/spoke-sh/keel/.keel/epics/VG6ggSPFR/PRD.md | 2026-04-11 | 2026-04-11 | high | high | Prior planning work captures GitHub-first ingress and Keeper-side acknowledgement responsibilities. |
