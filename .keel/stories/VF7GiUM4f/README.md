---
# system-managed
id: VF7GiUM4f
status: done
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T18:20:34
# authored
title: Project Heartbeat From Git And Worktree Activity
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfk7zv
index: 1
started_at: 2026-03-27T18:20:32
submitted_at: 2026-03-27T18:20:34
completed_at: 2026-03-27T18:20:35
---

# Project Heartbeat From Git And Worktree Activity

## Summary

Add the core read-model projection that derives heartbeat activity from repository state so later CLI and flow surfaces can stop treating `.keel/heartbeat` as the primary pacemaker input.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A reusable heartbeat projection derives the latest activity timestamp from dirty tracked files first and otherwise from reachable commit activity. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] The projection exposes which signal source won so downstream consumers do not need to re-run repository heuristics independently. <!-- verify: manual, SRS-01:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Deterministic tests cover dirty, clean, and unavailable repository-state cases without surfacing inode-level details as the user-facing contract. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->
