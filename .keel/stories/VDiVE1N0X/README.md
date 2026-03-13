---
id: VDiVE1N0X
title: AttachMissionBearingCommand
type: feat
status: done
created_at: 2026-03-12T21:03:50
updated_at: 2026-03-12T21:14:16
operator-signal: 
scope: VDiDhXVKy/VDiV5sPFH
index: 1
started_at: 2026-03-12T21:04:28
completed_at: 2026-03-12T21:14:16
---

# AttachMissionBearingCommand

## Summary

Add the mission-command path to attach a bearing to a mission in one deterministic command so mission-bearing lineage becomes explicit and consumable by readiness and flow surfaces.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A mission attach command accepts mission and bearing IDs and writes canonical mission/bearing lineage updates in board state. <!-- verify: cargo test attach_ -- --nocapture, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Mission readiness and activation checks account for mission-owned bearings introduced by this command. <!-- verify: cargo test attach_ -- --nocapture, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Invalid states (missing entities, bad ownership, duplicate attach attempts) fail fast with actionable recovery guidance. <!-- verify: cargo test attach_ -- --nocapture, SRS-NFR-01:start:end, proof: ac-3.log-->
