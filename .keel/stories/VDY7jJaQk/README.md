---
id: VDY7jJaQk
title: Audit And Stabilize Public Visibility Of Domain Models
type: refactor
status: done
created_at: 2026-03-10T23:35:00
scope: VDXBUEBAG/VDY7YBSFR
index: 3
started_at: 2026-03-11T03:15:27
updated_at: 2026-03-11T03:17:32
submitted_at: 2026-03-11T03:17:24
completed_at: 2026-03-11T03:17:32
---

# Audit And Stabilize Public Visibility Of Domain Models

## Summary

Audit all core domain models (Story, Voyage, Epic, etc.) to ensure they have the necessary pub visibility for library usage without leaking implementation details.

## Acceptance Criteria

- [x] [SRS-NFR-01/AC-01] All core entity types and their required fields are publicly accessible. <!-- verify: just build, SRS-NFR-01:start:end -->
- [x] [SRS-NFR-01/AC-02] No CLI-specific types (e.g. from clap) are required to use the public domain models. <!-- verify: manual, SRS-NFR-01:continues -->
