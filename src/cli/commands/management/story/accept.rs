//! Accept command - accept a verified story and move to done

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::application::story_lifecycle::StoryLifecycleService;
use crate::infrastructure::loader::load_board;

use super::guidance::{
    StoryLifecycleAction, error_with_recovery, guidance_for_action, print_human,
};

pub(crate) fn legacy_story_accept_flag_guidance(args: &[String]) -> Option<String> {
    let story_pos = args.iter().position(|arg| arg == "story")?;
    let accept_pos = args[story_pos + 1..]
        .iter()
        .position(|arg| arg == "accept")?
        + story_pos
        + 1;
    let accept_args = &args[accept_pos + 1..];

    if !accept_args.iter().any(|arg| arg == "--human") {
        return None;
    }

    let mut message = String::from(
        "`keel story accept` no longer accepts `--human`. Use `--role manager/product` to authorize acceptance.",
    );
    if accept_args.iter().any(|arg| arg == "--role") {
        message.push_str(" Do not combine `--human` with `--role`.");
    }
    Some(message)
}

/// Run the accept command
pub fn run(board_dir: &Path, id: &str, role: &str, reflect: Option<&str>) -> Result<()> {
    let actor_role = crate::domain::model::taxonomy::parse(role)
        .map_err(|err| anyhow!("Invalid role taxonomy `{role}`: {err}"))?;

    StoryLifecycleService::accept(board_dir, id, &actor_role, reflect)
        .map_err(|err| error_with_recovery(StoryLifecycleAction::Accept, id, err))?;

    let board = load_board(board_dir)?;
    let story = board.require_story(id)?;
    let guidance = guidance_for_action(StoryLifecycleAction::Accept, story.status, story.id());
    print_human(guidance.as_ref());

    Ok(())
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
                    .status(StoryState::NeedsHumanVerification)
                    .scope("test-epic/01-voyage"),
            )
            .build();

        run(temp.path(), "READY1", "engineer/software", None).unwrap();

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
                    .status(StoryState::NeedsHumanVerification)
                    .scope("test-epic/01-voyage"),
            )
            .build();

        run(temp.path(), "UPDATE1", "engineer/software", None).unwrap();

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
    fn accept_errors_on_manual_verification_without_manager_role() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH1")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: manual -->"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH1", "engineer/software", None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("manual acceptance criteria"),
            "Error should mention manual verification: {}",
            err
        );
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story accept 1vkqtsHH1 --role manager/product"));
    }

    #[test]
    fn accept_with_manager_role_succeeds_for_manual_stories() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH2")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: manual -->"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH2", "manager/product", None);
        assert!(
            result.is_ok(),
            "Should succeed with manager role: {:?}",
            result
        );
    }

    #[test]
    fn accept_without_manual_verification_succeeds_normally() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH3")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: echo ok -->"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH3", "engineer/software", None);
        assert!(
            result.is_ok(),
            "Should succeed for non-manual stories with any valid role: {:?}",
            result
        );
    }

    #[test]
    fn accept_without_verify_annotations_succeeds_normally() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH4")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Simple criteria"),
            )
            .build();

        let result = run(temp.path(), "1vkqtsHH4", "engineer/software", None);
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
                    .status(StoryState::NeedsHumanVerification)
                    .scope("test-epic/01-first"),
            )
            .build();

        run(temp.path(), "1vkqtsAAA", "engineer/software", None).unwrap();

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
                    .status(StoryState::NeedsHumanVerification)
                    .body("\n## Acceptance Criteria\n\n- [x] Something done"),
            )
            .build();

        run(
            temp.path(),
            "1vqNrfl01",
            "engineer/software",
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
                    .status(StoryState::NeedsHumanVerification)
                    .body("\n## Acceptance Criteria\n\n- [x] Something done"),
            )
            .build();

        run(
            temp.path(),
            "1vqNrfl02",
            "engineer/software",
            Some("Latency was key"),
        )
        .unwrap();

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
            .story(TestStory::new("1vqNrfl03").status(StoryState::NeedsHumanVerification))
            .build();

        run(temp.path(), "1vqNrfl03", "engineer/software", None).unwrap();

        let reflect_path = temp.path().join("stories/1vqNrfl03/REFLECT.md");
        // It now exists by default because of TestBoardBuilder
        assert!(reflect_path.exists(), "REFLECT.md should exist by default");
    }

    #[test]
    fn multiple_reflections_append() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("1vqNrfl04").status(StoryState::NeedsHumanVerification))
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
            "engineer/software",
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
