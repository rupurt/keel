# VOYAGE REPORT: Scope Lineage and Drift Detection

## Voyage Metadata
- **ID:** 1vyFn0OuN
- **Epic:** 1vyFgR2MA
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Parse Canonical Scope Lineage
- **ID:** 1vyGZfkjV
- **Status:** done

#### Summary
Parse canonical scope identifiers from PRD and SRS artifacts so planning can reason about in-scope and out-of-scope lineage without stripping away authored descriptive text.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] PRD `In Scope` and `Out of Scope` items use canonical identifiers in a parseable form. <!-- verify: cargo test -p keel prd_scope_parser_reads_canonical_scope_ids, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Voyage SRS scope statements reference parent PRD scope IDs for included and excluded scope items. <!-- verify: cargo test -p keel srs_scope_requires_parent_prd_scope_ids, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-02/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD/SRS scope fixtures produce deterministic parsed output. <!-- verify: cargo test -p keel scope_lineage_parser_is_deterministic, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path**
  - Insight: Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render
  - Suggested Action: Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies
  - Applies To: `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZfkjV/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZfkjV/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZfkjV/EVIDENCE/ac-2.log)

### Detect Scope Drift During Planning
- **ID:** 1vyGZflfJ
- **Status:** done

#### Summary
Detect scope drift during planning so voyages cannot quietly claim work outside the PRD’s approved boundary or omit required scope mappings.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Doctor diagnostics report unknown scope refs, missing scope mappings, and direct contradictions with PRD out-of-scope definitions. <!-- verify: cargo test -p keel doctor_reports_scope_drift_and_contradictions, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] [SRS-NFR-02/AC-01] Scope drift failures identify the artifact, offending scope ID, and contradiction type. <!-- verify: cargo test -p keel scope_drift_errors_are_actionable, SRS-NFR-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-03] [SRS-NFR-03/AC-01] Scope validation rejects legacy untagged compatibility paths instead of keeping fallback parsing. <!-- verify: cargo test -p keel scope_lineage_rejects_legacy_untagged_paths, SRS-NFR-03:start:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyIA4sQm: Scope Planning Diagnostics To Transition-Relevant Voyages**
  - Insight: reporting the exact same lineage rules across the whole board retroactively fails historical `done` voyages that cannot exercise `voyage plan`, so diagnostic scope must match transition reachability unless migration is explicit work
  - Suggested Action: apply planning coherence checks to non-terminal voyages by default, and handle historical migrations in separate board-cleanup stories
  - Applies To: `src/cli/commands/diagnostics/doctor/checks/*.rs`, planning coherence diagnostics
  - Category: architecture

- **1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path**
  - Insight: Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render
  - Suggested Action: Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies
  - Applies To: `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks
  - Category: architecture

- **1vyKOWjfv: Canonical Scope Contracts Need Explicit Activation Markers**
  - Insight: New planning contracts are safest when they activate off explicit markers like `SCOPE-*`; otherwise doctor treats historical prose as invalid structure and turns a targeted validator into migration noise.
  - Suggested Action: Define an activation marker whenever a new authored planning contract is introduced, and only enforce the stricter validator once that marker appears in the relevant artifacts.
  - Applies To: `src/domain/state_machine/invariants.rs`, `src/cli/commands/diagnostics/doctor/checks/voyages.rs`, PRD and SRS scope sections
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZflfJ/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZflfJ/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZflfJ/EVIDENCE/ac-2.log)

### Render Scope Lineage In Planning Surfaces
- **ID:** 1vyGZgiTK
- **Status:** done

#### Summary
Expose scope lineage and drift findings in voyage and epic planning surfaces so reviewers can see scope alignment without opening every source document by hand.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Planning read surfaces summarize linked scope items and highlight scope drift findings for both the voyage and the parent epic. <!-- verify: cargo test -p keel planning_show_renders_scope_lineage_and_drift, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Scope-lineage output includes enough linked context for planners to distinguish valid in-scope coverage from out-of-scope contradictions. <!-- verify: cargo test -p keel planning_show_scope_drift_output_is_reviewable, SRS-04:end, proof: ac-2.log-->
- [x] [SRS-05/AC-01] Planning views preserve authored descriptive scope text while still surfacing canonical scope IDs for machine-checkable lineage. <!-- verify: cargo test -p keel planning_show_preserves_scope_text_with_ids, SRS-05:start:end, proof: ac-3.log-->

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

- **1vyKXvBA1: Lineage Surfaces Need IDs And Prose Together**
  - Insight: Planning surfaces become much more reviewable when each lineage row carries the canonical ID, the authored prose, and the parent/child disposition context together; token-only output hides meaning, while prose-only output hides the contract.
  - Suggested Action: For future lineage read models, project one canonical row format that combines identifiers with authored descriptions and relation context before the CLI renderer touches it.
  - Applies To: `src/read_model/planning_show.rs`, `src/cli/commands/management/epic/show.rs`, `src/cli/commands/management/voyage/show.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZgiTK/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZgiTK/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZgiTK/EVIDENCE/ac-2.log)


