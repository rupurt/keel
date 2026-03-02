//! Accept command - accept a verified story and move to done

use std::path::Path;

use anyhow::Result;

use crate::application::story_lifecycle::StoryLifecycleService;

/// Run the accept command
pub fn run(board_dir: &Path, id: &str, human: bool, reflect: Option<&str>) -> Result<()> {
    StoryLifecycleService::accept(board_dir, id, human, reflect)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::validation::{CheckId, structural};
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use regex::Regex;
    use std::fs;

    #[test]
    fn accept_moves_story_to_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-voyage", "test-epic").status("in-progress"))
            .story(
                TestStory::new("READY1")
                    .title("Ready Story")
                    .stage(StoryState::NeedsHumanVerification)
                    .scope("test-epic/01-voyage"),
            )
            .build();

        run(temp.path(), "READY1", false, None).unwrap();

        // Status should be updated to done
        let story_path = temp.path().join("stories/READY1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: done"));
    }

    #[test]
    fn accept_updates_frontmatter() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-voyage", "test-epic").status("in-progress"))
            .story(
                TestStory::new("UPDATE1")
                    .title("Update Story")
                    .stage(StoryState::NeedsHumanVerification)
                    .scope("test-epic/01-voyage"),
            )
            .build();

        run(temp.path(), "UPDATE1", false, None).unwrap();

        let content = fs::read_to_string(temp.path().join("stories/UPDATE1/README.md")).unwrap();

        assert!(content.contains("status: done"));
        assert!(content.contains("completed_at:"));
        let completed_re =
            Regex::new(r"completed_at: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
        assert!(
            completed_re.is_match(&content),
            "completed_at should be datetime: {content}"
        );

        let date_problems = structural::check_date_consistency(
            &temp.path().join("stories/UPDATE1/README.md"),
            CheckId::StoryDateConsistency,
        );
        assert!(
            date_problems.is_empty(),
            "Story accept should satisfy doctor date checks: {date_problems:?}"
        );
    }

    #[test]
    fn accept_errors_on_manual_verification_without_human_flag() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH1")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: manual -->"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH1", false, None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("manual acceptance criteria"),
            "Error should mention manual verification: {}",
            err
        );
    }

    #[test]
    fn accept_with_human_flag_succeeds_for_manual_stories() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH2")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: manual -->"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH2", true, None);
        assert!(
            result.is_ok(),
            "Should succeed with --human flag: {:?}",
            result
        );
    }

    #[test]
    fn accept_without_manual_verification_succeeds_normally() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH3")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: echo ok -->"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH3", false, None);
        assert!(
            result.is_ok(),
            "Should succeed without --human for non-manual stories: {:?}",
            result
        );
    }

    #[test]
    fn accept_without_verify_annotations_succeeds_normally() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH4")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Simple criteria"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH4", false, None);
        assert!(
            result.is_ok(),
            "Should succeed for stories without verify annotations: {:?}",
            result
        );
    }

    #[test]
    fn accept_flat_updates_frontmatter_without_moving() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("1vkqtsAAA")
                    .title("Flat Story")
                    .stage(StoryState::NeedsHumanVerification)
                    .scope("test-epic/01-first"),
            )
            .build();

        run(temp.path(), "1vkqtsAAA", false, None).unwrap();

        // Story bundle README should still exist
        let story_path = temp.path().join("stories/1vkqtsAAA/README.md");
        assert!(story_path.exists(), "Story bundle README should exist");

        // Frontmatter should be updated
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: done"));
        assert!(content.contains("completed_at:"));
    }

    #[test]
    fn accept_with_reflect_appends_section() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vqNrfl01")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("\n## Acceptance Criteria\n\n- [x] Something done"),
            )
            .build();

        run(
            temp.path(),
            "1vqNrfl01",
            false,
            Some("Caching surprised us"),
        )
        .unwrap();

        let reflect_path = temp.path().join("stories/1vqNrfl01/REFLECT.md");
        assert!(reflect_path.exists(), "REFLECT.md should be created");

        let content = fs::read_to_string(reflect_path).unwrap();
        assert!(
            content.contains("Caching surprised us"),
            "Should contain reflection text"
        );
    }

    #[test]
    fn reflection_stored_as_dedicated_file() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vqNrfl02")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("\n## Acceptance Criteria\n\n- [x] Something done"),
            )
            .build();

        run(temp.path(), "1vqNrfl02", false, Some("Latency was key")).unwrap();

        let content = fs::read_to_string(temp.path().join("stories/1vqNrfl02/REFLECT.md")).unwrap();
        assert!(
            content.contains("Latency was key"),
            "Reflection should be in REFLECT.md: {}",
            content
        );
    }

    #[test]
    fn accept_without_reflect_unchanged() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("1vqNrfl03").stage(StoryState::NeedsHumanVerification))
            .build();

        run(temp.path(), "1vqNrfl03", false, None).unwrap();

        let reflect_path = temp.path().join("stories/1vqNrfl03/REFLECT.md");
        // It now exists by default because of TestBoardBuilder
        assert!(reflect_path.exists(), "REFLECT.md should exist by default");
    }

    #[test]
    fn multiple_reflections_append() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("1vqNrfl04").stage(StoryState::NeedsHumanVerification))
            .build();

        let s_dir = temp.path().join("stories/1vqNrfl04");
        fs::write(
            s_dir.join("REFLECT.md"),
            "# Reflection - Multi Reflect\n\n### L-01: First insight\n\nFirst observation about caching\n",
        )
        .unwrap();

        run(
            temp.path(),
            "1vqNrfl04",
            false,
            Some("### L-02: Second observation"),
        )
        .unwrap();

        let content = fs::read_to_string(s_dir.join("REFLECT.md")).unwrap();
        assert!(
            content.contains("First observation about caching"),
            "Original reflection should be preserved"
        );
        assert!(
            content.contains("---"),
            "Should have separator between reflections"
        );
        assert!(
            content.contains("Second observation"),
            "Should contain the new reflection"
        );
    }
}
