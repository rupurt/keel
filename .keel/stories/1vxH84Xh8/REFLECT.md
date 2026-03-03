# Reflection - Extend Adr Creation Inputs For Context Ownership

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Keep CLI parser, runtime mapping, and template tokens aligned

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Adding new `adr new` flags required updates across clap action enums, command tree wiring, runtime ArgMatches extraction, and template rendering inputs. |
| **Insight** | Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set. |
| **Suggested Action** | For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior. |
| **Applies To** | src/cli/commands/management/adr/mod.rs, src/cli/runtime.rs, src/cli/command_tree.rs, templates/adrs/ADR.md |
| **Observed At** | 2026-03-03T18:13:00Z |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** | |

## Observations

The implementation was straightforward after introducing a board-path helper for ADR creation, which made command persistence tests deterministic. The key risk was missing one parser layer, so explicit tests for both CLI parse behavior and saved frontmatter prevented drift.
