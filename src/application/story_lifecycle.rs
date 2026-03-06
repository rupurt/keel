//! Story lifecycle application service.
//!
//! This module owns orchestration for story lifecycle transitions so CLI
//! command handlers can remain thin interface adapters.

use std::fs;
use std::io::Write;
use std::path::Path;

use crate::application::domain_events::DomainEvent;
use crate::application::knowledge_context;
use crate::application::process_manager::DomainProcessManager;
use crate::domain::model::{Story, StoryState};
use crate::domain::state_machine::{
    BlockingMode, EnforcementPolicy, StoryTransition, TransitionEntity, TransitionIntent,
    enforce_transition, format_enforcement_error,
};
use crate::domain::transitions::{execute, transitions};
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;
use anyhow::{Context, Result, anyhow, bail};
use chrono::Local;
use owo_colors::OwoColorize;

pub struct StoryLifecycleService;

impl StoryLifecycleService {
    /// Start a story (backlog/rejected -> in-progress).
    pub fn start(board_dir: &Path, id: &str, version: Option<u64>) -> Result<()> {
        let board = load_board(board_dir)?;

        // Early fetch story info for warnings.
        let story = board.require_story(id)?;
        let was_rejected = story.status == StoryState::Rejected;
        let scope = story.frontmatter.scope.clone();
        let epic = story.epic().map(String::from);

        if let Some(expected) = version {
            let actual = board.snapshot_version();
            if actual != expected {
                return Err(anyhow!(
                    "Board state has changed (expected version {}, actual {}). Please review and try again.",
                    expected,
                    actual
                ));
            }
        }

        let transition = if story.status == StoryState::Rejected {
            StoryTransition::Restart
        } else {
            StoryTransition::Start
        };
        let intent = TransitionIntent::Story(transition);
        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            intent,
            EnforcementPolicy::RUNTIME,
        );
        if !enforcement.allows_transition() {
            return Err(anyhow!(format_enforcement_error(
                &format!("story {}", story.id()),
                intent,
                &enforcement.blocking_problems
            )));
        }

        DomainProcessManager::default().handle(
            board_dir,
            DomainEvent::StoryStarted {
                story_id: story.id().to_string(),
                scope: scope.clone(),
            },
        )?;

        let result = execute(board_dir, story.id(), &transitions::START)?;
        if story.frontmatter.started_at.is_none() {
            let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            set_started_at(&result.story.path, &now)?;
        }

        if was_rejected {
            println!("Restarted rejected story: {}", story.id());
        } else {
            println!("Started: {}", story.id());
        }

        knowledge_context::surface_ranked_knowledge(
            board_dir,
            "Relevant knowledge found for this scope:",
            epic.as_deref(),
            scope.as_deref(),
            5,
            Some("Consider applying these to your implementation."),
        )?;

