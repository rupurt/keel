---
id: VDcFgsuLw
title: Next Temporal Countdown Rendering
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T21:20:55
operator-signal: 
scope: VDakmCGYi/VDcFd5kmn
index: 1
started_at: 2026-03-11T21:06:53
completed_at: 2026-03-11T21:20:55
---

# Next Temporal Countdown Rendering

## Summary

Expose routine due-state through `keel next` so scheduled work is reviewable in
both human and JSON pull surfaces.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Human-readable `keel next` output shows due-now or next-run countdown context for routine work. <!-- verify: cargo test render_scheduled_routines_human_shows_due_and_countdown_context --bin keel, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] JSON `keel next` output includes structured gating rationale and next eligible time for scheduled work. <!-- verify: cargo test decision_to_json_includes_scheduled_routines_projection --bin keel, SRS-02:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] Countdown and gating text stay stable enough for CLI regression assertions. <!-- verify: cargo test render_scheduled_routines_human_shows_due_and_countdown_context --bin keel, SRS-NFR-02:start:end, proof: ac-3.log-->
- [x] [SRS-03/AC-01] Non-due routines are filtered out of actionable work selection before ranking. <!-- verify: cargo test calculate_next_filters_non_due_routine_scope_before_ranking --bin keel, SRS-03:start, proof: ac-4.log-->
- [x] [SRS-03/AC-02] Due routines participate in existing prioritization semantics without reordering unrelated actionable work. <!-- verify: cargo test calculate_next_keeps_due_routine_scope_in_existing_priority_order --bin keel, SRS-03:end, proof: ac-5.log-->
