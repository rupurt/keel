---
id: VDcFgruMk
title: Routine CLI Surfaces
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:44
operator-signal: 
scope: VDakm8eVW/VDcFd11nc
index: 3
---

# Routine CLI Surfaces

## Summary

Add the minimal CLI authoring and read surfaces that let operators create and
inspect routines without hand-editing board directories.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `keel routine new` scaffolds a valid routine bundle with required cadence and target-scope fields. <!-- verify: test, SRS-03:start -->
- [ ] [SRS-03/AC-02] `keel routine list` renders discoverable routine summaries without manual path knowledge. <!-- verify: test, SRS-03:continues -->
- [ ] [SRS-03/AC-03] `keel routine show <id>` renders cadence, target scope, and blueprint content from canonical storage. <!-- verify: test, SRS-03:end -->
- [ ] [SRS-04/AC-01] The routine scaffold keeps cadence settings, target scope, and blueprint narrative together in one human-editable artifact. <!-- verify: test, SRS-04:start:end -->
