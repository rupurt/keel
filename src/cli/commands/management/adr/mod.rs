//! ADR commands - new, list, show, accept, reject, deprecate, supersede

use std::fs;

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AdrAction {
    /// Create a new ADR
    New {
        /// ADR title
        title: String,
        /// Primary bounded context this ADR applies to
        #[arg(long)]
        context: Option<String>,
        /// Additional scopes this ADR applies to (repeatable)
        #[arg(long = "applies-to", action = clap::ArgAction::Append, value_name = "SCOPE")]
        applies_to: Vec<String>,
    },
    /// List all ADRs
    List {
        /// Filter by status (proposed, accepted, rejected, deprecated, superseded)
        #[arg(long)]
        status: Option<String>,
    },
    /// Show ADR details
    Show {
        /// ADR ID or title (fuzzy match)
        id: String,
    },
    /// Accept a proposed ADR
    Accept {
        /// ADR ID or title (fuzzy match)
        id: String,
    },
    /// Reject a proposed ADR
    Reject {
        /// ADR ID or title (fuzzy match)
        id: String,
        /// Reason for rejection
        reason: String,
    },
    /// Deprecate an accepted ADR (no longer recommended)
    Deprecate {
        /// ADR ID or title (fuzzy match)
        id: String,
        /// Reason for deprecation
        reason: String,
    },
    /// Supersede an ADR with a newer one
    Supersede {
        /// New ADR ID that replaces the old one (must be accepted)
        new_id: String,
        /// Old ADR ID to be superseded
        old_id: String,
    },
}

use crate::cli::table::Table;
use crate::domain::model::{Adr, Board};
use crate::infrastructure::config::find_board_dir;
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;
use crate::infrastructure::utils::slugify;

/// Run an ADR action through the ADR interface adapter.
pub fn run(action: AdrAction) -> Result<()> {
    match action {
        AdrAction::New {
            title,
            context,
            applies_to,
        } => run_new(&title, context.as_deref(), &applies_to),
        AdrAction::List { status } => run_list(status.as_deref()),
        AdrAction::Show { id } => run_show(&id),
        AdrAction::Accept { id } => run_accept(&id),
        AdrAction::Reject { id, reason } => run_reject(&id, &reason),
        AdrAction::Deprecate { id, reason } => run_deprecate(&id, &reason),
        AdrAction::Supersede { new_id, old_id } => run_supersede(&new_id, &old_id),
    }
}

/// Find the next ADR index
fn next_adr_index(board: &Board) -> u32 {
    board
        .adrs
        .values()
        .filter_map(|adr| adr.index())
        .max()
        .unwrap_or(0)
        + 1
}

/// Create a new ADR
pub fn run_new(title: &str, context: Option<&str>, applies_to: &[String]) -> Result<()> {
    let board_dir = find_board_dir()?;
    new_adr(&board_dir, title, context, applies_to)
}

fn yaml_quote(input: &str) -> String {
    format!("\"{}\"", input.replace('\\', "\\\\").replace('"', "\\\""))
}

fn format_context_value(context: Option<&str>) -> String {
    match context.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => yaml_quote(value),
        None => "null".to_string(),
    }
}

fn format_applies_to_value(applies_to: &[String]) -> String {
    let values: Vec<String> = applies_to
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(yaml_quote)
        .collect();

    if values.is_empty() {
        "[]".to_string()
    } else {
        format!("[{}]", values.join(", "))
    }
}

