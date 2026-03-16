---
id: VE3z5SB01
title: Pulse Reports Per Routine Materialization Outcome
type: feat
status: done
created_at: 2026-03-16T13:14:10
updated_at: 2026-03-16T13:29:27
operator-signal:
scope: VE3KrOPS/VE3yUoUUy
index: 3
started_at: 2026-03-16T13:28:16
completed_at: 2026-03-16T13:29:27
---

# Pulse Reports Per Routine Materialization Outcome

## Summary

Ensure the pulse command's non-scene, non-json output prints one structured line per routine showing its materialization outcome (Created, Skipped, Rejected) and reason.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Pulse text output includes one line per evaluated routine with outcome and reason <!-- verify: cargo test --bin keel pulse_human_output, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Created outcome shows the story ID that was materialized <!-- verify: cargo test --bin keel pulse_human_output, SRS-03:start:end, proof: ac-2.log-->
