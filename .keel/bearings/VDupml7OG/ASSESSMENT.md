---
id: VDupml7OG
---

# Collaborative Cryptographic Primitives Over Adversarial Transport — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 5 | Foundational security and intake model for Keeper-managed multi-player Keel |
| Confidence | 4 | The primitives and command model are technically clear, but key lifecycle and policy work remain |
| Effort | 4 | Requires CLI, Keeper ingress, evidence model, and future crypto plumbing |
| Risk | 3 | Main risks are complexity creep, over-signing, and key-management mistakes |

*Scores range from 1-5:*
- 1 = Very Low
- 2 = Low
- 3 = Medium
- 4 = High
- 5 = Very High

## Analysis

## Findings

- Keel should stay backend-agnostic and use append-only proofs as the storage
  portability layer. Transit is a strong backend, not a mandatory dependency.
  [SRC-04][SRC-07]
- FROST is the right primitive for quorum attestation of high-consequence
  transitions and checkpoints, but not for every event. [SRC-01]
- Keeper should own provider polling and routing, while Keel should own mission
  request parsing, validation, drafting, and application through native command
  surfaces. [SRC-07][SRC-08][SRC-09]
- A formal GitHub issue mission-request format is a credible first provider
  because it is simple, human-writable, and maps cleanly onto a normalized
  request envelope. [SRC-07][SRC-08][SRC-09]

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

- Transit-only dependency in Keel core (rejected because it over-couples storage
  choice to planning semantics). [SRC-04][SRC-07]
- Centralized signing server (rejected because it concentrates trust and creates
  a single point of failure). [SRC-01]
- Free-form issue parsing without a canonical schema (rejected because it is not
  replayable or reliably automatable). [SRC-07][SRC-08][SRC-09]

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-04][SRC-07]
[ ] Park → keep as a foundational design package until Keeper ingress and
    attestation work is scheduled [SRC-01][SRC-04][SRC-07][SRC-08][SRC-09]
[ ] Decline → document learnings [SRC-01]
