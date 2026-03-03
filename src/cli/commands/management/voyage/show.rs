//! Show voyage command

use std::fs;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::style;
use crate::infrastructure::loader::load_board;

/// Show voyage details
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;

    let voyage = board.require_voyage(id)?;

    let width = crate::cli::presentation::terminal::get_terminal_width();

    // Header
    println!("{}", style::heavy_rule(width, None));
    println!(
        "{}",
        style::header(
            voyage.id(),
            &voyage.frontmatter.title,
            style::styled_voyage_id
        )
    );
    println!("{}", style::heavy_rule(width, None));
    println!();

    println!("Epic:     {}", style::styled_epic_id(&voyage.epic_id));
    println!("Status:   {}", style::styled_voyage_stage(&voyage.status()));

    println!();
    println!("Path: {}", voyage.path.display().dimmed());

    // Read body content
    let content = fs::read_to_string(&voyage.path)?;
    if let Some(body) = extract_body(&content)
        && !body.trim().is_empty()
    {
        println!();
        println!("{}", style::rule(width, None));
        println!("{}", body.trim());
    }

    Ok(())
}

/// Extract body content after frontmatter (duplicated to avoid circular deps)
fn extract_body(content: &str) -> Option<&str> {
    let mut delimiter_count = 0;

    for (idx, line) in content.lines().enumerate() {
        if line == "---" {
            delimiter_count += 1;
            if delimiter_count == 2 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_body() {
        let content = "---\nid: test\n---\nBody content";
        assert_eq!(extract_body(content), Some("Body content"));

        let no_body = "---\nid: test\n---\n";
        assert_eq!(extract_body(no_body), None);
    }
}
