# Reflection - Implement Voyage Show Requirement Progress

## Knowledge

### L001: Voyage Requirement Views Need Both AC And Verify Mapping
| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building requirement-level voyage progress from story artifacts |
| **Insight** | Requirement linkage should combine AC references and verify requirement IDs; relying on one source undercounts coverage/verification state. |
| **Suggested Action** | Build requirement matrices from both marker channels, then deterministically sort rows and linked stories. |
| **Applies To** | `src/cli/commands/management/voyage/show.rs`, planning-read projections |
| **Observed At** | 2026-03-04T19:15:27Z |
| **Score** | 0.82 |
| **Confidence** | 0.9 |
| **Applied** | yes |

## Observations

Adding a report builder made it straightforward to test goal/scope extraction, matrix mapping, and progress signals independently.
The key tradeoff was balancing strict parsing with resilience to partially authored SRS files, so explicit placeholders were kept.
