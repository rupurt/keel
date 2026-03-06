---
source_type: Story
source: stories/1vyWRj000/REFLECT.md
scope: 1vyWLl000/1vyWNL000
source_story_id: 1vyWRj000
created_at: 2026-03-06T07:46:05
---

### 1vyWX1Qh7: Timebox External Verification Runners And Emit Log Paths

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | when keel delegates acceptance proofs to external tools such as VHS or future semantic judges |
| **Insight** | External verifier processes can hang without producing useful stderr, so the runner must enforce a timeout and always persist a log path or the queue stalls without actionable failure context. |
| **Suggested Action** | Wrap external verification tools in an explicit timeout, keep the failing workspace/tape/output paths in the error, and write a run log even on failure. |
| **Applies To** | `src/infrastructure/vhs.rs`, `src/infrastructure/dogfood_runner.rs`, `testdata/dogfood/scenarios/*.tape` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-06T15:46:00+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.88 |
| **Applied** | yes |
