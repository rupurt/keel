---
id: VDcFgtbNC
title: Scheduled Flow Lane Projection
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:45
operator-signal: 
scope: VDakmG8cH/VDcFd5Sop
index: 2
---

# Scheduled Flow Lane Projection

## Summary

Extend `keel flow` so scheduled automation demand is visible before or after a
pulse run.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] `keel flow` surfaces a scheduled lane or scheduled-capacity view driven by routine schedule state. <!-- verify: test, SRS-04:start -->
- [ ] [SRS-04/AC-02] Scheduled output distinguishes due-now automation from upcoming work with explicit operator guidance. <!-- verify: test, SRS-04:end -->
- [ ] [SRS-NFR-02/AC-02] Scheduled automation output remains stable and reviewable across flow render paths. <!-- verify: test, SRS-NFR-02:end -->
