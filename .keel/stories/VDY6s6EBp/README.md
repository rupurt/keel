---
id: VDY6s6EBp
title: Refactor VoyageEpicLifecycleService For Dependency Injection
type: refactor
status: done
created_at: 2026-03-10T23:25:00
scope: VDXBU7W4O/VDY6bQawh
index: 3
updated_at: 2026-03-11T04:37:13
started_at: 2026-03-11T04:31:02
completed_at: 2026-03-11T04:37:13
---

# Refactor VoyageEpicLifecycleService For Dependency Injection

## Summary

Refactor `VoyageEpicLifecycleService` to use instance-based methods and injected storage ports.

## Acceptance Criteria

- [x] [SRS-02/AC-02] `VoyageEpicLifecycleService` accepts relevant entity stores. <!-- verify: cargo test -p keel voyage_epic_lifecycle_di, SRS-02:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-02] All existing voyage/epic lifecycle tests pass. <!-- verify: cargo test -p keel voyage_epic_lifecycle, SRS-NFR-01:continues, proof: ac-2.log-->
