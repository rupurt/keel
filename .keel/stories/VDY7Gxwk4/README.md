---
id: VDY7Gxwk4
title: Support Environment Variable Overrides For Storage
type: feat
status: backlog
created_at: 2026-03-10T23:30:00
scope: VDXBUHZB0/VDY7AlCLy
index: 3
updated_at: 2026-03-11T02:26:38
---

# Support Environment Variable Overrides For Storage

## Summary

Enable users to override the storage backend using the `KEEL_STORAGE_BACKEND` environment variable.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `KEEL_STORAGE_BACKEND` overrides the value in `keel.toml`. <!-- verify: cargo test -p keel config_storage_env_override, SRS-03:start:end -->
