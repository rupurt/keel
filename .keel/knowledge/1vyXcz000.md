---
source_type: Story
source: stories/1vyWSC000/REFLECT.md
scope: 1vyWLl000/1vyWNL000
source_story_id: 1vyWSC000
created_at: 2026-03-06T08:01:59
---

### 1vyXcz000: Use hidden setup blocks and dynamic ID discovery in VHS planning flows

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Authoring VHS tapes for keel workflows that create new epics, voyages, and stories on a reset fixture board |
| **Insight** | The readable part of the tape should stay focused on the operator-facing workflow, while markdown authoring and ID plumbing happen in `Hide` blocks using `latest_id` discovery instead of fixed IDs. |
| **Suggested Action** | Keep visible commands to the user journey, generate authored artifacts in hidden heredocs, and derive IDs from the fixture state after each create step to preserve repeatability. |
| **Applies To** | `testdata/dogfood/scenarios/*.tape`, `src/infrastructure/dogfood_runner.rs` |
| **Linked Knowledge IDs** | 1vyWX1Qh7 |
| **Observed At** | 2026-03-06T08:02:30+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.93 |
| **Applied** | `epic-flow.tape` |
