---
id: VEvT3qXAl
title: Daily Status Surface Progress
type: feat
status: backlog
created_at: 2026-03-25T23:45:40
updated_at: 2026-03-25T23:45:40
index: 4
operator-signal: pulse
---

<!-- keel:pulse-materialization: VDfVxvWgf@2026-03-28T16:00:00Z -->

# Daily Status Surface Progress

## Summary

Materialized from routine `VDfVxvWgf` for eligible window ending `2026-03-28T16:00:00Z`.

## Acceptance Criteria

- [ ] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window.

## Routine Provenance

- Routine: `VDfVxvWgf`
- Target scope: `VE3IAG4jZ`
- Eligible window ends: `2026-03-28T16:00:00Z`

## Blueprint

- Review progress on compact status surfaces and richer mission drilldown.
- Tighten the output contract for `keel mission next --status`:
  - exactly three short bullets
  - each bullet names one actionable next step
  - no long-form explanation in status mode
- Define or refine the deeper `keel mission next` exploration contract:
  - status summary first
  - one or two levels deeper evidence below it
  - explicit "what should I do next?" guidance
- If the epic still lacks concrete planning, author or update `PRD.md` and create the next voyage.
- Record the result as one of:
  - compact status contract clarified
  - deeper mission-next contract clarified
  - next voyage created for implementation
  - blocker captured with evidence
