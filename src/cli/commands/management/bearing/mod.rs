//! Bearing commands - survey, assess, list, show

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use clap::Subcommand;
use owo_colors::OwoColorize;

pub(crate) mod guidance;

#[derive(Subcommand, Debug)]
pub enum BearingAction {
    /// Create a new bearing
    New {
        /// Bearing name (will be slugified for ID)
        name: String,
    },
    /// Add SURVEY.md to a bearing
    Survey {
        /// Bearing name or ID (fuzzy match)
        name: String,
    },
    /// Add ASSESSMENT.md to a bearing
    Assess {
        /// Bearing name or ID (fuzzy match)
        name: String,
    },
    /// List all bearings
    List {
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },
    /// Show bearing details
    Show {
        /// Bearing name or ID (fuzzy match)
        name: String,
    },
    /// Park a bearing for later
    Park {
        /// Bearing name or ID (fuzzy match)
        name: String,
    },
    /// Decline a bearing with reason
    Decline {
        /// Bearing name or ID (fuzzy match)
        name: String,
        /// Reason for declining
        reason: String,
    },
    /// Graduate bearing to epic
    Lay {
        /// Bearing name or ID (fuzzy match)
        name: String,
    },
}

pub mod new;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::table::Table;
use crate::domain::model::{Bearing, BearingStatus};
use crate::infrastructure::config::{find_board_dir, load_config};
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::scoring::{calculate_score, load_assessment};
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;
use guidance::{
    BearingLifecycleAction, error_with_recovery, guidance_for_action, informational_for_list,
    informational_for_show, print_human,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FogType {
    Blocking,
    Protecting,
    Clear,
}

impl FogType {
    fn as_banner(&self) -> &'static str {
        match self {
            Self::Blocking => "🚧 blocking fog",
            Self::Protecting => "🌫️ protecting fog",
            Self::Clear => "✨ clear skies",
        }
    }
}

/// Run the new command - creates a new bearing
pub fn run_new(name: &str) -> Result<()> {
    new::run(name)
}

/// Run a bearing action through the bearing interface adapter.
pub fn run(action: BearingAction) -> Result<()> {
    match action {
        BearingAction::New { name } => run_new(&name),
        BearingAction::Survey { name } => run_survey(&name),
        BearingAction::Assess { name } => run_assess(&name),
        BearingAction::List { status } => run_list(status.as_deref()),
        BearingAction::Show { name } => run_show(&name),
        BearingAction::Park { name } => run_park(&name),
        BearingAction::Decline { name, reason } => run_decline(&name, &reason),
        BearingAction::Lay { name } => run_lay(&name),
    }
}

/// Run the survey command - adds SURVEY.md to a bearing
pub fn run_survey(pattern: &str) -> Result<()> {
    run_survey_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Survey, pattern, err))
}

fn run_survey_impl(pattern: &str) -> Result<()> {
    use crate::domain::transitions::bearing::{bearing_transitions, execute};

    let board_dir = find_board_dir()?;
    let result = execute(&board_dir, pattern, &bearing_transitions::SURVEY)?;

    if let Some(path) = &result.file_created {
        println!("Created: {}", path);
    }
    println!("  {} → {}", result.from, result.to);
    let guidance = guidance_for_action(BearingLifecycleAction::Survey, &result.bearing_id);
    print_human(guidance.as_ref());

    Ok(())
}

/// Run the assess command - adds ASSESSMENT.md to a bearing
pub fn run_assess(pattern: &str) -> Result<()> {
    run_assess_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Assess, pattern, err))
}

fn run_assess_impl(pattern: &str) -> Result<()> {
    use crate::domain::transitions::bearing::{bearing_transitions, execute};

    let board_dir = find_board_dir()?;
    let result = execute(&board_dir, pattern, &bearing_transitions::ASSESS)?;

    if let Some(path) = &result.file_created {
        println!("Created: {}", path);
    }
    println!("  {} → {}", result.from, result.to);
    let guidance = guidance_for_action(BearingLifecycleAction::Assess, &result.bearing_id);
    print_human(guidance.as_ref());

    Ok(())
}

