//! Backfill missing started_at timestamps for existing board entities.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate, NaiveDateTime};

use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StartedAtBackfillStats {
    pub stories_updated: usize,
    pub voyages_updated: usize,
}

/// Backfill missing `started_at` timestamps.
///
/// - Story `started_at` is set to the midpoint between `created_at` and `submitted_at`.
/// - Voyage `started_at` is set to the earliest `started_at` among scoped stories.
/// - Legacy voyage `started:` keys are removed once `started_at` is present.
pub fn backfill(board_dir: &Path) -> Result<StartedAtBackfillStats> {
    let board = load_board(board_dir)?;
    let mut stats = StartedAtBackfillStats::default();

    // Track synthetic story started_at values we computed in this pass so voyage
    // backfill can consume them without a reload.
    let mut synthetic_story_started: HashMap<String, NaiveDateTime> = HashMap::new();

    for story in board.stories.values() {
        if story.frontmatter.started_at.is_some() {
            continue;
        }

        let Some(created_at) = story.frontmatter.created_at else {
            continue;
        };
        let Some(submitted_at) = story.frontmatter.submitted_at else {
            continue;
        };

        let started_at = midpoint(created_at, submitted_at);
        if write_started_at(&story.path, started_at, false)? {
            stats.stories_updated += 1;
        }
        synthetic_story_started.insert(story.id().to_string(), started_at);
    }

    for voyage in board.voyages.values() {
        let stories = board.stories_for_voyage(voyage);
        let earliest_story_started = stories
            .iter()
            .filter_map(|story| {
                story
                    .frontmatter
                    .started_at
                    .or_else(|| synthetic_story_started.get(story.id()).copied())
            })
            .min();

        let content = fs::read_to_string(&voyage.path)
            .with_context(|| format!("Failed to read voyage file: {}", voyage.path.display()))?;
        let legacy_started = parse_legacy_started(&content);
        let desired_started = voyage
            .frontmatter
            .started_at
            .or(earliest_story_started)
            .or(legacy_started);

        let Some(started_at) = desired_started else {
            continue;
        };

        let should_remove_legacy = legacy_started.is_some();
        if write_started_at(&voyage.path, started_at, should_remove_legacy)? {
            stats.voyages_updated += 1;
        }
    }

    Ok(stats)
}

fn midpoint(a: NaiveDateTime, b: NaiveDateTime) -> NaiveDateTime {
    let (start, end) = if a <= b { (a, b) } else { (b, a) };
    let delta = end - start;
    start + Duration::seconds(delta.num_seconds() / 2)
}

fn write_started_at(
    path: &Path,
    started_at: NaiveDateTime,
    remove_legacy_started: bool,
) -> Result<bool> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let mut mutations = vec![Mutation::set(
        "started_at",
        started_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    )];
    if remove_legacy_started {
        mutations.push(Mutation::remove("started"));
    }

    let updated = apply(&content, &mutations);
    if updated == content {
        return Ok(false);
    }

    fs::write(path, updated)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;
    Ok(true)
}

fn parse_legacy_started(content: &str) -> Option<NaiveDateTime> {
    let mut in_frontmatter = false;
    let mut delimiters = 0usize;

    for line in content.lines() {
        if line == "---" {
            delimiters += 1;
            in_frontmatter = delimiters == 1;
            if delimiters == 2 {
                break;
            }
            continue;
        }

        if !in_frontmatter {
            continue;
        }

        let Some(value) = line.strip_prefix("started:") else {
            continue;
        };
        let value = value.trim();
        if value.is_empty() {
            continue;
        }

        if let Ok(dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S") {
            return Some(dt);
        }

        if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            return date.and_hms_opt(0, 0, 0);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn write_story(board: &Path, id: &str, scope: &str, started_at: Option<&str>) {
        let story_dir = board.join("stories").join(id);
        fs::create_dir_all(&story_dir).unwrap();
        let started_line = started_at
            .map(|v| format!("started_at: {v}\n"))
            .unwrap_or_default();
        fs::write(
            story_dir.join("README.md"),
            format!(
                "---\nid: {id}\ntitle: Story {id}\ntype: feat\nstatus: needs-human-verification\nscope: {scope}\ncreated_at: 2026-03-01T08:00:00\nupdated_at: 2026-03-01T08:00:00\n{submitted}{}---\n\n# Story\n",
                started_line,
                submitted = "submitted_at: 2026-03-01T12:00:00\n"
            ),
        )
        .unwrap();
        fs::write(story_dir.join("REFLECT.md"), "# Reflection\n").unwrap();
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();
    }

    fn write_epic_and_voyage(board: &Path, epic: &str, voyage: &str, started_line: &str) {
        let epic_dir = board.join("epics").join(epic);
        fs::create_dir_all(epic_dir.join("voyages").join(voyage)).unwrap();
        fs::write(
            epic_dir.join("README.md"),
            format!(
                "---\nid: {epic}\ntitle: Epic {epic}\nstatus: tactical\ncreated_at: 2026-03-01T00:00:00\n---\n\n# Epic\n"
            ),
        )
        .unwrap();
        fs::write(
            epic_dir.join("PRD.md"),
            "# PRD\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n<!-- END FUNCTIONAL_REQUIREMENTS -->\n<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->\n<!-- END NON_FUNCTIONAL_REQUIREMENTS -->\n<!-- BEGIN SUCCESS_CRITERIA -->\n<!-- END SUCCESS_CRITERIA -->\n",
        )
        .unwrap();
        fs::write(
            epic_dir.join("voyages").join(voyage).join("README.md"),
            format!(
                "---\nid: {voyage}\ntitle: Voyage {voyage}\nstatus: in-progress\nepic: {epic}\ncreated_at: 2026-03-01T00:00:00\n{started_line}---\n\n# Voyage\n"
            ),
        )
        .unwrap();
    }

    #[test]
    fn backfill_sets_story_started_at_midpoint() {
        let temp = TempDir::new().unwrap();
        write_story(temp.path(), "S1", "e1/v1", None);

        let stats = backfill(temp.path()).unwrap();
        assert_eq!(stats.stories_updated, 1);

        let content = fs::read_to_string(temp.path().join("stories/S1/README.md")).unwrap();
        assert!(content.contains("started_at: 2026-03-01T10:00:00"));
    }

    #[test]
    fn backfill_sets_voyage_started_at_from_earliest_story_and_removes_legacy_key() {
        let temp = TempDir::new().unwrap();
        write_epic_and_voyage(temp.path(), "e1", "v1", "started: 2026-02-20\n");
        write_story(temp.path(), "S1", "e1/v1", Some("2026-03-01T10:00:00"));

        let story_dir = temp.path().join("stories").join("S2");
        fs::create_dir_all(&story_dir).unwrap();
        fs::write(
            story_dir.join("README.md"),
            "---\nid: S2\ntitle: Story S2\ntype: feat\nstatus: in-progress\nscope: e1/v1\ncreated_at: 2026-03-01T08:00:00\nupdated_at: 2026-03-01T08:00:00\nstarted_at: 2026-03-01T09:00:00\n---\n\n# Story\n",
        )
        .unwrap();
        fs::write(story_dir.join("REFLECT.md"), "# Reflection\n").unwrap();
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();

        let stats = backfill(temp.path()).unwrap();
        assert_eq!(stats.voyages_updated, 1);

        let voyage = fs::read_to_string(temp.path().join("epics/e1/voyages/v1/README.md")).unwrap();
        assert!(voyage.contains("started_at: 2026-03-01T09:00:00"));
        assert!(!voyage.contains("\nstarted: "));
    }
}