        Ok(())
    }

    /// Submit a story (in-progress -> needs-human-verification or done).
    pub fn submit(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let story = board.require_story(id)?;

        let intent = TransitionIntent::Story(StoryTransition::Submit);
        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            intent,
            EnforcementPolicy::RUNTIME,
        );
        if !enforcement.allows_transition() {
            return Err(anyhow!(format_enforcement_error(
                &format!("story {}", story.id()),
                intent,
                &enforcement.blocking_problems
            )));
        }

        knowledge_context::surface_ranked_knowledge(
            board_dir,
            "Relevant knowledge to reference in reflection:",
            story.epic(),
            story.scope(),
            5,
            Some("Link reused insights in REFLECT.md before submitting when appropriate."),
        )?;

        guard_reflection_knowledge_uniqueness(board_dir, story)?;

        let content = fs::read_to_string(&story.path)?;
        println!("Running verification for {}...", story.id());
        let report = verification::verify_story(board_dir, story.id(), &content)?;

        if !report.passed() {
            println!();
            verification::print_terminal_report(&report);
            return Err(anyhow!(
                "Verification failed for story {}. Please fix the issues and try again.",
                story.id()
            ));
        }

        let auto_complete = !report.requires_human_review();
        let spec = if !auto_complete {
            println!("Automated verification passed, but manual review is required.");
            &transitions::SUBMIT
        } else {
            println!("All automated verification passed. Auto-completing story.");
            &transitions::SUBMIT_DONE
        };

        if auto_complete {
            materialize_story_reflection_knowledge(board_dir, story)?;
        }

        let result = execute(board_dir, story.id(), spec)?;

        if result.to == StoryState::Done {
            println!("Completed: {}", result.story.id());
        } else {
            println!("Submitted: {}", result.story.id());
            println!("  Moved to: needs-human-verification");
        }

        crate::cli::commands::generate::run(board_dir)?;

        Ok(())
    }

    /// Accept a story (needs-human-verification -> done).
    pub fn accept(board_dir: &Path, id: &str, human: bool, reflect: Option<&str>) -> Result<()> {
        let board = load_board(board_dir)?;
        let story = board.require_story(id)?;

        let policy = if human {
            EnforcementPolicy {
                blocking_mode: BlockingMode::Strict,
                require_human_review_for_manual_acceptance: false,
                ..EnforcementPolicy::RUNTIME
            }
        } else {
            EnforcementPolicy::RUNTIME
        };

        let intent = TransitionIntent::Story(StoryTransition::Accept);
        let enforcement =
            enforce_transition(&board, TransitionEntity::Story(story), intent, policy);

        if !enforcement.allows_transition() {
            let mut msg = format_enforcement_error(
                &format!("story {}", story.id()),
                intent,
                &enforcement.blocking_problems,
            );

            if !human {
                msg.push_str("\nHint: Some checks require human oversight. Use --human if you have manually verified this.");
            }

            return Err(anyhow!(msg));
        }

        let reflect_path = reflect_path_for_story(story)?;
        if let Some(text) = reflect {
            append_accept_reflection(
                &reflect_path,
                &story.frontmatter.title,
                text,
                reflection_created_at(story),
            )?;
        }

        guard_reflection_knowledge_uniqueness(board_dir, story)?;
        materialize_story_reflection_knowledge(board_dir, story)?;

        let result = execute(board_dir, story.id(), &transitions::ACCEPT)?;

        println!("Accepted: {}", result.story.id());
        if reflect.is_some() {
            println!(
                "  ✓ Reflection recorded in {}",
                reflect_path.display().dimmed()
            );
        }

        DomainProcessManager::default().handle(
            board_dir,
            DomainEvent::StoryAccepted {
                story_id: result.story.id().to_string(),
                scope: result.story.scope().map(str::to_string),
            },
        )?;

        crate::cli::commands::generate::run(board_dir)?;

        Ok(())
    }

    /// Reject a story (needs-human-verification -> rejected).
    pub fn reject(board_dir: &Path, id: &str, reason: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let story = board.require_story(id)?;
        let intent = TransitionIntent::Story(StoryTransition::Reject);
        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            intent,
            EnforcementPolicy::RUNTIME,
        );
        if !enforcement.allows_transition() {
            return Err(anyhow!(format_enforcement_error(
                &format!("story {}", story.id()),
                intent,
                &enforcement.blocking_problems
            )));
        }

        let result = execute(board_dir, story.id(), &transitions::REJECT)?;
        append_rejection(&result.story.path, reason)?;

        println!("Rejected: {}", result.story.filename());
        println!("  Reason: {}", reason);

        Ok(())
    }

    /// Move a story to icebox.
    pub fn ice(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let story = board.require_story(id)?;
        let intent = TransitionIntent::Story(StoryTransition::Ice);
        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            intent,
            EnforcementPolicy::RUNTIME,
        );
        if !enforcement.allows_transition() {
            return Err(anyhow!(format_enforcement_error(
                &format!("story {}", story.id()),
                intent,
                &enforcement.blocking_problems
            )));
        }

        let result = execute(board_dir, story.id(), &transitions::ICE)?;

        println!("Iced: {}", result.story.filename());

        Ok(())
    }

    /// Move a story from icebox to backlog.
    pub fn thaw(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let story = board.require_story(id)?;
        let intent = TransitionIntent::Story(StoryTransition::Thaw);
        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            intent,
            EnforcementPolicy::RUNTIME,
        );
        if !enforcement.allows_transition() {
            return Err(anyhow!(format_enforcement_error(
                &format!("story {}", story.id()),
                intent,
                &enforcement.blocking_problems
            )));
        }

        let result = execute(board_dir, story.id(), &transitions::THAW)?;

        println!("Thawed: {}", result.story.filename());

        Ok(())
    }
}

