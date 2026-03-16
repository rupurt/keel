---
id: VE1vQc4Lh
title: Validate Bearing Dependencies in Doctor
type: feat
status: done
created_at: 2026-03-16T04:47:06
updated_at: 2026-03-16T05:03:04
operator-signal:
scope: VDiHwLLfY/VE1vAyNzt
index: 2
blocked_by:
  - VE1vOqhch
started_at: 2026-03-16T04:58:28
completed_at: 2026-03-16T05:03:04
---

# Validate Bearing Dependencies in Doctor

## Summary

Add a `check_bearing_dependencies` diagnostic that validates all `depends_on` references exist, detects cycles via DFS, and flags self-references. Register the check under the Sensory subsystem in `keel doctor`.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Doctor flags an error when `depends_on` contains a bearing ID that does not exist on the board. <!-- verify: cargo test -p keel-core doctor_flags_dangling_depends_on, SRS-02:start:end -->
- [x] [SRS-03/AC-01] Doctor flags an error when the dependency graph contains a cycle. <!-- verify: cargo test -p keel-core doctor_flags_cyclic_depends_on, SRS-03:start:end -->
- [x] [SRS-03/AC-02] Doctor flags an error when a bearing references itself in `depends_on`. <!-- verify: cargo test -p keel-core doctor_flags_self_reference_in_depends_on, SRS-03:start:end -->
- [x] [SRS-NFR-01/AC-01] Dependency validation scales linearly with bearing count. <!-- verify: cargo test -p keel-core dependency_validation_scales_linearly, SRS-NFR-01:start:end -->
