//! New story command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::domain::model::{AdrStatus, Board, StoryState};
use crate::domain::transitions::{TimestampUpdates, update_frontmatter};
use crate::infrastructure::duplicate_ids::{self, DuplicateEntity};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::story_id::generate_story_id;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;
use crate::infrastructure::utils::slugify;

/// Create a new story
pub fn run(title: &str, story_type: &str, epic: Option<&str>, voyage: Option<&str>) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    new_story(&board_dir, title, story_type, epic, voyage)
}

/// Insert a field into frontmatter after a specific field
pub fn insert_frontmatter_field(content: &str, after_field: &str, new_line: &str) -> String {
    let marker = format!(
        "{}
",
        after_field
    );
    if let Some(pos) = content.find(&marker) {
        let insert_pos = pos + marker.len();
        let mut result = content[..insert_pos].to_string();
        result.push_str(new_line);
        result.push('\n');
        result.push_str(&content[insert_pos..]);
        result
    } else {
        content.to_string()
    }
}

/// Find the next index number for a given scope
fn find_next_index(board: &Board, scope: &str) -> u32 {
    let max_seq = board
        .stories
        .values()
        .filter(|s| s.scope() == Some(scope))
        .filter_map(|s| s.index())
        .max()
        .unwrap_or(0);

    max_seq + 1
}

/// Find accepted ADRs that govern a given bounded context (epic)
fn find_governing_adrs(board: &Board, context: &str) -> Vec<String> {
    let mut adr_ids: Vec<String> = board
        .adrs
        .values()
        .filter(|adr| {
            if adr.frontmatter.status != AdrStatus::Accepted {
                return false;
            }
            if adr.frontmatter.context.as_deref() == Some(context) {
                return true;
            }
            adr.frontmatter.applies_to.iter().any(|s| s == context)
        })
        .map(|adr| adr.id().to_string())
        .collect();

    adr_ids.sort();
    adr_ids
}

struct BlockingAdr {
    id: String,
    title: String,
}

fn find_blocking_adrs(board: &Board, context: &str) -> Vec<BlockingAdr> {
    let mut blocking: Vec<BlockingAdr> = board
        .adrs
        .values()
        .filter(|adr| {
            if adr.frontmatter.status != AdrStatus::Proposed {
                return false;
            }
            if adr.frontmatter.context.as_deref() == Some(context) {
                return true;
            }
            adr.frontmatter.applies_to.iter().any(|s| s == context)
        })
        .map(|adr| BlockingAdr {
            id: adr.id().to_string(),
            title: adr.frontmatter.title.clone(),
        })
        .collect();

    blocking.sort_by(|a, b| a.id.cmp(&b.id));
    blocking
}

fn icebox_guidance_commands(story_id: &str) -> [String; 2] {
    [
        format!("keel story thaw {story_id}"),
        format!("keel story start {story_id}"),
    ]
}

fn render_icebox_guidance(story_id: &str) -> String {
    let [thaw, start] = icebox_guidance_commands(story_id);
    format!("\nNext steps:\n  {thaw}\n  {start}\n")
}