/// Update the status in a bearing's README.md
fn update_bearing_status(
    board_dir: &std::path::Path,
    bearing: &Bearing,
    new_status: BearingStatus,
) -> Result<()> {
    let readme_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("README.md");

    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read README.md: {}", readme_path.display()))?;

    let mut mutations = vec![Mutation::set("status", new_status.to_string())];
    if new_status == BearingStatus::Laid {
        let today = Local::now().format("%Y-%m-%d").to_string();
        mutations.push(Mutation::set("laid_at", today));
    }
    let updated = apply(&content, &mutations);

    fs::write(&readme_path, updated)
        .with_context(|| format!("Failed to write README.md: {}", readme_path.display()))?;

    Ok(())
}

/// Run the list command - shows all bearings with status and EV score
pub fn run_list(status_filter: Option<&str>) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let (config, _source) = load_config();
    let weights = config.current_weights();

    // Parse status filter if provided
    let filter_status: Option<BearingStatus> = status_filter
        .map(|s| s.parse().map_err(|_| anyhow!("Invalid status: {}", s)))
        .transpose()?;

    // Collect bearings with their scores
    let mut bearings_with_scores: Vec<(&Bearing, Option<f64>)> = board
        .bearings
        .values()
        .filter(|b| filter_status.is_none() || Some(b.frontmatter.status) == filter_status)
        .map(|b| {
            let score = get_bearing_score(&board_dir, b, &weights);
            (b, score)
        })
        .collect();

    // Sort by EV score (highest first), bearings without scores at the end
    bearings_with_scores.sort_by(|a, b| match (a.1, b.1) {
        (Some(score_a), Some(score_b)) => score_b.partial_cmp(&score_a).unwrap(),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.0.id().cmp(b.0.id()),
    });

    if bearings_with_scores.is_empty() {
        if filter_status.is_some() {
            println!("No bearings with status: {}", status_filter.unwrap());
        } else {
            println!("No bearings found.");
        }
        print_human(informational_for_list().as_ref());
        return Ok(());
    }

    let mut table = Table::new(&["ID", "STATUS", "SURVEY", "ASSESS", "EV"]);
    for (bearing, score) in bearings_with_scores {
        let survey = if bearing.has_survey { "✓" } else { "-" };
        let assessment = if bearing.has_assessment { "✓" } else { "-" };
        let score_str = score
            .map(|s| format!("{:.2}", s))
            .unwrap_or_else(|| "-".to_string());

        table.row(&[
            bearing.id(),
            &bearing.frontmatter.status.to_string(),
            survey,
            assessment,
            &score_str,
        ]);
    }
    table.print();
    print_human(informational_for_list().as_ref());

    Ok(())
}

