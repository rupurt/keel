---
created_at: 2026-03-04T10:21:20
---

# Reflection - Default New Stories To Icebox

## Knowledge

### 1vyDuwoz6: Intake Stage Must Be Canonical In Creation Code
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story creation had stage branching by voyage state |
| **Insight** | Canonical defaults are safest when enforced once in creation code and not left to template/state-condition combinations. |
| **Suggested Action** | Keep `story new` stage assignment unconditional and test it across unscoped/draft/planned scopes. |
| **Applies To** | `src/cli/commands/management/story/new.rs`, story template/frontmatter defaults |
| **Observed At** | 2026-03-04T18:19:47Z |
| **Score** | 0.8 |
| **Confidence** | 0.9 |
| **Applied** | yes |

## Observations

Changing the default stage alone was simple; proving no backlog fallback remained required scoped coverage tests.
Adding thaw/start guidance immediately after creation made the new flow explicit and reduced transition ambiguity.
