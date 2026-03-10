---
id: 1vzeVn000
title: Mission Aware Keel Next
type: feat
status: done
created_at: 2026-03-09T10:35:23
updated_at: 2026-03-09T10:35:23
started_at: 2026-03-09T10:35:23
scope: 1vzeJF000/1vzeMz000
index: 5
---

# Mission Aware Keel Next

## Summary

Make `keel next --agent` mission-aware so it recommends work creation when queue empty but mission incomplete.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel next --agent` returns mission recommendation when no stories ready but active mission has unmet goals <!-- verify: test, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Recommendation includes mission context: unmet goal summary and suggested action type <!-- verify: test, SRS-02:start:end -->
