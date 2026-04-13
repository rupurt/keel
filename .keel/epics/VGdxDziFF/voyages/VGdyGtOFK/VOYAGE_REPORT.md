# VOYAGE REPORT: Specify Stack-Aware Turn Next And Doctor Contracts

## Voyage Metadata
- **ID:** VGdyGtOFK
- **Epic:** VGdxDziFF
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 1/1 stories complete

## Implementation Narrative
### Add Stack-Aware Turn Next Mission Next And Doctor Projections
- **ID:** VGdyhczct
- **Status:** done

#### Summary
Define how Mission Stack state appears in the canonical operator commands so a
member repo can tell whether it may act, why it is blocked, and how that stack
state differs from the repo-local heartbeat contract.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel turn` requirements describe stack id, branch, local member role, stack mode, and checkpoint status. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel next` requirements describe stack-blocked, yield, or continue-local outcomes for gated repos. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] `keel mission next --status` requirements describe linked member missions, negotiations, or waiting receipts. <!-- verify: manual, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] `keel doctor` requirements describe Mission Stack violations such as wrong branch or missing checkpoint acknowledgment. <!-- verify: manual, SRS-04:start:end, proof: ac-4.log-->
- [x] [SRS-05/AC-01] The command contract covers both text and JSON output expectations. <!-- verify: manual, SRS-05:start:end, proof: ac-5.log-->
- [x] [SRS-NFR-02/AC-01] The stack-aware surface contract explicitly preserves repo-local heartbeat semantics. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-6.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VGdyhczct/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VGdyhczct/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VGdyhczct/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/VGdyhczct/EVIDENCE/ac-4.log)
- [ac-5.log](../../../../stories/VGdyhczct/EVIDENCE/ac-5.log)
- [ac-6.log](../../../../stories/VGdyhczct/EVIDENCE/ac-6.log)


