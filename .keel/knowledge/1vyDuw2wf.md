---
source_type: Story
source: stories/1vxyMtaKP/REFLECT.md
scope: 1vxyM0hvn/1vxyMT6nz
source_story_id: 1vxyMtaKP
---

### 1vyDuw2wf: Coherence Checks Need Canonical Pair Normalization

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Detecting reciprocal `blocked_by` contradictions in doctor checks |
| **Insight** | Pair-level diagnostics become deterministic and deduplicated only when pair IDs are normalized (`min/max`) before reporting. |
| **Suggested Action** | Always canonicalize relationship IDs before emitting pair-based doctor findings. |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/stories.rs` and similar relationship validators |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T03:05:21+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | yes |