fn new_adr(
    board_dir: &std::path::Path,
    title: &str,
    context: Option<&str>,
    applies_to: &[String],
) -> Result<()> {
    // Enforce Title Case
    if !crate::infrastructure::utils::is_title_case(title) {
        return Err(anyhow!(
            "ADR title '{}' must use Title Case (e.g. 'My ADR Title')",
            title
        ));
    }

    let board = load_board(board_dir)?;

    // Generate random ID and calculate index
    let id = crate::infrastructure::story_id::generate_story_id();
    let index = next_adr_index(&board);
    let slug = slugify(title);
    let date = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let context_value = format_context_value(context);
    let applies_to_value = format_applies_to_value(applies_to);

    // Build filename
    let filename = format!("{}-{}.md", id, slug);
    let adr_path = board_dir.join("adrs").join(&filename);

    // Check if file already exists
    if adr_path.exists() {
        return Err(anyhow!("ADR already exists: {}", adr_path.display()));
    }

    // Render template
    let title_string = title.to_string();
    let index_string = index.to_string();
    let content = template_rendering::render(
        templates::adr::ADR,
        &[
            ("id", &id),
            ("title", &title_string),
            ("decided_at", &date),
            ("index", &index_string),
            ("context", &context_value),
            ("applies_to", &applies_to_value),
        ],
    );

    // Ensure adrs directory exists
    let adrs_dir = board_dir.join("adrs");
    if !adrs_dir.exists() {
        fs::create_dir_all(&adrs_dir)
            .with_context(|| format!("Failed to create adrs directory: {}", adrs_dir.display()))?;
    }

    // Write file
    fs::write(&adr_path, content)
        .with_context(|| format!("Failed to write ADR: {}", adr_path.display()))?;

    println!("Created: {}", filename);
    println!("  ID: {}", id);
    println!("  Status: proposed");

    Ok(())
}

/// List all ADRs
pub fn run_list(status_filter: Option<&str>) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;

    // Parse status filter if provided
    let filter_status: Option<crate::domain::model::AdrStatus> = status_filter
        .map(|s| {
            s.parse().map_err(|_| {
                anyhow!(
                    "Invalid status: {}. Use: proposed, accepted, deprecated, superseded",
                    s
                )
            })
        })
        .transpose()?;

    // Collect and sort ADRs by ID
    let mut adrs: Vec<_> = board
        .adrs
        .values()
        .filter(|a| filter_status.is_none() || Some(a.frontmatter.status) == filter_status)
        .collect();

    adrs.sort_by(|a, b| a.id().cmp(b.id()));

    if adrs.is_empty() {
        if let Some(status) = status_filter {
            println!("No ADRs with status: {}", status);
        } else {
            println!("No ADRs found.");
        }
        return Ok(());
    }

    let mut table = Table::new(&["ID", "TITLE", "STATUS", "CONTEXT"]);
    for adr in adrs {
        let context = adr.frontmatter.context.as_deref().unwrap_or("-");
        table.row(&[
            adr.id(),
            &adr.frontmatter.title,
            &adr.frontmatter.status.to_string(),
            context,
        ]);
    }
    table.print();

    Ok(())
}

/// Show details for a specific ADR
pub fn run_show(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let adr = board.require_adr(pattern)?;

    // Print details
    println!("ADR: {}", adr.frontmatter.title);
    println!("{}", "=".repeat(50));
    println!("ID:      {}", adr.id());
    println!("Status:  {}", adr.frontmatter.status);
    if let Some(ctx) = &adr.frontmatter.context {
        println!("Context: {}", ctx);
    }
    if !adr.frontmatter.applies_to.is_empty() {
        println!("Applies: {}", adr.frontmatter.applies_to.join(", "));
    }
    if let Some(date) = adr.frontmatter.decided_at {
        println!("Date:    {}", date.format("%Y-%m-%d"));
    }
    println!();

    // Supersedes/superseded-by
    if !adr.frontmatter.supersedes.is_empty() {
        println!("Supersedes: {}", adr.frontmatter.supersedes.join(", "));
    }
    if let Some(by) = &adr.frontmatter.superseded_by {
        println!("Superseded by: {}", by);
    }

    // Print file path
    println!();
    println!("File: {}", adr.path.display());

    Ok(())
}

