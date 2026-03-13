---
id: daily-status-surface-progress
title: Daily Status Surface Progress
cadence:
  cron: 0 9 * * 6
  timezone: America/Los_Angeles
target-scope: VDm4ld6EX
created_at: 2026-03-13T11:43:53
updated_at: 2026-03-13T11:43:53
---

# Blueprint

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
