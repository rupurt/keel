---
# system-managed
id: VF7GiXG56
status: done
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T18:20:41
# authored
title: Remove File Heartbeat From Board Models And Caches
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfkizo
index: 1
started_at: 2026-03-27T18:20:39
submitted_at: 2026-03-27T18:20:40
completed_at: 2026-03-27T18:20:41
---

# Remove File Heartbeat From Board Models And Caches

## Summary

Delete the file-backed heartbeat control path from core board loading and supporting projections so the derived heartbeat becomes the only pacemaker signal left in code.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Core board loading, flow, and compatibility code no longer read `.keel/heartbeat` as a required heartbeat source. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Cache invalidation, graph surfaces, and any residual pacemaker plumbing stop treating the file as canonical system state. <!-- verify: manual, SRS-01:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Regression tests prove the board remains healthy and functional without a heartbeat file in the repository. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->