/// Update ADR status in its file
fn update_adr_status(
    adr: &Adr,
    new_status: crate::domain::model::AdrStatus,
    rejection_reason: Option<&str>,
) -> Result<()> {
    let content = fs::read_to_string(&adr.path)
        .with_context(|| format!("Failed to read ADR: {}", adr.path.display()))?;

    let mut mutations = vec![Mutation::set("status", new_status.to_string())];
    if let Some(reason) = rejection_reason {
        mutations.push(Mutation::set("rejection-reason", format!("\"{}\"", reason)));
    }
    let updated = apply(&content, &mutations);

    fs::write(&adr.path, updated)
        .with_context(|| format!("Failed to write ADR: {}", adr.path.display()))?;

    Ok(())
}

/// Accept a proposed ADR
pub fn run_accept(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let adr = board.require_adr(pattern)?;

    // Check if ADR is in proposed status
    if adr.frontmatter.status != crate::domain::model::AdrStatus::Proposed {
        return Err(anyhow!(
            "Cannot accept ADR {} - status is '{}', expected 'proposed'",
            adr.id(),
            adr.frontmatter.status
        ));
    }

    update_adr_status(adr, crate::domain::model::AdrStatus::Accepted, None)?;

    println!("Accepted: {}", adr.id());
    println!("  {} (proposed → accepted)", adr.frontmatter.title);

    Ok(())
}

/// Reject a proposed ADR with reason
pub fn run_reject(pattern: &str, reason: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let adr = board.require_adr(pattern)?;

    // Check if ADR is in proposed status
    if adr.frontmatter.status != crate::domain::model::AdrStatus::Proposed {
        return Err(anyhow!(
            "Cannot reject ADR {} - status is '{}', expected 'proposed'",
            adr.id(),
            adr.frontmatter.status
        ));
    }

    update_adr_status(adr, crate::domain::model::AdrStatus::Rejected, Some(reason))?;

    println!("Rejected: {}", adr.id());
    println!("  {} (proposed → rejected)", adr.frontmatter.title);
    println!("  Reason: {}", reason);

    Ok(())
}

/// Deprecate an accepted ADR with reason
pub fn run_deprecate(pattern: &str, reason: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let adr = board.require_adr(pattern)?;

    // Check if ADR is in accepted status
    if adr.frontmatter.status != crate::domain::model::AdrStatus::Accepted {
        return Err(anyhow!(
            "Cannot deprecate ADR {} - status is '{}', expected 'accepted'",
            adr.id(),
            adr.frontmatter.status
        ));
    }

    // Update status with deprecation reason
    let content = fs::read_to_string(&adr.path)
        .with_context(|| format!("Failed to read ADR: {}", adr.path.display()))?;

    let updated = apply(
        &content,
        &[
            Mutation::set(
                "status",
                crate::domain::model::AdrStatus::Deprecated.to_string(),
            ),
            Mutation::set("deprecation-reason", format!("\"{}\"", reason)),
        ],
    );

    fs::write(&adr.path, updated)
        .with_context(|| format!("Failed to write ADR: {}", adr.path.display()))?;

    println!("Deprecated: {}", adr.id());
    println!("  {} (accepted → deprecated)", adr.frontmatter.title);
    println!("  Reason: {}", reason);

    Ok(())
}

