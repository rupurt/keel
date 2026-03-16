---
id: VE1vQc4Lh
title: Validate Bearing Dependencies in Doctor
type: feat
status: backlog
created_at: 2026-03-16T04:47:06
updated_at: 2026-03-16T04:48:16
operator-signal:
scope: VDiHwLLfY/VE1vAyNzt
index: 2
blocked_by:
  - VE1vOqhch
---

# Validate Bearing Dependencies in Doctor

## Summary

Add a `check_bearing_dependencies` diagnostic that validates all `depends_on` references exist, detects cycles via DFS, and flags self-references. Register the check under the Sensory subsystem in `keel doctor`.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Doctor flags an error when `depends_on` contains a bearing ID that does not exist on the board. <!-- verify: test, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Doctor flags an error when the dependency graph contains a cycle. <!-- verify: test, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] Doctor flags an error when a bearing references itself in `depends_on`. <!-- verify: test, SRS-03:start:end -->
- [ ] [SRS-NFR-01/AC-01] Dependency validation scales linearly with bearing count. <!-- verify: test, SRS-NFR-01:start:end -->
