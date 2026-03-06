# VOYAGE REPORT: Epic Problem Hydration

## Voyage Metadata
- **ID:** 1vyFlAgHB
- **Epic:** 1vyFgR2MA
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Replace Epic Goal CLI With Problem Input
- **ID:** 1vyGZd1to
- **Status:** done

#### Summary
Replace the current epic-creation CLI contract so authored problem text is the only strategic input collected at scaffold time and CLI-owned goal hydration is removed.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel epic new` accepts a required `--problem` argument and rejects missing or empty problem values during CLI/runtime validation. <!-- verify: cargo test -p keel cli_parses_epic_new_with_required_problem, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] [SRS-NFR-02/AC-01] The new CLI path fails fast instead of injecting defaults or placeholder strategic text when required input is absent. <!-- verify: cargo test -p keel epic_new_problem_input_fails_fast_without_defaults, SRS-NFR-02:start:end, SRS-01:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyIq5M2c: Verify Annotation Chains Only Materialize One Requirement Token**
  - Insight: The verify-annotation parser keeps only one requirement phase token per AC, so the last `SRS-*:phase` entry controls voyage evidence-chain checks
  - Suggested Action: Split evidence-chain phases across separate ACs or put the functional requirement token last when a line carries both SRS and SRS-NFR references
  - Applies To: src/infrastructure/verification/parser.rs, .keel/stories/*/README.md
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZd1to/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyGZd1to/EVIDENCE/ac-2.log)

### Hydrate Epic Problem Into Fresh Scaffolds
- **ID:** 1vyGZdgBX
- **Status:** done

#### Summary
Hydrate authored problem text into fresh epic scaffolds so newly created epics start with real strategic context in the PRD and any remaining CLI-derived summary surface.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] `keel epic new --problem` writes authored narrative content into the PRD `## Problem Statement` section and any epic scaffold summary surface that depends on CLI strategic input. <!-- verify: cargo test -p keel epic_new_hydrates_problem_into_prd_and_summary_surface, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-03/AC-01] `keel epic new` leaves `Goals & Objectives` for direct PRD authoring instead of hydrating a single CLI-owned goal value. <!-- verify: cargo test -p keel epic_new_leaves_goal_table_for_direct_prd_authoring, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-04/AC-01] Template token inventory and rendering paths support the new problem-only strategic-input contract without placeholder drift or stale goal-token ownership. <!-- verify: cargo test -p keel epic_template_tokens_match_problem_only_contract, SRS-04:start:end, proof: ac-3.log-->
- [x] [SRS-02/AC-02] [SRS-NFR-01/AC-01] Identical `epic new --problem` inputs yield deterministic scaffold output. <!-- verify: cargo test -p keel epic_problem_scaffold_is_deterministic, SRS-NFR-01:start:end, SRS-02:end, proof: ac-4.log-->

#### Verified Evidence
- [ac-4.log](../../../../stories/1vyGZdgBX/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vyGZdgBX/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZdgBX/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZdgBX/EVIDENCE/ac-2.log)

### Keep Fresh Epic Scaffolds Doctor Clean
- **ID:** 1vyGZet0Z
- **Status:** done

#### Summary
Keep newly scaffolded epic artifacts structurally clean after the problem-only CLI cutover so doctor and template hygiene checks remain a reliable planning gate.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Freshly scaffolded epic artifacts remain doctor-clean and structurally coherent after the problem-only hydration behavior lands. <!-- verify: cargo test -p keel doctor_accepts_problem_only_epic_scaffold, SRS-05:start, proof: ac-1.log-->
- [x] [SRS-05/AC-02] Embedded epic templates remain placeholder-clean and free of obsolete CLI-owned goal-token behavior after the cutover. <!-- verify: cargo test -p keel epic_templates_drop_legacy_goal_token_usage, SRS-05:continues, proof: ac-2.log-->
- [x] [SRS-05/AC-03] [SRS-NFR-03/AC-01] Generated problem seed content and revised goal scaffolds stay concise, human-editable, and free of unresolved placeholders. <!-- verify: cargo test -p keel epic_problem_scaffold_is_placeholder_clean, SRS-NFR-03:start:end, SRS-05:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyKD7naI: Keep scaffold templates compliant with day-zero doctor contracts**
  - Insight: If newly scaffolded planning artifacts are expected to be immediately doctor-clean, the fix belongs in the template seed content rather than in weaker validation rules.
  - Suggested Action: When creation inputs or placeholder semantics change, regenerate a fresh artifact in tests and run doctor against it before changing any diagnostic gates.
  - Applies To: `templates/epic/[name]/PRD.md`, `src/cli/commands/management/epic/new.rs`, doctor scaffold checks
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZet0Z/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZet0Z/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZet0Z/EVIDENCE/ac-2.log)


