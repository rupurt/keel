//! Canonical throughput history projection for diagnostics rendering.

use chrono::{Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::model::Board;

pub const THROUGHPUT_HISTORY_SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_HISTORY_WEEKS: usize = 12;

/// Persistable snapshot containing weekly throughput and timing aggregates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThroughputHistory {
    pub schema_version: u32,
    /// Most recent week first.
    pub weekly: Vec<WeeklyThroughputBucket>,
}

/// Weekly aggregate bucket used by throughput sparkline rendering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeeklyThroughputBucket {
    /// Week start date (Monday).
    pub week_start: NaiveDate,
    /// Stories reaching done this week.
    pub stories_done: usize,
    /// Voyages reaching done this week.
    pub voyages_done: usize,
    /// Story cycle time (start -> submit/done), min value in hours.
    pub cycle_min_hours: Option<f64>,
    /// Story cycle time (start -> submit/done), median value in hours.
    pub cycle_median_hours: Option<f64>,
    /// Story cycle time (start -> submit/done), max value in hours.
    pub cycle_max_hours: Option<f64>,
    /// Story wait-for-acceptance time (submit -> done), median in hours.
    pub acceptance_wait_median_hours: Option<f64>,
}

/// Build the default rolling weekly projection anchored to the current UTC date.
pub fn project_default(board: &Board) -> ThroughputHistory {
    project(board, Utc::now().date_naive(), DEFAULT_HISTORY_WEEKS)
}

/// Build a rolling weekly projection anchored to the current UTC date.
pub fn project_recent(board: &Board, weeks: usize) -> ThroughputHistory {
    project(board, Utc::now().date_naive(), weeks)
}

/// Build weekly throughput history aggregates from the board.
pub fn project(board: &Board, anchor_date: NaiveDate, weeks: usize) -> ThroughputHistory {
    let week_starts = generate_week_starts(anchor_date, weeks);

    let weekly = week_starts
        .into_iter()
        .map(|week_start| bucket_for_week(board, week_start))
        .collect();

    ThroughputHistory {
        schema_version: THROUGHPUT_HISTORY_SCHEMA_VERSION,
        weekly,
    }
}

fn bucket_for_week(board: &Board, week_start: NaiveDate) -> WeeklyThroughputBucket {
    let week_end = week_start + Duration::days(7);

    let stories_done = board
        .stories
        .values()
        .filter(|story| {
            in_week(
                story.frontmatter.completed_at.map(|dt| dt.date()),
                week_start,
                week_end,
            )
        })
        .count();

    let voyages_done = board
        .voyages
        .values()
        .filter(|voyage| {
            in_week(
                voyage.frontmatter.completed_at.map(|dt| dt.date()),
                week_start,
                week_end,
            )
        })
        .count();

    let mut cycle_hours = Vec::new();
    let mut acceptance_wait_hours = Vec::new();

    for story in board.stories.values() {
        if let Some(started) = story.frontmatter.started_at {
            let cycle_end = story
                .frontmatter
                .submitted_at
                .or(story.frontmatter.completed_at);
            if let Some(end) = cycle_end
                && end >= started
                && in_week(Some(end.date()), week_start, week_end)
            {
                cycle_hours.push((end - started).num_seconds() as f64 / 3600.0);
            }
        }

        if let (Some(submitted), Some(completed)) = (
            story.frontmatter.submitted_at,
            story.frontmatter.completed_at,
        ) && completed >= submitted
            && in_week(Some(completed.date()), week_start, week_end)
        {
            acceptance_wait_hours.push((completed - submitted).num_seconds() as f64 / 3600.0);
        }
    }

    let cycle_min_hours = min_value(&cycle_hours);
    let cycle_median_hours = median(&cycle_hours);
    let cycle_max_hours = max_value(&cycle_hours);
    let acceptance_wait_median_hours = median(&acceptance_wait_hours);

    WeeklyThroughputBucket {
        week_start,
        stories_done,
        voyages_done,
        cycle_min_hours,
        cycle_median_hours,
        cycle_max_hours,
        acceptance_wait_median_hours,
    }
}

