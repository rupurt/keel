---
id: VDXIHgO6W
title: Define BoardStore And EntityStore Traits
type: feat
status: done
created_at: 2026-03-10T22:45:00
updated_at: 2026-03-10T23:08:39
scope: VDXBSiFXW/VDXHyy82b
index: 1
started_at: 2026-03-10T22:45:00
submitted_at: 2026-03-10T23:08:33
completed_at: 2026-03-10T23:08:39
---

# Define BoardStore And EntityStore Traits

## Summary

Define the core trait abstractions for Keel's storage layer.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `BoardStore` trait defined with `load` and `save` methods. <!-- verify: manual, SRS-01:start -->
- [x] [SRS-02/AC-01] `EntityStore<T>` trait defined with `get`, `list`, `put`, and `delete` methods. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-NFR-01/AC-01] Traits use abstract IDs rather than `PathBuf` for entity selection. <!-- verify: manual, SRS-NFR-01:start:end -->
