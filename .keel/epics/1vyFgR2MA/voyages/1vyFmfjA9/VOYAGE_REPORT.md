# VOYAGE REPORT: Goal-to-Requirement Lineage

## Voyage Metadata
- **ID:** 1vyFmfjA9
- **Epic:** 1vyFgR2MA
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Validate Goal Links In PRD Requirements
- **ID:** 1vyGZeEI7
- **Status:** done

#### Summary
Validate that each PRD requirement links back to strategic goals and make those failures actionable in doctor so orphaned goals and unlinked requirements cannot hide in planning.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] PRD FR/NFR requirement rows reference one or more valid `GOAL-*` identifiers. <!-- verify: cargo test -p keel prd_requirements_require_valid_goal_links, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-03/AC-01] Doctor diagnostics report invalid goal references, orphaned goals, and PRD requirements with no goal linkage. <!-- verify: cargo test -p keel doctor_reports_goal_lineage_gaps, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-02] [SRS-NFR-02/AC-01] Goal-lineage failures identify the offending goal ID, requirement ID, and artifact path. <!-- verify: cargo test -p keel goal_lineage_errors_are_actionable, SRS-NFR-02:start:end, proof: ac-3.log-->
- [x] [SRS-02/AC-02] [SRS-NFR-03/AC-01] Non-canonical or legacy goal tokens fail hard without compatibility aliases. <!-- verify: cargo test -p keel goal_lineage_rejects_legacy_tokens, SRS-NFR-03:start:end, SRS-02:end, proof: ac-4.log-->

#### Implementation Insights
- **1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers**
  - Insight: Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace
  - Suggested Action: Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes
  - Applies To: `src/domain/state_machine/*.rs`, planning document table parsers
  - Category: code

- **1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path**
  - Insight: Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render
  - Suggested Action: Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies
  - Applies To: `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks
  - Category: architecture


#### Verified Evidence
- [ac-4.log](../../../../stories/1vyGZeEI7/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vyGZeEI7/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZeEI7/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZeEI7/EVIDENCE/ac-2.log)

### Parse Canonical Goal Lineage
- **ID:** 1vyGZeNMa
- **Status:** done

#### Summary
Introduce the canonical goal-lineage parser so PRD goals become machine-readable planning inputs with stable `GOAL-*` identifiers and deterministic fanout behavior.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] PRD `Goals & Objectives` entries use canonical `GOAL-*` identifiers in a parseable table form. <!-- verify: cargo test -p keel goal_lineage_parser_reads_canonical_goal_rows, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD goal fixtures produce deterministic parsed output. <!-- verify: cargo test -p keel goal_lineage_parser_is_deterministic, SRS-NFR-01:start:end, SRS-01:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers**
  - Insight: Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace
  - Suggested Action: Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes
  - Applies To: `src/domain/state_machine/*.rs`, planning document table parsers
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZeNMa/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyGZeNMa/EVIDENCE/ac-2.log)

### Render Goal Coverage In Epic Planning
- **ID:** 1vyGZfiEk
- **Status:** done

#### Summary
Render goal-to-requirement lineage in epic planning views so reviewers can inspect whether strategic objectives are actually represented by the PRD requirement set.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Epic planning read surfaces summarize goal-to-requirement lineage directly from the PRD with enough detail to review objective coverage. <!-- verify: cargo test -p keel epic_show_renders_goal_lineage_summary, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-05/AC-01] Goal-lineage rendering preserves one-to-many goal fanout to requirements without unstable ordering. <!-- verify: cargo test -p keel epic_goal_lineage_preserves_one_to_many_fanout, SRS-05:start:end, proof: ac-2.log-->
- [x] [SRS-05/AC-02] Goal-coverage rendering preserves stable ordering and fanout counts for equivalent PRDs. <!-- verify: cargo test -p keel epic_goal_lineage_projection_is_deterministic, SRS-05:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path**
  - Insight: Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render
  - Suggested Action: Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies
  - Applies To: `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZfiEk/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZfiEk/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZfiEk/EVIDENCE/ac-2.log)


