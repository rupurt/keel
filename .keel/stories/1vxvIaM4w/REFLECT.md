---
created_at: 2026-03-04T16:13:11
---

# Reflection - Implement Verify Recommend For Active Detected Techniques

## Knowledge

### 1vyDuwmNc: Centralize technique status before rendering
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple commands (`config show`, `verify recommend`) need the same detected/disabled/active evaluation. |
| **Insight** | A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces. |
| **Suggested Action** | Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs` |
| **Observed At** | 2026-03-05T00:10:00Z |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |

## Observations

- Reusing one status report for both human and JSON outputs made deterministic tests straightforward and reduced surface-specific branching.
- The acceptance-criteria order in the story did not match AC numbering (`AC-03` appears before `AC-02`), which made evidence indexing error-prone during recording.
