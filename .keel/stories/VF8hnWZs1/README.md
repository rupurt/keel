---
# system-managed
id: VF8hnWZs1
status: done
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:37:45
# authored
title: Add Roles Surface For Workflow Topology
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkVGjy
index: 1
started_at: 2026-03-27T23:36:18
submitted_at: 2026-03-27T23:37:41
completed_at: 2026-03-27T23:37:45
---

# Add Roles Surface For Workflow Topology

## Summary

Add a direct roles surface so workflow topology stops living mainly inside config output and implied `next` behavior.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel roles` exposes configured role families, default lanes, contracts, and lane behavior in a concise human-readable surface. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel roles --json` exposes stable machine-readable role and lane data. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The role inspection output is deterministic for the same config. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-3.log-->
