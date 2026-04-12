---
# system-managed
id: VGYFpAdlQ
status: backlog
created_at: 2026-04-11T22:26:35
updated_at: 2026-04-11T22:31:17
# authored
title: Specify The Janitor Handoff And GitHub Connector Contract
type: feat
operator-signal:
scope: VGYFmJEuH/VGYFoW0Vc
index: 1
---

# Specify The Janitor Handoff And GitHub Connector Contract

## Summary

Define the first explicit handoff between Spoke Keeper and Keel so Keeper can
run janitor posture over a bound board, choose an allowed Keel board role per
action, and route GitHub maintenance work without direct `.keel` mutation.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The story defines a custody context that records Keeper identity/provenance, janitor posture, and selected Keel board role separately. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] The story defines the janitor automation envelope and human escalation boundary across the turn loop. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] The story defines the GitHub connector ingress/egress contract and provider acknowledgement path for janitor stewardship. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] The story names the first `keel` and `spoke` surfaces that must change to land the handoff. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-NFR-01/AC-01] The story keeps the handoff deterministic and replayable across retries and provider re-delivery. <!-- verify: manual, SRS-NFR-01:start:end -->
- [ ] [SRS-NFR-02/AC-01] The story preserves an explicit distinction between Keeper posture and Keel board-role routing. <!-- verify: manual, SRS-NFR-02:start:end -->
