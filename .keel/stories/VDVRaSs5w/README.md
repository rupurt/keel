---
id: VDVRaSs5w
title: Authorize Acceptance And Templates Through Topology
type: feat
status: backlog
created_at: 2026-03-10T15:27:51
updated_at: 2026-03-10T15:29:38
scope: VDVPODBXF/VDVPUCtqS
index: 4
---

# Authorize Acceptance And Templates Through Topology

## Summary

Move manual acceptance authorization and role-context template selection onto the resolved topology so configured roles and exact overrides drive behavior.

## Acceptance Criteria

- [ ] [SRS-06/AC-01] Manual acceptance checks use the resolved lane's `manual_accept` capability instead of literal `manager/*` matching. <!-- verify: cargo test -p keel story_accept_topology_, SRS-06:start:end, proof: ac-1.log-->
- [ ] [SRS-07/AC-01] Role context and guidance resolve from configured base roles, with exact `role_overrides` taking precedence when the full taxonomy matches. <!-- verify: cargo test -p keel role_context_topology_, SRS-07:start:end, proof: ac-2.log-->
