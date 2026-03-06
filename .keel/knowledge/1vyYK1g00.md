---
source_type: Story
source: stories/1vyWSF000/REFLECT.md
scope: 1vyWLl000/1vyWNV000
source_story_id: 1vyWSF000
created_at: 2026-03-06T08:50:03
---

### 1vyYK1g00: Judge Bundles Should Carry References And Hashes

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When defining provider-agnostic semantic judge inputs from existing story evidence. |
| **Insight** | The stable contract is metadata, normalized evidence references, and hashes, not embedded artifact contents or provider-specific fields. That keeps bundle serialization deterministic while leaving transport, prompting, and artifact loading to the external judge wrapper. |
| **Suggested Action** | Keep the bundle as a control-plane document: normalize proof refs into canonical `EVIDENCE/...` paths, sort the evidence inventory, and defer provider-specific payload shaping until the external `llm-judge` boundary. |
| **Applies To** | src/infrastructure/verification/judge_bundle.rs, src/infrastructure/verification/executor.rs, story record/verify judge integration |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-06T17:05:00+00:00 |
| **Score** | 0.77 |
| **Confidence** | 0.87 |
| **Applied** |  |
