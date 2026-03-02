---
id: 1vx8UtmC9
title: Remove Legacy Roots And Enforce Normalized Contracts
type: feat
status: done
created_at: 2026-03-02T11:00:03
updated_at: 2026-03-02T12:04:04
scope: 1vwq96cpt/1vx8TLqpp
index: 1
submitted_at: 2026-03-02T12:03:53
completed_at: 2026-03-02T12:04:04
---

# Remove Legacy Roots And Enforce Normalized Contracts

## Summary

Remove transitional legacy module roots and add explicit architecture contracts
that fail if old top-level families are reintroduced or normalized layer
boundaries are violated.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Architecture contract tests explicitly enforce normalized roots (`cli`, `application`, `domain`, `infrastructure`, `read_model`) and fail on forbidden legacy roots. <!-- verify: manual, SRS-05:start:end, proof: ac-1.log -->
- [x] [SRS-05/AC-02] Repository tree no longer contains active legacy top-level module families used before normalization. <!-- verify: manual, SRS-05:continues, proof: ac-2.log -->
- [x] [SRS-05/AC-03] Story-level evidence includes test output proving normalized contracts and behavior parity. <!-- verify: manual, SRS-05:end, proof: ac-3.log -->
