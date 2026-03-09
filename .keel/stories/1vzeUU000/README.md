---
id: 1vzeUU000
title: Mission Log And Digest
type: feat
status: icebox
created_at: 2026-03-09T10:34:02
updated_at: 2026-03-09T10:34:02
scope: 1vzeJF000/1vzeMq000
index: 6
---

# Mission Log And Digest

## Summary

Implement LOG.md append and digest commands.

## Acceptance Criteria

- [ ] [SRS-11/AC-01] `keel mission log <id> --entry "<text>"` appends timestamped entry to LOG.md <!-- verify: test --> <!-- SRS-11:start:end -->
- [ ] [SRS-12/AC-01] `keel mission digest <id>` compresses older entries into summary block at top of LOG.md <!-- verify: test --> <!-- SRS-12:start:end -->