/// Run the show command - display full bearing details
pub fn run_show(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let (config, _source) = load_config();
    let weights = config.current_weights();
    let bearing = board.require_bearing(pattern)?;
    let width = crate::cli::presentation::terminal::get_terminal_width();

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", bearing.frontmatter.title.bold()))
        .row("ID:", bearing.id().to_string())
        .row("Status:", bearing.frontmatter.status.to_string());

    if matches!(
        bearing.frontmatter.status,
        BearingStatus::Exploring | BearingStatus::Parked
    ) {
        let fog = classify_fog(&board_dir, bearing);
        metadata.push_row("Fog:", fog.as_banner().to_string());
    }
    metadata.push_optional_row(
        "Created:",
        bearing.frontmatter.created_at.map(|d| d.to_string()),
    );

    let mut documents = ShowSection::new("Documents");
    documents.push_lines(["  README.md:     ✓ (frontmatter)".to_string()]);
    documents.push_lines(["  BRIEF.md:      ✓ (research)".to_string()]);
    documents.push_lines([format!(
        "  SURVEY.md:     {}",
        if bearing.has_survey {
            "✓"
        } else {
            "not created"
        }
    )]);
    documents.push_lines([format!(
        "  ASSESSMENT.md: {}",
        if bearing.has_assessment {
            "✓"
        } else {
            "not created"
        }
    )]);

    let mut document = ShowDocument::new();
    document.push_key_values(metadata);
    document.push_rule(width);
    document.push_section(documents);

    if bearing.has_assessment {
        let assessment_path = board_dir
            .join("bearings")
            .join(bearing.id())
            .join("ASSESSMENT.md");

        if let Ok(factors) = load_assessment(&assessment_path) {
            let mut scoring = ShowSection::new("EV Scoring");
            let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(11);
            fields.push_row("Mode:", config.mode().to_string());
            fields.push_row("Impact:", format_factor(factors.impact));
            fields.push_row("Confidence:", format_factor(factors.confidence));
            fields.push_row("Effort:", format_factor(factors.effort));
            fields.push_row("Risk:", format_factor(factors.risk));
            scoring.push_key_values(fields);

            if factors.is_complete() {
                if let Ok(score) = calculate_score(&factors, &weights) {
                    scoring.push_lines([format!("  EV Score: {:.2}", score.weighted_score)]);
                }
            } else {
                scoring.push_lines([format!(
                    "  Missing factors: {}",
                    factors.missing_factors().join(", ")
                )]);
            }

            document.push_spacer();
            document.push_section(scoring);
        }
    }

    if let Some(reason) = &bearing.frontmatter.decline_reason {
        let mut decline = ShowSection::new("Decline Reason");
        decline.push_lines([reason.to_string()]);
        document.push_spacer();
        document.push_section(decline);
    }

    document.print();
    print_human(informational_for_show().as_ref());

    Ok(())
}

/// Run the park command - shelve a bearing for later
pub fn run_park(pattern: &str) -> Result<()> {
    run_park_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Park, pattern, err))
}

fn run_park_impl(pattern: &str) -> Result<()> {
    use crate::domain::transitions::bearing::{bearing_transitions, execute};

    let board_dir = find_board_dir()?;
    let result = execute(&board_dir, pattern, &bearing_transitions::PARK)?;

    println!("Parked: {} → {}", result.from, result.to);
    let guidance = guidance_for_action(BearingLifecycleAction::Park, &result.bearing_id);
    print_human(guidance.as_ref());

    Ok(())
}

/// Run the decline command - reject a bearing with reason
pub fn run_decline(pattern: &str, reason: &str) -> Result<()> {
    run_decline_impl(pattern, reason)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Decline, pattern, err))
}

fn run_decline_impl(pattern: &str, reason: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let bearing = board.require_bearing(pattern)?;

    let old_status = bearing.frontmatter.status;

    if old_status == BearingStatus::Declined {
        return Err(anyhow!("Bearing {} is already declined", bearing.id()));
    }

    update_bearing_status_with_decline(&board_dir, bearing, reason)?;

    println!("Declined: {} ({} → declined)", bearing.id(), old_status);
    println!("  Reason: {}", reason);
    let guidance = guidance_for_action(BearingLifecycleAction::Decline, bearing.id());
    print_human(guidance.as_ref());

    Ok(())
}

/// Update status to declined and add decline_reason to frontmatter
fn update_bearing_status_with_decline(
    board_dir: &std::path::Path,
    bearing: &Bearing,
    reason: &str,
) -> Result<()> {
    let readme_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("README.md");

    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read README.md: {}", readme_path.display()))?;

    let updated = apply(
        &content,
        &[
            Mutation::set("status", "declined"),
            Mutation::set("decline_reason", format!("\"{}\"", reason)),
        ],
    );

    fs::write(&readme_path, updated)
        .with_context(|| format!("Failed to write README.md: {}", readme_path.display()))?;

    Ok(())
}

