# VOYAGE REPORT: Verification Technique Command Surface Cutover

## Voyage Metadata
- **ID:** 1vxvFrNta
- **Epic:** 1vxqMtskC
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Refactor Config Show Into Technique Flag Matrix
- **ID:** 1vxvIZRXy
- **Status:** done

#### Summary
Refactor `keel config show` to present verification techniques as a canonical flag matrix and machine-readable payload, while keeping `keel config mode` unchanged.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel config show` renders one row per built-in/custom technique with `label`, `detected`, `disabled`, and `active`, where `label` is the hyphenated technique id. <!-- verify: cargo test -p keel config_show_renders_technique_flag_matrix, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] The matrix includes all techniques regardless of active/disabled state, with deterministic ordering. <!-- verify: cargo test -p keel config_show_lists_all_techniques_deterministically, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-02/AC-01] `keel config show` no longer prints scoring output, and `keel config mode` remains behaviorally unchanged. <!-- verify: cargo test -p keel config_show_omits_scoring_and_config_mode_regression, SRS-02:start:end, proof: ac-3.log-->
- [x] [SRS-01/AC-03] `keel config show --json` emits deterministic machine-readable rows using the same `label/detected/disabled/active` contract. <!-- verify: cargo test -p keel config_show_json_contract, SRS-01:end, proof: ac-4.log-->

#### Implementation Insights
- **1vyDuwBfG: Prefer direct status flags over aggregated recommendation blocks**
  - Insight: A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic.
  - Suggested Action: Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands.
  - Applies To: `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs`
  - Category: architecture


#### Verified Evidence
- [ac-4.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-2.log)

### Remove Planning Show Recommendations And Update Planning Guidance
- **ID:** 1vxvIa2RC
- **Status:** done

#### Summary
Remove recommendation sections from planning read commands and update architect planning guidance to rely on `config show` and `verify recommend` for verification technique planning.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] `keel epic show`, `keel voyage show`, and `keel story show` no longer render verification-technique recommendation sections. <!-- verify: cargo test -p keel planning_show_omits_verification_recommendations, SRS-05:start, proof: ac-1.log-->
- [x] [SRS-05/AC-02] Existing planning/evidence/progress sections remain intact after recommendation removal. <!-- verify: cargo test -p keel planning_show_preserves_existing_sections, SRS-05:end, proof: ac-2.log-->
- [x] [SRS-06/AC-01] `AGENTS.md` planning workflow explicitly references `just keel config show` (inventory) and `just keel verify recommend` (detected+active options) for verification planning. <!-- verify: manual, SRS-06:start:end, proof: ac-3.log -->

#### Implementation Insights
- **1vyDuwUUO: Keep recommendation sourcing decoupled from planning read surfaces**
  - Insight: Moving recommendation concerns to dedicated commands (`config show` inventory + `verify recommend`) keeps planning show outputs focused on planning state and avoids mixed concerns.
  - Suggested Action: Keep epic/voyage/story show projections limited to planning progress/evidence summaries; centralize recommendation logic in verification/config read models.
  - Applies To: `src/read_model/planning_show.rs`, `src/cli/commands/management/verify.rs`, `AGENTS.md`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxvIa2RC/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIa2RC/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIa2RC/EVIDENCE/ac-2.log)

### Implement Verify Recommend For Active Detected Techniques
- **ID:** 1vxvIaM4w
- **Status:** done

#### Summary
Introduce `keel verify recommend` as the recommendation surface, filtered to detected and active techniques only, with advisory-only behavior and machine-readable output.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] `keel verify recommend` renders only techniques where `detected=true` and `active=true` (where `active = detected && !disabled`). <!-- verify: cargo test -p keel verify_recommend_filters_detected_active, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-03] `keel verify recommend` is advisory-only and does not execute verification tools/commands. <!-- verify: cargo test -p keel verify_recommend_has_no_execution_side_effects, SRS-NFR-03:start:end, proof: ac-2.log-->
- [x] [SRS-04/AC-02] `keel verify recommend --json` emits deterministic machine-readable recommendations using the same filter contract. <!-- verify: cargo test -p keel verify_recommend_json_contract, SRS-04:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyDuwmNc: Centralize technique status before rendering**
  - Insight: A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces.
  - Suggested Action: Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code.
  - Applies To: `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxvIaM4w/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIaM4w/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIaM4w/EVIDENCE/ac-2.log)

### Hard Cutover Verify Command To Subcommands
- **ID:** 1vxvIaPe8
- **Status:** done

#### Summary
Perform a hard cutover of verification execution to `keel verify run`, preserving execution semantics while making legacy `keel verify` fail fast with migration guidance.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel verify run` executes the existing verification flow with parity for target selection (`<id>` and `--all`). <!-- verify: cargo test -p keel verify_run_preserves_execution_semantics, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-03] Bare `keel verify` exits non-zero and prints explicit recovery guidance to use `keel verify run`. <!-- verify: cargo test -p keel verify_root_fails_fast_with_run_guidance, SRS-NFR-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-02] `keel verify run --json` returns deterministic machine-readable execution results equivalent to the text path. <!-- verify: cargo test -p keel verify_run_json_contract, SRS-03:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyDuwu3r: Parse legacy forms but block execution paths**
  - Insight: Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path.
  - Suggested Action: For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands.
  - Applies To: `src/cli/command_tree.rs`, `src/cli/runtime.rs`, `src/cli/commands/management/verify.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxvIaPe8/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIaPe8/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIaPe8/EVIDENCE/ac-2.log)


