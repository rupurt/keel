---
created_at: 2026-03-17T01:00:00
---

# Reflection - Bridge Engine And VCS Via Auto Staging

## Knowledge

## Observations

Implemented as a single `auto_stage_board()` call at the runtime dispatch exit point rather than threading git-add through 31 command call sites. Controlled by `workflow.auto_stage` in keel.toml (default false). When enabled, every successful CLI command runs `git add .keel/` to keep the index synchronized with board state.
