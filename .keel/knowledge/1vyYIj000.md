---
source_type: Story
source: stories/1vyWSD000/REFLECT.md
scope: 1vyWLl000/1vyWNL000
source_story_id: 1vyWSD000
created_at: 2026-03-06T08:45:14
---

### 1vyYIj000: Dogfood Evidence Needs Its Own Board

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When a dogfood or e2e harness needs a real keel story to own evidence while the exercised workspace must remain disposable. |
| **Insight** | Persisting tape artifacts into the primary `.keel` or the disposable scenario workspace creates contract drift: the primary board stops being immutable, while the resettable workspace loses durable proof ownership. A separate artifact board keeps ownership, manifests, and evidence stable without polluting the runtime board. |
| **Suggested Action** | For future dogfood flows, separate execution state from evidence ownership. Keep the executable workspace resettable and route rendered artifacts into a dedicated keel board whose stories reference the canonical scenario sources. |
| **Applies To** | testdata/dogfood/**, src/infrastructure/dogfood_*, src/infrastructure/verification/** |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-06T16:45:00+00:00 |
| **Score** | 0.89 |
| **Confidence** | 0.90 |
| **Applied** |  |