fn set_started_at(path: &Path, datetime: &str) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read story: {}", path.display()))?;
    let updated = apply(&content, &[Mutation::set("started_at", datetime)]);
    if updated != content {
        fs::write(path, updated)
            .with_context(|| format!("Failed to write story: {}", path.display()))?;
    }
    Ok(())
}

fn reflect_path_for_story(story: &Story) -> Result<std::path::PathBuf> {
    story
        .path
        .parent()
        .map(|dir| dir.join("REFLECT.md"))
        .with_context(|| format!("Story {} has no bundle directory", story.id()))
}

fn reflection_created_at(story: &Story) -> chrono::NaiveDateTime {
    story
        .frontmatter
        .submitted_at
        .unwrap_or_else(|| Local::now().naive_local())
}

fn append_accept_reflection(
    reflect_path: &Path,
    title: &str,
    text: &str,
    created_at: chrono::NaiveDateTime,
) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(reflect_path)
        .with_context(|| {
            format!(
                "Failed to open REFLECT.md for story: {}",
                reflect_path.display()
            )
        })?;

    if reflect_path.metadata()?.len() > 0 {
        write!(file, "\n---\n\n{}\n", text).with_context(|| "Failed to append reflection")?;
    } else {
        write!(
            file,
            "---\ncreated_at: {}\n---\n\n# Reflection - {}\n\n## Knowledge\n\n## Observations\n\n{}\n",
            created_at.format("%Y-%m-%dT%H:%M:%S"),
            title,
            text
        )
        .with_context(|| "Failed to write reflection")?;
    }
    Ok(())
}

fn guard_reflection_knowledge_uniqueness(board_dir: &Path, story: &Story) -> Result<()> {
    let reflect_path = reflect_path_for_story(story)?;
    if !reflect_path.exists() {
        return Ok(());
    }

    let candidates = crate::read_model::knowledge::parse_reflection_candidates(
        &reflect_path,
        story.scope(),
        Some(story.id()),
    )?;
    if candidates.is_empty() {
        return Ok(());
    }

    let existing = crate::read_model::knowledge::scan_all_knowledge(board_dir)?;
    let conflicts = crate::read_model::knowledge::detect_similarity_conflicts(
        &candidates,
        &existing,
        crate::read_model::knowledge::NEAR_DUPLICATE_KNOWLEDGE_THRESHOLD,
    );

    if conflicts.is_empty() {
        return Ok(());
    }

    let details = conflicts
        .into_iter()
        .map(|conflict| {
            format!(
                "{} -> {} ({:.2})",
                conflict.candidate_id, conflict.existing_id, conflict.similarity_score
            )
        })
        .collect::<Vec<_>>()
        .join(", ");

    bail!(
        "Reflection knowledge is too similar to existing knowledge: {}. Link the existing `.keel/knowledge/<id>.md` file in REFLECT.md or add an explicit Linked Knowledge IDs reference before retrying.",
        details
    );
}

