---
id: VDz8zh8q2
title: Bridge Engine And VCS Via Auto Staging
cadence:
  cron: "0 0 * * *"
  timezone: UTC
  deadline: 48h
target-scope: VE4hiOYHj
created_at: 2026-03-14T14:45:00
updated_at: 2026-03-14T14:45:00
---

# Blueprint

Design and implement an "Auto-Staging" feature for the Keel CLI.

- **Current Problem:** There is a gap between the Engine calculating state (and writing files to `.keel`) and the VCS recording that state. This leads to untracked files and "dirty tree" status reports.
- **Goal:** Add an optional or default-on mechanism where Keel commands automatically `git add` the artifacts they generate.
- **Exit Criteria:** Keel CLI commands (new, start, submit, record, etc.) keep the Git index synchronized with the `.keel` directory state.
