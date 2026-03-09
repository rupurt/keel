---
id: 1vzWgY000
title: Legacy Bearing Migration
type: feat
status: done
created_at: 2026-03-09T02:13:58
updated_at: 2026-03-09T09:29:19
scope: 1vzWfz000/1vzWg8000
index: 7
started_at: 2026-03-09T09:26:28
completed_at: 2026-03-09T09:29:19
---

# Legacy Bearing Migration

## Summary

Create migration path for legacy laid-bearing artifacts created before this lineage contract.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Add an explicit migration/repair flow for pre-contract bearings missing lineage metadata. <!-- verify: cargo test --lib check_bearing_lineage_epic_flags_laid_without_epic, SRS-05:start -->
- [x] [SRS-05/AC-02] Ensure migration checks scale linearly on board-size representative fixtures and remain non-interactive. <!-- verify: cargo test --lib lineage_migration_scales_linearly_on_board_size, SRS-05:end -->
