# VOYAGE REPORT: Bearing Contract Cutover and Migration

## Voyage Metadata
- **ID:** 1vzQtq000
- **Epic:** 1vzQpr000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Replace Bearing Scaffolds With The Evidence Document Contract
- **ID:** 1vzQwj000
- **Status:** done

#### Summary
Replace the bearing scaffolds so the canonical document bundle becomes `BRIEF.md` for framing, `EVIDENCE.md` for cited research capture, and `ASSESSMENT.md` for synthesis, with no remaining survey-era scaffold output.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel bearing new` scaffolds `README.md`, `BRIEF.md`, `EVIDENCE.md`, and `ASSESSMENT.md`, and generated document references omit `SURVEY.md`. <!-- verify: cargo test -p keel bearing_scaffold_uses_evidence_document_contract, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] `BRIEF.md` is framing-only while `EVIDENCE.md` owns cited research capture structure, so the templates no longer duplicate findings across both documents. <!-- verify: cargo test -p keel bearing_brief_and_evidence_templates_have_distinct_responsibilities, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] [SRS-NFR-01/AC-01] No supported scaffold or generator path in scope continues to emit survey-era document names or compatibility aliases. <!-- verify: cargo test -p keel bearing_scaffold_contract_has_no_survey_aliases, SRS-NFR-01:start:end, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzQwj000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzQwj000/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/1vzQwj000/EVIDENCE/ac-3.log)

### Cut Over Bearing Lifecycle Commands And Guidance To Research Language
- **ID:** 1vzQwk000
- **Status:** done

#### Summary
Cut the CLI and guidance surfaces over from survey semantics to research semantics so bearing operators see one canonical command path and one consistent lifecycle vocabulary.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Bearing lifecycle commands, clap help, and runtime dispatch replace the survey-era command path with one canonical research command path. <!-- verify: cargo test -p keel bearing_research_command_contract, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Generated next-step guidance, command examples, and user-facing docs use research language consistently for the evidence stage. <!-- verify: cargo test -p keel bearing_research_guidance_and_docs_are_consistent, SRS-02:continues, proof: ac-2.log-->
- [x] [SRS-02/AC-03] [SRS-NFR-02/AC-01] Equivalent bearing states produce deterministic research-stage guidance and migration messages across human-readable and JSON command output. <!-- verify: cargo test -p keel bearing_research_guidance_is_deterministic, SRS-NFR-02:start:end, SRS-02:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzQwk000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzQwk000/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/1vzQwk000/EVIDENCE/ac-3.log)

### Enforce Hard-Cutover Validation And Migrate Bearing Fixtures
- **ID:** 1vzQwl000
- **Status:** done

#### Summary
Fail fast on legacy survey-era bearing artifacts and migrate in-repo fixtures and example boards so the hard cutover leaves no supported workflow or test data on the old contract.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel doctor` and readiness checks fail bearings that still rely on `SURVEY.md` or otherwise violate the new framing/evidence/assessment contract, with actionable recovery guidance. <!-- verify: cargo test -p keel bearing_doctor_rejects_legacy_survey_contract, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Fixture boards, generated examples, and test helpers in scope are migrated so supported tests and workflows no longer depend on survey-era artifacts or commands. <!-- verify: cargo test -p keel bearing_fixture_boards_use_evidence_contract, SRS-04:start, proof: ac-2.log-->
- [x] [SRS-04/AC-02] [SRS-NFR-01/AC-02] Legacy survey-era paths fail hard without compatibility aliases in loaders, validators, or lifecycle transitions. <!-- verify: cargo test -p keel bearing_hard_cutover_rejects_legacy_survey_paths, SRS-NFR-01:start:end, SRS-04:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzQwl000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzQwl000/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/1vzQwl000/EVIDENCE/ac-3.log)


