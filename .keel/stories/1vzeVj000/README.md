---
id: 1vzeVj000
title: Mission Doctor Checks
type: feat
status: done
created_at: 2026-03-09T10:35:19
updated_at: 2026-03-09T10:35:19
scope: 1vzeJF000/1vzeMv000
index: 6
---

# Mission Doctor Checks

## Summary

Implement mission doctor checks: MissionGoalAchieved, MissionActiveNoWork, MissionOrphanedLineage.

## Acceptance Criteria

- [x] [SRS-04/AC-01] MissionGoalAchieved check flags Info when all board-verifiable goals for an active mission are met <!-- verify: test --> <!-- SRS-04:start:end -->
- [x] [SRS-05/AC-01] MissionActiveNoWork check warns when mission is Active but no mission-scoped entities are in non-terminal state <!-- verify: test --> <!-- SRS-05:start:end -->
- [x] [SRS-06/AC-01] MissionOrphanedLineage check errors when entity has mission field referencing nonexistent mission ID <!-- verify: test --> <!-- SRS-06:start:end -->
