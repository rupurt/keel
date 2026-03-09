---
id: 1vzWgV000
title: Goal Link Persistence
type: feat
status: done
created_at: 2026-03-09T02:13:55
updated_at: 2026-03-09T09:19:56
scope: 1vzWfz000/1vzWg5000
index: 2
started_at: 2026-03-09T09:13:03
completed_at: 2026-03-09T09:19:56
---

# Goal Link Persistence

## Summary

Persist machine-readable goal references from bearing `BRIEF.md` into laid-bearing frontmatter.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Parse `BRIEF.md` Success Criteria entries and persist validated epic-goal link metadata on the bearing. <!-- verify: cargo test --lib bearing_lay_persists_valid_goal_references, SRS-03:start:end -->
- [x] [SRS-04/AC-01] Reject invalid goal references (or unknown goals) with a deterministic validation error before write. <!-- verify: cargo test --lib bearing_lay_rejects_unknown_goal_references, SRS-04:start:end -->
