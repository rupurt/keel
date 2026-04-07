# Collaborative Cryptographic Primitives Over Adversarial Transport - Product Requirements

> Define the Keeper-managed multiplayer security foundations for attestation, auditability, private payload handling, and provider-neutral mission request ingress.

## Problem Statement

Keel needs a security model that works for multi-player operation under Keeper
without coupling planning truth to a single storage backend or provider. The
system has to protect high-consequence state transitions, preserve replayable
provider ingress, and support private payload boundaries without turning every
workflow move into heavyweight cryptographic ceremony.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Formalize Keeper-managed security boundaries for multiplayer Keel. | Architecture readiness | Boundary and attestation model captured |
| GOAL-02 | Define a provider-neutral mission request model that Keel and Keeper can share. | Interface readiness | Command and ingress contract captured |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keel Maintainer | Shapes planning and execution rules | Clear boundaries between planning truth, ingress, and audit layers |
| Keeper Operator | Runs multi-project reactor workflows | Deterministic ingress, replay, and attestation contracts |

## Scope

### In Scope

- [SCOPE-01] Define Keeper-managed trust boundaries for planning truth, provider ingress, routing, and execution.
- [SCOPE-02] Define the append-only audit and checkpoint contract that keeps Keel backend-agnostic.
- [SCOPE-03] Define the first-class mission request command and ingress boundary between Keel and Keeper.

### Out of Scope

- [SCOPE-04] Production rollout of threshold key ceremonies or fleet-wide key management automation.
- [SCOPE-05] Full implementation of every external provider or connector surface.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Specify which Keel and Keeper lifecycle moves require ordinary audit logging versus threshold attestation. | GOAL-01 | must | Prevents over-signing while protecting high-consequence state. |
| FR-02 | Specify a provider-neutral mission request envelope and the native `keel mission request` command family. | GOAL-02 | must | Keeps request semantics in Keel and provider polling in Keeper. |
| FR-03 | Define the backend contract for append, checkpoint, inclusion proof, and consistency proof operations. | GOAL-01 | must | Preserves backend portability while keeping audit guarantees explicit. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Preserve deterministic, replayable evidence for provider ingress and checkpoint lineage. | GOAL-01, GOAL-02 | must | Security design is not useful if replay and attribution drift. |
| NFR-02 | Keep the design modular enough that Transit remains optional rather than mandatory. | GOAL-01 | must | Avoids over-coupling Keel planning semantics to one backend. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Verify that mission and epic lineage cleanly separate Keel command semantics from Keeper provider ingress semantics.
- Verify that the security model distinguishes checkpoint attestation, payload secrecy, and provider provenance rather than collapsing them into one mechanism.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Keeper remains the operator-facing runtime boundary for multi-player Keel. | The trust model and ingress split would need revision. | Re-check against Keeper architecture as the first voyage is planned. |
| Threshold attestation remains a boundary concern rather than a per-event default. | The design could become operationally heavy and mis-scoped. | Re-check during voyage planning and implementation slicing. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which DKG or resharing flow fits Keeper fleet membership without centralizing trust? | Planner | Open |
| Which transitions truly require threshold attestation versus ordinary signed or hashed evidence? | Planner | Open |
| How should provider revisions be represented so retries and edits remain auditable? | Planner | Open |
| Should mission request application stop at mission creation or also seed exploratory planning artifacts? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Define a Keeper-managed security boundary that keeps Keel authoritative for planning truth.
- [ ] Specify a backend-agnostic audit model based on canonical events, checkpoints, and append-only proofs.
- [ ] Identify which lifecycle transitions justify threshold attestation and which should remain lightweight.
- [ ] Define the provider-neutral mission request boundary shared by Keel and Keeper.
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

## Findings

- Keel should stay backend-agnostic and use append-only proofs as the storage portability layer. Transit is a strong backend, not a mandatory dependency. [SRC-04][SRC-07]
- FROST is the right primitive for quorum attestation of high-consequence transitions and checkpoints, but not for every event. [SRC-01]
- Keeper should own provider polling and routing, while Keel should own mission request parsing, validation, drafting, and application through native command surfaces. [SRC-07][SRC-08][SRC-09]
- A formal GitHub issue mission-request format is a credible first provider because it is simple, human-writable, and maps cleanly onto a normalized request envelope. [SRC-07][SRC-08][SRC-09]

## Opportunity Cost

If this work is ignored, Keeper will likely grow provider-specific intake logic,
ad hoc trust assumptions, and a fragmented audit surface. The cost is delayed
delivery on UI polish or less foundational platform work, but the trade is
favorable because security and intake semantics are harder to retrofit later.

## Dependencies

- Canonical event and request encodings in Keel [SRC-07][SRC-08][SRC-09]
- Keeper provider polling and acknowledgement paths [SRC-07][SRC-08][SRC-09]
- Key lifecycle design for threshold signing, including DKG or resharing [SRC-01]
- Replayable evidence storage for provider payload digests or copies [SRC-04][SRC-07]
- Policy definitions for which transitions require quorum attestation [SRC-01][SRC-07]

## Alternatives Considered

- Transit-only dependency in Keel core (rejected because it over-couples storage choice to planning semantics). [SRC-04][SRC-07]
- Centralized signing server (rejected because it concentrates trust and creates a single point of failure). [SRC-01]
- Free-form issue parsing without a canonical schema (rejected because it is not replayable or reliably automatable). [SRC-07][SRC-08][SRC-09]

## Research Provenance

*Source records from bearing evidence:*

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9591 | 2026-04-06 | 2026-04-06 | high | high | FROST threshold Schnorr specification, round structure, ciphersuites, and security considerations. |
| SRC-04 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9162 | 2026-04-06 | 2026-04-06 | high | high | Inclusion and consistency proofs for append-only transparency logs. |
| SRC-05 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9180 | 2026-04-06 | 2026-04-06 | high | high | HPKE for recipient-sealed payloads and connector-secret content boundaries. |
| SRC-06 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9496 | 2026-04-06 | 2026-04-06 | high | high | Prime-order group abstractions including `ristretto255`; relevant to FROST ciphersuite choice. |
| SRC-07 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/docs/architecture/keeper.md | 2026-04-06 | 2026-04-06 | high | high | Keeper architecture: Keel planning authority, reactor inbox/outbox model, connector ingress, and GitHub-first routing. |
| SRC-08 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper/src/lib.rs | 2026-04-06 | 2026-04-06 | medium | high | Current keeper HTTP surface is minimal and lacks mission-request ingestion. |
| SRC-09 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper-cli/src/main.rs | 2026-04-06 | 2026-04-06 | medium | high | Current keeper CLI is scriptable but only exposes missions, start, and status operations. |

---

*This PRD was seeded from bearing `VDupml7OG`. See `bearings/VDupml7OG/` for original research.*
