---
source_type: Story
source: stories/1vxqNFNdN/REFLECT.md
scope: 1vxqMtskC/1vxqN5jnA
source_story_id: 1vxqNFNdN
created_at: 2026-03-04T12:50:28
---

### 1vyDuwiA5: Deterministic ranking requires total-order tie breaks

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Recommendation scores can tie across techniques when confidence and keyword matches are equivalent. |
| **Insight** | Deterministic ordering is guaranteed only when ranking sorts by score and then by stable id as a total-order tie breaker. |
| **Suggested Action** | Keep recommendation outputs sorted by `(score desc, id asc)` and normalize lists/sets before scoring. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T20:49:05+00:00 |
| **Score** | 0.80 |
| **Confidence** | 0.90 |
| **Applied** | yes |
