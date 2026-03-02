//! Shared invariants for keel validation
//!
//! This module defines the core validation rules used by both `keel doctor` and `keel next`.
//! Having a single source of truth eliminates drift between what doctor validates and what
//! commands implicitly require.
//!
//! # Invariants
//!
//! ## Voyage Ready for Work
//! A voyage is ready for work when:
//! - Status is `Planned` or `InProgress`
//! - Has requirements defined in the SRS FUNCTIONAL_REQUIREMENTS section
//!
//! ## Story Workable
//! A story is workable when:
//! - In `backlog` stage
//! - If scoped: voyage is ready for work
//! - If unscoped: always workable (no voyage to block on)
//!
//! # Coherence Rules
//!
//! Coherence rules ensure consistency between parent and child entities.
//! These are implemented as pure functions in [`state_machine::validation`].
//!
//! ## Voyage-Story Coherence
//! - **Draft voyages**: All stories should be in icebox (still being planned)
//! - **Planned voyages**: No stories should be active (not started yet)
//! - **In-progress voyages**: Normal operation (any story state allowed)
//! - **Done voyages**: No constraints (voyage is complete)
//! - **Non-draft voyages**: Must have at least one story
//! - **All stories done**: Voyage should be marked done (auto-fixable)
//!
//! ## Epic-Voyage Coherence
//! - **Done epics**: All voyages should be done
//! - **All voyages done**: Epic should be marked done (auto-fixable)
//!
//! See [`state_machine::validation::validate_voyage_story_coherence`] and
//! [`state_machine::validation::validate_epic_voyage_coherence`] for implementations.

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::domain::model::{Board, Story, StoryState, Voyage, VoyageState};
use std::collections::HashSet;
use std::fs;

static REQ_TABLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\|\s*(SRS-\d+)\s*\|").unwrap());
static AC_REQ_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(SRS-\d+)/AC-\d+\]").unwrap());
static REQ_REF_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bSRS-[A-Z0-9-]+\b").unwrap());

// Re-export coherence validation functions for a unified API
#[allow(unused_imports)] // SuggestedFix exported for public API completeness
pub use crate::domain::state_machine::validation::{
    EpicVoyageViolation, SuggestedFix, VoyageStoryViolation, validate_epic_voyage_coherence,
    validate_voyage_story_coherence,
};

/// Check if a voyage is ready for work.
///
/// A voyage is ready for work when:
/// 1. Status is `Planned` or `InProgress` (not `Draft` or `Done`)
/// 2. Has at least one requirement defined in the SRS
///
/// # Arguments
/// * `voyage` - The voyage to check
/// * `requirements` - List of requirement IDs (e.g., ["SRS-01", "SRS-02"])
#[allow(dead_code)] // Used by tests now, production use in future stories (SRS-05, SRS-06)
pub fn voyage_ready_for_work(voyage: &Voyage, requirements: &[String]) -> bool {
    let status_ok = matches!(
        voyage.status(),
        VoyageState::Planned | VoyageState::InProgress
    );
    let has_requirements = !requirements.is_empty();

    status_ok && has_requirements
}

/// Check if a story is workable.
///
/// A story is workable when:
/// 1. In `backlog` stage
/// 2. If scoped: voyage is ready for work
/// 3. If unscoped: always workable
///
/// Note: Dependency checking was removed. Dependencies will be derived from
/// SRS traceability in a future story.
///
/// # Arguments
/// * `story` - The story to check
/// * `board` - The board containing voyages
/// * `_board_dir` - Path to the board directory (for SRS file access)
#[allow(dead_code)] // Used by tests now, production use in future stories (SRS-05, SRS-06)
pub fn story_workable(story: &Story, board: &Board, _board_dir: &Path) -> bool {
    // Must be in backlog stage
    if story.stage != StoryState::Backlog {
        return false;
    }

    // Check voyage readiness
    match story.scope() {
        None => true, // Unscoped = always workable
        Some(scope) => {
            // Parse scope into epic/voyage
            let parts: Vec<&str> = scope.split('/').collect();
            if parts.len() != 2 {
                return true; // Invalid scope format, allow through
            }

            let voyage_id = parts[1];

            // Check if voyage exists
            let voyage = match board.voyages.get(voyage_id) {
                Some(v) => v,
                None => return false, // Voyage doesn't exist
            };

            // Get requirements from SRS file
            let srs_path = voyage.path.parent().unwrap().join("SRS.md");
            let requirements = parse_requirements(&srs_path);

            voyage_ready_for_work(voyage, &requirements)
        }
    }
}