/// Run the lay command - graduate bearing to epic
pub fn run_lay(pattern: &str) -> Result<()> {
    run_lay_impl(pattern)
        .map_err(|err| error_with_recovery(BearingLifecycleAction::Lay, pattern, err))
}

fn run_lay_impl(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let bearing = board.require_bearing(pattern)?;

    let old_status = bearing.frontmatter.status;

    // Check if bearing can be laid
    match old_status {
        BearingStatus::Declined => {
            return Err(anyhow!(
                "Cannot lay declined bearing: {}. Reason: {}",
                bearing.id(),
                bearing
                    .frontmatter
                    .decline_reason
                    .as_deref()
                    .unwrap_or("unknown")
            ));
        }
        BearingStatus::Parked => {
            return Err(anyhow!(
                "Cannot lay parked bearing: {}. Unpark it first.",
                bearing.id()
            ));
        }
        BearingStatus::Laid => {
            return Err(anyhow!("Bearing {} has already been laid", bearing.id()));
        }
        _ => {}
    }

    // Warn if no assessment
    if !bearing.has_assessment {
        eprintln!(
            "Warning: No ASSESSMENT.md for {}. Proceeding without EV evaluation.",
            bearing.id()
        );
    }

    // Create the epic
    let epic_id = bearing.id();
    let epic_dir = board_dir.join("epics").join(epic_id);

    if epic_dir.exists() {
        return Err(anyhow!(
            "Epic already exists: {}. Choose a different bearing name.",
            epic_id
        ));
    }

    fs::create_dir_all(&epic_dir)
        .with_context(|| format!("Failed to create epic directory: {}", epic_dir.display()))?;

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    // Create README.md with bearing reference
    let readme_content = template_rendering::render(
        templates::epic::README,
        &[
            ("id", epic_id),
            ("title", &bearing.frontmatter.title),
            ("created_at", &now),
        ],
    );
    // Insert bearing reference after status line
    let readme_with_bearing = readme_content.replace(
        "status: planned",
        &format!("status: planned\nbearing: {}", epic_id),
    );
    let readme_path = epic_dir.join("README.md");
    fs::write(&readme_path, readme_with_bearing)
        .with_context(|| format!("Failed to write epic README: {}", readme_path.display()))?;

    // Create PRD.md seeded with bearing content
    let prd_content = create_prd_from_bearing(&board_dir, bearing)?;
    let prd_path = epic_dir.join("PRD.md");
    fs::write(&prd_path, prd_content)
        .with_context(|| format!("Failed to write PRD: {}", prd_path.display()))?;

    // Update bearing status to laid
    update_bearing_status(&board_dir, bearing, BearingStatus::Laid)?;

    println!("Laid: {} → epics/{}/", bearing.id(), epic_id);
    println!("  Epic created with PRD.md seeded from bearing documents");
    println!("  Bearing status: {} → laid", old_status);
    let guidance = guidance_for_action(BearingLifecycleAction::Lay, bearing.id());
    print_human(guidance.as_ref());

    // Regenerate board
    crate::cli::commands::generate::run(&board_dir)?;

    Ok(())
}

