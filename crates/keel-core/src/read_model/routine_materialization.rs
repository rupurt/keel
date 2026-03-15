//! Shared routine materialization read model.
//!
//! Centralizes the canonical per-window materialization marker so pulse and
//! flow can both reason about exact-once routine state without duplicating
//! body scanning logic.

use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use chrono::{DateTime, SecondsFormat, Utc};

use crate::domain::model::Board;
use crate::read_model::scheduled_routines::{ScheduledRoutineProjection, ScheduledRoutineState};

const MATERIALIZATION_MARKER_PREFIX: &str = "<!-- keel:pulse-materialization: ";
const MATERIALIZATION_MARKER_SUFFIX: &str = " -->";

/// Build the canonical materialization key for one routine due window.
pub fn materialization_key(routine_id: &str, next_eligible_at: DateTime<Utc>) -> String {
    format!(
        "{}@{}",
        routine_id,
        next_eligible_at.to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

/// Build the materialization key for a scheduled due routine projection.
pub fn projection_materialization_key(routine: &ScheduledRoutineProjection) -> Option<String> {
    matches!(routine.state, ScheduledRoutineState::Due)
        .then(|| {
            routine
                .next_eligible_at
                .map(|next_eligible_at| materialization_key(&routine.id, next_eligible_at))
        })
        .flatten()
}

/// Render the canonical body marker stored in materialized stories.
pub fn materialization_marker(materialization_key: &str) -> String {
    format!("{MATERIALIZATION_MARKER_PREFIX}{materialization_key}{MATERIALIZATION_MARKER_SUFFIX}")
}

/// Extract one canonical materialization key from a story body.
pub fn extract_materialization_key(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let key = line
            .strip_prefix(MATERIALIZATION_MARKER_PREFIX)?
            .strip_suffix(MATERIALIZATION_MARKER_SUFFIX)?;
        Some(key.to_string())
    })
}

/// Scan the board for existing routine materializations keyed by due window.
pub fn existing_materializations(board: &Board) -> Result<HashMap<String, String>> {
    let mut stories: Vec<_> = board.stories.values().collect();
    stories.sort_by_key(|story| story.id());

    let mut materialized = HashMap::new();
    for story in stories {
        let content = match fs::read_to_string(&story.path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        if let Some(key) = extract_materialization_key(&content) {
            materialized
                .entry(key)
                .or_insert_with(|| story.id().to_string());
        }
    }

    Ok(materialized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use chrono::TimeZone;

    #[test]
    fn materialization_key_uses_canonical_timestamp_format() {
        let key = materialization_key(
            "routine-due",
            Utc.with_ymd_and_hms(2026, 1, 12, 17, 0, 0).unwrap(),
        );

        assert_eq!(key, "routine-due@2026-01-12T17:00:00Z");
    }

    #[test]
    fn extract_materialization_key_reads_canonical_marker() {
        let content =
            "<!-- keel:pulse-materialization: routine-due@2026-01-12T17:00:00Z -->\n# Story\n";

        assert_eq!(
            extract_materialization_key(content).as_deref(),
            Some("routine-due@2026-01-12T17:00:00Z")
        );
    }

    #[test]
    fn existing_materializations_collects_materialized_story_ids() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1").body(
                    "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] planned\n\n<!-- keel:pulse-materialization: routine-due@2026-01-12T17:00:00Z -->",
                ),
            )
            .story(TestStory::new("S2"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let materialized = existing_materializations(&board).unwrap();

        assert_eq!(
            materialized.get("routine-due@2026-01-12T17:00:00Z"),
            Some(&"S1".to_string())
        );
        assert_eq!(materialized.len(), 1);
    }
}