fn in_week(date: Option<NaiveDate>, week_start: NaiveDate, week_end: NaiveDate) -> bool {
    date.map(|d| d >= week_start && d < week_end)
        .unwrap_or(false)
}

fn min_value(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(f64::min)
}

fn max_value(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(f64::max)
}

fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

fn start_of_week_monday(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

fn generate_week_starts(from: NaiveDate, weeks: usize) -> Vec<NaiveDate> {
    let mut result = Vec::with_capacity(weeks);
    let mut current = start_of_week_monday(from);
    for _ in 0..weeks {
        result.push(current);
        current -= Duration::days(7);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{
        Board, Story, StoryFrontmatter, StoryState, StoryType, Voyage, VoyageFrontmatter,
        VoyageState,
    };
    use std::path::PathBuf;

    fn make_story(
        id: &str,
        started_at: &str,
        submitted_at: Option<&str>,
        completed_at: Option<&str>,
    ) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: id.to_string(),
                title: format!("Story {id}"),
                story_type: StoryType::Feat,
                status: StoryState::Done,
                scope: None,
                milestone: None,
                created_at: None,
                updated_at: None,
                started_at: Some(parse_dt(started_at)),
                completed_at: completed_at.map(parse_dt),
                submitted_at: submitted_at.map(parse_dt),
                index: None,
                governed_by: vec![],
                role: None,
            },
            path: PathBuf::from(format!("{id}.md")),
            stage: StoryState::Done,
        }
    }

    fn make_voyage(id: &str, completed_at: Option<&str>) -> Voyage {
        Voyage {
            frontmatter: VoyageFrontmatter {
                id: id.to_string(),
                title: format!("Voyage {id}"),
                goal: None,
                status: VoyageState::Done,
                epic: Some("epic-1".to_string()),
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: completed_at.map(parse_dt),
            },
            path: PathBuf::from(format!("{id}/README.md")),
            epic_id: "epic-1".to_string(),
        }
    }

    fn parse_dt(raw: &str) -> chrono::NaiveDateTime {
        chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%dT%H:%M:%S").unwrap()
    }

    #[test]
    fn project_computes_weekly_story_and_voyage_throughput() {
        let mut board = Board::new(PathBuf::from(".keel"));
        let anchor = NaiveDate::from_ymd_opt(2026, 3, 2).unwrap();
        let week_start = start_of_week_monday(anchor);

        let completed_this_week = week_start
            .and_hms_opt(12, 0, 0)
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();

        board.stories.insert(
            "s1".to_string(),
            make_story(
                "s1",
                "2026-01-01T08:00:00",
                Some(&completed_this_week),
                Some(&completed_this_week),
            ),
        );
        board.voyages.insert(
            "v1".to_string(),
            make_voyage("v1", Some(&completed_this_week)),
        );

        let history = project(&board, anchor, 4);

        assert_eq!(history.weekly.len(), 4);
        assert_eq!(history.weekly[0].stories_done, 1);
        assert_eq!(history.weekly[0].voyages_done, 1);
    }

    #[test]
    fn project_uses_submit_time_for_cycle_and_tracks_acceptance_wait() {
        let mut board = Board::new(PathBuf::from(".keel"));
        let anchor = NaiveDate::from_ymd_opt(2026, 3, 2).unwrap();
        let week_start = start_of_week_monday(anchor);

        let started = week_start.and_hms_opt(9, 0, 0).unwrap();
        let submitted = started + Duration::hours(10);
        let completed = submitted + Duration::hours(5);

        board.stories.insert(
            "s1".to_string(),
            make_story(
                "s1",
                &started.format("%Y-%m-%dT%H:%M:%S").to_string(),
                Some(&submitted.format("%Y-%m-%dT%H:%M:%S").to_string()),
                Some(&completed.format("%Y-%m-%dT%H:%M:%S").to_string()),
            ),
        );

        let history = project(&board, anchor, 1);
        let bucket = &history.weekly[0];

        assert_eq!(bucket.cycle_min_hours, Some(10.0));
        assert_eq!(bucket.cycle_median_hours, Some(10.0));
        assert_eq!(bucket.cycle_max_hours, Some(10.0));
        assert_eq!(bucket.acceptance_wait_median_hours, Some(5.0));
    }
}
