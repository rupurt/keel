---
id: VDg1N07jJ
title: Add Interactive Knowledge Graph Navigation
type: feat
status: done
created_at: 2026-03-12T10:52:42
updated_at: 2026-03-12T11:18:13
operator-signal: 
scope: VDg0dAPVS/VDg0f0aHW
index: 2
started_at: 2026-03-12T11:08:13
completed_at: 2026-03-12T11:18:13
---

# Add Interactive Knowledge Graph Navigation

## Summary

Add the interactive `keel knowledge graph` TTY experience so operators can zoom, focus, and explore the canonical whole-project graph without leaving the terminal, while still degrading cleanly when an interactive viewport is unavailable.

## Acceptance Criteria

- [x] [SRS-01/AC-01] [SRS-NFR-02/AC-01] [SRS-NFR-03/AC-01] The default `keel knowledge graph` command opens an interactive TTY surface with viewport-safe navigation controls and falls back cleanly when no interactive terminal is available. <!-- verify: cargo test --bin keel knowledge_graph_interactive_mode_uses_tty_controls_and_viewport, SRS-01:start:end, SRS-NFR-02:start:end, SRS-NFR-03:start:end, proof: ac-1.log-->
