---
source_type: Story
source: stories/1vxH84jzB/REFLECT.md
scope: 1vxGy5tco/1vxGzVpw5
source_story_id: 1vxH84jzB
created_at: 2026-03-03T11:50:46
---

### 1vyDuwFj5: Assert check identity and severity for hard-cutover gates

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Updating terminal artifact coherence enforcement for doctor and story transitions |
| **Insight** | Message-only assertions can pass even if a hard error silently downgrades to a warning; check-id plus severity assertions prevent this regression class |
| **Suggested Action** | For each enforcement rule, add at least one integration test that asserts both `check_id` and `severity` |
| **Applies To** | `src/cli/commands/diagnostics/doctor/mod.rs`, `src/domain/state_machine/gating.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T19:30:00+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |
