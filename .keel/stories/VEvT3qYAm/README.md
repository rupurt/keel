---
id: VEvT3qYAm
title: Bridge Engine And VCS Via Auto Staging
type: feat
status: backlog
created_at: 2026-03-25T23:45:40
updated_at: 2026-03-25T23:45:40
index: 5
operator-signal: pulse
---

<!-- keel:pulse-materialization: VDz8zh8q2@2026-03-26T00:00:00Z -->

# Bridge Engine And VCS Via Auto Staging

## Summary

Materialized from routine `VDz8zh8q2` for eligible window ending `2026-03-26T00:00:00Z`.

## Acceptance Criteria

- [ ] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window.

## Routine Provenance

- Routine: `VDz8zh8q2`
- Target scope: `VE3IAG4jZ`
- Eligible window ends: `2026-03-26T00:00:00Z`

## Blueprint

Design and implement an "Auto-Staging" feature for the Keel CLI.

- **Current Problem:** There is a gap between the Engine calculating state (and writing files to `.keel`) and the VCS recording that state. This leads to untracked files and "dirty tree" status reports.
- **Goal:** Add an optional or default-on mechanism where Keel commands automatically `git add` the artifacts they generate.
- **Exit Criteria:** Keel CLI commands (new, start, submit, record, etc.) keep the Git index synchronized with the `.keel` directory state.