/// Create a new story
fn new_story(
    board_dir: &Path,
    title: &str,
    story_type: &str,
    epic: Option<&str>,
    voyage: Option<&str>,
) -> Result<()> {
    duplicate_ids::ensure_unique_ids(board_dir, DuplicateEntity::Story, "keel story new")?;

    // Enforce Title Case
    if !crate::infrastructure::utils::is_title_case(title) {
        return Err(anyhow!(
            "Story title '{}' must use Title Case (e.g. 'My Story Title')",
            title
        ));
    }

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let _slug = slugify(title);

    let scope = match (epic, voyage) {
        (Some(e), Some(m)) => Some(format!("{}/{}", e, m)),
        (Some(e), None) => Some(e.to_string()),
        _ => None,
    };

    let board = load_board(board_dir)?;

    if let Some(context) = epic {
        let blocking = find_blocking_adrs(&board, context);
        if !blocking.is_empty() {
            let adr = &blocking[0];
            return Err(anyhow!(
                "Cannot create story in '{}' context


                 {} ({}) is proposed and blocks

                 new work in this area until accepted.


                 To proceed:
  
                 keel adr accept {}
  
                 keel adr reject {} \"reason\"",
                context,
                adr.id,
                adr.title,
                adr.id,
                adr.id
            ));
        }
    }

    if let (Some(epic_id), Some(voyage_id)) = (epic, voyage) {
        let voyage = board.require_voyage(voyage_id)?;
        if voyage.epic_id != epic_id {
            return Err(anyhow!(
                "Cannot create scoped story: voyage '{}' is in epic '{}', expected '{}'",
                voyage_id,
                voyage.epic_id,
                epic_id
            ));
        }
    }

    let index = scope.as_ref().map(|s| find_next_index(&board, s));

    let governing_adrs = epic
        .map(|e| find_governing_adrs(&board, e))
        .unwrap_or_default();

    let story_id = generate_story_id();
    let story_bundle_dir = board_dir.join("stories").join(&story_id);
    fs::create_dir_all(&story_bundle_dir).with_context(|| {
        format!(
            "Failed to create story directory: {}",
            story_bundle_dir.display()
        )
    })?;

    let story_path = story_bundle_dir.join("README.md");

    let mut content = template_rendering::render(
        templates::story::STORY,
        &[
            ("id", &story_id),
            ("title", title),
            ("type", story_type),
            ("created_at", &now),
            ("updated_at", &now),
        ],
    );

    if let Some(s) = &scope {
        content = insert_frontmatter_field(
            &content,
            &format!("updated_at: {}", now),
            &format!("scope: {}", s),
        );
    }
    if let (Some(s), Some(seq)) = (&scope, index) {
        content = insert_frontmatter_field(
            &content,
            &format!("scope: {}", s),
            &format!("index: {}", seq),
        );
    }
    if !governing_adrs.is_empty() {
        let insert_after = if let (Some(_), Some(seq)) = (&scope, index) {
            format!("index: {}", seq)
        } else if let Some(s) = &scope {
            format!("scope: {}", s)
        } else {
            format!("updated_at: {}", now)
        };
        let governed_by_value = format!("[{}]", governing_adrs.join(", "));
        content = insert_frontmatter_field(
            &content,
            &insert_after,
            &format!("governed-by: {}", governed_by_value),
        );
    }

    // Canonical hard-cutover path: all newly created stories enter Icebox.
    content = update_frontmatter(
        &content,
        StoryState::Icebox,
        &TimestampUpdates::updated_only(),
    )?;

    fs::write(&story_path, content)
        .with_context(|| format!("Failed to write story: {}", story_path.display()))?;

    // Create EVIDENCE directory
    let evidence_dir = story_bundle_dir.join("EVIDENCE");
    fs::create_dir_all(&evidence_dir).with_context(|| {
        format!(
            "Failed to create evidence directory: {}",
            evidence_dir.display()
        )
    })?;

    println!("Created: stories/{}/", story_id);

    crate::cli::commands::generate::run(board_dir)?;
    print!("{}", render_icebox_guidance(&story_id));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::diagnostics::doctor::checks::stories::check_index_validation;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestAdr, TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    fn story_content_by_title(board_dir: &Path, title: &str) -> Option<String> {
        let stories_dir = board_dir.join("stories");
        for entry in fs::read_dir(stories_dir).ok()?.flatten() {
            let readme = entry.path().join("README.md");
            if !readme.exists() {
                continue;
            }
            let content = fs::read_to_string(readme).ok()?;
            if content.contains(&format!("title: {title}")) {
                return Some(content);
            }
        }
        None
    }

    #[test]
    fn render_template_replaces_placeholders() {
        let template = "Story {{title}} created at {{created_at}}";
        let result = template_rendering::render(
            template,
            &[("title", "World"), ("created_at", "2026-03-02T00:00:00")],
        );
        assert_eq!(result, "Story World created at 2026-03-02T00:00:00");
    }

    #[test]
    fn insert_frontmatter_field_inserts_after_marker() {
        let content = "---
id: test
status: in-progress
---
";
        let result = insert_frontmatter_field(content, "id: test", "title: New");
        assert_eq!(
            result,
            "---
id: test
title: New
status: in-progress
---
"
        );
    }

    #[test]
    fn new_story_creates_file() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("planned"))
            .story(
                TestStory::new("EXIST1")
                    .title("Existing Story")
                    .stage(StoryState::Backlog),
            )
            .build();
        let board_dir = temp.path();

        new_story(board_dir, "Test Feature", "feat", None, None).unwrap();

        let stories_dir = board_dir.join("stories");
        let story_dirs: Vec<_> = fs::read_dir(&stories_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();

        // Should have original + new story
        assert!(story_dirs.len() >= 2);

        // Find the new story README
        let mut found = false;
        for entry in story_dirs {
            let readme = entry.path().join("README.md");
            let reflect = entry.path().join("REFLECT.md");
            let evidence = entry.path().join("EVIDENCE");

            if readme.exists() {
                let content = fs::read_to_string(readme).unwrap();
                if content.contains("title: Test Feature") {
                    found = true;
                    assert!(content.contains("type: feat"));
                    assert!(content.contains("status: icebox"));
                    assert!(!reflect.exists(), "REFLECT.md should NOT exist in bundle");
                    assert!(
                        evidence.is_dir(),
                        "EVIDENCE directory should exist in bundle"
                    );
                    break;
                }
            }
        }
        assert!(found, "New story bundle should exist with correct content");
    }

    #[test]
    fn new_story_fails_when_duplicate_story_ids_exist() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("DUP1").title("Duplicate One"))
            .build();
        let board_dir = temp.path();

        let existing = fs::read_to_string(board_dir.join("stories/DUP1/README.md")).unwrap();
        fs::create_dir_all(board_dir.join("stories/DUP2")).unwrap();
        fs::write(board_dir.join("stories/DUP2/README.md"), existing).unwrap();

        let err = new_story(board_dir, "Blocked Story", "feat", None, None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("duplicate story IDs"));
        assert!(err.contains("keel doctor"));
    }

    #[test]
    fn story_new_defaults_to_icebox() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1").status("planned"))
            .build();
        let board_dir = temp.path();

        new_story(board_dir, "Unscoped Story", "feat", None, None).unwrap();
        new_story(board_dir, "Scoped Story", "feat", Some("epic1"), Some("v1")).unwrap();

        let unscoped = story_content_by_title(board_dir, "Unscoped Story").unwrap();
        let scoped = story_content_by_title(board_dir, "Scoped Story").unwrap();
        assert!(unscoped.contains("status: icebox"));
        assert!(scoped.contains("status: icebox"));
        assert!(!unscoped.contains("status: backlog"));
        assert!(!scoped.contains("status: backlog"));
    }

    #[test]
    fn story_new_icebox_guidance() {
        let rendered = render_icebox_guidance("STORY123");
        assert!(rendered.contains("keel story thaw STORY123"));
        assert!(rendered.contains("keel story start STORY123"));
    }

    #[test]
    fn story_new_planned_voyage_doctor_coherence() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("epic1/v1")
                    .index(1)
                    .stage(StoryState::Icebox),
            )
            .build();
        let board_dir = temp.path();

        new_story(
            board_dir,
            "Second Planned Story",
            "feat",
            Some("epic1"),
            Some("v1"),
        )
        .unwrap();

        let board = load_board(board_dir).unwrap();
        let problems = check_index_validation(&board);
        assert!(
            problems.iter().all(|p| !p.message.contains("out of order")),
            "new story default should not trigger planned-voyage out-of-order warning: {:?}",
            problems
        );
    }

    #[test]
    fn story_new_canonical_stage_path() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("draft-v", "epic1").status("draft"))
            .voyage(TestVoyage::new("planned-v", "epic1").status("planned"))
            .build();
        let board_dir = temp.path();

        new_story(board_dir, "Canonical Unscoped", "feat", None, None).unwrap();
        new_story(
            board_dir,
            "Canonical Draft Scoped",
            "feat",
            Some("epic1"),
            Some("draft-v"),
        )
        .unwrap();
        new_story(
            board_dir,
            "Canonical Planned Scoped",
            "feat",
            Some("epic1"),
            Some("planned-v"),
        )
        .unwrap();

        for title in [
            "Canonical Unscoped",
            "Canonical Draft Scoped",
            "Canonical Planned Scoped",
        ] {
            let content = story_content_by_title(board_dir, title).unwrap();
            assert!(
                content.contains("status: icebox"),
                "expected icebox stage for {}:\n{}",
                title,
                content
            );
            assert!(
                !content.contains("status: backlog"),
                "legacy backlog default should not remain for {}:\n{}",
                title,
                content
            );
        }
    }

    #[test]
    fn new_story_with_governing_adr_has_governed_by() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").status("in-progress"))
            .adr(
                TestAdr::new("ADR-0001")
                    .title("Keel ADR")
                    .status("accepted")
                    .context("keel"),
            )
            .build();
        let board_dir = temp.path();

        new_story(
            board_dir,
            "Test Story",
            "feat",
            Some("keel"),
            Some("01-test"),
        )
        .unwrap();

        let stories_dir = board_dir.join("stories");
        let story_dirs: Vec<_> = fs::read_dir(&stories_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect();

        let mut found = false;
        for entry in story_dirs {
            let readme = entry.path().join("README.md");
            if readme.exists() {
                let content = fs::read_to_string(readme).unwrap();
                if content.contains("title: Test Story") {
                    found = true;
                    assert!(
                        content.contains("governed-by: [ADR-0001]"),
                        "Story should have governed-by with ADR-0001. Content: {}",
                        content
                    );
                    break;
                }
            }
        }
        assert!(
            found,
            "New story bundle should exist with governed-by field"
        );
    }

    #[test]
    fn test_find_next_index() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").scope("epic1/v1").index(1))
            .story(TestStory::new("S2").scope("epic1/v1").index(2))
            .story(TestStory::new("S3").scope("epic2/v1").index(5))
            .build();
        let board = load_board(temp.path()).unwrap();

        assert_eq!(find_next_index(&board, "epic1/v1"), 3);
        assert_eq!(find_next_index(&board, "epic2/v1"), 6);
        assert_eq!(find_next_index(&board, "unknown"), 1);
    }

    #[test]
    fn test_find_governing_adrs() {
        let temp = TestBoardBuilder::new()
            .adr(TestAdr::new("ADR-1").status("accepted").context("epic1"))
            .adr(TestAdr::new("ADR-2").status("proposed").context("epic1"))
            .adr(TestAdr::new("ADR-3").status("accepted").context("other"))
            .build();
        let board = load_board(temp.path()).unwrap();

        let adrs = find_governing_adrs(&board, "epic1");
        assert_eq!(adrs.len(), 1);
        assert_eq!(adrs[0], "ADR-1");
    }

    #[test]
    fn test_find_blocking_adrs() {
        let temp = TestBoardBuilder::new()
            .adr(TestAdr::new("ADR-1").status("proposed").context("epic1"))
            .adr(TestAdr::new("ADR-2").status("accepted").context("epic1"))
            .build();
        let board = load_board(temp.path()).unwrap();

        let blocking = find_blocking_adrs(&board, "epic1");
        assert_eq!(blocking.len(), 1);
        assert_eq!(blocking[0].id, "ADR-1");
    }
}
