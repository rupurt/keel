---
id: VE56ttVZY
title: Bridge Engine And VCS Via Auto Staging
type: feat
status: done
scope: VE4hiOYHj/VE5NSQ2T2
created_at: 2026-03-17T00:51:29
updated_at: 2026-03-16T18:58:08
index: 3
operator-signal: pulse
started_at: 2026-03-16T17:52:27
completed_at: 2026-03-16T17:56:08
---

<!-- keel:pulse-materialization: VDz8zh8q2@2026-03-18T00:00:00Z -->

# Bridge Engine And VCS Via Auto Staging

## Summary

Materialized from routine `VDz8zh8q2` for eligible window ending `2026-03-18T00:00:00Z`.

## Acceptance Criteria

- [x] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window. <!-- verify: cargo build, SRS-ROUTINE:start:end, proof: ac-1.log-->

## Routine Provenance

- Routine: `VDz8zh8q2`
- Target scope: `VE4hiOYHj`
- Eligible window ends: `2026-03-18T00:00:00Z`

## Blueprint

Design and implement an "Auto-Staging" feature for the Keel CLI.

- **Current Problem:** There is a gap between the Engine calculating state (and writing files to `.keel`) and the VCS recording that state. This leads to untracked files and "dirty tree" status reports.
- **Goal:** Add an optional or default-on mechanism where Keel commands automatically `git add` the artifacts they generate.
- **Exit Criteria:** Keel CLI commands (new, start, submit, record, etc.) keep the Git index synchronized with the `.keel` directory state.
