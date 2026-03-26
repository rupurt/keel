---
# system-managed
id: VEzDZzvEg
status: backlog
created_at: 2026-03-26T08:09:15
updated_at: 2026-03-26T08:14:20
# authored
title: Migrate Flow Scene Rendering Onto Txt Scene
type: feat
operator-signal:
scope: VEzA7KvXB/VEzDZy6Eq
index: 2
---

# Migrate Flow Scene Rendering Onto Txt Scene

## Summary

Rewire `keel flow --scene` to consume `txt-scene` for every scene variant while preserving the current electrical metaphor, watch/battery indicators, and width-stable output.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] The `flow --scene` path renders the open-circuit, unhealthy, unplugged, and powered variants through `txt-scene` primitives instead of command-local width-aware scene helpers. <!-- verify: cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_has_stable_width_without_color, SRS-02:start, proof: ac-1.log -->
- [ ] [SRS-02/AC-02] Flow-scene regressions preserve the existing watch-pressure and battery-pack annotations after the migration. <!-- verify: cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_surfaces_watch_pressure_without_color, SRS-02:continues, proof: ac-2.log -->
- [ ] [SRS-NFR-01/AC-01] Color and no-color `flow --scene` renders keep identical visible widths across every emitted line after ANSI stripping. <!-- verify: cargo test -p keel cli::commands::diagnostics::flow::tests::render_power_scene_has_stable_width_with_color, SRS-NFR-01:start:end, SRS-02:end, proof: ac-3.log -->
