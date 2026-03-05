# Reflection - Render Concrete Evidence In Story Show

## Knowledge

### 1vyDuwUSB: Evidence UX Needs Structured Inventory Layers
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering acceptance evidence directly in `story show` |
| **Insight** | A clear split between linked proofs, supplementary artifacts, and media playback hints makes acceptance decisions possible without opening files manually. |
| **Suggested Action** | Keep evidence rendering model-driven and test each layer (metadata, excerpts, missing warnings, placeholders) independently. |
| **Applies To** | `src/cli/commands/management/story/show.rs`, future evidence/report renderers |
| **Observed At** | 2026-03-04T19:29:27Z |
| **Score** | 0.84 |
| **Confidence** | 0.9 |
| **Applied** | yes |

## Observations

Frontmatter metadata parsing plus text-excerpt handling was straightforward once evidence modeling moved out of print logic.
The main risk was artifact classification drift, so extension-based tests were added for media/supplementary separation.
