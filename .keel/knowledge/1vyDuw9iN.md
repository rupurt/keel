---
source_type: Story
source: stories/1vxyMr3U2/REFLECT.md
scope: 1vxyM0hvn/1vxyMT6nz
source_story_id: 1vxyMr3U2
---

### 1vyDuw9iN: Work Item Comparator Is Not Lexical

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building deterministic pairwise vectors for story IDs with numeric suffixes |
| **Insight** | `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`) |
| **Suggested Action** | Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests |
| **Applies To** | `src/cli/commands/management/next_support/*` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T02:40:28+00:00 |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** | yes |
