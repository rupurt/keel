# Schema Hardening and Cleanup - System Design Document

> Remove legacy migration fixes from doctor and ensure pure canonical schema usage.

**Epic:** [1vv7YWzw2](../../README.md) | **SRS:** [SRS.md](SRS.md)

## System Overview

This voyage hardens the system by removing support for legacy data formats and ensuring that only the canonical schema is used.

## Components

### Doctor Cleanup
- Remove `migrate_story_frontmatter` and `migrate_voyage_readme` from `src/commands/diagnostics/doctor/fixes.rs`.
- Update `doctor` checks to error on non-canonical fields instead of offering to fix them.

### Strict Parsing
- Update `src/model/mod.rs` to enforce strict RFC3339-like datetime strings.
- Remove flexible parsing that allowed date-only strings.

### Model Hardening
- Remove `priority` and `depends` fields from `StoryFrontmatter` in `src/model/story.rs`.
- Remove `created` alias from `created_at` fields.

## Constraints & Considerations

- This is a "hard" cutover. Existing boards that haven't been migrated will fail to parse.
- Ensure that all tests are updated to use the canonical format.
