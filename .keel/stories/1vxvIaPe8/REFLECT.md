---
created_at: 2026-03-04T15:55:40
---

# Reflection - Hard Cutover Verify Command To Subcommands

## Knowledge

### 1vyDuwu3r: Parse legacy forms but block execution paths
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | CLI hard cutovers where old invocations should fail fast with recovery guidance |
| **Insight** | Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path. |
| **Suggested Action** | For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands. |
| **Applies To** | `src/cli/command_tree.rs`, `src/cli/runtime.rs`, `src/cli/commands/management/verify.rs` |
| **Observed At** | 2026-03-04T23:46:00Z |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

- The JSON contract for `verify run` was straightforward once execution output was normalized into a shared payload builder used by both single-story and all-story paths.
- A lot of downstream messaging referenced `keel verify`; updating guidance and doctor recovery text in the same slice prevented mixed-command confusion after the cutover.
