---
id: 1vzeUJ000
title: Mission Templates And Directory Scaffold
type: feat
status: done
created_at: 2026-03-09T10:33:51
updated_at: 2026-03-09T13:34:39
scope: 1vzeJF000/1vzeMk000
index: 2
started_at: 2026-03-09T13:32:26
completed_at: 2026-03-09T13:34:39
---

# Mission Templates And Directory Scaffold

## Summary

Create mission directory structure and template files. Define .keel/missions/<id>/
layout with README.md, CHARTER.md, and LOG.md templates. CHARTER.md must include
Goals table with MG-XX IDs, Constraints section, and Halting Rules section.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Mission directory created at .keel/missions/<id>/ with README.md, CHARTER.md, and LOG.md <!-- verify: cargo test --lib infrastructure::templates::tests::mission_readme_links_charter_and_log, SRS-05:start, proof: ac-1.log-->
- [x] [SRS-05/AC-02] CHARTER.md scaffold has Goals table with ID, Description, Verification columns <!-- verify: cargo test --lib infrastructure::templates::tests::mission_charter_has_goals_table, proof: ac-2.log-->
- [x] [SRS-05/AC-03] CHARTER.md scaffold has Constraints section <!-- verify: cargo test --lib infrastructure::templates::tests::mission_charter_has_constraints_section, proof: ac-3.log-->
- [x] [SRS-05/AC-04] CHARTER.md scaffold has Halting Rules section <!-- verify: cargo test --lib infrastructure::templates::tests::mission_charter_has_halting_rules_section, SRS-05:end, proof: ac-4.log-->
