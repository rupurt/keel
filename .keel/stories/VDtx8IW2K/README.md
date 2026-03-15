---
id: VDtx8IW2K
title: Bridge Engine And VCS Via Auto Staging
type: feat
status: done
scope: VDseuzIFg
milestone: null
created_at: 2026-03-15T03:03:41
updated_at: 2026-03-15T13:40:19
started_at: 2026-03-14T23:24:18
completed_at: 2026-03-15T13:40:19
submitted_at: 2026-03-15T07:15:33
index: 2
governed-by: []
blocked_by: []
role: null
operator-signal: pulse
---

<!-- keel:pulse-materialization: bridge-engine-and-vcs-via-auto-staging@2026-03-16T00:00:00Z -->

# Bridge Engine And VCS Via Auto Staging

## Summary

Materialized from routine `bridge-engine-and-vcs-via-auto-staging` for eligible window ending `2026-03-16T00:00:00Z`.

## Acceptance Criteria

- [x] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window. <!-- verify: manual, SRS-ROUTINE:start, SRS-ROUTINE:end -->

## Routine Provenance

- Routine: `bridge-engine-and-vcs-via-auto-staging`
- Target scope: `VDseuzIFg`
- Eligible window ends: `2026-03-16T00:00:00Z`

## Blueprint

Design and implement an "Auto-Staging" feature for the Keel CLI.

- **Current Problem:** There is a gap between the Engine calculating state (and writing files to `.keel`) and the VCS recording that state. This leads to untracked files and "dirty tree" status reports.
- **Goal:** Add an optional or default-on mechanism where Keel commands automatically `git add` the artifacts they generate.
- **Exit Criteria:** Keel CLI commands (new, start, submit, record, etc.) keep the Git index synchronized with the `.keel` directory state.
