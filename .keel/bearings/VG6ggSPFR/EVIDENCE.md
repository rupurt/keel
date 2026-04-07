---
id: VG6ggSPFR
---

# Keeper Provider Mission Request Ingress Research — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/docs/architecture/keeper.md | 2026-04-07 | 2026-04-07 | high | high | Keeper architecture already assigns provider ingress, connector routing, inbox/outbox handling, and GitHub-first external integration to Keeper. |
| SRC-02 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper/src/lib.rs | 2026-04-07 | 2026-04-07 | medium | high | The current Keeper service surface does not yet expose mission-request ingestion endpoints or workflows. |
| SRC-03 | manual | workspace | /home/alex/workspace/spoke-sh/keel/.keel/bearings/VDupml7OG/MISSION_REQUESTS.md | 2026-04-07 | 2026-04-07 | high | high | The existing research package already defines a GitHub issue activation prefix and a provider-neutral request envelope. |

## Technical Research

## Key Findings

1. Keeper is the correct place for provider polling, normalization, and acknowledgement because those responsibilities already sit on the runtime side of the Keel/Keeper split. [SRC-01][SRC-02]
2. The missing piece is a formal ingress flow that lowers provider artifacts into native Keel commands instead of mutating planning state directly. [SRC-01][SRC-03]
3. GitHub issues are a credible first ingress provider because the activation title and body schema are already specified. [SRC-03]

## Feasibility

This bearing is feasible as a dedicated ingress mission because the architecture
already reserves connector ingress and routed envelopes for Keeper; the main work
is to formalize the normalization and acknowledgement contract.

## Unknowns

- How Keeper should version normalized request revisions when provider content changes
- Which provider failures should create escalation events versus silent retries
