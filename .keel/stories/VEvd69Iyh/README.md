---
id: VEvd69Iyh
title: Bridge Engine And VCS Via Auto Staging
type: feat
status: icebox
created_at: 2026-03-26T00:25:32
updated_at: 2026-03-26T06:14:19
index: 3
operator-signal: pulse
---

<!-- keel:pulse-materialization: VDz8zh8q2@2026-03-27T00:00:00Z -->

# Bridge Engine And VCS Via Auto Staging

## Summary

Materialized from routine `VDz8zh8q2` for eligible window ending `2026-03-27T00:00:00Z`.

## Acceptance Criteria

- [ ] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window.

## Routine Provenance

- Routine: `VDz8zh8q2`
- Target scope: `VE3IAG4jZ`
- Eligible window ends: `2026-03-27T00:00:00Z`

## Consolidated Duplicate History

Use this story as the canonical review anchor for this routine topic.

- Iced duplicate: `VEvT3qYAm` from eligible window `2026-03-26T00:00:00Z`
- Merge note: no blueprint delta was introduced between the duplicate materializations

## Pre-Watch Materialization History

This watch-scoped story replaces earlier completions that were materialized under retired project-operations epics before watch scope became canonical.

- Retired predecessors: `VDtx8IW2K` and `VDuknDnI2` from eligible window `2026-03-16T00:00:00Z`, plus `VDzFTQdG2` from eligible window `2026-03-17T00:00:00Z`
- Merge note: blueprint intent is unchanged across the legacy epic-scoped runs and the current watch-scoped run

## Blueprint

Design and implement an "Auto-Staging" feature for the Keel CLI.

- **Current Problem:** There is a gap between the Engine calculating state (and writing files to `.keel`) and the VCS recording that state. This leads to untracked files and "dirty tree" status reports.
- **Goal:** Add an optional or default-on mechanism where Keel commands automatically `git add` the artifacts they generate.
- **Exit Criteria:** Keel CLI commands (new, start, submit, record, etc.) keep the Git index synchronized with the `.keel` directory state.
