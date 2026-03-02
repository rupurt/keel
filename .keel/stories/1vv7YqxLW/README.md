---
id: 1vv7YqxLW
title: Purge Unused Compatibility Fields from Model Structs
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:48:03
scope: 1vv7YWzw2/1vv7YeGDR
index: 3
submitted_at: 2026-02-24T21:48:03
completed_at: 2026-02-24T21:48:03
---

# Purge Unused Compatibility Fields from Model Structs

## Summary

Remove deprecated and compatibility fields from the core data models.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Remove `priority` and `depends` from `StoryFrontmatter` in `src/model/story.rs`. <!-- verify: manual, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Remove `#[serde(alias = "created")]` from all `created_at` fields. <!-- verify: manual, SRS-03:end, proof: ac-2.log-->
