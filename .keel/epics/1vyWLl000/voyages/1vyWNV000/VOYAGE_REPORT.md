# VOYAGE REPORT: Artifact-Aware Judge Contract

## Voyage Metadata
- **ID:** 1vyWNV000
- **Epic:** 1vyWLl000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Build Artifact Bundle Materialization
- **ID:** 1vyWRl000
- **Status:** done

#### Summary
Implement bundle materialization so the verification executor can package a story's evidence into the canonical artifact-judge input before invoking any external semantic judge.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] The verification executor materializes the artifact bundle from story evidence before `llm-judge` runs. <!-- verify: cargo test -p keel verification_executor_materializes_judge_bundle, SRS-02:start:end, proof: ac-1.log-->

#### Implementation Insights
- **1vyYK1g00: Judge Bundles Should Carry References And Hashes**
  - Insight: The stable contract is metadata, normalized evidence references, and hashes, not embedded artifact contents or provider-specific fields. That keeps bundle serialization deterministic while leaving transport, prompting, and artifact loading to the external judge wrapper.
  - Suggested Action: Keep the bundle as a control-plane document: normalize proof refs into canonical `EVIDENCE/...` paths, sort the evidence inventory, and defer provider-specific payload shaping until the external `llm-judge` boundary.
  - Applies To: src/infrastructure/verification/judge_bundle.rs, src/infrastructure/verification/executor.rs, story record/verify judge integration
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWRl000/EVIDENCE/ac-1.log)

### Persist Judge Outputs In Verification Evidence
- **ID:** 1vyWRm000
- **Status:** done

#### Summary
Persist semantic judge outputs as normal story evidence so `verify run` and `story record --judge` both produce auditable transcripts and leave failed judge runs inspectable.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] `keel verify run` and `keel story record --judge` persist judge transcripts/results as evidence and report failures against the evaluated acceptance criterion. <!-- verify: cargo test -p keel judge_results_persist_as_story_evidence, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-03/AC-01] Failed judge runs preserve the artifact bundle and transcript/debug outputs for manual inspection. <!-- verify: cargo test -p keel judge_results_persist_as_story_evidence, SRS-NFR-03:start:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyYK1g00: Judge Bundles Should Carry References And Hashes**
  - Insight: The stable contract is metadata, normalized evidence references, and hashes, not embedded artifact contents or provider-specific fields. That keeps bundle serialization deterministic while leaving transport, prompting, and artifact loading to the external judge wrapper.
  - Suggested Action: Keep the bundle as a control-plane document: normalize proof refs into canonical `EVIDENCE/...` paths, sort the evidence inventory, and defer provider-specific payload shaping until the external `llm-judge` boundary.
  - Applies To: src/infrastructure/verification/judge_bundle.rs, src/infrastructure/verification/executor.rs, story record/verify judge integration
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWRm000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWRm000/EVIDENCE/ac-2.log)

### Define Artifact Judge Bundle Contract
- **ID:** 1vyWSF000
- **Status:** done

#### Summary
Define the machine-readable artifact bundle that semantic judges will consume so tape-driven evidence can be evaluated without tying keel to any one model provider.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] The artifact bundle schema captures story metadata, acceptance-criterion text, and references to tape-driven evidence artifacts needed for judging. <!-- verify: cargo test -p keel artifact_judge_bundle_schema_captures_story_context, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-01] The artifact bundle schema serializes deterministically for equivalent inputs. <!-- verify: cargo test -p keel artifact_judge_bundle_schema_captures_story_context, SRS-NFR-01:start:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyYK1g00: Judge Bundles Should Carry References And Hashes**
  - Insight: The stable contract is metadata, normalized evidence references, and hashes, not embedded artifact contents or provider-specific fields. That keeps bundle serialization deterministic while leaving transport, prompting, and artifact loading to the external judge wrapper.
  - Suggested Action: Keep the bundle as a control-plane document: normalize proof refs into canonical `EVIDENCE/...` paths, sort the evidence inventory, and defer provider-specific payload shaping until the external `llm-judge` boundary.
  - Applies To: src/infrastructure/verification/judge_bundle.rs, src/infrastructure/verification/executor.rs, story record/verify judge integration
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWSF000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWSF000/EVIDENCE/ac-2.log)

### Wire Provider Agnostic Llm Judge Execution
- **ID:** 1vyWSG000
- **Status:** done

#### Summary
Replace the current diff-only judge stub with an external contract that accepts an artifact bundle path, leaving provider-specific prompting and transport outside the keel crate.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `llm-judge` is invoked through a provider-agnostic external contract that receives an artifact-bundle path instead of relying on `git diff` text alone. <!-- verify: cargo test -p keel llm_judge_uses_artifact_bundle_contract, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-02/AC-01] The judge integration introduces no vendor-specific SDK or transport dependency into keel. <!-- verify: cargo test -p keel llm_judge_uses_artifact_bundle_contract, SRS-NFR-02:start:end, proof: ac-2.log-->

#### Implementation Insights
- **1vyYK1g00: Judge Bundles Should Carry References And Hashes**
  - Insight: The stable contract is metadata, normalized evidence references, and hashes, not embedded artifact contents or provider-specific fields. That keeps bundle serialization deterministic while leaving transport, prompting, and artifact loading to the external judge wrapper.
  - Suggested Action: Keep the bundle as a control-plane document: normalize proof refs into canonical `EVIDENCE/...` paths, sort the evidence inventory, and defer provider-specific payload shaping until the external `llm-judge` boundary.
  - Applies To: src/infrastructure/verification/judge_bundle.rs, src/infrastructure/verification/executor.rs, story record/verify judge integration
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWSG000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWSG000/EVIDENCE/ac-2.log)


