---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9wpT7/KNOWLEDGE.md
created_at: 2026-03-02T10:42:29
---

### gYNMP5JXn: Regression Parity Needs Cross-Command Coverage

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | During migration of command handlers to shared application/read-model layers |
| **Insight** | Policy thresholds can drift silently unless `next` and `flow` are asserted together at the same boundary conditions |
| **Suggested Action** | Add paired regression tests that validate both command-level decisions and dashboard summaries for each queue policy boundary |
| **Applies To** | `src/next/*`, `src/flow/*`, `src/commands/story/*`, `src/command_regression_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuw5Ob |
| **Observed At** |  |
| **Score** | 0.80 |
| **Confidence** | 0.89 |
| **Applied** | Added `command_regression_tests` cases for human-block and flow-block boundaries plus lifecycle start/submit/accept chain |