/// Supersede an ADR with a newer one
///
/// The new ADR must be in accepted status. This command:
/// - Sets old ADR status to superseded and adds `superseded-by: <new-id>`
/// - Appends old ID to the new ADR's `supersedes` list
pub fn run_supersede(new_pattern: &str, old_pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;

    // Find both ADRs
    let new_adr = board.require_adr(new_pattern)?;
    let old_adr = board.require_adr(old_pattern)?;

    // Validate: new ADR must be accepted
    if new_adr.frontmatter.status != crate::domain::model::AdrStatus::Accepted {
        return Err(anyhow!(
            "Cannot supersede: new ADR {} is '{}', expected 'accepted'",
            new_adr.id(),
            new_adr.frontmatter.status
        ));
    }

    // Validate: old ADR must be accepted (can't supersede something already superseded/deprecated)
    if old_adr.frontmatter.status != crate::domain::model::AdrStatus::Accepted {
        return Err(anyhow!(
            "Cannot supersede ADR {} - status is '{}', expected 'accepted'",
            old_adr.id(),
            old_adr.frontmatter.status
        ));
    }

    // Update old ADR: status → superseded, add superseded-by
    let old_content = fs::read_to_string(&old_adr.path)
        .with_context(|| format!("Failed to read ADR: {}", old_adr.path.display()))?;

    let old_updated = apply(
        &old_content,
        &[
            Mutation::set("status", "superseded"),
            Mutation::set("superseded-by", new_adr.id()),
        ],
    );

    // Update new ADR: add old ID to supersedes list
    let new_content = fs::read_to_string(&new_adr.path)
        .with_context(|| format!("Failed to read ADR: {}", new_adr.path.display()))?;

    let mut supersedes = new_adr.frontmatter.supersedes.clone();
    supersedes.push(old_adr.id().to_string());
    let supersedes = supersedes.join(", ");
    let new_updated = apply(
        &new_content,
        &[Mutation::set("supersedes", format!("[{}]", supersedes))],
    );

    // Write both files (as atomic as we can without transactions)
    fs::write(&old_adr.path, old_updated)
        .with_context(|| format!("Failed to write ADR: {}", old_adr.path.display()))?;
    fs::write(&new_adr.path, new_updated)
        .with_context(|| format!("Failed to write ADR: {}", new_adr.path.display()))?;

    println!("Superseded: {} → {}", old_adr.id(), new_adr.id());
    println!("  {} (accepted → superseded)", old_adr.frontmatter.title);
    println!("  Replaced by: {}", new_adr.frontmatter.title);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_board(temp: &TempDir) -> PathBuf {
        let root = temp.path();
        let adrs_dir = root.join("adrs");
        fs::create_dir_all(&adrs_dir).unwrap();

        // Create an existing ADR
        fs::write(
            adrs_dir.join("1vkqtsHH1-test-decision.md"),
            r#"---
id: 1vkqtsHH1
index: 1
title: Test Decision
status: accepted
context: null
applies-to: []
supersedes: []
superseded-by: null
decided_at: 2026-01-15T00:00:00
---

# Test Decision
"#,
        )
        .unwrap();

        root.to_path_buf()
    }

    #[test]
    fn next_adr_index_increments() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_board(&temp);
        let board = load_board(&board_dir).unwrap();

        let next_idx = next_adr_index(&board);
        assert_eq!(next_idx, 2);
    }

    #[test]
    fn next_adr_index_handles_empty_board() {
        let temp = TempDir::new().unwrap();
        let board_dir = temp.path();
        fs::create_dir_all(board_dir.join("adrs")).unwrap();
        let board = load_board(board_dir).unwrap();

        let next_idx = next_adr_index(&board);
        assert_eq!(next_idx, 1);
    }

    #[test]
    fn new_adr_persists_context_and_applies_to_in_frontmatter() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_board(&temp);

        new_adr(
            &board_dir,
            "Scoped Decision",
            Some("work-management"),
            &["queue-policy".to_string(), "story-lifecycle".to_string()],
        )
        .unwrap();

        let adrs_dir = board_dir.join("adrs");
        let adr_path = fs::read_dir(&adrs_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                fs::read_to_string(path)
                    .unwrap()
                    .contains("title: Scoped Decision")
            })
            .expect("New ADR file should exist");

        let content = fs::read_to_string(&adr_path).unwrap();
        assert!(content.contains("context: \"work-management\""));
        assert!(content.contains("applies-to: [\"queue-policy\", \"story-lifecycle\"]"));

        let board = load_board(&board_dir).unwrap();
        let adr = board
            .adrs
            .values()
            .find(|adr| adr.frontmatter.title == "Scoped Decision")
            .expect("ADR should be loaded");
        assert_eq!(adr.frontmatter.context.as_deref(), Some("work-management"));
        assert_eq!(
            adr.frontmatter.applies_to,
            vec!["queue-policy".to_string(), "story-lifecycle".to_string()]
        );
    }
}
