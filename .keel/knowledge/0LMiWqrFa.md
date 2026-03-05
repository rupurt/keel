---
source_type: Voyage
source: epics/1vxyM0hvn/voyages/1vxyMT6nz/KNOWLEDGE.md
created_at: 2026-03-04T19:11:59
---

### 0LMiWqrFa: Work Item Comparator Is Not Lexical

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building deterministic pairwise vectors for story IDs with numeric suffixes |
| **Insight** | `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`) |
| **Suggested Action** | Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests |
| **Applies To** | `src/cli/commands/management/next_support/*` |
| **Linked Knowledge IDs** | 1vyDuw9iN |
| **Observed At** |  |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** | yes |
