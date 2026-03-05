---
id: 1vuM0Nbhn
title: Merge GateProblem and Problem Types
type: chore
status: done
created_at: 2026-02-22T18:49:03
updated_at: 2026-02-22T19:03:23
scope: 1vuLyP3zx/1vuM0BauX
index: 1
submitted_at: 2026-02-22T09:29:30
completed_at: 2026-02-22T09:29:31
started_at: 2026-02-22T09:24:31
---

# Merge GateProblem and Problem Types

## Summary

This story unified the error reporting system by merging `GateProblem` into `Problem` and `GateSeverity` into `Severity`.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `GateProblem` and `Problem` types are merged into a single `Problem` type in `doctor/types.rs` <!-- verify: ! grep -q "enum GateProblem" src/validation/types.rs, proof: ac-1.log, SRS-01:start:end -->
- [x] [SRS-01/AC-02] `GateSeverity` is merged with `Severity` <!-- verify: ! grep -q "enum GateSeverity" src/validation/types.rs, proof: ac-2.log, SRS-01:start:end -->
