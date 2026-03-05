---
source_type: Story
source: stories/1vwqCffzr/REFLECT.md
scope: 1vwq96cpt/1vwq9wpT7
source_story_id: 1vwqCffzr
---

### 1vyDuw5Ob: Regression Parity Needs Cross-Command Coverage

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | During migration of command handlers to shared application/read-model layers |
| **Insight** | Policy thresholds can drift silently unless `next` and `flow` are asserted together at the same boundary conditions |
| **Suggested Action** | Add paired regression tests that validate both command-level decisions and dashboard summaries for each queue policy boundary |
| **Applies To** | `src/next/*`, `src/flow/*`, `src/commands/story/*`, `src/command_regression_tests.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T18:21:00+00:00 |
| **Score** | 0.80 |
| **Confidence** | 0.89 |
| **Applied** | Added `command_regression_tests` cases for human-block and flow-block boundaries plus lifecycle start/submit/accept chain |
