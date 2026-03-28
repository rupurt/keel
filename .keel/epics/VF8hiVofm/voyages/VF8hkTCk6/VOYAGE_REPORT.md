# VOYAGE REPORT: Canonical Command Catalog And CLI Taxonomy

## Voyage Metadata
- **ID:** VF8hkTCk6
- **Epic:** VF8hiVofm
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 2/2 stories complete

## Implementation Narrative
### Define Canonical CLI Command Catalog
- **ID:** VF8hnQirj
- **Status:** done

#### Summary
Define the static catalog that describes Keel's public command surface so later help, turn, scene, and routing features can depend on one authoritative taxonomy.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] A canonical command descriptor set covers the public CLI commands with family, capability, turn-phase, docs-slug, and scene-support metadata. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-02/AC-01] Catalog ordering and descriptor values are covered by deterministic tests. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF8hnQirj/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF8hnQirj/EVIDENCE/ac-2.log)

### Drive Help Text And Capability Guidance From Catalog
- **ID:** VF8hnTVs9
- **Status:** done

#### Summary
Cut the existing help-group and capability-classification logic over to the canonical catalog so the CLI stops teaching one taxonomy while code uses another, and make the catalog's scene-support metadata queryable for later voyages.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] The help-family rendering is generated from the canonical command catalog rather than a separate hard-coded narrative block. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Capability guidance classification is resolved from the canonical command metadata rather than an independent enum map. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] Scene-capable commands are queryable from the catalog without maintaining a separate hard-coded scene list. <!-- verify: manual, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-NFR-01/AC-01] Public command names and family vocabulary remain stable after the cutover. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-4.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF8hnTVs9/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF8hnTVs9/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF8hnTVs9/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/VF8hnTVs9/EVIDENCE/ac-4.log)


