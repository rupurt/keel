---
id: VDVRaRR5x
title: Route Next Through Configured Lanes
type: feat
status: backlog
created_at: 2026-03-10T15:27:51
updated_at: 2026-03-10T15:29:38
scope: VDVPODBXF/VDVPUCtqS
index: 2
---

# Route Next Through Configured Lanes

## Summary

Replace hardcoded `manager` and `engineer` queue routing in `keel next` with topology-driven lane resolution and lane capability checks.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] `keel next --role <taxonomy>` resolves configured base role families to their default lanes and rejects unknown families with guidance based on configured default role examples. <!-- verify: cargo test -p keel next_role_topology_, SRS-04:start:end, proof: ac-1.log-->
- [ ] [SRS-05/AC-01] [SRS-NFR-02/AC-01] `keel next --parallel` is allowed only for lanes with `parallel = true`, and repeated resolution of the same role/config yields identical lane and capability results. <!-- verify: cargo test -p keel next_parallel_topology_, SRS-05:start:end, SRS-NFR-02:start:end, proof: ac-2.log-->