/// Create PRD content from bearing documents
fn create_prd_from_bearing(board_dir: &Path, bearing: &Bearing) -> Result<String> {
    let bearing_dir = board_dir.join("bearings").join(bearing.id());

    // Read BRIEF.md content
    let brief_path = bearing_dir.join("BRIEF.md");
    let brief_content = fs::read_to_string(&brief_path)
        .with_context(|| format!("Failed to read BRIEF.md: {}", brief_path.display()))?;

    // Extract sections from BRIEF.md
    let hypothesis = extract_section(&brief_content, "## Hypothesis");
    let problem_space = extract_section(&brief_content, "## Problem Space");
    let success_criteria = extract_section(&brief_content, "## Success Criteria");

    // Read ASSESSMENT.md if it exists
    let assessment_path = bearing_dir.join("ASSESSMENT.md");
    let assessment_content = if assessment_path.exists() {
        Some(
            fs::read_to_string(&assessment_path)
                .with_context(|| "Failed to read ASSESSMENT.md".to_string())?,
        )
    } else {
        None
    };

    // Extract assessment analysis
    let analysis = assessment_content
        .as_ref()
        .map(|c| extract_section(c, "## Analysis"))
        .unwrap_or_default();

    // Build PRD content
    let mut prd = format!("# {} - Product Requirements\n\n", bearing.frontmatter.title);

    // Add hypothesis as value proposition
    if !hypothesis.is_empty() {
        prd.push_str(&format!("> {}\n\n", hypothesis.trim()));
    } else {
        prd.push_str("> Research-informed outcome for this epic.\n\n");
    }

    // Problem Statement from bearing
    prd.push_str("## Problem Statement\n\n");
    if !problem_space.is_empty() {
        prd.push_str(&format!("{}\n\n", problem_space.trim()));
    } else {
        prd.push_str("This epic addresses the validated problem surfaced by bearing research.\n\n");
    }

    prd.push_str("## Goals & Objectives\n\n");
    prd.push_str("| Goal | Success Metric | Target |\n");
    prd.push_str("|------|----------------|--------|\n");
    prd.push_str("| Validate bearing recommendation in delivery flow | Adoption signal | Initial rollout complete |\n\n");

    prd.push_str("## Users\n\n");
    prd.push_str("| Persona | Description | Primary Need |\n");
    prd.push_str("|---------|-------------|--------------|\n");
    prd.push_str("| Product/Delivery Owner | Coordinates planning and execution | Reliable strategic direction |\n\n");

    prd.push_str("## Scope\n\n");
    prd.push_str("### In Scope\n\n");
    prd.push_str("- Deliver the bearing-backed capability slice for this epic.\n\n");
    prd.push_str("### Out of Scope\n\n");
    prd.push_str("- Unrelated platform-wide refactors outside bearing findings.\n\n");

    // Requirements
    prd.push_str("## Requirements\n\n");
    prd.push_str("### Functional Requirements\n\n");
    prd.push_str("<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n");
    prd.push_str("| ID | Requirement | Priority | Rationale |\n");
    prd.push_str("|----|-------------|----------|-----------|\n");
    prd.push_str("| FR-01 | Implement the core user workflow identified in bearing research. | must | Converts research recommendation into executable product capability. |\n");
    prd.push_str("<!-- END FUNCTIONAL_REQUIREMENTS -->\n\n");

    prd.push_str("### Non-Functional Requirements\n\n");
    prd.push_str("<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->\n");
    prd.push_str("| ID | Requirement | Priority | Rationale |\n");
    prd.push_str("|----|-------------|----------|-----------|\n");
    prd.push_str("| NFR-01 | Ensure deterministic behavior and operational visibility for the delivered workflow. | must | Keeps delivery safe and auditable during rollout. |\n");
    prd.push_str("<!-- END NON_FUNCTIONAL_REQUIREMENTS -->\n\n");

    prd.push_str("## Verification Strategy\n\n");
    prd.push_str("- Prove functional behavior through story-level verification evidence mapped to voyage requirements.\n");
    prd.push_str(
        "- Validate non-functional posture with operational checks and documented artifacts.\n\n",
    );

    prd.push_str("## Assumptions\n\n");
    prd.push_str("| Assumption | Impact if Wrong | Validation |\n");
    prd.push_str("|------------|-----------------|------------|\n");
    prd.push_str("| Bearing findings reflect current user needs | Scope may need re-planning | Re-check feedback during first voyage |\n\n");

    prd.push_str("## Open Questions & Risks\n\n");
    prd.push_str("| Question/Risk | Owner | Status |\n");
    prd.push_str("|---------------|-------|--------|\n");
    prd.push_str(
        "| Which rollout constraints should gate broader adoption? | Product | Open |\n\n",
    );

    // Success Criteria from bearing
    prd.push_str("## Success Criteria\n\n");
    prd.push_str("<!-- BEGIN SUCCESS_CRITERIA -->\n");
    if !success_criteria.is_empty() {
        let mut has_checkbox = false;
        for line in success_criteria.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("- [") {
                has_checkbox = true;
                prd.push_str(trimmed);
                prd.push('\n');
            }
        }
        if !has_checkbox {
            prd.push_str("- [ ] Bearing-backed workflow can be executed end-to-end in production conditions.\n");
        }
    } else {
        prd.push_str(
            "- [ ] Bearing-backed workflow can be executed end-to-end in production conditions.\n",
        );
    }
    prd.push_str("<!-- END SUCCESS_CRITERIA -->\n\n");

    // Analysis from assessment
    if !analysis.is_empty() {
        prd.push_str("## Research Analysis\n\n");
        prd.push_str("*From bearing assessment:*\n\n");
        prd.push_str(&format!("{}\n", analysis.trim()));
    }

    // Reference back to bearing
    prd.push_str("\n---\n\n");
    prd.push_str(&format!(
        "*This PRD was seeded from bearing `{}`. See `bearings/{}/` for original research.*\n",
        bearing.id(),
        bearing.id()
    ));

    Ok(prd)
}

