---
id: 1vzeUa000
title: Mission Achievement Gate Logic
type: feat
status: done
created_at: 2026-03-09T10:34:08
updated_at: 2026-03-09T10:34:08
started_at: 2026-03-09T10:34:08
scope: 1vzeJF000/1vzeMv000
index: 4
---

# Mission Achievement Gate Logic

## Summary

Implement achievement gate that rejects `keel mission achieve` when board goals are unmet.

## Acceptance Criteria

- [x] [SRS-07/AC-01] Achievement gate evaluates each board-verifiable goal against current board state <!-- verify: test, SRS-07:start:end -->
- [x] [SRS-07/AC-02] Gate rejects transition when any board goal is unmet, returning diagnostic list <!-- verify: test, SRS-07:start:end -->
