# Replace File-Based Heartbeat With Derived Git Activity Model - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-27T17:12:58-07:00

Decomposed the mission into epic `VF7Geb3Wa` with two pass-aligned voyages: `VF7Gfk7zv` for the derived heartbeat surface plus flow fallback, and `VF7Gfkizo` for removing the file-backed pacemaker path and updating hooks, diagnostics, and docs. Authored six scoped stories so both passes now have explicit execution slices.

## 2026-03-27T16:48:48-07:00

Created the mission as a two-pass migration. Pass 1 will add a derived `keel heartbeat` surface and switch flow to it with a temporary compatibility fallback; pass 2 will remove the file-based heartbeat path and update hooks, diagnostics, and documentation around the Git/worktree-derived model.

## 2026-03-27T18:21:12

Mission achieved by local system user 'alex'