/// Parse requirement IDs from an SRS file's FUNCTIONAL_REQUIREMENTS section.
///
/// # Arguments
/// * `srs_path` - Path to the SRS.md file
///
/// # Returns
/// Vector of requirement IDs (e.g., ["SRS-01", "SRS-02"])
#[allow(dead_code)] // Used by tests now, production use in future stories (SRS-05, SRS-06)
pub fn parse_requirements(srs_path: &Path) -> Vec<String> {
    let srs_content = match std::fs::read_to_string(srs_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut requirements = Vec::new();
    let mut in_table = false;

    for line in srs_content.lines() {
        if line.contains("BEGIN FUNCTIONAL_REQUIREMENTS") {
            in_table = true;
            continue;
        }
        if line.contains("END FUNCTIONAL_REQUIREMENTS") {
            break;
        }
        if in_table
            && let Some(cap) = REQ_TABLE_REGEX.captures(line)
            && let Some(req_id) = cap.get(1)
        {
            requirements.push(req_id.as_str().to_string());
        }
    }

    requirements
}

/// Return SRS requirements for a voyage that are not covered by any story
/// annotations in acceptance criteria.
///
/// This function is the core traceability check used by both planning and doctor
/// validation so behavior stays consistent.
#[allow(dead_code)] // Used by multiple command modules and tests
pub fn uncovered_requirements_for_voyage(voyage: &Voyage, board: &Board) -> Vec<String> {
    let srs_path = voyage.path.parent().unwrap().join("SRS.md");
    let all_requirements: HashSet<String> = parse_requirements(&srs_path).into_iter().collect();

    if all_requirements.is_empty() {
        return Vec::new();
    }

    let voyage_scope = format!("{}/{}", voyage.epic_id, voyage.id());
    let mut covered: HashSet<String> = HashSet::new();

    for story in board.stories.values() {
        if story.scope() != Some(&voyage_scope) {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for caps in AC_REQ_RE.captures_iter(&content) {
            covered.insert(caps[1].to_string());
        }

        for caps in REQ_REF_RE.captures_iter(&content) {
            covered.insert(caps[0].to_string());
        }
    }

    let mut missing: Vec<String> = all_requirements.difference(&covered).cloned().collect();
    missing.sort();

    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    use crate::domain::model::{StoryFrontmatter, VoyageFrontmatter};

    fn make_story(id: &str, stage: StoryState, scope: Option<&str>) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: id.to_string(),
                title: format!("Story {}", id),
                story_type: crate::domain::model::StoryType::Feat,
                status: stage,
                scope: scope.map(|s| s.to_string()),
                milestone: None,
                created_at: None,
                updated_at: None,
                submitted_at: None,
                completed_at: None,
                index: None,
                governed_by: vec![],
                role: None,
            },
            path: std::path::PathBuf::from(format!("{}.md", id)),
            stage,
        }
    }

    fn make_voyage(id: &str, status: VoyageState, path: &Path) -> Voyage {
        Voyage {
            frontmatter: VoyageFrontmatter {
                id: id.to_string(),
                title: format!("Voyage {}", id),
                goal: None,
                status,
                epic: Some("test-epic".to_string()),
                index: None,
                created_at: None,
                updated_at: None,
                completed_at: None,
            },
            path: path.to_path_buf(),
            epic_id: "test-epic".to_string(),
        }
    }

    fn make_board(stories: Vec<Story>) -> Board {
        let mut board = Board::new(std::path::PathBuf::new());
        for story in stories {
            board.stories.insert(story.id().to_string(), story);
        }
        board
    }

    fn create_voyage_with_srs(root: &Path, epic: &str, voyage: &str, req_count: usize) {
        let voyage_path = root.join("epics").join(epic).join("voyages").join(voyage);
        fs::create_dir_all(&voyage_path).unwrap();

        // Create SRS with requirements
        let mut srs = String::from("# SRS\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n");
        srs.push_str("| ID | Requirement | Verification |\n");
        srs.push_str("|----|-------------|--------------|\n");
        for i in 1..=req_count {
            srs.push_str(&format!("| SRS-{:02} | Requirement {} | test |\n", i, i));
        }
        srs.push_str("<!-- END FUNCTIONAL_REQUIREMENTS -->\n");

        fs::write(voyage_path.join("SRS.md"), srs).unwrap();
        fs::write(voyage_path.join("README.md"), "---\nid: voyage\n---\n").unwrap();
    }

    // ==================== voyage_ready_for_work tests ====================

    #[test]
    fn voyage_ready_for_work_returns_true_for_planned_with_requirements() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::Planned,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string(), "SRS-02".to_string()];

        assert!(voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_true_for_in_progress_with_requirements() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::InProgress,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string()];

        assert!(voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_false_for_draft() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::Draft,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string()];

        assert!(!voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_false_for_done() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::Done,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string()];

        assert!(!voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_false_without_requirements() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::InProgress,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements: Vec<String> = vec![];

        assert!(!voyage_ready_for_work(&voyage, &requirements));
    }

    // ==================== story_workable tests ====================

    #[test]
    fn story_workable_returns_false_for_in_progress_story() {
        let temp = TempDir::new().unwrap();
        let story = make_story("S1", StoryState::InProgress, None);
        let board = make_board(vec![story.clone()]);

        assert!(!story_workable(&story, &board, temp.path()));
    }

    #[test]
    fn story_workable_returns_true_for_unscoped_backlog_story() {
        let temp = TempDir::new().unwrap();
        let story = make_story("S1", StoryState::Backlog, None);
        let board = make_board(vec![story.clone()]);

        assert!(story_workable(&story, &board, temp.path()));
    }

    #[test]
    fn story_workable_returns_true_for_scoped_with_ready_voyage() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create voyage with requirements
        create_voyage_with_srs(root, "test-epic", "01-voyage", 3);

        let story = make_story("S1", StoryState::Backlog, Some("test-epic/01-voyage"));
        let mut board = make_board(vec![story.clone()]);

        // Add voyage to board
        board.voyages.insert(
            "01-voyage".to_string(),
            make_voyage(
                "01-voyage",
                VoyageState::InProgress,
                &root.join("epics/test-epic/voyages/01-voyage/README.md"),
            ),
        );

        assert!(story_workable(&story, &board, root));
    }

    #[test]
    fn story_workable_returns_false_for_scoped_with_draft_voyage() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create voyage with requirements
        create_voyage_with_srs(root, "test-epic", "01-voyage", 3);

        let story = make_story("S1", StoryState::Backlog, Some("test-epic/01-voyage"));
        let mut board = make_board(vec![story.clone()]);

        // Add draft voyage to board
        board.voyages.insert(
            "01-voyage".to_string(),
            make_voyage(
                "01-voyage",
                VoyageState::Draft,
                &root.join("epics/test-epic/voyages/01-voyage/README.md"),
            ),
        );

        assert!(!story_workable(&story, &board, root));
    }

    // ==================== parse_requirements tests ====================

    #[test]
    fn parse_requirements_extracts_ids_from_srs() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        create_voyage_with_srs(root, "test-epic", "01-voyage", 3);

        let srs_path = root.join("epics/test-epic/voyages/01-voyage/SRS.md");
        let requirements = parse_requirements(&srs_path);

        assert_eq!(requirements.len(), 3);
        assert!(requirements.contains(&"SRS-01".to_string()));
        assert!(requirements.contains(&"SRS-02".to_string()));
        assert!(requirements.contains(&"SRS-03".to_string()));
    }

    #[test]
    fn parse_requirements_returns_empty_for_missing_file() {
        let temp = TempDir::new().unwrap();
        let srs_path = temp.path().join("nonexistent/SRS.md");

        let requirements = parse_requirements(&srs_path);

        assert!(requirements.is_empty());
    }

    #[test]
    fn parse_requirements_returns_empty_when_no_requirements_section() {
        let temp = TempDir::new().unwrap();
        let srs_path = temp.path().join("SRS.md");

        fs::write(&srs_path, "# SRS\n\nNo requirements here.").unwrap();

        let requirements = parse_requirements(&srs_path);

        assert!(requirements.is_empty());
    }

    #[test]
    fn uncovered_requirements_for_voyage_reports_missing_ids() {
        use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
| SRS-02 | Requirement 2 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-voyage", "test-epic").srs_content(srs))
            .story(
                TestStory::new("1")
                    .scope("test-epic/01-voyage")
                    .body("- [ ] [SRS-01/AC-01] coverage point"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-voyage").unwrap();

        let missing = uncovered_requirements_for_voyage(voyage, &board);

        assert_eq!(missing, vec!["SRS-02".to_string()]);
    }

    #[test]
    fn uncovered_requirements_for_voyage_returns_none_when_all_covered() {
        use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
| SRS-02 | Requirement 2 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-voyage", "test-epic").srs_content(srs))
            .story(
                TestStory::new("1")
                    .scope("test-epic/01-voyage")
                    .body("- [ ] [SRS-01/AC-01] coverage point"),
            )
            .story(
                TestStory::new("2")
                    .scope("test-epic/01-voyage")
                    .body("- [ ] [SRS-02/AC-01] second point"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-voyage").unwrap();

        let missing = uncovered_requirements_for_voyage(voyage, &board);

        assert!(missing.is_empty());
    }
}
