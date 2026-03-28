---
# system-managed
id: VF7GiUM4f
status: backlog
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T17:15:07
# authored
title: Project Heartbeat From Git And Worktree Activity
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfk7zv
index: 1
---

# Project Heartbeat From Git And Worktree Activity

## Summary

Add the core read-model projection that derives heartbeat activity from repository state so later CLI and flow surfaces can stop treating `.keel/heartbeat` as the primary pacemaker input.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] A reusable heartbeat projection derives the latest activity timestamp from dirty tracked files first and otherwise from reachable commit activity. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-01/AC-02] The projection exposes which signal source won so downstream consumers do not need to re-run repository heuristics independently. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-NFR-01/AC-01] Deterministic tests cover dirty, clean, and unavailable repository-state cases without surfacing inode-level details as the user-facing contract. <!-- verify: manual, SRS-NFR-01:start:end -->
