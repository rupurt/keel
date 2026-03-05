---
source_type: Voyage
source: epics/1vxyM0hvn/voyages/1vxyMT6nz/KNOWLEDGE.md
scope: null
source_story_id: null
---

### EqSN1h8Jj: Deterministic Projection Requires Ordered Containers End-To-End

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering `next --parallel` output in both human and JSON projections |
| **Insight** | Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs. |
| **Suggested Action** | Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths. |
| **Applies To** | `src/cli/commands/management/next.rs` and other CLI JSON projection builders |
| **Linked Knowledge IDs** | 1vyDuwMlz |
| **Observed At** |  |
| **Score** | 0.88 |
| **Confidence** | 0.94 |
| **Applied** | yes |
