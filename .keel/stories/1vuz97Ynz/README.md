---
id: 1vuz97Ynz
title: Enforce Human Next Queue Boundary
type: feat
status: done
created_at: 2026-02-24T12:36:41
updated_at: 2026-02-24T14:10:10
scope: 1vuz8K4NM/1vuz8VYmc
index: 3
submitted_at: 2026-02-24T14:08:45
completed_at: 2026-02-24T14:10:10
started_at: 2026-02-24T13:22:43
---

# Enforce Human Next Queue Boundary

## Summary

Enforce the actor boundary that human-mode `keel next` never returns implementation work and only surfaces human-queue actions.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Update human-mode selection logic so `calculate_next(..., agent_mode=false, ...)` cannot emit `NextDecision::Work`. <!-- verify: manual, SRS-03:start -->
- [x] [SRS-03/AC-02] Ensure human-mode outcomes are restricted to human queue decision kinds only. <!-- verify: manual, SRS-03:continues -->
- [x] [SRS-03/AC-03] Add tests covering mixed queue states where human mode previously surfaced implementation work. <!-- verify: manual, SRS-03:end -->
