---
id: VDY6ryx89
title: Consolidate Domain And Application Ports
type: refactor
status: backlog
created_at: 2026-03-10T23:25:00
scope: VDXBU7W4O/VDY6bQawh
index: 1
updated_at: 2026-03-11T02:25:04
---

# Consolidate Domain And Application Ports

## Summary

Consolidate the overlapping trait definitions in `src/application/ports.rs` and `src/domain/port/mod.rs`. Move all repository and storage abstractions to the domain layer.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Repository traits consolidated in `src/domain/port/mod.rs`. <!-- verify: inspection, SRS-01:start -->
- [ ] [SRS-02/AC-01] `EntityStore<T>` made the canonical CRUD interface. <!-- verify: inspection, SRS-02:start -->
- [ ] [SRS-04/AC-01] `src/application/ports.rs` removed. <!-- verify: inspection, SRS-04:start:end -->
