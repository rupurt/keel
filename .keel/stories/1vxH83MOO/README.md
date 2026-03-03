---
id: 1vxH83MOO
title: Align CLI Contracts For Creation Commands
type: feat
status: backlog
created_at: 2026-03-02T20:13:03
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzV3oR
index: 2
---

# Align CLI Contracts For Creation Commands

## Summary

Align creation command interfaces with ownership policy so only user-owned inputs are exposed while system-owned fields remain runtime-managed.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Verify `epic new`, `voyage new`, `story new`, `bearing new`, and `adr new` expose only approved user-owned creation inputs.
- [ ] [SRS-02/AC-02] Confirm no creation command introduces CLI flags for system-owned fields (`id`, `index`, `status`, `*_at`).
- [ ] [SRS-02/AC-03] Make `voyage new --goal` required at CLI parse time and keep runtime validation behavior coherent.
