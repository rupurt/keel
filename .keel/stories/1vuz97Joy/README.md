---
id: 1vuz97Joy
title: Align Flow Derivation Bottleneck Messaging and Tests
type: feat
status: done
created_at: 2026-02-24T12:36:41
updated_at: 2026-02-24T14:24:11
scope: 1vuz8K4NM/1vuz8VYmc
index: 4
submitted_at: 2026-02-24T14:18:41
completed_at: 2026-02-24T14:24:11
started_at: 2026-02-24T13:27:41
---

# Align Flow Derivation Bottleneck Messaging and Tests

## Summary

Align flow-state derivation and bottleneck messaging with the same queue policy used by `next` so thresholds and state labels do not drift.

## Acceptance Criteria

- [x] [SRS-04/AC-01] Update flow derivation and bottleneck threshold checks to use shared policy constants. <!-- verify: manual, SRS-04:start -->
- [x] [SRS-04/AC-02] Ensure flow assessment messaging reflects the same classification boundaries used by `next`. <!-- verify: manual, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add cross-module tests for boundary conditions that assert consistent `next` and `flow` classification results. <!-- verify: manual, SRS-04:end -->
