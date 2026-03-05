---
id: 1vuz9X6iw
title: Route Doctor Checks Through Gate Evaluators
type: feat
status: done
created_at: 2026-02-24T12:37:07
updated_at: 2026-02-24T17:25:16
scope: 1vuz8K4NM/1vuz8dYT5
index: 3
submitted_at: 2026-02-24T17:25:15
completed_at: 2026-02-24T17:25:16
started_at: 2026-02-24T15:01:11
---

# Route Doctor Checks Through Gate Evaluators

## Summary

Make doctor transition and completion checks reuse the same gate-evaluation paths and reporting policy semantics as runtime enforcement.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Refactor doctor checks to consume shared gate outputs for transition and completion validation where applicable. <!-- verify: manual, SRS-03:start -->
- [x] [SRS-03/AC-02] Ensure doctor uses reporting policy semantics that surface warnings without runtime-style blocking. <!-- verify: manual, SRS-03:continues -->
- [x] [SRS-03/AC-03] Add tests that prove doctor findings are derived from the same rule set as runtime enforcement. <!-- verify: manual, SRS-03:end -->
