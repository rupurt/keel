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
use crate::domain::model::StoryState;
use crate::domain::state_machine::{
    BlockingMode, EnforcementPolicy, StoryTransition, TransitionEntity, TransitionIntent,
    enforce_transition, format_enforcement_error,
};
use crate::domain::transitions::{execute, transitions};
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;
use anyhow::{Context, Result, anyhow};
use chrono::Local;
use owo_colors::OwoColorize;

pub struct StoryLifecycleService;

impl StoryLifecycleService {
    /// Start a story (backlog/rejected -> in-progress).
    pub fn start(board_dir: &Path, id: &str, version: Option<u64>) -> Result<()> {
        let board = load_board(board_dir)?;

        // Early fetch story info for warnings.
        let story = board.require_story(id)?;
        let was_rejected = story.stage == StoryState::Rejected;
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

        let transition = if story.stage == StoryState::Rejected {
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

        let spec = if report.requires_human_review() {
            println!("Automated verification passed, but manual review is required.");
            &transitions::SUBMIT
        } else {
            println!("All automated verification passed. Auto-completing story.");
            &transitions::SUBMIT_DONE
        };

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

        let result = execute(board_dir, story.id(), &transitions::ACCEPT)?;

        println!("Accepted: {}", result.story.id());

        if let Some(text) = reflect {
            let story_bundle_dir = result.story.path.parent().unwrap();
            let reflect_path = story_bundle_dir.join("REFLECT.md");

            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&reflect_path)
                .with_context(|| {
                    format!(
                        "Failed to open REFLECT.md for story: {}",
                        reflect_path.display()
                    )
                })?;

            if reflect_path.metadata()?.len() > 0 {
                write!(file, "\n---\n\n{}\n", text)
                    .with_context(|| "Failed to append reflection")?;
            } else {
                write!(
                    file,
                    "# Reflection - {}\n\n{}\n",
                    result.story.frontmatter.title, text
                )
                .with_context(|| "Failed to write reflection")?;
            }
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

    #[test]
    fn accept_requires_human_flag_for_manual_verification_stories() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("MANUAL01")
                    .title("Manual Verification Story")
                    .stage(StoryState::NeedsHumanVerification)
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
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [x] Manual verification <!-- verify: manual -->"),
            )
            .build();

        StoryLifecycleService::accept(temp.path(), "MANUAL02", true, None).unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let story = board.require_story("MANUAL02").unwrap();
        assert_eq!(story.stage, StoryState::Done);
    }
}
