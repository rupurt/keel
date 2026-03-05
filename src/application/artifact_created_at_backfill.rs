//! Backfill missing created_at frontmatter for reflection and knowledge artifacts.

use std::path::Path;

use anyhow::Result;

use crate::domain::model::{Board, Story, Voyage};
use crate::infrastructure::artifact_frontmatter;
use crate::infrastructure::loader::load_board;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactCreatedAtBackfillStats {
    pub story_reflect_updated: usize,
    pub voyage_knowledge_updated: usize,
}

pub fn backfill(board_dir: &Path) -> Result<ArtifactCreatedAtBackfillStats> {
    let board = load_board(board_dir)?;
    let mut stats = ArtifactCreatedAtBackfillStats::default();

    for story in board.stories.values() {
        let Some(created_at) = story_reflect_created_at(story) else {
            continue;
        };
        let Some(story_dir) = story.path.parent() else {
            continue;
        };

        let reflect_path = story_dir.join("REFLECT.md");
        if !reflect_path.exists() {
            continue;
        }

        if artifact_frontmatter::ensure_created_at(&reflect_path, created_at)? {
            stats.story_reflect_updated += 1;
        }
    }

    for voyage in board.voyages.values() {
        let Some(created_at) = voyage_knowledge_created_at(&board, voyage) else {
            continue;
        };
        let Some(voyage_dir) = voyage.path.parent() else {
            continue;
        };

        let knowledge_path = voyage_dir.join("KNOWLEDGE.md");
        if !knowledge_path.exists() {
            continue;
        }

        if artifact_frontmatter::ensure_created_at(&knowledge_path, created_at)? {
            stats.voyage_knowledge_updated += 1;
        }
    }

    Ok(stats)
}

fn story_reflect_created_at(story: &Story) -> Option<chrono::NaiveDateTime> {
    story
        .frontmatter
        .submitted_at
        .or(story.frontmatter.completed_at)
        .or(story.frontmatter.started_at)
        .or(story.frontmatter.created_at)
}

fn voyage_knowledge_created_at(board: &Board, voyage: &Voyage) -> Option<chrono::NaiveDateTime> {
    board
        .stories_for_voyage(voyage)
        .into_iter()
        .filter_map(|story| story.frontmatter.submitted_at)
        .max()
        .or(voyage.frontmatter.completed_at)
        .or(voyage.frontmatter.started_at)
        .or(voyage.frontmatter.created_at)
}

#[cfg(test)]
mod tests {
    use super::backfill;
    use std::fs;
    use tempfile::TempDir;

    fn write_story(board: &Path, id: &str, scope: &str, submitted_at: &str) {
        let story_dir = board.join("stories").join(id);
        fs::create_dir_all(&story_dir).unwrap();
        fs::write(
            story_dir.join("README.md"),
            format!(
                "---\nid: {id}\ntitle: Story {id}\ntype: feat\nstatus: done\nscope: {scope}\ncreated_at: 2026-03-01T08:00:00\nsubmitted_at: {submitted_at}\ncompleted_at: 2026-03-01T13:00:00\n---\n\n# Story\n"
            ),
        )
        .unwrap();
        fs::write(story_dir.join("REFLECT.md"), "# Reflection\n").unwrap();
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();
    }

    fn write_voyage(board: &Path, epic: &str, voyage: &str, knowledge: bool) {
        let epic_dir = board.join("epics").join(epic);
        let voyage_dir = epic_dir.join("voyages").join(voyage);
        fs::create_dir_all(&voyage_dir).unwrap();
        fs::write(
            epic_dir.join("README.md"),
            format!(
                "---\nid: {epic}\ntitle: Epic {epic}\ncreated_at: 2026-03-01T00:00:00\n---\n\n# Epic\n"
            ),
        )
        .unwrap();
        fs::write(
            epic_dir.join("PRD.md"),
            "# PRD\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n<!-- END FUNCTIONAL_REQUIREMENTS -->\n<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->\n<!-- END NON_FUNCTIONAL_REQUIREMENTS -->\n<!-- BEGIN SUCCESS_CRITERIA -->\n<!-- END SUCCESS_CRITERIA -->\n",
        )
        .unwrap();
        fs::write(
            voyage_dir.join("README.md"),
            format!(
                "---\nid: {voyage}\ntitle: Voyage {voyage}\nstatus: done\nepic: {epic}\ncreated_at: 2026-03-01T00:00:00\ncompleted_at: 2026-03-02T00:00:00\n---\n\n# Voyage\n"
            ),
        )
        .unwrap();
        fs::write(voyage_dir.join("SRS.md"), "# SRS\n").unwrap();
        fs::write(voyage_dir.join("SDD.md"), "# SDD\n").unwrap();
        if knowledge {
            fs::write(voyage_dir.join("KNOWLEDGE.md"), "# Knowledge\n").unwrap();
        }
    }

    use std::path::Path;

    #[test]
    fn backfill_sets_story_reflect_created_at_from_submission() {
        let temp = TempDir::new().unwrap();
        write_voyage(temp.path(), "e1", "v1", false);
        write_story(temp.path(), "S1", "e1/v1", "2026-03-04T11:00:00");

        let stats = backfill(temp.path()).unwrap();
        assert_eq!(stats.story_reflect_updated, 1);

        let content = fs::read_to_string(temp.path().join("stories/S1/REFLECT.md")).unwrap();
        assert!(content.contains("created_at: 2026-03-04T11:00:00"));
    }

    #[test]
    fn backfill_sets_voyage_knowledge_created_at_from_latest_story_submission() {
        let temp = TempDir::new().unwrap();
        write_voyage(temp.path(), "e1", "v1", true);
        write_story(temp.path(), "S1", "e1/v1", "2026-03-04T11:00:00");
        write_story(temp.path(), "S2", "e1/v1", "2026-03-05T09:30:00");

        let stats = backfill(temp.path()).unwrap();
        assert_eq!(stats.voyage_knowledge_updated, 1);

        let content =
            fs::read_to_string(temp.path().join("epics/e1/voyages/v1/KNOWLEDGE.md")).unwrap();
        assert!(content.contains("created_at: 2026-03-05T09:30:00"));
    }
}
