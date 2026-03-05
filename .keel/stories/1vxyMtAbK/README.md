---
id: 1vxyMtAbK
title: Story Blocked By Metadata Override
type: feat
status: backlog
created_at: 2026-03-04T18:23:15
updated_at: 2026-03-04T18:25:50
scope: 1vxyM0hvn/1vxyMT6nz
---

# Story Blocked By Metadata Override

## Summary

Add optional story-level `blocked_by` metadata so planners can explicitly encode parallel constraints regardless of inferred semantic safety.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] Story frontmatter accepts optional `blocked_by` list of story IDs. <!-- verify: cargo test --lib next_parallel_blocked_by_frontmatter_parses, SRS-05:start:end -->
- [ ] [SRS-05/AC-02] `blocked_by` overrides inferred allow decisions and forces pairwise blocking in `next --parallel`. <!-- verify: cargo test --lib next_parallel_blocked_by_override_enforced, SRS-05:start:end -->
