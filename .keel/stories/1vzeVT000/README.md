---
id: 1vzeVT000
title: Mission Show And List Commands
type: feat
status: done
created_at: 2026-03-09T10:35:03
updated_at: 2026-03-09T13:41:44
scope: 1vzeJF000/1vzeMq000
index: 8
---

# Mission Show And List Commands

## Summary

Implement `keel mission show` and `keel mission list` commands for mission visibility.

## Acceptance Criteria

- [x] [SRS-05/AC-01] `keel mission show <id>` displays title, status, goals, child entities, and LOG summary <!-- verify: test --> <!-- SRS-05:start:end -->
- [x] [SRS-06/AC-01] `keel mission list` displays all missions with id, title, status, and child count <!-- verify: test --> <!-- SRS-06:start:end -->
- [x] [SRS-05/AC-02] Show command supports --json output <!-- verify: test --> <!-- SRS-05:start:end -->
