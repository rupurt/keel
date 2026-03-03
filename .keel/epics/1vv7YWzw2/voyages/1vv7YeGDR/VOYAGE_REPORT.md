# VOYAGE REPORT: Schema Hardening and Cleanup

## Voyage Metadata
- **ID:** 1vv7YeGDR
- **Epic:** 1vv7YWzw2
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Enforce Strict Datetime Parsing in All Frontmatter
- **ID:** 1vv7YqiJv
- **Status:** done

#### Summary
Enforce strict datetime parsing for all timestamp fields, removing support for date-only formats.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Update `src/model/mod.rs` to remove `deserialize_flexible_datetime` and use `deserialize_strict_datetime` everywhere. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Update all tests to use the strict `YYYY-MM-DDTHH:MM:SS` format. <!-- verify: manual, SRS-02:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vv7YqiJv/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vv7YqiJv/EVIDENCE/ac-2.log)

### Remove Legacy Migration Fixes from Doctor
- **ID:** 1vv7Yqwi3
- **Status:** done

#### Summary
Remove the code responsible for auto-fixing legacy frontmatter formats in `keel doctor`.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Delete `migrate_story_frontmatter` and `migrate_voyage_readme` from `src/commands/diagnostics/doctor/fixes.rs`. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Update `src/commands/diagnostics/doctor/checks/stories.rs` to report errors instead of warnings for legacy fields. <!-- verify: manual, SRS-01:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vv7Yqwi3/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vv7Yqwi3/EVIDENCE/ac-2.log)

### Purge Unused Compatibility Fields from Model Structs
- **ID:** 1vv7YqxLW
- **Status:** done

#### Summary
Remove deprecated and compatibility fields from the core data models.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Remove `priority` and `depends` from `StoryFrontmatter` in `src/model/story.rs`. <!-- verify: manual, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Remove `#[serde(alias = "created")]` from all `created_at` fields. <!-- verify: manual, SRS-03:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vv7YqxLW/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vv7YqxLW/EVIDENCE/ac-2.log)


