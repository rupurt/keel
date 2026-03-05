---
created_at: 2026-03-04T16:27:11
---

# Knowledge - 1vxvFrNta

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Implement Verify Recommend For Active Detected Techniques (1vxvIaM4w)

### 1vyDuwmNc: Centralize technique status before rendering

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple commands (`config show`, `verify recommend`) need the same detected/disabled/active evaluation. |
| **Insight** | A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces. |
| **Suggested Action** | Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs` |
| **Applied** | yes |



---

## Story: Refactor Config Show Into Technique Flag Matrix (1vxvIZRXy)

### 1vyDuwBfG: Prefer direct status flags over aggregated recommendation blocks

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Config introspection commands where automation depends on deterministic machine-readable state |
| **Insight** | A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic. |
| **Suggested Action** | Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands. |
| **Applies To** | `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs` |
| **Applied** | yes |



---

## Story: Remove Planning Show Recommendations And Update Planning Guidance (1vxvIa2RC)

### 1vyDuwUUO: Keep recommendation sourcing decoupled from planning read surfaces

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Planning read commands and verification-technique discovery can drift when both surfaces try to rank/recommend techniques. |
| **Insight** | Moving recommendation concerns to dedicated commands (`config show` inventory + `verify recommend`) keeps planning show outputs focused on planning state and avoids mixed concerns. |
| **Suggested Action** | Keep epic/voyage/story show projections limited to planning progress/evidence summaries; centralize recommendation logic in verification/config read models. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/verify.rs`, `AGENTS.md` |
| **Applied** | yes |



---

## Story: Hard Cutover Verify Command To Subcommands (1vxvIaPe8)

### 1vyDuwu3r: Parse legacy forms but block execution paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | CLI hard cutovers where old invocations should fail fast with recovery guidance |
| **Insight** | Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path. |
| **Suggested Action** | For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands. |
| **Applies To** | `src/cli/command_tree.rs`, `src/cli/runtime.rs`, `src/cli/commands/management/verify.rs` |
| **Applied** | yes |



---

## Synthesis

### tDRpzXuPC: Centralize technique status before rendering

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple commands (`config show`, `verify recommend`) need the same detected/disabled/active evaluation. |
| **Insight** | A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces. |
| **Suggested Action** | Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs` |
| **Linked Knowledge IDs** | 1vyDuwmNc |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |

### gEqMvGXEE: Prefer direct status flags over aggregated recommendation blocks

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Config introspection commands where automation depends on deterministic machine-readable state |
| **Insight** | A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic. |
| **Suggested Action** | Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands. |
| **Applies To** | `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs` |
| **Linked Knowledge IDs** | 1vyDuwBfG |
| **Score** | 0.86 |
| **Confidence** | 0.90 |
| **Applied** | yes |

### ng7SivrJS: Keep recommendation sourcing decoupled from planning read surfaces

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Planning read commands and verification-technique discovery can drift when both surfaces try to rank/recommend techniques. |
| **Insight** | Moving recommendation concerns to dedicated commands (`config show` inventory + `verify recommend`) keeps planning show outputs focused on planning state and avoids mixed concerns. |
| **Suggested Action** | Keep epic/voyage/story show projections limited to planning progress/evidence summaries; centralize recommendation logic in verification/config read models. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/verify.rs`, `AGENTS.md` |
| **Linked Knowledge IDs** | 1vyDuwUUO |
| **Score** | 0.79 |
| **Confidence** | 0.89 |
| **Applied** | yes |

### 9c2bFuX0o: Parse legacy forms but block execution paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | CLI hard cutovers where old invocations should fail fast with recovery guidance |
| **Insight** | Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path. |
| **Suggested Action** | For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands. |
| **Applies To** | `src/cli/command_tree.rs`, `src/cli/runtime.rs`, `src/cli/commands/management/verify.rs` |
| **Linked Knowledge IDs** | 1vyDuwu3r |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |

