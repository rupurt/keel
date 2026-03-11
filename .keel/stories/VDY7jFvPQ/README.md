---
id: VDY7jFvPQ
title: Export Domain Ports In Public API
type: refactor
status: needs-human-verification
created_at: 2026-03-10T23:35:00
scope: VDXBUEBAG/VDY7YBSFR
index: 2
updated_at: 2026-03-11T04:30:27
started_at: 2026-03-11T04:30:02
submitted_at: 2026-03-11T04:30:27
---

# Export Domain Ports In Public API

## Summary

Ensure that the domain ports (storage traits) are easily accessible from the library root.

## Acceptance Criteria

- [x] [SRS-02/AC-01] `BoardStore` and `EntityStore` traits are re-exported in `lib.rs` or via a clear public path. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log-->
