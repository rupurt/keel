---
source_type: Story
source: stories/1vyGZd1to/REFLECT.md
scope: 1vyFgR2MA/1vyFlAgHB
source_story_id: 1vyGZd1to
created_at: 2026-03-05T16:16:10
---

### 1vyIq5M2c: Verify Annotation Chains Only Materialize One Requirement Token

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | when one acceptance criterion is linked to both a functional SRS requirement and an SRS-NFR requirement |
| **Insight** | The verify-annotation parser keeps only one requirement phase token per AC, so the last `SRS-*:phase` entry controls voyage evidence-chain checks |
| **Suggested Action** | Split evidence-chain phases across separate ACs or put the functional requirement token last when a line carries both SRS and SRS-NFR references |
| **Applies To** | src/infrastructure/verification/parser.rs, .keel/stories/*/README.md |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T16:18:00+00:00 |
| **Score** | 0.75 |
| **Confidence** | 0.90 |
| **Applied** | yes |
