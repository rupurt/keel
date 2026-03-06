# VOYAGE REPORT: Canonical PRD Requirement Lineage

## Voyage Metadata
- **ID:** 1vyFiQPoH
- **Epic:** 1vyFgR2MA
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Gate Voyage Planning On PRD Lineage
- **ID:** 1vyGZEO8S
- **Status:** done

#### Summary
Enforce the canonical PRD-to-SRS lineage contract during voyage planning so invalid or legacy source references never transition a voyage to `planned`.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Validate every voyage SRS requirement row so the `Source` column contains exactly one existing parent `FR-*` or `NFR-*` from the epic PRD. <!-- verify: cargo test -p keel srs_source_requires_exactly_one_canonical_prd_parent, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-01] `voyage plan` hard-blocks when any SRS requirement is missing a parent source, references a non-existent parent, or uses a non-canonical legacy token. <!-- verify: cargo test -p keel voyage_plan_blocks_invalid_prd_lineage, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-02] [SRS-NFR-02/AC-01] Blocking errors name the artifact path, offending source token, and expected canonical form. <!-- verify: cargo test -p keel prd_lineage_gate_errors_are_actionable, SRS-NFR-02:start:end, proof: ac-3.log-->
- [x] [SRS-03/AC-03] [SRS-NFR-03/AC-01] Legacy `PRD-*` or custom source-token aliases are rejected instead of silently accepted. <!-- verify: cargo test -p keel prd_lineage_rejects_legacy_source_aliases, SRS-NFR-03:start:end, proof: ac-4.log-->

#### Implementation Insights
- **1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers**
  - Insight: Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace
  - Suggested Action: Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes
  - Applies To: `src/domain/state_machine/*.rs`, planning document table parsers
  - Category: code


#### Verified Evidence
- [ac-4.log](../../../../stories/1vyGZEO8S/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vyGZEO8S/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZEO8S/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZEO8S/EVIDENCE/ac-2.log)

### Render Epic Requirement Coverage
- **ID:** 1vyGZEZNc
- **Status:** done

#### Summary
Surface epic-wide parent requirement coverage from the same lineage rules used by runtime enforcement so planners and reviewers can spot uncovered PRD requirements across voyages.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Doctor diagnostics report PRD-to-SRS lineage problems using the same coherence rules used by planning transitions. <!-- verify: cargo test -p keel doctor_and_gate_share_prd_lineage_rules, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-05/AC-01] Epic planning projections aggregate parent FR/NFR coverage across all voyages and identify uncovered parent requirements with linked-child counts. <!-- verify: cargo test -p keel epic_show_aggregates_prd_requirement_coverage_across_voyages, SRS-05:start:end, proof: ac-2.log-->
- [x] [SRS-06/AC-01] Coverage aggregation preserves exactly-one-parent ownership for each SRS requirement while allowing one parent FR/NFR to fan out to multiple voyage requirements without double counting. <!-- verify: cargo test -p keel prd_requirement_coverage_preserves_one_to_many_parent_fanout, SRS-06:start:end, proof: ac-3.log-->
- [x] [SRS-05/AC-02] [SRS-NFR-01/AC-02] Equivalent board state yields stable ordering for parent requirements, linked children, and uncovered-requirement output. <!-- verify: cargo test -p keel epic_prd_coverage_projection_is_deterministic, SRS-NFR-01:start:end, proof: ac-4.log-->

#### Implementation Insights
- **1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers**
  - Insight: Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace
  - Suggested Action: Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes
  - Applies To: `src/domain/state_machine/*.rs`, planning document table parsers
  - Category: code

- **1vyIA4sQm: Scope Planning Diagnostics To Transition-Relevant Voyages**
  - Insight: reporting the exact same lineage rules across the whole board retroactively fails historical `done` voyages that cannot exercise `voyage plan`, so diagnostic scope must match transition reachability unless migration is explicit work
  - Suggested Action: apply planning coherence checks to non-terminal voyages by default, and handle historical migrations in separate board-cleanup stories
  - Applies To: `src/cli/commands/diagnostics/doctor/checks/*.rs`, planning coherence diagnostics
  - Category: architecture


#### Verified Evidence
- [ac-4.log](../../../../stories/1vyGZEZNc/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vyGZEZNc/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZEZNc/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZEZNc/EVIDENCE/ac-2.log)

### Parse Canonical PRD Requirement Lineage
- **ID:** 1vyGZEowA
- **Status:** done

#### Summary
Introduce the canonical parser and lineage model that extracts epic `FR-*` and `NFR-*` requirements from `PRD.md` and makes them reusable across planning gates, diagnostics, and coverage projections.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Parse canonical parent `FR-*` and `NFR-*` rows from an epic `PRD.md` into a reusable lineage model keyed by epic ID. <!-- verify: cargo test -p keel prd_lineage_parser_builds_canonical_parent_map, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-03] The lineage model exposes enough canonical parent metadata for downstream coverage and enforcement paths to reuse one shared parse result per epic. <!-- verify: cargo test -p keel prd_lineage_model_exposes_reusable_parent_metadata, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-01/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD fixtures produce deterministically ordered lineage output. <!-- verify: cargo test -p keel prd_lineage_parser_is_deterministic, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vyGZEowA/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyGZEowA/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyGZEowA/EVIDENCE/ac-2.log)


