---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxqN5jnA/KNOWLEDGE.md
created_at: 2026-03-04T13:06:23
---

### Cn18kLEPl: Deterministic ranking requires total-order tie breaks

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Recommendation scores can tie across techniques when confidence and keyword matches are equivalent. |
| **Insight** | Deterministic ordering is guaranteed only when ranking sorts by score and then by stable id as a total-order tie breaker. |
| **Suggested Action** | Keep recommendation outputs sorted by `(score desc, id asc)` and normalize lists/sets before scoring. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Linked Knowledge IDs** | 1vyDuwiA5 |
| **Observed At** |  |
| **Score** | 0.80 |
| **Confidence** | 0.90 |
| **Applied** | yes |
