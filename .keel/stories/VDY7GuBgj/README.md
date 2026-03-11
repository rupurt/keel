---
id: VDY7GuBgj
title: Implement Storage Backend Validation Logic
type: feat
status: backlog
created_at: 2026-03-10T23:30:00
scope: VDXBUHZB0/VDY7AlCLy
index: 2
updated_at: 2026-03-11T02:26:38
---

# Implement Storage Backend Validation Logic

## Summary

Add validation to ensure that only supported storage backends can be configured.

## Acceptance Criteria

- [ ] [SRS-NFR-01/AC-01] Config loader errors when an unknown backend is specified. <!-- verify: cargo test -p keel config_storage_validation, SRS-NFR-01:start:end -->
