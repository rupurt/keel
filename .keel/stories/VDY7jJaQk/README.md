---
id: VDY7jJaQk
title: Audit And Stabilize Public Visibility Of Domain Models
type: refactor
status: backlog
created_at: 2026-03-10T23:35:00
scope: VDXBUEBAG/VDY7YBSFR
index: 3
updated_at: 2026-03-11T02:28:55
---

# Audit And Stabilize Public Visibility Of Domain Models

## Summary

Audit all core domain models (`Story`, `Voyage`, `Epic`, etc.) to ensure they have the necessary `pub` visibility for library usage without leaking implementation details.

## Acceptance Criteria

- [ ] [SRS-NFR-01/AC-01] All core entity types and their required fields are publicly accessible. <!-- verify: compilation, SRS-NFR-01:start:end -->
- [ ] [SRS-NFR-01/AC-02] No CLI-specific types (e.g. from `clap`) are required to use the public domain models. <!-- verify: inspection, SRS-NFR-01:continues -->
