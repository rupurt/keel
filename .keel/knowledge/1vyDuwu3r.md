---
source_type: Story
source: stories/1vxvIaPe8/REFLECT.md
scope: 1vxqMtskC/1vxvFrNta
source_story_id: 1vxvIaPe8
created_at: 2026-03-04T15:55:40
---

### 1vyDuwu3r: Parse legacy forms but block execution paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | CLI hard cutovers where old invocations should fail fast with recovery guidance |
| **Insight** | Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path. |
| **Suggested Action** | For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands. |
| **Applies To** | `src/cli/command_tree.rs`, `src/cli/runtime.rs`, `src/cli/commands/management/verify.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T23:46:00+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |
