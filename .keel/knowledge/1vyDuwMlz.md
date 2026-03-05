---
source_type: Story
source: stories/1vxyMtVpK/REFLECT.md
scope: 1vxyM0hvn/1vxyMT6nz
source_story_id: 1vxyMtVpK
---

### 1vyDuwMlz: Deterministic Projection Requires Ordered Containers End-To-End

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering `next --parallel` output in both human and JSON projections |
| **Insight** | Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs. |
| **Suggested Action** | Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths. |
| **Applies To** | `src/cli/commands/management/next.rs` and other CLI JSON projection builders |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T03:01:50+00:00 |
| **Score** | 0.88 |
| **Confidence** | 0.94 |
| **Applied** | yes |
