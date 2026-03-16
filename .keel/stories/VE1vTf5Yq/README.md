---
id: VE1vTf5Yq
title: Factor Dependency State into Bearing Sort Order
type: feat
status: done
created_at: 2026-03-16T04:47:17
updated_at: 2026-03-16T05:10:09
operator-signal:
scope: VDiHwLLfY/VE1vAyNzt
index: 3
blocked_by:
  - VE1vQc4Lh
started_at: 2026-03-16T05:03:26
completed_at: 2026-03-16T05:10:09
---

# Factor Dependency State into Bearing Sort Order

## Summary

Extend bearing sort order in `bearing list` and `next` to demote bearings whose `depends_on` targets are not in a terminal state. Bearings with unresolved dependencies sort below those that are ready to research.

## Acceptance Criteria

- [x] [SRS-04/AC-01] `bearing list` sorts bearings with unresolved dependencies below bearings whose dependencies are all terminal. <!-- verify: cargo test -p keel bearing_list_demotes_unresolved_dependencies, SRS-04:start:end -->
