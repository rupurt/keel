---
id: VDzfi1DvX
title: Implement System Notifications for Charged Capacitors
type: feat
status: done
created_at: 2026-03-15T19:32:07
updated_at: 2026-03-15T19:34:51
operator-signal: 
scope: VDseuzIFg
index: 13
started_at: 2026-03-15T19:32:22
submitted_at: 2026-03-15T19:34:50
completed_at: 2026-03-15T19:34:51
---

# Implement System Notifications for Charged Capacitors

## Summary

Describe the goal and context of this story.

## Acceptance Criteria

- [x] [SRS-NOTIFICATION/AC-01] `keel::infrastructure::config::WorkflowConfig` includes a `notification_command` option. <!-- verify: manual, SRS-NOTIFICATION:start, SRS-NOTIFICATION:end -->
- [x] [SRS-NOTIFICATION/AC-02] `keel config show` renders the `notification_command` if configured. <!-- verify: manual, SRS-NOTIFICATION:start, SRS-NOTIFICATION:end -->
- [x] [SRS-NOTIFICATION/AC-03] `keel pulse` triggers the `notification_command` if the system requires human input (charged capacitors). <!-- verify: manual, SRS-NOTIFICATION:start, SRS-NOTIFICATION:end -->
- [x] [SRS-NOTIFICATION/AC-04] Default `notification_command` provides a sensible `tmux display-message` if running inside tmux. <!-- verify: manual, SRS-NOTIFICATION:start, SRS-NOTIFICATION:end -->
