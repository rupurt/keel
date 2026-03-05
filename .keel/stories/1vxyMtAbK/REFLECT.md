# Reflection - Story Blocked By Metadata Override

## Knowledge

### L001: Frontmatter Field Additions Need Builder + Literal Sweep
| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding a new key to `StoryFrontmatter` that is constructed in many tests and read models |
| **Insight** | `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation. |
| **Suggested Action** | When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks. |
| **Applies To** | `src/domain/model/story.rs`, `src/test_helpers.rs`, read-model fixture tests |
| **Observed At** | 2026-03-05T02:56:11Z |
| **Score** | 0.81 |
| **Confidence** | 0.92 |
| **Applied** | yes |

## Observations

- Adding deterministic `blocked_by` override logic at the same threshold gate layer kept the behavior explicit and easy to test with one focused pairwise case.
- The `just keel` recipe currently cannot pass multi-word `--msg` arguments to `story record` due unquoted arg forwarding, so evidence notes were recorded as single-token proof messages.
- Focused tests (`next_parallel_blocked_by_frontmatter_parses`, `next_parallel_blocked_by_override_enforced`) provided direct AC traceability before broader hygiene runs.
