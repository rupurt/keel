---
id: 1vuz9i4Ja
title: Normalize Epic Completion Field to Completed At
type: feat
status: done
created_at: 2026-02-24T12:37:18
updated_at: 2026-02-24T16:26:20
scope: 1vuz8K4NM/1vuz8jNo3
index: 4
submitted_at: 2026-02-24T06:23:38
completed_at: 2026-02-24T06:23:39
started_at: 2026-02-24T06:18:39
---

# Normalize Epic Completion Field to Completed At

## Summary

Normalize epic completion handling to `completed_at` across command handlers, loaders/models, and doctor diagnostics.

## Acceptance Criteria

- [x] [SRS-04/AC-01] Update epic completion and reopen flows to read/write `completed_at` as the canonical completion timestamp field. <!-- verify: manual, SRS-04:start -->
- [x] [SRS-04/AC-02] Remove or migrate remaining references to legacy epic completion field names in loaders, serializers, and doctor validations. <!-- verify: manual, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add tests covering epic completion and reopen behavior plus doctor checks to confirm only `completed_at` is accepted. <!-- verify: manual, SRS-04:end -->
