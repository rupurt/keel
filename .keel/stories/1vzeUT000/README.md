---
id: 1vzeUT000
title: Mission Transition Commands
type: feat
status: icebox
created_at: 2026-03-09T10:34:01
updated_at: 2026-03-09T10:34:01
scope: 1vzeJF000/1vzeMq000
index: 3
---

# Mission Transition Commands

## Summary

Implement mission lifecycle transition commands: pause, achieve, verify, abandon.

## Acceptance Criteria

- [ ] [SRS-07/AC-01] `keel mission pause <id>` transitions Active → Paused <!-- verify: test --> <!-- SRS-07:start:end -->
- [ ] [SRS-08/AC-01] `keel mission achieve <id>` transitions Active → Achieved when all board-verifiable goals are met <!-- verify: test --> <!-- SRS-08:start:end -->
- [ ] [SRS-08/AC-02] Achievement is rejected when any board-verifiable goal is unmet, with diagnostic output <!-- verify: test --> <!-- SRS-08:start:end -->
- [ ] [SRS-09/AC-01] `keel mission verify <id>` transitions Achieved → Verified (terminal) <!-- verify: test --> <!-- SRS-09:start:end -->
- [ ] [SRS-10/AC-01] `keel mission abandon <id>` transitions Active or Paused → Abandoned (terminal) <!-- verify: test --> <!-- SRS-10:start:end -->
