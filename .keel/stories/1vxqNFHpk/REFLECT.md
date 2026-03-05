---
created_at: 2026-03-04T13:05:08
---

# Reflection - Surface Technique Recommendations In Planning Shows

## Knowledge

### 1vyDuwLvf: Centralized recommendation projection keeps show commands coherent
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple read commands need consistent recommendation output while using different local data sources. |
| **Insight** | A shared recommendation report model plus per-command input extraction avoids drift between epic/voyage/story rendering. |
| **Suggested Action** | Add new recommendation behavior in `verification_techniques` first, then wire each show command through the same renderer helper. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/management/*/show.rs` |
| **Observed At** | 2026-03-04T12:55:00Z |
| **Score** | 0.85 |
| **Confidence** | 0.93 |
| **Applied** | yes |

## Observations

Recommendation ranking and usage-state labeling were straightforward once project signal detection and used-technique inference were deterministic. The hardest part was keeping advisory output explicit so show commands never imply execution side effects. Coverage-focused AC tests on section presence, usage status, and non-execution behavior prevented regression while refactoring shared read-model code.
