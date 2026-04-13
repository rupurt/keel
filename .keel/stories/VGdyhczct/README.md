---
# system-managed
id: VGdyhczct
status: backlog
created_at: 2026-04-12T21:56:12
updated_at: 2026-04-12T21:56:51
# authored
title: Add Stack-Aware Turn Next Mission Next And Doctor Projections
type: feat
operator-signal:
scope: VGdxDziFF/VGdyGtOFK
index: 1
---

# Add Stack-Aware Turn Next Mission Next And Doctor Projections

## Summary

Define how Mission Stack state appears in the canonical operator commands so a
member repo can tell whether it may act, why it is blocked, and how that stack
state differs from the repo-local heartbeat contract.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `keel turn` requirements describe stack id, branch, local member role, stack mode, and checkpoint status. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] `keel next` requirements describe stack-blocked, yield, or continue-local outcomes for gated repos. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] `keel mission next --status` requirements describe linked member missions, negotiations, or waiting receipts. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] `keel doctor` requirements describe Mission Stack violations such as wrong branch or missing checkpoint acknowledgment. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-05/AC-01] The command contract covers both text and JSON output expectations. <!-- verify: manual, SRS-05:start:end -->
- [ ] [SRS-NFR-02/AC-01] The stack-aware surface contract explicitly preserves repo-local heartbeat semantics. <!-- verify: manual, SRS-NFR-02:start:end -->
