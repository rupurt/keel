//! Throughput calculations from historical data
//!
//! Planned feature for flow bottleneck visualization (voyage 16).
//! Currently used by tests; will be wired to commands later.

use chrono::{Datelike, Local, NaiveDate};

use crate::domain::model::Board;

/// Throughput metrics over time
#[allow(dead_code)] // Planned feature (voyage 16)
#[derive(Debug, Default)]
pub struct Throughput {
    /// Stories completed per week (most recent first)
    pub stories_per_week: Vec<WeekThroughput>,
    /// Bearings laid per month (most recent first)
    pub bearings_per_month: Vec<MonthThroughput>,
    /// Average stories per week over the period
    pub avg_stories_per_week: f64,
    /// Average bearings per month over the period
    pub avg_bearings_per_month: f64,
}

/// Stories completed in a specific week
#[allow(dead_code)] // Planned feature (voyage 16)
#[derive(Debug)]
pub struct WeekThroughput {
    /// Start of the week (Monday)
    pub week_start: NaiveDate,
    /// Number of stories completed
    pub count: usize,
}

/// Bearings laid in a specific month
#[allow(dead_code)] // Planned feature (voyage 16)
#[derive(Debug)]
pub struct MonthThroughput {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Number of bearings laid
    pub count: usize,
}

/// Calculate throughput metrics for the given number of weeks
#[allow(dead_code)] // Planned feature (voyage 16)
pub fn calculate_throughput(board: &Board, weeks: usize) -> Throughput {
    let today = Local::now().date_naive();
    let mut throughput = Throughput::default();
    let current_week_start = start_of_week_monday(today);

    // Calculate story throughput per week
    let week_starts = generate_week_starts(today, weeks);
    for week_start in &week_starts {
        let week_end = *week_start + chrono::Duration::days(7);
        let count = board
            .stories
            .values()
            .filter(|s| {
                s.frontmatter
                    .completed_at
                    .map(|dt| {
                        let d = dt.date();
                        d >= *week_start && d < week_end
                    })
                    .unwrap_or(false)
            })
            .count();

        throughput.stories_per_week.push(WeekThroughput {
            week_start: *week_start,
            count,
        });
    }

    // Calculate average stories per week
    if !throughput.stories_per_week.is_empty() {
        let total: usize = throughput.stories_per_week.iter().map(|w| w.count).sum();
        let denominator = effective_story_weeks(board, weeks, current_week_start);
        if denominator > 0 {
            throughput.avg_stories_per_week = total as f64 / denominator as f64;
        }
    }

    // Calculate bearing throughput per month (last 6 months)
    let months = generate_months(today, 6);
    for (year, month) in &months {
        let count = board
            .bearings
            .values()
            .filter(|b| {
                b.frontmatter
                    .laid_at
                    .map(|d| d.year() == *year && d.month() == *month)
                    .unwrap_or(false)
            })
            .count();

        throughput.bearings_per_month.push(MonthThroughput {
            year: *year,
            month: *month,
            count,
        });
    }

    // Calculate average bearings per month
    if !throughput.bearings_per_month.is_empty() {
        let total: usize = throughput.bearings_per_month.iter().map(|m| m.count).sum();
        throughput.avg_bearings_per_month =
            total as f64 / throughput.bearings_per_month.len() as f64;
    }

    throughput
}

/// Determine how many weeks should be used for story throughput averaging.
///
/// Uses a growing denominator until enough completed-story history exists to fill
/// the requested window, then caps at the requested window size.
fn effective_story_weeks(
    board: &Board,
    requested_weeks: usize,
    current_week_start: NaiveDate,
) -> usize {
    if requested_weeks == 0 {
        return 0;
    }

    let oldest_completion_week = board
        .stories
        .values()
        .filter_map(|s| {
            s.frontmatter
                .completed_at
                .map(|dt| start_of_week_monday(dt.date()))
        })
        .min();

    match oldest_completion_week {
        Some(oldest_week) => {
            let elapsed_weeks = ((current_week_start - oldest_week).num_days() / 7) + 1;
            (elapsed_weeks.max(1) as usize).min(requested_weeks)
        }
        None => requested_weeks,
    }
}

fn start_of_week_monday(date: NaiveDate) -> NaiveDate {
    date - chrono::Duration::days(date.weekday().num_days_from_monday() as i64)
}

/// Generate week start dates going back the specified number of weeks
#[allow(dead_code)] // Planned feature (voyage 16)
fn generate_week_starts(from: NaiveDate, weeks: usize) -> Vec<NaiveDate> {
    let mut result = Vec::with_capacity(weeks);
    let mut current = start_of_week_monday(from);

    for _ in 0..weeks {
        result.push(current);
        current -= chrono::Duration::days(7);
    }

    result
}

