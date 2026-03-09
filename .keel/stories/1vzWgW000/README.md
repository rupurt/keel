---
id: 1vzWgW000
title: Lineage Doctor Checks
type: feat
status: done
created_at: 2026-03-09T02:13:56
updated_at: 2026-03-09T09:24:18
scope: 1vzWfz000/1vzWg8000
index: 5
started_at: 2026-03-09T09:20:11
completed_at: 2026-03-09T09:24:18
---

# Lineage Doctor Checks

## Summary

Implement doctor diagnostics for stale or missing bearing lineage states.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add hard-fail diagnostics for laid bearings missing `epic` lineage. <!-- verify: cargo test --lib check_bearing_lineage_epic_flags_laid_without_epic, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Add hard-fail diagnostics for invalid goal-lineage references, including offending artifact and suggested remediation command. <!-- verify: cargo test --lib check_bearing_lineage_goals_flags_invalid_format, SRS-02:start:end -->
