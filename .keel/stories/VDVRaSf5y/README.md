---
id: VDVRaSf5y
title: Validate Topology Selectors And Overlap
type: feat
status: done
created_at: 2026-03-10T15:27:51
updated_at: 2026-03-10T20:38:02
scope: VDVPODBXF/VDVPUCtqS
index: 3
started_at: 2026-03-10T20:37:00
completed_at: 2026-03-10T20:38:02
---

# Validate Topology Selectors And Overlap

## Summary

Add hard-fail validation for topology integrity so bad defaults, bad references, bad selectors, or cross-lane overlap are caught before routing and rendering drift.

## Acceptance Criteria

- [x] [SRS-09/AC-01] `keel doctor` fails on missing defaults, bad role-to-lane references, unknown selectors, and cross-lane overlap. <!-- verify: cargo test -p keel doctor_topology_, SRS-09:start:end, proof: ac-1.log-->
- [x] [SRS-09/AC-02] [SRS-NFR-03/AC-01] Selector compilation surfaces precise hard failures and never silently drops invalid or unknown patterns. <!-- verify: cargo test -p keel workflow_topology_selector_errors_, SRS-09:continues:end, SRS-NFR-03:start:end, proof: ac-2.log-->
