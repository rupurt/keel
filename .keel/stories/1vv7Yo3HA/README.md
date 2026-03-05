---
id: 1vv7Yo3HA
title: Enhance ADR Blocking Messages in Next Decision
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:42:14
scope: 1vv7YWzw2/1vv7YcwBg
index: 1
submitted_at: 2026-02-24T21:42:13
completed_at: 2026-02-24T21:42:14
started_at: 2026-02-24T21:38:57
---

# Enhance ADR Blocking Messages in Next Decision

## Summary

Improve the feedback provided to agents when they are blocked by a proposed ADR.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Update `src/next/format.rs` to include the specific ADR ID and title in the `Blocked` decision output. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure agents understand exactly which architectural decision is pending. <!-- verify: manual, SRS-01:end, proof: ac-2.log-->
