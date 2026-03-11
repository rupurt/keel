---
id: VDY6s2c9T
title: Refactor StoryLifecycleService For Dependency Injection
type: refactor
status: done
created_at: 2026-03-10T23:25:00
scope: VDXBU7W4O/VDY6bQawh
index: 2
updated_at: 2026-03-11T04:30:52
started_at: 2026-03-11T03:58:26
submitted_at: 2026-03-11T04:06:05
completed_at: 2026-03-11T04:30:52
---

# Refactor StoryLifecycleService For Dependency Injection

## Summary

Refactor `StoryLifecycleService` to use instance-based methods and injected storage ports.

## Acceptance Criteria

- [x] [SRS-01/AC-02] `StoryLifecycleService` accepts `Arc<dyn BoardStore>` and `Arc<dyn EntityStore<Story>>`. <!-- verify: cargo test -p keel story_lifecycle_di, SRS-01:end, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Service methods no longer take `board_dir: &Path`. <!-- verify: manual, SRS-04:continues:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] All existing story lifecycle tests pass with mock stores. <!-- verify: cargo test -p keel story_lifecycle, SRS-NFR-01:start, proof: ac-3.log-->
