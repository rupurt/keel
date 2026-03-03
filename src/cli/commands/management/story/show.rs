//! Show story command

use std::fs;
use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::style;
use crate::infrastructure::loader::load_board;
use crate::read_model::evidence;

/// Run the show story command
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Run the show story command with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;

    // Find the story
    let story = board.require_story(id)?;

    // Read body content early so we can count ACs
    let content = fs::read_to_string(&story.path)?;
    let body = extract_body(&content);

    // Count ACs
    let (checked, total) = body.map(count_acs).unwrap_or((0, 0));

    let width = crate::cli::presentation::terminal::get_terminal_width();

    // Header
    println!("{}", style::heavy_rule(width, None));
    println!(
        "{}",
        style::header(story.id(), &story.frontmatter.title, style::styled_story_id)
    );
    println!("{}", style::heavy_rule(width, None));
    println!();

    // Metadata
    println!("Type:     {}", style::styled_type(&story.story_type()));
    println!("Status:   {}", style::styled_stage(&story.stage));

    if let Some(scope) = &story.frontmatter.scope {
        let styled_scope = if scope.contains('/') {
            let parts: Vec<_> = scope.split('/').collect();
            if parts.len() >= 2 {
                format!(
                    "{}/{}",
                    style::styled_epic_id(parts[0]),
                    style::styled_voyage_id(parts[1])
                )
            } else {
                style::styled_id(scope)
            }
        } else {
            style::styled_epic_id(scope)
        };
        println!("Scope:    {}", styled_scope);
    }

    if let Some(created_at) = story.frontmatter.created_at {
        println!("Created:  {}", created_at.dimmed());
    }

    if let Some(updated_at) = story.frontmatter.updated_at {
        println!("Updated:  {}", updated_at.dimmed());
    }

    // Progress bar
    if total > 0 {
        println!();
        println!(
            "Progress: {}",
            style::progress_bar(checked, total, 20, None)
        );
    }

    println!();
    println!("Path: {}", story.path.display().dimmed());

    // Body content with colored ACs and syntax-highlighted code blocks
    if let Some(body_text) = body
        && !body_text.trim().is_empty()
    {
        println!();
        println!("{}", style::rule(width, None));

        let mut code_block: Option<(String, String)> = None; // (lang, accumulated code)

        for line in body_text.trim().lines() {
            let trimmed = line.trim();

            // Check for fenced code block boundaries
            if trimmed.starts_with("```") {
                if let Some((lang, code)) = code_block.take() {
                    // Closing fence — highlight and print the buffered block
                    if let Some(highlighted) = style::highlight_code_block(&code, &lang) {
                        print!("{}", highlighted);
                    } else {
                        // Unknown language — print dimmed as fallback
                        for code_line in code.lines() {
                            println!("{}", code_line.dimmed());
                        }
                    }
                } else {
                    // Opening fence — start buffering
                    let lang = trimmed.trim_start_matches('`').trim().to_string();
                    code_block = Some((lang, String::new()));
                }
                continue;
            }

            // If inside a code block, buffer the line
            if let Some((_, ref mut code)) = code_block {
                code.push_str(line);
                code.push('\n');
                continue;
            }

            // Normal line rendering
            if trimmed.starts_with("- [x]")
                || trimmed.starts_with("- [X]")
                || trimmed.starts_with("- [ ]")
            {
                println!("{}", style::styled_ac(trimmed));
            } else if trimmed.starts_with("# ") || trimmed.starts_with("## ") {
                println!("{}", line.bold());
            } else {
                println!("{}", line);
            }
        }

        // Handle unclosed code block (print what we have)
        if let Some((lang, code)) = code_block {
            if let Some(highlighted) = style::highlight_code_block(&code, &lang) {
                print!("{}", highlighted);
            } else {
                for code_line in code.lines() {
                    println!("{}", code_line.dimmed());
                }
            }
        }
    }

    // Evidence chain contributions
    let entries = evidence::collect_story_evidence(story.id(), story.title(), &content);
    if !entries.is_empty() {
        println!();
        println!("{}", "Evidence Chain".bold());
        for entry in &entries {
            let styled_entry = style::styled_evidence_entry(entry);
            // The story ID is already colored by styled_evidence_entry? No, let's check.
            println!("{}", styled_entry);
        }
    }

    Ok(())
}

/// Extract body content after frontmatter
fn extract_body(content: &str) -> Option<&str> {
    let mut delimiter_count = 0;

    for (idx, line) in content.lines().enumerate() {
        if line == "---" {
            delimiter_count += 1;
            if delimiter_count == 2 {
                // Find byte offset of the next line
                let lines: Vec<&str> = content.lines().collect();
                if idx + 1 < lines.len() {
                    let prefix_len: usize = lines[..=idx].iter().map(|l| l.len() + 1).sum();
                    return Some(&content[prefix_len..]);
                }
            }
        }
    }

    None
}

/// Count checked and total acceptance criteria in body text
fn count_acs(body: &str) -> (usize, usize) {
    let mut checked = 0;
    let mut total = 0;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            checked += 1;
            total += 1;
        } else if trimmed.starts_with("- [ ]") {
            total += 1;
        }
    }

    (checked, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn show_displays_story() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("planned"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Test Story")
                    .stage(StoryState::Backlog)
                    .scope("test-epic/01-first")
                    .body("\n# Test Story\n\n## Acceptance Criteria\n\n- [ ] First criterion\n"),
            )
            .build();

        // Just verify it doesn't error - output goes to stdout
        run_with_dir(temp.path(), "FEAT0001").unwrap();
    }

    #[test]
    fn extract_body_works() {
        let content = "---\nid: test\n---\n\n# Body\n\nContent here.";
        let body = extract_body(content).unwrap();
        assert!(body.contains("# Body"));
        assert!(body.contains("Content here."));
    }

    #[test]
    fn count_acs_counts_correctly() {
        let body = "## Acceptance Criteria\n\n- [x] Done\n- [ ] Not done\n- [x] Also done\n";
        let (checked, total) = count_acs(body);
        assert_eq!(checked, 2);
        assert_eq!(total, 3);
    }
}
