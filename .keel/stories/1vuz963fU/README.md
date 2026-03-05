---
id: 1vuz963fU
title: Define Queue Policy Module and Threshold Constants
type: feat
status: done
created_at: 2026-02-24T12:36:40
updated_at: 2026-02-24T13:21:46
scope: 1vuz8K4NM/1vuz8VYmc
index: 1
submitted_at: 2026-02-24T13:21:45
completed_at: 2026-02-24T13:21:46
started_at: 2026-02-24T12:59:13
---

# Define Queue Policy Module and Threshold Constants

## Summary

Create the canonical queue policy module that defines threshold constants, queue categories, and derivation helpers used by both `keel next` and `keel flow`.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add a queue policy module that defines canonical thresholds and decision categories for next and flow. <!-- verify: manual, SRS-01:start -->
- [x] [SRS-01/AC-02] Replace duplicated threshold constants in next and flow call sites with policy exports. <!-- verify: manual, SRS-01:continues -->
- [x] [SRS-01/AC-03] Add unit tests that lock policy defaults and helper behavior. <!-- verify: manual, SRS-01:end -->
