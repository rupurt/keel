---
id: VDY7jFvPQ
title: Export Domain Ports In Public API
type: refactor
status: backlog
created_at: 2026-03-10T23:35:00
scope: VDXBUEBAG/VDY7YBSFR
index: 2
updated_at: 2026-03-11T02:28:55
---

# Export Domain Ports In Public API

## Summary

Ensure that the domain ports (storage traits) are easily accessible from the library root.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `BoardStore` and `EntityStore` traits are re-exported in `lib.rs` or via a clear public path. <!-- verify: compilation, SRS-02:start:end -->
