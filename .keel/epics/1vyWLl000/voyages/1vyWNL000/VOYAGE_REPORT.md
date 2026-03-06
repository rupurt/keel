# VOYAGE REPORT: Tape-Driven Dogfood Workflow Suite

## Voyage Metadata
- **ID:** 1vyWNL000
- **Epic:** 1vyWLl000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 5/5 stories complete

## Implementation Narrative
### Build Tape Runner And Reset Harness
- **ID:** 1vyWRj000
- **Status:** done

#### Summary
Build the canonical local entrypoint that resets the secondary workspace, runs named dogfood scenarios, and reports failures without making the suite part of default CI or pre-commit paths.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] A local opt-in runner executes named dogfood scenarios from the secondary workspace and reports actionable failure context. <!-- verify: cargo test -p keel dogfood_runner_executes_named_scenarios, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-03/AC-01] The runner remains absent from default CI and pre-commit workflows. <!-- verify: cargo test -p keel dogfood_runner_is_opt_in_and_not_wired_into_default_checks, SRS-NFR-03:start:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyWX1Qh7: Timebox External Verification Runners And Emit Log Paths**
  - Insight: External verifier processes can hang without producing useful stderr, so the runner must enforce a timeout and always persist a log path or the queue stalls without actionable failure context.
  - Suggested Action: Wrap external verification tools in an explicit timeout, keep the failing workspace/tape/output paths in the error, and write a run log even on failure.
  - Applies To: `src/infrastructure/vhs.rs`, `src/infrastructure/dogfood_runner.rs`, `testdata/dogfood/scenarios/*.tape`
  - Category: testing


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWRj000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWRj000/EVIDENCE/ac-2.log)

### Author Bearing Workflow Dogfood Tapes
- **ID:** 1vyWRk000
- **Status:** done

#### Summary
Author the bearing-phase VHS scenarios so keel can dogfood the research workflow from creation through graduation with the same proof model used for implementation work.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] A tape-driven bearing workflow covers `bearing new`, `bearing survey`, `bearing assess`, and `bearing lay` on the secondary workspace. <!-- verify: bash -lc 'vhs validate testdata/dogfood/scenarios/bearing-flow.tape && cargo test -p keel bearing_flow_tape_covers_research_lifecycle', SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-02] The bearing workflow remains repeatable on the same fixture state. <!-- verify: cargo test -p keel bearing_flow_tape_avoids_fixed_entity_ids, SRS-NFR-01:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyXi6000: Author transition-created bearing artifacts after the lifecycle step**
  - Insight: The bearing lifecycle commands create `SURVEY.md` and `ASSESSMENT.md` themselves, so authoring those files before the transition causes hard failures; the correct flow is transition first, then fill the generated artifact.
  - Suggested Action: In tapes or scripts, treat `bearing survey` and `bearing assess` as the scaffold-creation step, then write authored content into the generated files before continuing.
  - Applies To: `testdata/dogfood/scenarios/bearing-flow.tape`, `templates/bearings/*.md`
  - Category: testing


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWRk000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWRk000/EVIDENCE/ac-2.log)

### Create Secondary Dogfood Workspace
- **ID:** 1vyWSB000
- **Status:** done

#### Summary
Establish a checked-in secondary workspace with its own `.keel` board and deterministic reset path so dogfood runs exercise real workflow state without touching the repository's primary board.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] A checked-in secondary workspace exists with its own `.keel` board and enough authored fixture state to support epic and bearing workflow tapes. <!-- verify: cargo test -p keel dogfood_workspace_scaffold_has_secondary_board, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] The secondary workspace exposes a deterministic reset path. <!-- verify: cargo test -p keel dogfood_workspace_reset_preserves_primary_board, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The deterministic reset path leaves the repository's primary `.keel` board unchanged. <!-- verify: cargo test -p keel dogfood_workspace_reset_preserves_primary_board, SRS-NFR-02:start:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyIq5M2c: Verify Annotation Chains Only Materialize One Requirement Token**
  - Insight: The verify-annotation parser keeps only one requirement phase token per AC, so the last `SRS-*:phase` entry controls voyage evidence-chain checks
  - Suggested Action: Split evidence-chain phases across separate ACs or put the functional requirement token last when a line carries both SRS and SRS-NFR references
  - Applies To: src/infrastructure/verification/parser.rs, .keel/stories/*/README.md
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWSB000/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyWSB000/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyWSB000/EVIDENCE/ac-2.log)

### Author Epic Workflow Dogfood Tapes
- **ID:** 1vyWSC000
- **Status:** done

#### Summary
Author the epic-phase VHS scenarios so keel can dogfood epic creation, voyage/story decomposition, and the steering surfaces agents use to decide what to do next.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] A tape-driven epic workflow covers epic creation, voyage/story decomposition, and the core planning flow on the secondary workspace. <!-- verify: bash -lc 'vhs validate testdata/dogfood/scenarios/epic-flow.tape && cargo test -p keel epic_flow_tape_covers_creation_and_decomposition', SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] The epic workflow surfaces `keel next` and `keel flow` at the steering points needed to guide implementation. <!-- verify: cargo test -p keel epic_flow_tape_surfaces_next_and_flow, SRS-03:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] The epic workflow remains repeatable on the same fixture state. <!-- verify: cargo test -p keel epic_flow_tape_avoids_fixed_entity_ids, SRS-NFR-01:start, proof: ac-3.log-->

#### Implementation Insights
- **1vyXcz000: Use hidden setup blocks and dynamic ID discovery in VHS planning flows**
  - Insight: The readable part of the tape should stay focused on the operator-facing workflow, while markdown authoring and ID plumbing happen in `Hide` blocks using `latest_id` discovery instead of fixed IDs.
  - Suggested Action: Keep visible commands to the user journey, generate authored artifacts in hidden heredocs, and derive IDs from the fixture state after each create step to preserve repeatability.
  - Applies To: `testdata/dogfood/scenarios/*.tape`, `src/infrastructure/dogfood_runner.rs`
  - Category: testing


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWSC000/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyWSC000/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyWSC000/EVIDENCE/ac-2.log)

### Link Tape Evidence Into Verification Manifests
- **ID:** 1vyWSD000
- **Status:** done

#### Summary
Close the loop between tape execution and keel's proof model by storing rendered artifacts, companion transcripts, and manifest hashes under a dedicated dogfood artifact board whose stories own each scenario.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Dogfood runs persist rendered VHS outputs and companion transcript/log artifacts under story `EVIDENCE/` and record them in verification manifests. <!-- verify: cargo test -p keel dogfood_vhs_evidence_enters_manifest, SRS-05:start:end, proof: ac-1.log-->
- [x] [SRS-06/AC-01] Dogfood planning artifacts and story annotations document the tape/transcript/manifest proof chain clearly enough for `voyage plan` and `keel doctor` to pass. <!-- verify: just keel doctor, SRS-06:start:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyYIj000: Dogfood Evidence Needs Its Own Board**
  - Insight: Persisting tape artifacts into the primary `.keel` or the disposable scenario workspace creates contract drift: the primary board stops being immutable, while the resettable workspace loses durable proof ownership. A separate artifact board keeps ownership, manifests, and evidence stable without polluting the runtime board.
  - Suggested Action: For future dogfood flows, separate execution state from evidence ownership. Keep the executable workspace resettable and route rendered artifacts into a dedicated keel board whose stories reference the canonical scenario sources.
  - Applies To: testdata/dogfood/**, src/infrastructure/dogfood_*, src/infrastructure/verification/**
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWSD000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWSD000/EVIDENCE/ac-2.log)