/// Extract content under a markdown section header
fn extract_section(content: &str, header: &str) -> String {
    let mut in_section = false;
    let mut result = String::new();

    for line in content.lines() {
        if line.starts_with(header) {
            in_section = true;
            continue;
        }

        if in_section {
            // Stop at next section header
            if line.starts_with("## ") || line.starts_with("# ") {
                break;
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim().to_string()
}

/// Get the EV score for a bearing, if assessment exists and is complete
fn get_bearing_score(
    board_dir: &Path,
    bearing: &Bearing,
    weights: &crate::infrastructure::config::ModeWeights,
) -> Option<f64> {
    if !bearing.has_assessment {
        return None;
    }

    let assessment_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("ASSESSMENT.md");

    let factors = load_assessment(&assessment_path).ok()?;
    if !factors.is_complete() {
        return None;
    }

    calculate_score(&factors, weights)
        .ok()
        .map(|s| s.weighted_score)
}

/// Classify a bearing's exploration fog state.
fn classify_fog(board_dir: &Path, bearing: &Bearing) -> FogType {
    if bearing.has_survey && bearing.has_assessment {
        return FogType::Clear;
    }

    let brief_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("BRIEF.md");

    let brief_content = fs::read_to_string(&brief_path).unwrap_or_default();
    let open_questions = extract_section(&brief_content, "## Open Questions");
    let open_question_count = open_questions
        .lines()
        .filter(|line| line.trim_start().starts_with("- "))
        .count();
    let success_criteria = extract_section(&brief_content, "## Success Criteria");
    let unchecked_criteria = success_criteria
        .lines()
        .filter(|line| line.trim_start().starts_with("- [ ]"))
        .count();

    if bearing.frontmatter.status == BearingStatus::Exploring {
        if !bearing.has_survey && open_question_count > 0 {
            return FogType::Blocking;
        }

        if unchecked_criteria > 2 {
            return FogType::Blocking;
        }
    }

    match bearing.frontmatter.status {
        BearingStatus::Exploring | BearingStatus::Parked => FogType::Protecting,
        _ => FogType::Protecting,
    }
}

/// Format a factor value for display
fn format_factor(value: Option<u8>) -> String {
    value
        .map(|v| format!("{}/5", v))
        .unwrap_or_else(|| "-".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_bearing(temp: &TempDir) -> PathBuf {
        create_test_bearing_with_status(temp, "exploring")
    }

    fn create_test_bearing_with_status(temp: &TempDir, status: &str) -> PathBuf {
        let root = temp.path();
        let bearing_dir = root.join("bearings/test-research");
        fs::create_dir_all(&bearing_dir).unwrap();

        fs::write(
            bearing_dir.join("README.md"),
            format!(
                r#"---
id: test-research
title: Test Research
status: {}
---
"#,
                status
            ),
        )
        .unwrap();

        fs::write(bearing_dir.join("BRIEF.md"), "# Test Research — Brief\n").unwrap();

        root.to_path_buf()
    }

    fn create_fog_test_bearing(
        temp: &TempDir,
        status: &str,
        has_survey: bool,
        has_assessment: bool,
        open_questions: usize,
        unchecked_criteria: usize,
    ) -> PathBuf {
        let root = create_test_bearing_with_status(temp, status);
        let bearing_dir = root.join("bearings/test-research");

        let mut open_questions_section = String::new();
        for i in 0..open_questions {
            open_questions_section.push_str(&format!("- Question {}\n", i + 1));
        }

        let mut criteria_section = String::new();
        for i in 0..unchecked_criteria {
            criteria_section.push_str(&format!("- [ ] criterion {}\n", i + 1));
        }

        fs::write(
            bearing_dir.join("README.md"),
            format!(
                r#"---
id: test-research
title: Test Research
status: {}
---
"#,
                status
            ),
        )
        .unwrap();

        fs::write(
            bearing_dir.join("BRIEF.md"),
            format!(
                r#"# Test Research — Brief

## Success Criteria
{}

## Open Questions
{}
"#,
                criteria_section, open_questions_section
            ),
        )
        .unwrap();

        if has_survey {
            fs::write(bearing_dir.join("SURVEY.md"), "# Survey").unwrap();
        }
        if has_assessment {
            fs::write(bearing_dir.join("ASSESSMENT.md"), "# Assessment").unwrap();
        }

        root
    }

    // Note: survey/assess/park transition tests are in transitions/bearing_engine.rs
    // Command-level tests below verify other command workflows

    #[test]
    fn park_updates_status() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        update_bearing_status(&board_dir, bearing, BearingStatus::Parked).unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: parked"));
    }

    #[test]
    fn decline_updates_status_and_adds_reason() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        update_bearing_status_with_decline(&board_dir, bearing, "Not viable").unwrap();

        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: declined"));
        assert!(readme.contains("decline_reason: \"Not viable\""));
    }

    #[test]
    fn extract_section_finds_content() {
        let content = r#"# Title

## Hypothesis

This is the hypothesis content.
It spans multiple lines.

## Problem Space

Different section content.
"#;
        let hypothesis = extract_section(content, "## Hypothesis");
        assert!(hypothesis.contains("hypothesis content"));
        assert!(hypothesis.contains("multiple lines"));
        assert!(!hypothesis.contains("Different section"));
    }

    #[test]
    fn create_prd_from_bearing_includes_reference() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing(&temp);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        let prd = create_prd_from_bearing(&board_dir, bearing).unwrap();

        assert!(prd.contains("Test Research - Product Requirements"));
        assert!(prd.contains("seeded from bearing `test-research`"));
        assert!(prd.contains("bearings/test-research/"));
    }

    #[test]
    fn classify_fog_blocking() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_fog_test_bearing(&temp, "exploring", false, false, 2, 0);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        assert_eq!(classify_fog(&board_dir, bearing), FogType::Blocking);
    }

    #[test]
    fn classify_fog_protecting() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_fog_test_bearing(&temp, "exploring", true, false, 1, 0);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        assert_eq!(classify_fog(&board_dir, bearing), FogType::Protecting);
    }

    #[test]
    fn classify_fog_clear() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_fog_test_bearing(&temp, "exploring", true, true, 8, 5);
        let board = load_board(&board_dir).unwrap();
        let bearing = board.bearings.get("test-research").unwrap();

        assert_eq!(classify_fog(&board_dir, bearing), FogType::Clear);
    }
}
