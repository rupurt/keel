---
# system-managed
id: VF7GiVC4g
status: backlog
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T17:15:07
# authored
title: Add Keel Heartbeat Command Surface
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfk7zv
index: 2
---

# Add Keel Heartbeat Command Surface

## Summary

Expose the new derived heartbeat projection through `keel heartbeat` so operators can inspect the exact signal that will govern energized versus unplugged flow behavior.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `keel heartbeat` reports the latest activity timestamp, age, and source from the derived heartbeat projection. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-02/AC-02] The command renders idle or unavailable states without requiring `.keel/heartbeat` to exist. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-NFR-01/AC-01] The operator-facing command output stays platform-stable and does not make inode behavior part of the documented semantics. <!-- verify: manual, SRS-NFR-01:start:end -->
