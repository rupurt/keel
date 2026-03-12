---
id: VDcFgruMk
title: Routine CLI Surfaces
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T20:50:27
operator-signal: 
scope: VDakm8eVW/VDcFd11nc
index: 3
started_at: 2026-03-11T20:42:13
completed_at: 2026-03-11T20:50:27
---

# Routine CLI Surfaces

## Summary

Add the minimal CLI authoring and read surfaces that let operators create and
inspect routines without hand-editing board directories.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel routine new` scaffolds a valid routine bundle with required cadence and target-scope fields. <!-- verify: cargo test routine_new_scaffolds_valid_single_bundle_with_opaque_cadence_mapping --bin keel, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] `keel routine list` renders discoverable routine summaries without manual path knowledge. <!-- verify: cargo test routine_list_renders_discoverable_sorted_summaries --bin keel, SRS-03:continues, proof: ac-2.log-->
- [x] [SRS-03/AC-03] `keel routine show <id>` renders cadence, target scope, and blueprint content from canonical storage. <!-- verify: cargo test routine_show_renders_cadence_scope_and_blueprint_from_canonical_storage --bin keel, SRS-03:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] The routine scaffold keeps cadence settings, target scope, and blueprint narrative together in one human-editable artifact. <!-- verify: cargo test routine_new_scaffolds_valid_single_bundle_with_opaque_cadence_mapping --bin keel, SRS-04:start:end, proof: ac-4.log-->
