---
# system-managed
id: VF7GiVp4h
status: done
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T18:20:38
# authored
title: Use Derived Heartbeat In Flow With Compatibility Fallback
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfk7zv
index: 3
started_at: 2026-03-27T18:20:37
submitted_at: 2026-03-27T18:20:38
completed_at: 2026-03-27T18:20:39
---

# Use Derived Heartbeat In Flow With Compatibility Fallback

## Summary

Cut `keel flow --scene` over to the derived heartbeat signal so flow behavior no longer depends on the legacy heartbeat file and pass 2 can remove the old path cleanly.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel flow --scene` uses the derived heartbeat as its primary energization input when deciding whether to render powered or unplugged state. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] `keel flow --scene` no longer depends on the legacy file-backed heartbeat path, allowing the migration to remove that path without changing flow behavior. <!-- verify: manual, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-04/AC-01] Regression coverage proves energized and unplugged scenarios across the derived heartbeat model so the file-backed path can be deleted safely. <!-- verify: manual, SRS-04:start:end, proof: ac-3.log-->
