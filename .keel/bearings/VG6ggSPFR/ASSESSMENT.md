---
id: VG6ggSPFR
---

# Keeper Provider Mission Request Ingress Research — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 4 | Provider ingress is the operational bridge between external requests and native Keel planning state. [SRC-01][SRC-03] |
| Confidence | 4 | Keeper already owns provider routing and the GitHub-first mission-request envelope is defined. [SRC-01][SRC-03] |
| Effort | 3 | The work is to normalize, validate, and acknowledge inbound requests through Keeper rather than inventing a new runtime role. [SRC-01][SRC-02][SRC-03] |
| Risk | 2 | The main risk is overfitting the first provider, which is manageable by enforcing the provider-neutral request envelope. [SRC-01][SRC-03] |

*Scores range from 1-5:*
- 1 = Very Low
- 2 = Low
- 3 = Medium
- 4 = High
- 5 = Very High

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

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-03]
[ ] Park → revisit later [SRC-01]
[ ] Decline → document learnings [SRC-01]
