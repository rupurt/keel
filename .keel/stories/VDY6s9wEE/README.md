---
id: VDY6s9wEE
title: Update CLI Wiring To Inject Storage Adapters
type: refactor
status: backlog
created_at: 2026-03-10T23:25:00
scope: VDXBU7W4O/VDY6bQawh
index: 4
updated_at: 2026-03-11T02:25:04
---

# Update CLI Wiring To Inject Storage Adapters

## Summary

Update the CLI command handlers to initialize the `FileSystemAdapter` and inject it into the refactored application services.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Command handlers for `story`, `voyage`, and `epic` use dependency injection. <!-- verify: cargo test -p keel cli_regression, SRS-03:start:end -->
- [ ] [SRS-NFR-01/AC-03] CLI behavior is identical to the pre-refactor state. <!-- verify: cargo test -p keel cli_regression, SRS-NFR-01:end -->
