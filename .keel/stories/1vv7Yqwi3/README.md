---
id: 1vv7Yqwi3
title: Remove Legacy Migration Fixes from Doctor
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:45:14
scope: 1vv7YWzw2/1vv7YeGDR
index: 1
submitted_at: 2026-02-24T21:45:14
completed_at: 2026-02-24T21:45:14
---

# Remove Legacy Migration Fixes from Doctor

## Summary

Remove the code responsible for auto-fixing legacy frontmatter formats in `keel doctor`.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Delete `migrate_story_frontmatter` and `migrate_voyage_readme` from `src/commands/diagnostics/doctor/fixes.rs`. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Update `src/commands/diagnostics/doctor/checks/stories.rs` to report errors instead of warnings for legacy fields. <!-- verify: manual, SRS-01:end, proof: ac-2.log-->
