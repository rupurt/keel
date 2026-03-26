---
# system-managed
id: VEzDa0PEr
status: done
created_at: 2026-03-26T08:09:15
updated_at: 2026-03-26T08:19:12
# authored
title: Extract Txt Scene Primitives Into A Reusable Workspace Crate
type: feat
operator-signal:
scope: VEzA7KvXB/VEzDZy6Eq
index: 1
started_at: 2026-03-26T08:16:11
completed_at: 2026-03-26T08:19:12
---

# Extract Txt Scene Primitives Into A Reusable Workspace Crate

## Summary

Move the reusable width-aware scene primitives out of `keel-cli` and into a standalone `txt-scene` workspace crate so the flow pilot can consume one canonical rendering seam.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add `crates/txt-scene` to the workspace and expose `visible_width`, `SceneLine`, `SceneFrame`, and `ScenePalette` without depending on `keel-core` or `keel-cli`. <!-- verify: cargo test -p txt-scene, SRS-01:start, proof: ac-1.log -->
- [x] [SRS-01/AC-02] `keel-cli` imports the extracted primitives from `txt-scene` and no longer defines active duplicate `SceneLine`, `SceneFrame`, or `ScenePalette` implementations locally. <!-- verify: cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_has_stable_width_without_color, SRS-01:continues, proof: ac-2.log -->
- [x] [SRS-NFR-02/AC-01] The extracted crate proves ANSI-stripped visible-width padding and color-disabling behavior through direct unit tests. <!-- verify: cargo test -p txt-scene, SRS-NFR-02:start:end, SRS-01:end, proof: ac-3.log -->
