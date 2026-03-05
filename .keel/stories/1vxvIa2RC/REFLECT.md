---
created_at: 2026-03-04T16:27:11
---

# Reflection - Remove Planning Show Recommendations And Update Planning Guidance

## Knowledge

### 1vyDuwUUO: Keep recommendation sourcing decoupled from planning read surfaces
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Planning read commands and verification-technique discovery can drift when both surfaces try to rank/recommend techniques. |
| **Insight** | Moving recommendation concerns to dedicated commands (`config show` inventory + `verify recommend`) keeps planning show outputs focused on planning state and avoids mixed concerns. |
| **Suggested Action** | Keep epic/voyage/story show projections limited to planning progress/evidence summaries; centralize recommendation logic in verification/config read models. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/verify.rs`, `AGENTS.md` |
| **Observed At** | 2026-03-05T00:25:00Z |
| **Score** | 0.79 |
| **Confidence** | 0.89 |
| **Applied** | yes |

## Observations

- Compile-time destructuring tests were a reliable way to prove recommendation fields are no longer part of planning-show projections.
- The main risk in this slice was semantic drift in AGENTS guidance; updating command references in the same commit prevented stale planning instructions.
