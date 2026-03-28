---
# system-managed
id: VF8hnUos4
status: done
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:32:20
# authored
title: Centralize Scene Contracts And Scene Metadata
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkUKk1
index: 1
started_at: 2026-03-27T23:24:25
submitted_at: 2026-03-27T23:32:16
completed_at: 2026-03-27T23:32:20
---

# Centralize Scene Contracts And Scene Metadata

## Summary

Create central scene contracts that describe the `--scene` surfaces and their canonical dependencies so scene semantics can be tested and documented from one place.

## Acceptance Criteria

- [x] [SRS-03/AC-01] A central scene-contract registry describes each public `--scene` surface and its canonical dependency signals. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Heartbeat-driven and routing-aware scenes are represented through the central scene contracts rather than ad hoc lists. <!-- verify: manual, SRS-04:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Introducing the scene contracts does not change existing scene meaning or visual behavior. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->
