---
id: VE1vOqhch
title: Add Depends On Field to Bearing Frontmatter
type: feat
status: done
created_at: 2026-03-16T04:46:59
updated_at: 2026-03-16T04:58:05
operator-signal:
scope: VDiHwLLfY/VE1vAyNzt
index: 1
started_at: 2026-03-16T04:49:05
completed_at: 2026-03-16T04:58:05
---

# Add Depends On Field to Bearing Frontmatter

## Summary

Add an optional `depends_on` field to BearingFrontmatter so operators can declare explicit dependency edges between bearings. Update the test fixture to support the new field.

## Acceptance Criteria

- [x] [SRS-01/AC-01] BearingFrontmatter deserializes `depends_on: [BRG-A, BRG-B]` from bearing README.md and exposes it as `Option<Vec<String>>`. <!-- verify: cargo test -p keel-core bearing_frontmatter_deserializes_depends_on, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Bearings without `depends_on` in frontmatter load with `None` (backward compatible). <!-- verify: cargo test -p keel-core bearing_frontmatter_handles_defaults, SRS-01:start:end, proof: ac-2.log-->
