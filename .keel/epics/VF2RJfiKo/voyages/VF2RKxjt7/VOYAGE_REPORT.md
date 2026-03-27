# VOYAGE REPORT: Downstream Adoption And Upgrade Docs

## Voyage Metadata
- **ID:** VF2RKxjt7
- **Epic:** VF2RJfiKo
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Document Downstream Project Engine Contracts
- **ID:** VF2SGZd4b
- **Status:** done

#### Summary
Document how downstream repositories use `AGENTS.md` and `INSTRUCTIONS.md` to make Keel the active project-management engine, and use `port` to show the concrete seams between upstream canonical guidance and local adaptation.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] A workflow page explains how `AGENTS.md` and `INSTRUCTIONS.md` act as the downstream operating contract when a repository adopts Keel as its project-management engine. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] The page uses `port` to show what remains canonical from upstream Keel and what gets adapted inside a downstream project. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The guidance stays vendor-neutral by focusing on repository contracts, command surfaces, and operating patterns rather than any single harness provider. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF2SGZd4b/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF2SGZd4b/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF2SGZd4b/EVIDENCE/ac-3.log)

### Document Keel Upgrade And Upstream Instruction Sync
- **ID:** VF2SHEMJo
- **Status:** done

#### Summary
Document the maintenance path for downstream repositories: how to upgrade Keel, diff upstream instruction files, reapply local adaptations, and validate that the project-management engine still matches the repo.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] A workflow page explains how to upgrade Keel and sync upstream instruction changes while preserving repo-specific wrappers, proof contracts, and validation loops. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF2SHEMJo/EVIDENCE/ac-1.log)

### Integrate Downstream Workflow Docs Into Public Navigation
- **ID:** VF2SI9kho
- **Status:** done

#### Summary
Integrate the new downstream adoption and upgrade pages into the public docs information architecture, including adjacent workflow cross-links and the existing visual language.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] The new downstream workflow pages are added to the public docs navigation and linked from adjacent onboarding or workflow pages. <!-- verify: manual, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-01] The pages reuse the existing public docs visual language and components so they read as part of the product docs rather than as an appendix. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF2SI9kho/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF2SI9kho/EVIDENCE/ac-2.log)


