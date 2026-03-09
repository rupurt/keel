---
id: 1vzWgX000
title: Correction Documentation
type: feat
status: done
created_at: 2026-03-09T02:13:57
updated_at: 2026-03-09T09:26:15
scope: 1vzWfz000/1vzWg8000
index: 6
started_at: 2026-03-09T09:24:32
completed_at: 2026-03-09T09:26:15
---

# Correction Documentation

## Summary

Add validation and migration documentation for lineage checks and corrective actions.

## Acceptance Criteria

- [x] [SRS-04/AC-02] Define and document correction commands for stale/invalid lineage values. <!-- verify: cargo test --lib lay_unknown_goal_recovery_maps_to_brief, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add regression notes for deterministic validation behavior under repeated read/validation cycles. <!-- verify: cargo test --lib lineage_validation_is_deterministic_across_repeated_cycles, SRS-04:end -->
