---
id: VDUt9jjC6
title: Inject Role Context Into Next Guidance
type: feat
status: backlog
created_at: 2026-03-10T13:11:05
updated_at: 2026-03-10T13:16:40
scope: VDTpFlMKc/VDUsc8KXy
index: 2
---

# Inject Role Context Into Next Guidance

## Summary

Attach the selected role template to `keel next` output so harnesses receive context with the work pull.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `keel next --role <TAXONOMY> --json` includes the resolved role context in actionable guidance output <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-02/AC-02] Human-readable `keel next --role <TAXONOMY>` output surfaces the selected role template and queue-lane expectations <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Unsupported role bases fail with deterministic errors listing the supported template families <!-- verify: test --> <!-- SRS-03:start:end -->
