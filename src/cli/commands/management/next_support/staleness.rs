#![allow(dead_code)]
//! Staleness calculation for scope-based prioritization

use chrono::NaiveDate;

use crate::domain::model::Board;

/// Get the last completion date for stories in a given scope
///
/// Returns None if no stories in the scope have been completed
pub fn last_completion_date(board: &Board, scope: &str) -> Option<NaiveDate> {
    board
        .stories
        .values()
        .filter(|s| s.scope() == Some(scope))
        .filter(|s| s.frontmatter.status.is_terminal())
        .filter_map(|s| s.frontmatter.completed_at.map(|dt| dt.date()))
        .max()
}

/// Calculate staleness score for a scope (days since last completion)
///
/// Higher score = more stale = higher priority
/// Returns u32::MAX for scopes with no completions (highest priority)
pub fn staleness_score(board: &Board, scope: &str, today: NaiveDate) -> u32 {
    match last_completion_date(board, scope) {
        Some(date) => (today - date).num_days().max(0) as u32,
        None => u32::MAX, // Never completed = highest staleness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::StoryFactory;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_story(
        id: &str,
        scope: &str,
        status: StoryState,
        completed_at: Option<NaiveDate>,
    ) -> crate::domain::model::Story {
        let mut factory = StoryFactory::new(id).scope(scope).stage(status);
        if let Some(date) = completed_at {
            factory = factory.completed_at(date.and_hms_opt(12, 0, 0).unwrap());
        }
        factory.build()
    }

    fn make_board(stories: Vec<crate::domain::model::Story>) -> Board {
        let mut story_map = HashMap::new();
        for s in stories {
            story_map.insert(s.id().to_string(), s);
        }
        Board {
            root: PathBuf::from("test"),
            stories: story_map,
            epics: HashMap::new(),
            voyages: HashMap::new(),
            bearings: HashMap::new(),
            adrs: HashMap::new(),
        }
    }

    #[test]
    fn last_completion_returns_most_recent() {
        let stories = vec![
            make_story(
                "S1",
                "epic/voyage",
                StoryState::Done,
                Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
            ),
            make_story(
                "S2",
                "epic/voyage",
                StoryState::Done,
                Some(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()),
            ),
            make_story(
                "S3",
                "epic/voyage",
                StoryState::Done,
                Some(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()),
            ),
        ];
        let board = make_board(stories);

        let last = last_completion_date(&board, "epic/voyage");
        assert_eq!(last, Some(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()));
    }

    #[test]
    fn last_completion_filters_by_scope() {
        let stories = vec![
            make_story(
                "S1",
                "epic-a/voyage",
                StoryState::Done,
                Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
            ),
            make_story(
                "S2",
                "epic-b/voyage",
                StoryState::Done,
                Some(NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()),
            ),
        ];
        let board = make_board(stories);

        let last = last_completion_date(&board, "epic-a/voyage");
        assert_eq!(last, Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()));
    }

    #[test]
    fn last_completion_returns_none_for_no_completions() {
        let stories = vec![make_story("S1", "epic/voyage", StoryState::Backlog, None)];
        let board = make_board(stories);

        let last = last_completion_date(&board, "epic/voyage");
        assert_eq!(last, None);
    }

    #[test]
    fn staleness_score_calculates_days() {
        let stories = vec![make_story(
            "S1",
            "epic/voyage",
            StoryState::Done,
            Some(NaiveDate::from_ymd_opt(2026, 1, 10).unwrap()),
        )];
        let board = make_board(stories);
        let today = NaiveDate::from_ymd_opt(2026, 1, 20).unwrap();

        let score = staleness_score(&board, "epic/voyage", today);
        assert_eq!(score, 10); // 10 days since completion
    }

    #[test]
    fn staleness_score_returns_max_for_no_completions() {
        let stories = vec![make_story("S1", "epic/voyage", StoryState::Backlog, None)];
        let board = make_board(stories);
        let today = NaiveDate::from_ymd_opt(2026, 1, 20).unwrap();

        let score = staleness_score(&board, "epic/voyage", today);
        assert_eq!(score, u32::MAX);
    }
}
