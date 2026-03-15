use keel::application::process_manager::{DomainProcessManager, LiveProcessActionExecutor};
use keel::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
/// Accept command - accept a verified story and move to done
use keel::infrastructure::storage::filesystem::FileSystemAdapter;
use std::path::Path;

use std::sync::Arc;

use anyhow::{Result, anyhow};

use keel::application::story_lifecycle::StoryLifecycleService;
use keel::infrastructure::loader::load_board;
use keel::read_model::workflow_topology;

use super::guidance::{
    StoryLifecycleAction, error_with_recovery_for_accept_role, guidance_for_action, print_human,
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

    let management_role = workflow_topology::current_default_role_examples()
        .map(|(management_role, _)| management_role)
        .unwrap_or_else(|| "manager".to_string());
    let mut message = format!(
        "`keel story accept` no longer accepts `--human`. Use `--role {management_role}` to authorize acceptance."
    );
    if accept_args.iter().any(|arg| arg == "--role") {
        message.push_str(" Do not combine `--human` with `--role`.");
    }
    Some(message)
}

/// Run the accept command
pub fn run(ctx: &spoke_auth::ExecutionContext, board_dir: &Path, id: &str, role: &str, reflect: Option<&str>) -> Result<()> {
    let adapter = Arc::new(FileSystemAdapter::new(board_dir));
    let voyage_service = Arc::new(VoyageEpicLifecycleService::new(
        board_dir.to_path_buf(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
    ));
    let executor = LiveProcessActionExecutor::new(voyage_service.clone());
    let process_manager = Arc::new(DomainProcessManager::new(executor));

    let service = StoryLifecycleService::new(
        board_dir.to_path_buf(),
        adapter.clone(),
        adapter,
        process_manager,
    );

    let actor_role = keel::domain::model::taxonomy::parse(role)
        .map_err(|err| anyhow!("Invalid role taxonomy `{role}`: {err}"))?;
    let accept_role_example = workflow_topology::load_for_board(board_dir)
        .map(|topology| topology.management_role_example().to_string())
        .unwrap_or_else(|_| "manager".to_string());

    service.accept(ctx, id, &actor_role, reflect).map_err(|err| {
        error_with_recovery_for_accept_role(
            StoryLifecycleAction::Accept,
            id,
            err,
            &accept_role_example,
        )
    })?;

    let board = load_board(board_dir)?;
    let story = board.require_story(id)?;
    let guidance = guidance_for_action(StoryLifecycleAction::Accept, story.status, story.id());
    print_human(guidance.as_ref());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::StoryState;
    use keel::infrastructure::validation::{CheckId, structural};
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use regex::Regex;
    use std::fs;

    fn write_custom_topology_config(path: &Path) {
        fs::write(
            path.join("keel.toml"),
            r#"[workflow.defaults]
management_role = "director"
delivery_role = "maker"
management_lane = "review"
delivery_lane = "delivery"

[roles.director]
default_lane = "review"
operational_contract = "director-core"

[roles.maker]
default_lane = "delivery"
operational_contract = "maker-core"

[lanes.review]
description = "Review and approvals"
include = ["story.needs-human-verification"]
parallel = false
manual_accept = true
priority = 100

[lanes.delivery]
description = "Implementation work"
include = ["story.backlog", "story.in-progress"]
parallel = true
manual_accept = false
priority = 50
"#,
        )
        .unwrap();
    }

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

        run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "READY1", "manager", None).unwrap();

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

        run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "UPDATE1", "manager", None).unwrap();

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
    fn story_accept_topology_rejects_non_manual_accept_lane_with_configured_management_guidance() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH1")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: manual -->"),
            )
            .build();
        write_custom_topology_config(temp.path());

        let result = run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vkqtsHH1", "maker", None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("manual acceptance criteria"),
            "Error should mention manual verification: {}",
            err
        );
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story accept 1vkqtsHH1 --role director"));
    }

    #[test]
    fn story_accept_topology_allows_manual_accept_for_configured_management_role() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH2")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: manual -->"),
            )
            .build();
        write_custom_topology_config(temp.path());

        let result = run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vkqtsHH2", "director", None);
        assert!(
            result.is_ok(),
            "Should succeed with configured management role: {:?}",
            result
        );
    }

    #[test]
    fn story_accept_topology_allows_any_valid_delivery_role_for_non_manual_stories() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsHH3")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Check this <!-- verify: echo ok -->"),
            )
            .build();
        write_custom_topology_config(temp.path());

        let result = run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vkqtsHH3", "maker", None);
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

        let result = run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vkqtsHH4", "operator", None);
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

        run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vkqtsAAA", "manager", None).unwrap();

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
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "1vqNrfl01",
            "manager",
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

        run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vqNrfl02", "manager", Some("Latency was key")).unwrap();

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

        let reflect_path = temp.path().join("stories/1vqNrfl03/REFLECT.md");

        run(&spoke_auth::ExecutionContext::new_local("test".into()), temp.path(), "1vqNrfl03", "manager", None).unwrap();

        assert!(
            !reflect_path.exists(),
            "REFLECT.md should NOT be created if not provided"
        );
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
            &spoke_auth::ExecutionContext::new_local("test".into()),
            temp.path(),
            "1vqNrfl04",
            "manager",
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
