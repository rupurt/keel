---
source_type: Voyage
source: epics/1vxyM0hvn/voyages/1vxyMT6nz/KNOWLEDGE.md
scope: null
source_story_id: null
---

### vjKuUwTsz: Unknown Context Should Force Risk Floor

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Pairwise scoring for partial architectural metadata in `next --parallel` |
| **Insight** | Unresolved semantic context is easiest to keep safe when scoring applies an explicit risk floor and confidence ceiling instead of only additive penalties |
| **Suggested Action** | Keep conservative fallback thresholds as first-class scoring invariants and assert them directly in tests |
| **Applies To** | `src/cli/commands/management/next_support/parallel_*.rs` |
| **Linked Knowledge IDs** | 1vyDuwXCw |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | yes |
