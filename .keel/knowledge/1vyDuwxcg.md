---
source_type: Story
source: stories/1vxH84Xh8/REFLECT.md
scope: 1vxGy5tco/1vxGzV3oR
source_story_id: 1vxH84Xh8
---

### 1vyDuwxcg: Keep CLI parser, runtime mapping, and template tokens aligned

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Adding new `adr new` flags required updates across clap action enums, command tree wiring, runtime ArgMatches extraction, and template rendering inputs. |
| **Insight** | Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set. |
| **Suggested Action** | For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior. |
| **Applies To** | src/cli/commands/management/adr/mod.rs, src/cli/runtime.rs, src/cli/command_tree.rs, templates/adrs/ADR.md |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T18:13:00+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** |  |
