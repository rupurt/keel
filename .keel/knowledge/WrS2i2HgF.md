---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzVpw5/KNOWLEDGE.md
scope: null
source_story_id: null
---

### WrS2i2HgF: Assert check identity and severity for hard-cutover gates

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Updating terminal artifact coherence enforcement for doctor and story transitions |
| **Insight** | Message-only assertions can pass even if a hard error silently downgrades to a warning; check-id plus severity assertions prevent this regression class |
| **Suggested Action** | For each enforcement rule, add at least one integration test that asserts both `check_id` and `severity` |
| **Applies To** | `src/cli/commands/diagnostics/doctor/mod.rs`, `src/domain/state_machine/gating.rs` |
| **Linked Knowledge IDs** | 1vyDuwFj5 |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |
