---
# system-managed
id: VF0XEmObl
status: done
created_at: 2026-03-26T13:33:37
updated_at: 2026-03-26T13:44:31
# authored
title: Split Speccy Into Focused Modules Without Behavior Changes
type: feat
operator-signal:
scope: VF0XAFqlF/VF0XBQxJ5
index: 1
started_at: 2026-03-26T13:39:01
submitted_at: 2026-03-26T13:44:28
completed_at: 2026-03-26T13:44:31
---

# Split Speccy Into Focused Modules Without Behavior Changes

## Summary

Split the new `speccy` crate into focused source modules so catalog loading, hook definitions, rendering, and frontmatter mutation are no longer mixed in one file. This pass must preserve the current supported behavior and keep the public surface stable while the internal structure becomes explicit.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `crates/speccy/src/lib.rs` becomes a thin public boundary that re-exports focused modules instead of owning the full implementation directly. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Existing `speccy` render, catalog, and frontmatter mutation behavior remains covered by automated tests after the module split. <!-- verify: manual, SRS-01:start:end, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Keel still compiles against `speccy` without any intended behavior changes at the end of the first pass. <!-- verify: manual, SRS-01:start:end, proof: ac-3.log-->
