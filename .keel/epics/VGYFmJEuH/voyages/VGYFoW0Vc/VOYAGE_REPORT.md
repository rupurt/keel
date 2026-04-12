# VOYAGE REPORT: Plan The Janitor Handoff And GitHub Connector Bridge

## Voyage Metadata
- **ID:** VGYFoW0Vc
- **Epic:** VGYFmJEuH
- **Status:** done
- **Goal:** Define the first executable contract for Keeper-managed janitor stewardship over Keel and the GitHub connector flow it depends on.

## Execution Summary
**Progress:** 1/1 stories complete

## Implementation Narrative
### Specify The Janitor Handoff And GitHub Connector Contract
- **ID:** VGYFpAdlQ
- **Status:** done

#### Summary
Define the first explicit handoff between Spoke Keeper and Keel so Keeper can
run janitor posture over a bound board, choose an allowed Keel board role per
action, and route GitHub maintenance work without direct `.keel` mutation.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] The story defines a custody context that records Keeper identity/provenance, janitor posture, and selected Keel board role separately. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] The story defines the janitor automation envelope and human escalation boundary across the turn loop. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] The story defines the GitHub connector ingress/egress contract and provider acknowledgement path for janitor stewardship. <!-- verify: manual, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] The story names the first `keel` and `spoke` surfaces that must change to land the handoff. <!-- verify: manual, SRS-04:start:end, proof: ac-4.log-->
- [x] [SRS-NFR-01/AC-01] The story keeps the handoff deterministic and replayable across retries and provider re-delivery. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-5.log-->
- [x] [SRS-NFR-02/AC-01] The story preserves an explicit distinction between Keeper posture and Keel board-role routing. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-6.log-->
- [x] [SRS-NFR-03/AC-01] The story isolates GitHub-specific logic to connector ingress/egress so later providers can reuse the same janitor custody model. <!-- verify: manual, SRS-NFR-03:start:end, proof: ac-7.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-4.log)
- [ac-5.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-5.log)
- [ac-6.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-6.log)
- [ac-7.log](../../../../stories/VGYFpAdlQ/EVIDENCE/ac-7.log)


