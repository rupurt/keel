---
id: VE3z3roGf
title: Guard Materialization Against Terminal Voyage Scope
type: feat
status: done
created_at: 2026-03-16T13:14:04
updated_at: 2026-03-16T13:21:13
operator-signal:
scope: VE3KrOPS/VE3yUoUUy
index: 1
started_at: 2026-03-16T13:18:17
completed_at: 2026-03-16T13:21:13
---

# Guard Materialization Against Terminal Voyage Scope

## Summary

Extend `validate_target_scope()` in `routine_materialization.rs` to reject materialization when the target voyage is in a terminal state (done). Pulse should skip the routine with a clear rejection reason instead of creating an orphaned story.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Pulse skips materialization when target voyage status is done <!-- verify: cargo test --bin keel pulse_rejects, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Skipped routine produces a Rejected outcome with reason explaining terminal scope <!-- verify: cargo test --bin keel pulse_rejects, SRS-01:start:end, proof: ac-2.log-->
