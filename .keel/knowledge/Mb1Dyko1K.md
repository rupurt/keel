---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzV3oR/KNOWLEDGE.md
created_at: 2026-03-03T08:10:40
---

### Mb1Dyko1K: Keep CLI parser, runtime mapping, and template tokens aligned

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Adding new `adr new` flags required updates across clap action enums, command tree wiring, runtime ArgMatches extraction, and template rendering inputs. |
| **Insight** | Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set. |
| **Suggested Action** | For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior. |
| **Applies To** | src/cli/commands/management/adr/mod.rs, src/cli/runtime.rs, src/cli/command_tree.rs, templates/adrs/ADR.md |
| **Linked Knowledge IDs** | 1vyDuwxcg |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** |  |