fn materialize_story_reflection_knowledge(board_dir: &Path, story: &Story) -> Result<()> {
    let reflect_path = reflect_path_for_story(story)?;
    if !reflect_path.exists() {
        return Ok(());
    }

    let _ = crate::read_model::knowledge::materialize_reflection_knowledge(
        board_dir,
        &reflect_path,
        story.scope(),
        Some(story.id()),
    )?;
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn is_relevant_knowledge(
    knowledge: &crate::read_model::knowledge::Knowledge,
    epic_id: Option<&str>,
    scope: Option<&str>,
) -> bool {
    if let Some(s) = scope
        && knowledge.scope.as_deref() == Some(s)
    {
        return true;
    }

    if let Some(e) = epic_id
        && let Some(l_scope) = &knowledge.scope
        && l_scope.starts_with(e)
    {
        return true;
    }

    false
}

pub(crate) fn append_rejection(story_path: &Path, reason: &str) -> Result<()> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let content = fs::read_to_string(story_path)
        .with_context(|| format!("Failed to read story: {}", story_path.display()))?;

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(story_path)
        .with_context(|| {
            format!(
                "Failed to open story for rejection: {}",
                story_path.display()
            )
        })?;

    if !content.contains("## Rejections") {
        write!(file, "\n## Rejections\n").with_context(|| "Failed to write rejections header")?;
    }

    write!(file, "\n### {}\n\n{}\n", today, reason)
        .with_context(|| "Failed to append rejection")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::StoryLifecycleService;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use std::fs;

    #[test]
    fn accept_requires_human_flag_for_manual_verification_stories() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("MANUAL01")
                    .title("Manual Verification Story")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Manual verification <!-- verify: manual -->"),
            )
            .build();

        let err = StoryLifecycleService::accept(temp.path(), "MANUAL01", false, None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("--human"),
            "manual verification enforcement should require --human flag: {err}"
        );
    }

    #[test]
    fn accept_allows_manual_verification_stories_with_human_flag() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("MANUAL02")
                    .title("Manual Verification Story")
                    .status(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Manual verification <!-- verify: manual -->"),
            )
            .build();

        StoryLifecycleService::accept(temp.path(), "MANUAL02", true, None).unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let story = board.require_story("MANUAL02").unwrap();
        assert_eq!(story.status, StoryState::Done);
    }

    #[test]
    fn submit_blocks_near_duplicate_reflection_knowledge() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("OLD000001")
                    .title("Existing Story")
                    .status(StoryState::Done)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] done <!-- verify: manual, SRS-01:start:end -->",
                    ),
            )
            .story(
                TestStory::new("NEW000001")
                    .title("New Story")
                    .status(StoryState::InProgress)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] done <!-- verify: manual, SRS-01:start:end -->",
                    ),
            )
            .build();

        fs::write(
            temp.path().join("stories/OLD000001/REFLECT.md"),
            r#"# Reflection - Existing Story

## Knowledge

### 1AbCdE239: Prefer Linked Reflection Knowledge

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | when a story discovers reusable reflection guidance |
| **Insight** | Duplicate reflection knowledge should be linked instead of restated |
| **Suggested Action** | Reference the existing catalog file in REFLECT.md |
"#,
        )
        .unwrap();
        fs::write(
            temp.path().join("stories/NEW000001/REFLECT.md"),
            r#"# Reflection - New Story

## Knowledge

### 1AbCdE240: Prefer Linked Reflection Knowledge

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | when a story discovers reusable reflection guidance |
| **Insight** | Duplicate reflection knowledge should be linked instead of restated |
| **Suggested Action** | Reference the existing catalog file in REFLECT.md |
"#,
        )
        .unwrap();

        let err = StoryLifecycleService::submit(temp.path(), "NEW000001")
            .unwrap_err()
            .to_string();
        assert!(err.contains("too similar to existing knowledge"));
        assert!(err.contains("1AbCdE239"));
    }

    #[test]
    fn accept_materializes_unique_reflection_knowledge_when_story_reaches_done() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("DONE00001")
                    .title("Done Story")
                    .status(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] done <!-- verify: manual, SRS-01:start:end -->",
                    ),
            )
            .build();

        fs::write(
            temp.path().join("stories/DONE00001/REFLECT.md"),
            r#"# Reflection - Done Story

## Knowledge

### 1AbCdE241: Materialize Reflection Knowledge

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | when a human accepts a story after verification |
| **Insight** | Unique reflection knowledge should move into dedicated files |
| **Suggested Action** | Rewrite the reflection to link the knowledge file after acceptance |

## Observations

The guard should run before acceptance completes.
"#,
        )
        .unwrap();

        StoryLifecycleService::accept(temp.path(), "DONE00001", true, None).unwrap();

        let knowledge_path = temp.path().join("knowledge/1AbCdE241.md");
        assert!(knowledge_path.exists());

        let reflect = fs::read_to_string(temp.path().join("stories/DONE00001/REFLECT.md")).unwrap();
        assert!(reflect.contains("- [1AbCdE241](../../knowledge/1AbCdE241.md)"));
        assert!(!reflect.contains("### 1AbCdE241: Materialize Reflection Knowledge"));
    }
}
