//! List stories command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::table::Table;
use crate::domain::model::{Board, Story, StoryState};
use crate::infrastructure::loader::load_board;

/// List stories with optional filters
pub fn run(stage: Option<&str>, epic: Option<&str>, reflections: bool) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let stories = list_stories(&board, stage, epic);

    let mut table = Table::new(&["ID", "TYPE", "TITLE", "STAGE", "SCOPE"]);
    for story in &stories {
        let styled_scope = story.scope().map(|s| {
            if s.contains('/') {
                let parts: Vec<_> = s.split('/').collect();
                if parts.len() >= 2 {
                    format!(
                        "{}/{}",
                        crate::cli::style::styled_epic_id(parts[0]),
                        crate::cli::style::styled_voyage_id(parts[1])
                    )
                } else {
                    crate::cli::style::styled_id(s)
                }
            } else {
                crate::cli::style::styled_epic_id(s)
            }
        });

        table.row(&[
            &crate::cli::style::styled_story_id(story.id()),
            &story.story_type().to_string(),
            &story.frontmatter.title,
            &story.stage.to_string(),
            &styled_scope.unwrap_or_else(|| "-".to_string()),
        ]);
    }
    table.print();

    if reflections {
        println!();
        println!("{}", "Reflections".bold().underline());
        for story in stories {
            let reflect_path = story.path.parent().unwrap().join("REFLECT.md");
            if reflect_path.exists() {
                let content = std::fs::read_to_string(reflect_path)?;
                println!();
                println!(
                    "{} - {}:",
                    crate::cli::style::styled_story_id(story.id()),
                    story.frontmatter.title
                );
                println!("{}", content.trim());
            }
        }
    }

    Ok(())
}

fn list_stories<'a>(board: &'a Board, stage: Option<&str>, epic: Option<&str>) -> Vec<&'a Story> {
    let stage_filter = stage.and_then(|s| match s.to_lowercase().as_str() {
        "backlog" => Some(StoryState::Backlog),
        "in-progress" => Some(StoryState::InProgress),
        "needs-human-verification" => Some(StoryState::NeedsHumanVerification),
        "done" => Some(StoryState::Done),
        "rejected" => Some(StoryState::Rejected),
        "icebox" => Some(StoryState::Icebox),
        _ => None,
    });

    let mut stories: Vec<_> = board
        .stories
        .values()
        .filter(|s| stage_filter.is_none() || Some(&s.stage) == stage_filter.as_ref())
        .filter(|s| epic.is_none() || s.epic() == epic)
        .collect();

    // Sort by: Epic index (asc), Voyage index (asc), Story index (asc)
    stories.sort_by(|a, b| {
        // 1. Epic index (asc)
        let epic_a = a.epic().and_then(|id| board.epics.get(id));
        let epic_b = b.epic().and_then(|id| board.epics.get(id));
        let epic_idx_a = epic_a.and_then(|e| e.frontmatter.index).unwrap_or(0);
        let epic_idx_b = epic_b.and_then(|e| e.frontmatter.index).unwrap_or(0);

        let epic_cmp = epic_idx_a.cmp(&epic_idx_b);
        if epic_cmp != std::cmp::Ordering::Equal {
            return epic_cmp;
        }

        // 2. Voyage index (asc)
        let voyage_a = a.voyage().and_then(|id| board.voyages.get(id));
        let voyage_b = b.voyage().and_then(|id| board.voyages.get(id));
        let voyage_idx_a = voyage_a.and_then(|v| v.frontmatter.index).unwrap_or(0);
        let voyage_idx_b = voyage_b.and_then(|v| v.frontmatter.index).unwrap_or(0);

        let voyage_cmp = voyage_idx_a.cmp(&voyage_idx_b);
        if voyage_cmp != std::cmp::Ordering::Equal {
            return voyage_cmp;
        }

        // 3. Story index (asc)
        let story_idx_a = a.index().unwrap_or(0);
        let story_idx_b = b.index().unwrap_or(0);

        let story_cmp = story_idx_a.cmp(&story_idx_b);
        if story_cmp != std::cmp::Ordering::Equal {
            return story_cmp;
        }

        // Fallback to ID (asc)
        a.id().cmp(b.id())
    });
    stories
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn test_list_stories_filtering() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::InProgress)
                    .scope("epic1"),
            )
            .story(
                TestStory::new("S2")
                    .stage(StoryState::Backlog)
                    .scope("epic2"),
            )
            .build();
        let board = load_board(temp.path()).unwrap();

        let all = list_stories(&board, None, None);
        assert_eq!(all.len(), 2);

        let in_progress = list_stories(&board, Some("in-progress"), None);
        assert_eq!(in_progress.len(), 1);
        assert_eq!(in_progress[0].id(), "S1");

        let by_epic = list_stories(&board, None, Some("epic2"));
        assert_eq!(by_epic.len(), 1);
        assert_eq!(by_epic[0].id(), "S2");
    }

    #[test]
    fn test_list_stories_sorting() {
        use crate::test_helpers::{TestEpic, TestVoyage};
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1").index(1))
            .epic(TestEpic::new("E2").index(2))
            .voyage(TestVoyage::new("V1", "E1").index(1))
            .voyage(TestVoyage::new("V2", "E1").index(2))
            .story(TestStory::new("S1").scope("E1/V1").index(1))
            .story(TestStory::new("S2").scope("E1/V1").index(2))
            .story(TestStory::new("S3").scope("E1/V2").index(1))
            .story(TestStory::new("S4").scope("E2/V1").index(1)) // E2 has index 2
            .build();
        let board = load_board(temp.path()).unwrap();

        let stories = list_stories(&board, None, None);

        // Expected order (ASC):
        // 1. E1/V1/S1 (Epic index 1, Voyage index 1, Story index 1)
        // 2. E1/V1/S2 (Epic index 1, Voyage index 1, Story index 2)
        // 3. E1/V2/S3 (Epic index 1, Voyage index 2)
        // 4. E2/V1/S4 (Epic index 2)

        assert_eq!(stories[0].id(), "S1");
        assert_eq!(stories[1].id(), "S2");
        assert_eq!(stories[2].id(), "S3");
        assert_eq!(stories[3].id(), "S4");
    }
}
