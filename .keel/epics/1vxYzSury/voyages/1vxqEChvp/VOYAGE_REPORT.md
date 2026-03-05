# VOYAGE REPORT: Icebox-First Story Intake

## Voyage Metadata
- **ID:** 1vxqEChvp
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 1/1 stories complete

## Implementation Narrative
### Default New Stories To Icebox
- **ID:** 1vxqELhgo
- **Status:** done

#### Summary
Change story creation defaults so all new stories start in `icebox`, with explicit thaw/start guidance and regression tests proving planned-voyage coherence no longer fails on intake.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel story new` creates unscoped and scoped stories with `status: icebox` in persisted frontmatter by default. <!-- verify: cargo test --lib story_new_defaults_to_icebox, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Story creation output includes explicit recovery/next-step guidance for thawing and starting work from the new default stage. <!-- verify: cargo test --lib story_new_icebox_guidance, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] A regression test demonstrates that creating/linking a new story into a planned voyage does not trigger immediate doctor coherence failure due to stage default. <!-- verify: cargo test --lib story_new_planned_voyage_doctor_coherence, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-03/AC-02] [SRS-NFR-01/AC-01] Stage default enforcement is covered by a single canonical creation path test (no alternate backlog default path remains). <!-- verify: cargo test --lib story_new_canonical_stage_path, SRS-NFR-01:start:end, proof: ac-4.log-->

#### Implementation Insights
- **1vyDuwoz6: Intake Stage Must Be Canonical In Creation Code**
  - Insight: Canonical defaults are safest when enforced once in creation code and not left to template/state-condition combinations.
  - Suggested Action: Keep `story new` stage assignment unconditional and test it across unscoped/draft/planned scopes.
  - Applies To: `src/cli/commands/management/story/new.rs`, story template/frontmatter defaults
  - Category: architecture


#### Verified Evidence
- [ac-4.log](../../../../stories/1vxqELhgo/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxqELhgo/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqELhgo/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqELhgo/EVIDENCE/ac-2.log)


