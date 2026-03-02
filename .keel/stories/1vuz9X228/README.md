---
id: 1vuz9X228
title: Remove Duplicate Command Side Checks and Error Formatters
type: feat
status: done
created_at: 2026-02-24T12:37:07
updated_at: 2026-02-24T18:57:09
scope: 1vuz8K4NM/1vuz8dYT5
index: 4
submitted_at: 2026-02-24T18:54:34
completed_at: 2026-02-24T18:57:09
started_at: 2026-02-24T15:45:50
---

# Remove Duplicate Command Side Checks and Error Formatters

## Summary

Remove duplicate command-side checks and centralize transition error formatting so validation outcomes are consistent across callers.

## Acceptance Criteria

- [x] [SRS-04/AC-01] Consolidate transition/gate error formatting into a shared formatter used by runtime and reporting paths. <!-- verify: manual, SRS-04:start -->
- [x] [SRS-04/AC-02] Remove duplicated side checks in command handlers when equivalent checks are provided by gate evaluators. <!-- verify: manual, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add assertions or snapshots that validate standardized error message structure across key transitions. <!-- verify: manual, SRS-04:end -->
