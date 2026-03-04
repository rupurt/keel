# Knowledge - 1vxpomgnN

> Automated synthesis of story reflections.

## Story: Render Concrete Evidence In Story Show (1vxppkEH9)

### L001: Evidence UX Needs Structured Inventory Layers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering acceptance evidence directly in `story show` |
| **Insight** | A clear split between linked proofs, supplementary artifacts, and media playback hints makes acceptance decisions possible without opening files manually. |
| **Suggested Action** | Keep evidence rendering model-driven and test each layer (metadata, excerpts, missing warnings, placeholders) independently. |
| **Applies To** | `src/cli/commands/management/story/show.rs`, future evidence/report renderers |
| **Applied** | yes |



---

## Story: Implement Voyage Show Requirement Progress (1vxppkB6M)

### L001: Voyage Requirement Views Need Both AC And Verify Mapping

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building requirement-level voyage progress from story artifacts |
| **Insight** | Requirement linkage should combine AC references and verify requirement IDs; relying on one source undercounts coverage/verification state. |
| **Suggested Action** | Build requirement matrices from both marker channels, then deterministically sort rows and linked stories. |
| **Applies To** | `src/cli/commands/management/voyage/show.rs`, planning-read projections |
| **Applied** | yes |



---

## Story: Implement Epic Show Planning Summary (1vxppkN0w)

### L001: Planning Show Parsing Needs Scaffold Filters

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Extracting PRD summaries from partially authored templates |
| **Insight** | Requirement parsing must explicitly ignore scaffold rows like `TODO`/template defaults or placeholder mode appears complete when it is not. |
| **Suggested Action** | Keep placeholder filters and add fixture tests that assert empty summaries on scaffold-only PRDs. |
| **Applies To** | `src/cli/commands/management/epic/show.rs`, planning projection parsers |
| **Applied** | yes |



---

## Story: Define Planning Show Output Contracts (1vxppk4Oj)

### L001: Centralized show projections reduce drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple show commands were independently parsing PRD/SRS/story evidence with diverging placeholder and ordering rules. |
| **Insight** | A shared read-model projection layer stabilizes data contracts, keeps ordering deterministic, and lets renderers remain thin. |
| **Suggested Action** | Add new planning/read surfaces by extending `read_model::planning_show` first, then adapt renderer output only. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/*/show.rs` |
| **Applied** | yes |



---

