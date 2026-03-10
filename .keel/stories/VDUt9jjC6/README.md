---
id: VDUt9jjC6
title: Inject Role Context Into Next Guidance
type: feat
status: done
created_at: 2026-03-10T13:11:05
updated_at: 2026-03-10T14:56:26
scope: VDTpFlMKc/VDUsc8KXy
index: 2
started_at: 2026-03-10T14:49:10
completed_at: 2026-03-10T14:56:26
---

# Inject Role Context Into Next Guidance

## Summary

Attach the selected role template to `keel next` output so harnesses receive context with the work pull.

## Acceptance Criteria

- [x] [SRS-02/AC-01] `keel next --role <TAXONOMY> --json` includes the resolved role context in actionable guidance output <!-- verify: cargo test --lib decision_to_json_with_role_context_includes_resolved_template_payload, SRS-02:start, proof: ac-1.log -->
- [x] [SRS-02/AC-02] Human-readable `keel next --role <TAXONOMY>` output surfaces the selected role template and queue-lane expectations <!-- verify: cargo test --lib render_human_guidance_surfaces_role_context_and_queue_lane, SRS-02:end, proof: ac-2.log -->
- [x] [SRS-03/AC-01] Unsupported role bases fail with deterministic errors listing the supported template families <!-- verify: cargo test --lib unsupported_role_families_fail_with_supported_template_list, SRS-03:start:end, proof: ac-3.log -->
