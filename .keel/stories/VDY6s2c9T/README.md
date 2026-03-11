---
id: VDY6s2c9T
title: Refactor StoryLifecycleService For Dependency Injection
type: refactor
status: backlog
created_at: 2026-03-10T23:25:00
scope: VDXBU7W4O/VDY6bQawh
index: 2
updated_at: 2026-03-11T02:25:04
---

# Refactor StoryLifecycleService For Dependency Injection

## Summary

Refactor `StoryLifecycleService` to use instance-based methods and injected storage ports.

## Acceptance Criteria

- [ ] [SRS-01/AC-02] `StoryLifecycleService` accepts `Arc<dyn BoardStore>` and `Arc<dyn EntityStore<Story>>`. <!-- verify: cargo test -p keel story_lifecycle_di, SRS-01:end -->
- [ ] [SRS-04/AC-02] Service methods no longer take `board_dir: &Path`. <!-- verify: inspection, SRS-04:continues:end -->
- [ ] [SRS-NFR-01/AC-01] All existing story lifecycle tests pass with mock stores. <!-- verify: cargo test -p keel story_lifecycle, SRS-NFR-01:start -->
