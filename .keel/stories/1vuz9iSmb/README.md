---
id: 1vuz9iSmb
title: Purge Legacy Terminology from CLI and Documentation
type: feat
status: done
created_at: 2026-02-24T12:37:18
updated_at: 2026-02-24T16:43:01
scope: 1vuz8K4NM/1vuz8jNo3
index: 5
submitted_at: 2026-02-24T00:00:00
completed_at: 2026-02-24T00:00:00
---

# Purge Legacy Terminology from CLI and Documentation

## Summary

Purge legacy state and workflow terminology from CLI help, docs, and tests so canonical language is the only supported vocabulary.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Replace legacy terminology in CLI help/output strings with canonical state names and workflow terms only. <!-- verify: manual, SRS-05:start -->
- [x] [SRS-05/AC-02] Update architecture and user-facing docs to remove legacy wording and align examples with canonical terminology. <!-- verify: manual, SRS-05:continues -->
- [x] [SRS-05/AC-03] Add or update tests/checks that fail when deprecated terminology reappears in CLI snapshots or documentation fixtures. <!-- verify: manual, SRS-05:end -->
