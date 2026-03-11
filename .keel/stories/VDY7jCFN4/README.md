---
id: VDY7jCFN4
title: Restructure Lib Rs For Layer Exports
type: refactor
status: backlog
created_at: 2026-03-10T23:35:00
scope: VDXBUEBAG/VDY7YBSFR
index: 1
updated_at: 2026-03-11T02:28:55
---

# Restructure Lib Rs For Layer Exports

## Summary

Restructure `src/lib.rs` to explicitly export the core layers of Keel as public modules.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `application`, `domain`, `infrastructure`, and `read_model` are exported as `pub mod` in `lib.rs`. <!-- verify: compilation, SRS-01:start:end -->
- [ ] [SRS-03/AC-01] `src/cli` remains private or not re-exported in `lib.rs`. <!-- verify: inspection, SRS-03:start:end -->
