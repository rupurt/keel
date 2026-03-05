# Knowledge - 1vxvFrNta

> Automated synthesis of story reflections.

## Story: Refactor Config Show Into Technique Flag Matrix (1vxvIZRXy)

### L001: Prefer direct status flags over aggregated recommendation blocks

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Config introspection commands where automation depends on deterministic machine-readable state |
| **Insight** | A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic. |
| **Suggested Action** | Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands. |
| **Applies To** | `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs` |
| **Applied** | yes |



---

## Story: Implement Verify Recommend For Active Detected Techniques (1vxvIaM4w)

### L001: Centralize technique status before rendering

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple commands (`config show`, `verify recommend`) need the same detected/disabled/active evaluation. |
| **Insight** | A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces. |
| **Suggested Action** | Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs` |
| **Applied** | yes |



---

## Story: Remove Planning Show Recommendations And Update Planning Guidance (1vxvIa2RC)

### L001: Keep recommendation sourcing decoupled from planning read surfaces

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

### L001: Parse legacy forms but block execution paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | CLI hard cutovers where old invocations should fail fast with recovery guidance |
| **Insight** | Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path. |
| **Suggested Action** | For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands. |
| **Applies To** | `src/cli/command_tree.rs`, `src/cli/runtime.rs`, `src/cli/commands/management/verify.rs` |
| **Applied** | yes |



---

