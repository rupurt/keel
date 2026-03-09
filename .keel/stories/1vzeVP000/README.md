---
id: 1vzeVP000
title: Mission New Command
type: feat
status: icebox
created_at: 2026-03-09T10:34:59
updated_at: 2026-03-09T10:34:59
scope: 1vzeJF000/1vzeMq000
index: 7
---

# Mission New Command

## Summary

Implement `keel mission new` command that creates .keel/missions/<id>/ directory with README.md, CHARTER.md scaffold, and LOG.md scaffold.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `keel mission new "<title>"` creates mission directory under .keel/missions/ <!-- verify: test --> <!-- SRS-01:start:end -->
- [ ] [SRS-01/AC-02] Created README.md has frontmatter with id, title, status=defining, created_at <!-- verify: test --> <!-- SRS-01:start:end -->
- [ ] [SRS-01/AC-03] Created CHARTER.md has Goals table, Constraints, and Halting Rules scaffold sections <!-- verify: test --> <!-- SRS-01:start:end -->
- [ ] [SRS-01/AC-04] Created LOG.md has initial scaffold with header <!-- verify: test --> <!-- SRS-01:start:end -->
