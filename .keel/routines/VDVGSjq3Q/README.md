---
id: VDVGSjq3Q
title: Eliminate Report Tail Friction
cadence:
  cron: "0 0 * * *"
  timezone: UTC
  deadline: 24h
target-scope: VE4hiOYHj
created_at: 2026-03-14T14:45:00
updated_at: 2026-03-14T14:55:00
---

# Blueprint

Investigate and implement a graph-based artifact computation system that generates Voyage and Compliance reports automatically during state transitions.

- **Current Problem:** Reports are often generated as a "tail" after implementation, leading to dirty Git trees.
- **Insight:** The agent harness often has to manually patch frontmatter because CLI transitions are sometimes incomplete or the metadata is mixed with authored content.
- **Goal:** 
  1. Move report generation into core state transition logic.
  2. Group frontmatter into "auto-generated" and "authored" sections to make agentic patching safer.
- **Exit Criteria:** Keel update that eliminates the need for manual `keel generate` and simplifies frontmatter management.
