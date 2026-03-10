---
id: 1vzeVf000
title: Mission Lineage Field And Loader
type: feat
status: done
created_at: 2026-03-09T10:35:15
updated_at: 2026-03-09T10:35:15
started_at: 2026-03-09T10:35:15
scope: 1vzeJF000/1vzeMv000
index: 5
---

# Mission Lineage Field And Loader

## Summary

Add optional `mission` field to epic, bearing, and ADR frontmatter with loader support.

## Acceptance Criteria

- [x] [SRS-01/AC-01] EpicFrontmatter, BearingFrontmatter, and AdrFrontmatter structs have optional `mission: Option<String>` field <!-- verify: test, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Loader parses `mission` field from YAML frontmatter for epics, bearings, and ADRs <!-- verify: test, SRS-02:start:end -->
