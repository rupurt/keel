# Reflection - Pairwise Blocker Rendering For Parallel Next

## Knowledge

### 1vyDuwzyf: Keep Blocker Schema Shared Across Human and JSON Paths
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering pairwise confidence blockers in CLI and machine-readable output |
| **Insight** | A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync |
| **Suggested Action** | Build future blocker explanations from the same canonical blocker payload and only vary presentation |
| **Applies To** | `src/cli/commands/management/next.rs`, `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Observed At** | 2026-03-05T02:50:59Z |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |

## Observations

- Reusing threshold-gate output for rendering minimized duplication and made both AC tests straightforward.
- JSON details are externally tagged via `JsonDetails`, so tests must assert through `details.parallel_work.*` paths.
- The workflow guard that blocks reflection in backlog stage is useful; starting the story first keeps lifecycle artifacts coherent.
