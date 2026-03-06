---
id: 1vyGZet0Z
title: Keep Fresh Epic Scaffolds Doctor Clean
type: feat
status: backlog
created_at: 2026-03-05T13:49:38
updated_at: 2026-03-05T14:10:01
scope: 1vyFgR2MA/1vyFlAgHB
---

# Keep Fresh Epic Scaffolds Doctor Clean

## Summary

Keep newly scaffolded epic artifacts structurally clean after the problem-only CLI cutover so doctor and template hygiene checks remain a reliable planning gate.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] Freshly scaffolded epic artifacts remain doctor-clean and structurally coherent after the problem-only hydration behavior lands. <!-- verify: cargo test -p keel doctor_accepts_problem_only_epic_scaffold, SRS-05:start -->
- [ ] [SRS-05/AC-02] Embedded epic templates remain placeholder-clean and free of obsolete CLI-owned goal-token behavior after the cutover. <!-- verify: cargo test -p keel epic_templates_drop_legacy_goal_token_usage, SRS-05:continues -->
- [ ] [SRS-05/AC-03] [SRS-NFR-03/AC-01] Generated problem seed content and revised goal scaffolds stay concise, human-editable, and free of unresolved placeholders. <!-- verify: cargo test -p keel epic_problem_scaffold_is_placeholder_clean, SRS-NFR-03:start:end, SRS-05:end -->
