# Replace File-Based Heartbeat With Derived Git Activity Model - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-27T16:48:48-07:00

Created the mission as a two-pass migration. Pass 1 will add a derived `keel heartbeat` surface and switch flow to it with a temporary compatibility fallback; pass 2 will remove the file-based heartbeat path and update hooks, diagnostics, and documentation around the Git/worktree-derived model.