/// Generate (year, month) pairs going back the specified number of months
#[allow(dead_code)] // Planned feature (voyage 16)
fn generate_months(from: NaiveDate, months: usize) -> Vec<(i32, u32)> {
    let mut result = Vec::with_capacity(months);
    let mut year = from.year();
    let mut month = from.month();

    for _ in 0..months {
        result.push((year, month));
        if month == 1 {
            month = 12;
            year -= 1;
        } else {
            month -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{
        Bearing, BearingFrontmatter, BearingStatus, Story, StoryFrontmatter, StoryState, StoryType,
    };
    use std::path::PathBuf;

    fn make_completed_story(id: &str, completed_at: NaiveDate) -> Story {
        Story::new(
            StoryFrontmatter {
                id: id.to_string(),
                title: format!("Story {}", id),
                story_type: StoryType::Feat,
                status: StoryState::Done,
                scope: None,
                milestone: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: Some(completed_at.and_hms_opt(12, 0, 0).unwrap()),
                submitted_at: None,
                index: None,
                governed_by: vec![],
                blocked_by: vec![],
                role: None,
            },
            PathBuf::from(format!("{}.md", id)),
        )
    }

    fn make_laid_bearing(id: &str, laid_at: NaiveDate) -> Bearing {
        Bearing {
            frontmatter: BearingFrontmatter {
                id: id.to_string(),
                title: format!("Bearing {}", id),
                status: BearingStatus::Laid,
                index: None,
                created_at: None,
                decline_reason: None,
                laid_at: Some(laid_at.and_hms_opt(0, 0, 0).unwrap()),
            },
            path: PathBuf::from(format!("{}.md", id)),
            has_survey: true,
            has_assessment: true,
        }
    }

    #[test]
    fn generate_week_starts_returns_correct_mondays() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 29).unwrap(); // Thursday
        let weeks = generate_week_starts(today, 3);

        assert_eq!(weeks.len(), 3);
        // Most recent Monday first
        assert_eq!(weeks[0], NaiveDate::from_ymd_opt(2026, 1, 26).unwrap());
        assert_eq!(weeks[1], NaiveDate::from_ymd_opt(2026, 1, 19).unwrap());
        assert_eq!(weeks[2], NaiveDate::from_ymd_opt(2026, 1, 12).unwrap());
    }

    #[test]
    fn generate_months_returns_correct_months() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 29).unwrap();
        let months = generate_months(today, 3);

        assert_eq!(months.len(), 3);
        assert_eq!(months[0], (2026, 1));
        assert_eq!(months[1], (2025, 12));
        assert_eq!(months[2], (2025, 11));
    }

    #[test]
    fn calculate_throughput_counts_stories_per_week() {
        let today = Local::now().date_naive();
        let this_week =
            today - chrono::Duration::days(today.weekday().num_days_from_monday() as i64);
        let last_week = this_week - chrono::Duration::days(7);

        let mut board = Board::new(PathBuf::from("test"));
        // 2 stories completed this week
        board.stories.insert(
            "1".to_string(),
            make_completed_story("1", this_week + chrono::Duration::days(1)),
        );
        board.stories.insert(
            "2".to_string(),
            make_completed_story("2", this_week + chrono::Duration::days(2)),
        );
        // 1 story completed last week
        board.stories.insert(
            "3".to_string(),
            make_completed_story("3", last_week + chrono::Duration::days(1)),
        );

        let throughput = calculate_throughput(&board, 4);

        assert_eq!(throughput.stories_per_week.len(), 4);
        assert_eq!(throughput.stories_per_week[0].count, 2); // This week
        assert_eq!(throughput.stories_per_week[1].count, 1); // Last week
    }

    #[test]
    fn calculate_throughput_uses_growing_denominator_for_partial_history() {
        let today = Local::now().date_naive();
        let this_week = start_of_week_monday(today);

        let mut board = Board::new(PathBuf::from("test"));
        // 4 stories this week
        for i in 0..4 {
            board.stories.insert(
                format!("{}", i),
                make_completed_story(
                    &format!("{}", i),
                    this_week + chrono::Duration::days(i as i64),
                ),
            );
        }

        let throughput = calculate_throughput(&board, 4);

        // With only one observed week of completions, use denominator = 1
        assert!((throughput.avg_stories_per_week - 4.0).abs() < 0.01);
    }

    #[test]
    fn calculate_throughput_caps_denominator_at_window_size() {
        let today = Local::now().date_naive();
        let this_week = start_of_week_monday(today);

        let mut board = Board::new(PathBuf::from("test"));
        // One story in each week of the 4-week window
        board.stories.insert(
            "w0".to_string(),
            make_completed_story("w0", this_week + chrono::Duration::days(1)),
        );
        board.stories.insert(
            "w1".to_string(),
            make_completed_story("w1", this_week - chrono::Duration::days(6)),
        );
        board.stories.insert(
            "w2".to_string(),
            make_completed_story("w2", this_week - chrono::Duration::days(13)),
        );
        board.stories.insert(
            "w3".to_string(),
            make_completed_story("w3", this_week - chrono::Duration::days(20)),
        );
        // Older completion should not expand denominator beyond requested window
        board.stories.insert(
            "old".to_string(),
            make_completed_story("old", this_week - chrono::Duration::days(35)),
        );

        let throughput = calculate_throughput(&board, 4);
        assert!((throughput.avg_stories_per_week - 1.0).abs() < 0.01);
    }

    #[test]
    fn calculate_throughput_counts_bearings_per_month() {
        let today = Local::now().date_naive();
        let this_month = NaiveDate::from_ymd_opt(today.year(), today.month(), 15).unwrap();

        let mut board = Board::new(PathBuf::from("test"));
        board
            .bearings
            .insert("b1".to_string(), make_laid_bearing("b1", this_month));
        board
            .bearings
            .insert("b2".to_string(), make_laid_bearing("b2", this_month));

        let throughput = calculate_throughput(&board, 4);

        assert_eq!(throughput.bearings_per_month.len(), 6);
        assert_eq!(throughput.bearings_per_month[0].count, 2);
    }
}
