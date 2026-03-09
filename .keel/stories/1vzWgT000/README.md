---
id: 1vzWgT000
title: Epic Lineage Field
type: feat
status: done
created_at: 2026-03-09T02:13:53
updated_at: 2026-03-09T09:11:07
scope: 1vzWfz000/1vzWg5000
index: 1
started_at: 2026-03-09T09:07:38
completed_at: 2026-03-09T09:11:07
---

# Epic Lineage Field

## Summary

Implement persistent `epic` lineage persistence during `keel bearing lay` transitions.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add `epic` frontmatter persistence for newly laid bearings and preserve existing frontmatter fields. <!-- verify: cargo test --lib bearing_lay_persists_epic_lineage_field, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Ensure the persisted value is the destination epic ID (for both selected and created epics) in a deterministic format. <!-- verify: cargo test --lib bearing_lay_epic_field_preserves_existing_frontmatter, SRS-02:start:end -->
