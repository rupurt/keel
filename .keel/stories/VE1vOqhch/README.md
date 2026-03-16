---
id: VE1vOqhch
title: Add Depends On Field to Bearing Frontmatter
type: feat
status: backlog
created_at: 2026-03-16T04:46:59
updated_at: 2026-03-16T04:48:16
operator-signal:
scope: VDiHwLLfY/VE1vAyNzt
index: 1
---

# Add Depends On Field to Bearing Frontmatter

## Summary

Add an optional `depends_on` field to BearingFrontmatter so operators can declare explicit dependency edges between bearings. Update the test fixture to support the new field.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] BearingFrontmatter deserializes `depends_on: [BRG-A, BRG-B]` from bearing README.md and exposes it as `Option<Vec<String>>`. <!-- verify: test, SRS-01:start:end -->
- [ ] [SRS-01/AC-02] Bearings without `depends_on` in frontmatter load with `None` (backward compatible). <!-- verify: test, SRS-01:start:end -->
