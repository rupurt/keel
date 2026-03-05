---
source_type: Story
source: stories/1vxH83MOO/REFLECT.md
scope: 1vxGy5tco/1vxGzV3oR
source_story_id: 1vxH83MOO
---

### 1vyDuwuNj: Keep CLI contract updates end-to-end

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | When changing creation command flags and required inputs |
| **Insight** | Command tree flags, runtime mappers, and user-facing suggestion strings drift unless updated in the same slice. |
| **Suggested Action** | Pair every CLI contract edit with parser rejection tests for removed flags and updates to generated command hints. |
| **Applies To** | src/cli/command_tree.rs, src/cli/runtime.rs, src/cli_tests.rs, src/cli/presentation/flow/next_up.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T16:09:32+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | yes |
