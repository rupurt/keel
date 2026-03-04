# VOYAGE REPORT: Planning Read Surfaces And Evidence Visibility

## Voyage Metadata
- **ID:** 1vxpomgnN
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Define Planning Show Output Contracts
- **ID:** 1vxppk4Oj
- **Status:** done

#### Summary
Define a shared planning/evidence projection contract that all three `show` commands consume, including deterministic section ordering and missing-data placeholders.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Introduce shared projection types and builders for epic/voyage/story `show` data so command renderers read one canonical contract. <!-- verify: cargo test --lib planning_show_projection_contract, SRS-05:start, proof: ac-1.log -->
- [x] [SRS-05/AC-02] Add parsing utilities that extract authored planning sections (problem, goals/objectives, key requirements, verification strategy) from PRD/SRS content while ignoring scaffold comments. <!-- verify: cargo test --lib planning_doc_extractor, SRS-05:end, proof: ac-2.log -->
- [x] [SRS-05/AC-03] Add deterministic-order tests proving projections emit stable section, requirement, story, and artifact ordering across equivalent board states. <!-- verify: cargo test --lib planning_show_projection_deterministic, SRS-NFR-01:start:end, proof: ac-3.log -->

#### Implementation Insights
- **L001: Centralized show projections reduce drift**
  - Insight: A shared read-model projection layer stabilizes data contracts, keeps ordering deterministic, and lets renderers remain thin.
  - Suggested Action: Add new planning/read surfaces by extending `read_model::planning_show` first, then adapt renderer output only.
  - Applies To: `src/read_model/planning_show.rs`, `src/cli/commands/management/*/show.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxppk4Oj/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxppk4Oj/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppk4Oj/EVIDENCE/ac-2.log)

### Implement Voyage Show Requirement Progress
- **ID:** 1vxppkB6M
- **Status:** done

#### Summary
Upgrade `keel voyage show` so it reports voyage intent, scope boundaries, and requirement-level completion/verification progress instead of dumping raw markdown.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel voyage show <id>` renders high-level goal plus explicit in-scope/out-of-scope summary extracted from voyage docs. <!-- verify: cargo test --lib voyage_show_goal_scope, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] `keel voyage show <id>` renders a requirements table listing each SRS requirement with linked stories and completion/verification status. <!-- verify: cargo test --lib voyage_show_requirement_matrix, SRS-03:continues, proof: ac-2.log-->
- [x] [SRS-03/AC-03] `keel voyage show <id>` renders progress indicators for both stories and requirements so completion state is immediately visible. <!-- verify: cargo test --lib voyage_show_progress, SRS-03:end, proof: ac-3.log-->
- [x] [SRS-03/AC-04] [SRS-NFR-01/AC-02] Voyage requirement and story rows are deterministically sorted so equivalent board state yields stable output. <!-- verify: cargo test --lib voyage_show_deterministic_ordering, SRS-NFR-01:start:end, proof: ac-4.log-->

#### Implementation Insights
- **L001: Voyage Requirement Views Need Both AC And Verify Mapping**
  - Insight: Requirement linkage should combine AC references and verify requirement IDs; relying on one source undercounts coverage/verification state.
  - Suggested Action: Build requirement matrices from both marker channels, then deterministically sort rows and linked stories.
  - Applies To: `src/cli/commands/management/voyage/show.rs`, planning-read projections
  - Category: code


#### Verified Evidence
- [ac-4.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-2.log)

### Render Concrete Evidence In Story Show
- **ID:** 1vxppkEH9
- **Status:** done

#### Summary
Rework `keel story show` evidence output to display real proof details (metadata, excerpts, artifact files, and media assets) so human acceptance can happen directly from command output.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] For each AC with verify annotations, `keel story show <id>` renders command/manual mode, proof filename, and parsed proof metadata (`recorded_at`, command/mode) when available. <!-- verify: cargo test --lib story_show_proof_metadata, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] `keel story show <id>` surfaces concrete artifact lists, explicitly separating annotation-linked proofs from supplementary artifacts and media files (for example `.gif`, `.png`, `.mp4`, `.webm`) and includes whole-asset playback guidance. <!-- verify: cargo test --lib story_show_artifact_inventory, SRS-04:continues, proof: ac-2.log-->
- [x] [SRS-04/AC-03] `keel story show <id>` includes readable proof excerpts capped at 10 lines for text proofs, plus missing-proof warnings so acceptance decisions do not require separate file navigation. <!-- verify: cargo test --lib story_show_proof_excerpt_10_lines_and_warnings, SRS-04:end, proof: ac-3.log-->
- [x] [SRS-04/AC-04] [SRS-NFR-02/AC-02] Evidence sections render explicit placeholder text when no proof artifacts exist or when evidence directories are absent. <!-- verify: cargo test --lib story_show_missing_evidence_placeholders, SRS-NFR-02:start:end, proof: ac-4.log-->

#### Implementation Insights
- **L001: Evidence UX Needs Structured Inventory Layers**
  - Insight: A clear split between linked proofs, supplementary artifacts, and media playback hints makes acceptance decisions possible without opening files manually.
  - Suggested Action: Keep evidence rendering model-driven and test each layer (metadata, excerpts, missing warnings, placeholders) independently.
  - Applies To: `src/cli/commands/management/story/show.rs`, future evidence/report renderers
  - Category: architecture


#### Verified Evidence
- [ac-4.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-2.log)

### Implement Epic Show Planning Summary
- **ID:** 1vxppkN0w
- **Status:** done

#### Summary
Upgrade `keel epic show` to render an actionable planning report: authored summary, requirement/verification readiness, artifact visibility, and completion progress with ETA.

#### Acceptance Criteria
- [x] [SRS-01/AC-02] `keel epic show <id>` renders authored problem statement, goals/objectives, and key requirements in a compact planning summary section. <!-- verify: cargo test --lib epic_show_planning_summary, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel epic show <id>` renders progress metrics (voyages/stories complete) plus a time-to-complete estimate derived from a 4-week throughput window with fallback messaging when data is insufficient. <!-- verify: cargo test --lib epic_show_eta_projection_4w, SRS-02:start, proof: ac-2.log-->
- [x] [SRS-02/AC-02] `keel epic show <id>` renders verification readiness including automated/manual requirement coverage and linked artifact inventory (text + media). <!-- verify: cargo test --lib epic_show_verification_surface, SRS-02:continues, proof: ac-3.log-->
- [x] [SRS-02/AC-03] `keel epic show <id>` renders project-aware automated verification recommendations (for example stack-specific tooling suggestions) with rationale tied to detected project signals. <!-- verify: cargo test --lib epic_show_verification_recommendations, SRS-02:end, proof: ac-4.log-->
- [x] [SRS-02/AC-04] [SRS-NFR-02/AC-01] When authored planning sections or evidence are missing, `epic show` prints explicit placeholders/warnings instead of omitting sections. <!-- verify: cargo test --lib epic_show_missing_data_placeholders, SRS-NFR-02:start:end, proof: ac-5.log-->

#### Implementation Insights
- **L001: Planning Show Parsing Needs Scaffold Filters**
  - Insight: Requirement parsing must explicitly ignore scaffold rows like `TODO`/template defaults or placeholder mode appears complete when it is not.
  - Suggested Action: Keep placeholder filters and add fixture tests that assert empty summaries on scaffold-only PRDs.
  - Applies To: `src/cli/commands/management/epic/show.rs`, planning projection parsers
  - Category: code


#### Verified Evidence
- [ac-4.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-1.log)
- [ac-5.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-5.log)
- [ac-3.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-2.log)


