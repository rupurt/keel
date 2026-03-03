---
id: 1vxH83MOO
title: Align CLI Contracts For Creation Commands
type: feat
status: needs-human-verification
created_at: 2026-03-02T20:13:03
updated_at: 2026-03-03T08:10:40
scope: 1vxGy5tco/1vxGzV3oR
index: 2
started_at: 2026-03-02T21:56:52
submitted_at: 2026-03-03T08:10:40
---

# Align CLI Contracts For Creation Commands

## Summary

Align creation command interfaces with ownership policy so only user-owned inputs are exposed while system-owned fields remain runtime-managed.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Verify `epic new`, `voyage new`, `story new`, `bearing new`, and `adr new` expose only approved user-owned creation inputs. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Confirm no creation command introduces CLI flags for system-owned fields (`id`, `index`, `status`, `*_at`). <!-- verify: manual, SRS-02:continues, proof: ac-2.log-->
- [x] [SRS-02/AC-03] Make `voyage new --goal` required at CLI parse time and keep runtime validation behavior coherent. <!-- verify: manual, SRS-02:end, proof: ac-3.log-->
