---
id: VE3z5SB01
title: Pulse Reports Per Routine Materialization Outcome
type: feat
status: backlog
created_at: 2026-03-16T13:14:10
updated_at: 2026-03-16T13:15:04
operator-signal:
scope: VE3KrOPS/VE3yUoUUy
index: 3
---

# Pulse Reports Per Routine Materialization Outcome

## Summary

Ensure the pulse command's non-scene, non-json output prints one structured line per routine showing its materialization outcome (Created, Skipped, Rejected) and reason.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Pulse text output includes one line per evaluated routine with outcome and reason <!-- verify: test -->
- [ ] [SRS-03/AC-02] Created outcome shows the story ID that was materialized <!-- verify: test -->
