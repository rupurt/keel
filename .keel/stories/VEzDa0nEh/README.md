---
# system-managed
id: VEzDa0nEh
status: done
created_at: 2026-03-26T08:09:15
updated_at: 2026-03-26T08:21:00
# authored
title: Document Txt Scene Adoption Boundaries And Next Migrations
type: feat
operator-signal:
scope: VEzA7KvXB/VEzDZy6Eq
index: 3
started_at: 2026-03-26T08:20:34
submitted_at: 2026-03-26T08:20:53
completed_at: 2026-03-26T08:21:00
---

# Document Txt Scene Adoption Boundaries And Next Migrations

## Summary

Capture the pilot boundary for `txt-scene`, document what remains command-local after the `flow` migration, and record the next scene migrations in explicit execution order.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Voyage artifacts state that `txt-scene` owns the reusable width-aware scene primitives while `flow` remains the pilot command-local semantic surface. <!-- verify: manual, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] The next migration order is recorded explicitly as `doctor`, `workshop`, `watch`, then `health` so later slices do not re-open prioritization drift. <!-- verify: manual, SRS-03:end, proof: ac-2.log -->
