---
id: VDY7jCFN4
title: Restructure Lib Rs For Layer Exports
type: refactor
status: done
created_at: 2026-03-10T23:35:00
scope: VDXBUEBAG/VDY7YBSFR
index: 1
updated_at: 2026-03-11T04:30:53
started_at: 2026-03-11T04:06:16
submitted_at: 2026-03-11T04:30:26
completed_at: 2026-03-11T04:30:53
---

# Restructure Lib Rs For Layer Exports

## Summary

Restructure `src/lib.rs` to explicitly export the core layers of Keel as public modules.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `application`, `domain`, `infrastructure`, and `read_model` are exported as `pub mod` in `lib.rs`. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-01] `src/cli` remains private or not re-exported in `lib.rs`. <!-- verify: manual, SRS-03:start:end, proof: ac-2.log-->
