#![allow(dead_code)]
//! Traceability matrix and lineage tracking
//!
//! Maps requirements to implementations and identifies coverage gaps.

use std::collections::HashMap;
use std::fs;

use crate::domain::model::Board;
use crate::domain::state_machine::invariants;
use crate::infrastructure::verification::parse_ac_references;

/// Traceability matrix mapping requirements to IMPLEMENTATIONS (stories).
#[derive(Debug, Default)]
pub struct TraceabilityMatrix {
    /// Maps SRS requirement ID (e.g. "SRS-01") to list of story IDs satisfying it.
    pub requirement_to_stories: HashMap<String, Vec<String>>,
    /// Maps story ID to list of SRS requirements it satisfies.
    pub story_to_requirements: HashMap<String, Vec<String>>,
}

/// Build traceability matrix for the entire board.
pub fn build_matrix(board: &Board) -> TraceabilityMatrix {
    let mut matrix = TraceabilityMatrix::default();

    for story in board.stories.values() {
        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let refs = parse_ac_references(&content);
        let mut story_reqs = Vec::new();

        for ac_ref in refs {
            story_reqs.push(ac_ref.srs_id.clone());
            matrix
                .requirement_to_stories
                .entry(ac_ref.srs_id)
                .or_default()
                .push(story.id().to_string());
        }

        story_reqs.sort();
        story_reqs.dedup();
        matrix
            .story_to_requirements
            .insert(story.id().to_string(), story_reqs);
    }

    matrix
}

/// Calculate story-to-story implementation dependencies based on SRS order.
pub fn derive_implementation_dependencies(board: &Board) -> HashMap<String, Vec<String>> {
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();

    // Group stories by voyage/scope to enforce ordering only within tactical units.
    let mut scope_to_requirements: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for story in board.stories.values() {
        let Some(scope) = story.scope() else {
            continue;
        };

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let refs = parse_ac_references(&content);
        for ac_ref in refs {
            scope_to_requirements
                .entry(scope.to_string())
                .or_default()
                .entry(ac_ref.srs_id)
                .or_default()
                .push(story.id().to_string());
        }
    }

    // For each voyage, determine story dependencies based on SRS requirement order.
    for voyage in board.voyages.values() {
        let srs_path = voyage.path.parent().unwrap().join("SRS.md");
        if !srs_path.exists() {
            continue;
        }

        let requirements = invariants::parse_requirements(&srs_path);
        let scope = voyage.scope_path();
        let Some(req_to_stories) = scope_to_requirements.get(&scope) else {
            continue;
        };

        // Track stories implementing each requirement.
        // A story depends on all stories implementing PREVIOUS requirements in the SRS.
        let mut previous_stories: Vec<String> = Vec::new();

        for req_id in requirements {
            // NFRs (Non-Functional Requirements) don't create ordering constraints.
            if req_id.contains("NFR") {
                continue;
            }

            if let Some(stories) = req_to_stories.get(&req_id) {
                for story_id in stories {
                    let mut story_deps = previous_stories.clone();
                    // Don't depend on yourself if multiple stories implement same req.
                    story_deps.retain(|id| id != story_id);

                    deps.entry(story_id.clone()).or_default().extend(story_deps);
                }

                // Add these stories to "previous" for the next requirement in SRS.
                previous_stories.extend(stories.iter().cloned());
                previous_stories.sort();
                previous_stories.dedup();
            }
        }
    }

    // Deduplicate and flatten transitive dependencies.
    for story_deps in deps.values_mut() {
        story_deps.sort();
        story_deps.dedup();
    }

    deps
}

