# Collaborative Cryptographic Primitives Over Adversarial Transport - Product Requirements

> Keel's collaborative coordination primitives (ping/pong/poke, story lifecycle, mission routing) compose naturally over Transit's cryptographically-ordered append-only log to produce a distributed workflow engine where cooperative agents achieve consensus-free coordination with full cryptographic auditability — at fundamentally lower cost than adversarial distributed systems.

The structural properties already present in Keel's primitives (immutable pings, monotonic status transitions, idempotent poke, deterministic routing) are exactly the properties required for convergent replicated state, and Transit's verifiable lineage provides the integrity substrate without contaminating the collaborative hot path.

## Problem Statement

Traditional distributed systems assume adversarial participants and pay coordination costs accordingly: Byzantine fault tolerance requires O(n²) message complexity, consensus protocols add latency per round, and identity systems front-load authentication before every action. These costs scale poorly as participant count grows.

Keel operates in a fundamentally different regime — cooperative agents coordinating work through a shared workflow engine. The threat model isn't "participants may lie" but "the transport may be hostile." This distinction changes which cryptographic primitives are load-bearing:

- **Not needed at the collaborative layer:** consensus, BFT, pre-action authentication, non-malleability
- **Needed at the transport layer:** ordering guarantees, tamper evidence, content integrity, non-repudiation
- **Needed at the boundary:** attestation (proving actions happened), not authorization (proving actions are allowed)

Transit (github.com/spoke-sh/transit) provides the transport-layer guarantees through append-only streams with cryptographic segment digests, manifest roots, and lineage checkpoints. The research question is how Keel's collaborative protocols should be designed to maximally exploit this separation.

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

- [SCOPE-01] Deliver the bearing-backed capability slice for this epic.

### Out of Scope

- [SCOPE-02] Unrelated platform-wide refactors outside bearing findings.

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
- [ ] Formal mapping between Keel primitives (ping/pong/poke, story lifecycle) and CRDT-like convergence properties — demonstrating that collaborative operations are commutative, idempotent, and monotonic by construction
- [ ] Identity model design that distinguishes environmental identity (LocalSystem), credential identity (Authenticated), and emergent/attested identity (contribution history verified by Transit lineage) — with clear boundaries for where each applies
- [ ] Proof that the staged verification model (checksums on hot path, cryptographic digests at segment boundaries, manifest roots at publication) preserves collaborative throughput while providing adversarial-grade auditability
- [ ] Architectural specification for how Keel's inbox maps to Transit streams, how poke maps to branch-and-merge, and how story lifecycle transitions map to lineage checkpoints
- [ ] Analysis of scaling properties — demonstrating that adding cooperative agents increases system capability (more ping resolvers, richer routing, parallel story execution) rather than increasing coordination cost
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

## Findings

- Schnorr threshold signatures (FROST) provide an efficient way for agents to collectively authorize state changes. [SRC-01]

## Opportunity Cost

By investing in distributed cryptographic primitives now, we are deferring work on advanced TUI animations and deeper historical state visualization.

## Dependencies

- Requires a shared public key infrastructure (PKI) or a discovery mechanism for agent public keys. [SRC-01]

## Alternatives Considered

- Centralized signing server (rejected due to single point of failure). [SRC-01]

---

*This PRD was seeded from bearing `VDupml7OG`. See `bearings/VDupml7OG/` for original research.*
