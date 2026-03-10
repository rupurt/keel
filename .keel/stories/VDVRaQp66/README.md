---
id: VDVRaQp66
title: Add Workflow Topology Config Model
type: feat
status: backlog
created_at: 2026-03-10T15:27:51
updated_at: 2026-03-10T15:29:38
scope: VDVPODBXF/VDVPUCtqS
index: 1
---

# Add Workflow Topology Config Model

## Summary

Add the config schema and effective-topology resolver that seed default roles and lanes, carry lane behavior, and compile selector inputs into one canonical source catalog.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] [SRS-NFR-01/AC-01] Add workflow-topology config structs and seeded default resolution so boards with no topology overrides still resolve `manager`/`operator` roles and `management`/`delivery` lanes. <!-- verify: cargo test -p keel workflow_topology_, SRS-01:start:end, SRS-NFR-01:start:end, proof: ac-1.log-->
- [ ] [SRS-02/AC-01] `keel config show` renders the effective seeded defaults, configured role families, lane definitions, and exact overrides rather than only raw authored fragments. <!-- verify: cargo test -p keel config_show_workflow_topology_, SRS-02:start:end, proof: ac-2.log-->
- [ ] [SRS-03/AC-01] Lane config captures `description`, ordered `include`/`exclude`, `parallel`, `manual_accept`, and `priority` fields and compiles selector inputs against the canonical source catalog. <!-- verify: cargo test -p keel workflow_topology_lane_config_, SRS-03:start:end, proof: ac-3.log-->
