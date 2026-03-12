---
id: VDcFgtbNC
title: Scheduled Flow Lane Projection
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T21:57:42
operator-signal: 
scope: VDakmG8cH/VDcFd5Sop
index: 2
started_at: 2026-03-11T21:47:30
completed_at: 2026-03-11T21:57:42
---

# Scheduled Flow Lane Projection

## Summary

Extend `keel flow` so scheduled automation demand is visible before or after a
pulse run.

## Acceptance Criteria

- [x] [SRS-04/AC-01] `keel flow` surfaces a scheduled lane or scheduled-capacity view driven by routine schedule state. <!-- verify: cargo test build_output_surfaces_scheduled_capacity_from_routine_schedule_state --bin keel, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Scheduled output distinguishes due-now automation from upcoming work with explicit operator guidance. <!-- verify: cargo test render_annotated_flow_shows_scheduled_capacity_guidance --bin keel, SRS-04:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-02] Scheduled automation output remains stable and reviewable across flow render paths. <!-- verify: cargo test render_annotated_flow_keeps_scheduled_output_stable_across_widths --bin keel, SRS-NFR-02:end, proof: ac-3.log-->
