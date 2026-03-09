---
id: 1vzWgV000
title: Goal Link Persistence
type: feat
status: backlog
created_at: 2026-03-09T02:13:55
updated_at: 2026-03-09T02:18:17
scope: 1vzWfz000/1vzWg5000
index: 2
---

# Goal Link Persistence

## Summary

Persist machine-readable goal references from bearing `BRIEF.md` into laid-bearing frontmatter.

## Acceptance Criteria

- [ ] [SRS-01/AC-03] Parse `BRIEF.md` Success Criteria entries and persist validated epic-goal link metadata on the bearing.
- [ ] [SRS-01/AC-04] Reject invalid goal references (or unknown goals) with a deterministic validation error before write.
