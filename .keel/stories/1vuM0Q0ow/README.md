---
id: 1vuM0Q0ow
title: Refactor Doctor to Delegate to Centralized Check Modules
type: refactor
status: done
created_at: 2026-02-22T18:49:06
updated_at: 2026-02-22T19:08:03
scope: 1vuLyP3zx/1vuM0BauX
index: 4
submitted_at: 2026-02-22T00:00:00
completed_at: 2026-02-22T00:00:00
started_at: 2026-02-22T09:24:33
---

# Refactor Doctor to Delegate to Centralized Check Modules

## Summary

TODO: Describe the story

## Acceptance Criteria

- [x] [SRS-04/AC-01] `doctor` delegates to unified transition gates for domain rule validation <!-- verify: manual, proof: ac-1.log, SRS-04:start:end -->
- [x] [SRS-04/AC-02] Shared check functions are called by both `doctor` and `gating.rs` <!-- verify: manual, SRS-04:start:end -->