/// Check if a story is "parallel safe" (has no implementation dependencies).
pub fn is_parallel_safe(story_id: &str, dependencies: &HashMap<String, Vec<String>>) -> bool {
    dependencies
        .get(story_id)
        .is_none_or(|deps| deps.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn empty_board_produces_empty_matrix() {
        let temp = TestBoardBuilder::new().build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let matrix = build_matrix(&board);
        assert!(matrix.requirement_to_stories.is_empty());
    }

    #[test]
    fn story_with_single_marker() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("STORY1")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] req 1"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let matrix = build_matrix(&board);

        assert_eq!(
            matrix.requirement_to_stories.get("SRS-01").unwrap(),
            &["STORY1"]
        );
        assert_eq!(
            matrix.story_to_requirements.get("STORY1").unwrap(),
            &["SRS-01"]
        );
    }

    #[test]
    fn story_with_multiple_markers_deduplicates() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("STORY1")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] req 1\n- [ ] [SRS-01/AC-02] also req 1"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let matrix = build_matrix(&board);

        assert_eq!(matrix.story_to_requirements.get("STORY1").unwrap().len(), 1);
    }

    #[test]
    fn multiple_stories_same_requirement() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("STORY1")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] part a"),
            )
            .story(
                TestStory::new("STORY2")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-02] part b"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let matrix = build_matrix(&board);

        let stories = matrix.requirement_to_stories.get("SRS-01").unwrap();
        assert!(stories.contains(&"STORY1".to_string()));
        assert!(stories.contains(&"STORY2".to_string()));
    }

    #[test]
    fn done_stories_are_included() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("DONE1")
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] done"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let matrix = build_matrix(&board);

        assert!(matrix.requirement_to_stories.contains_key("SRS-01"));
    }

    #[test]
    fn nfr_style_srs_ids_are_captured() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("STORY1")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-NFR-01/AC-01] performance"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let matrix = build_matrix(&board);

        assert!(matrix.requirement_to_stories.contains_key("SRS-NFR-01"));
    }

    #[test]
    fn backward_compat_no_errors_on_empty_board() {
        let temp = TestBoardBuilder::new().build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);
        assert!(deps.is_empty());
    }

    #[test]
    fn derive_dep_srs_order_helper() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        assert_eq!(deps.get("S2").unwrap(), &["S1"]);
        assert!(!deps.contains_key("S1") || deps.get("S1").unwrap().is_empty());
    }

    #[test]
    fn derive_dep_story_on_srs_02_depends_on_srs_01_stories() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1A")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1a"),
            )
            .story(
                TestStory::new("S1B")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-02] req1b"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        let s2_deps = deps.get("S2").unwrap();
        assert!(s2_deps.contains(&"S1A".to_string()));
        assert!(s2_deps.contains(&"S1B".to_string()));
    }

    #[test]
    fn derive_dep_transitive_deps_are_flattened() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\n| SRS-03 | req3 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .story(
                TestStory::new("S3")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-03/AC-01] req3"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        let s3_deps = deps.get("S3").unwrap();
        assert!(s3_deps.contains(&"S1".to_string()));
        assert!(s3_deps.contains(&"S2".to_string()));
    }

    #[test]
    fn derive_dep_no_deps_for_single_srs() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        assert!(!deps.contains_key("S1") || deps.get("S1").unwrap().is_empty());
    }

    #[test]
    fn derive_dep_deps_only_within_same_scope() {
        let srs1 = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let srs2 = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | other | test |\nEND FUNCTIONAL_REQUIREMENTS";

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-v1", "keel").srs_content(srs1))
            .voyage(TestVoyage::new("02-v2", "keel").srs_content(srs2))
            .story(
                TestStory::new("V1S1")
                    .scope("keel/01-v1")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("V2S1")
                    .scope("keel/02-v2")
                    .body("- [ ] [SRS-01/AC-01] other"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        // V2S1 should NOT depend on V1S1 even though both implementation SRS-01
        // because implementation deps are local to the voyage scope.
        assert!(!deps.contains_key("V2S1") || deps.get("V2S1").unwrap().is_empty());
    }

    #[test]
    fn derive_dep_nfr_requirements_dont_create_ordering() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-NFR-01 | perf | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("N1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-NFR-01/AC-01] perf"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        // S2 should depend on S1, but NOT on N1.
        let s2_deps = deps.get("S2").unwrap();
        assert!(s2_deps.contains(&"S1".to_string()));
        assert!(!s2_deps.contains(&"N1".to_string()));
    }

    #[test]
    fn derive_dep_stories_without_scope_have_no_deps() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").body("- [ ] [SRS-01/AC-01] req1"))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        assert!(deps.is_empty());
    }

    #[test]
    fn derive_dep_story_touching_gap_srs_numbers() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-03 | req3 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S3")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-03/AC-01] req3"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let deps = derive_implementation_dependencies(&board);

        // S3 should depend on S1 even though SRS-02 is missing.
        assert_eq!(deps.get("S3").unwrap(), &["S1"]);
    }
}
