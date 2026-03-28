---
# system-managed
id: VF7GiVp4h
status: backlog
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T17:15:07
# authored
title: Use Derived Heartbeat In Flow With Compatibility Fallback
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfk7zv
index: 3
---

# Use Derived Heartbeat In Flow With Compatibility Fallback

## Summary

Cut `keel flow --scene` over to the derived heartbeat signal while preserving a narrow compatibility fallback to the legacy heartbeat file until pass 2 removes it.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `keel flow --scene` uses the derived heartbeat as its primary energization input when deciding whether to render powered or unplugged state. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] The legacy file-backed heartbeat is consulted only as a bounded compatibility fallback when the derived heartbeat is unavailable during pass 1. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] Regression coverage proves energized, unplugged, and fallback scenarios so pass 2 can delete the file-backed path safely. <!-- verify: manual, SRS-04:start:end -->
