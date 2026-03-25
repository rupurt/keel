//! Doctor command - board health diagnostics and automated fixing

pub mod render;

use crate::cli::style::VisualPadding;
use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use keel::read_model::diagnostics::fixes::run_fixes;
use keel::read_model::diagnostics::validate_report as validate;

/// Run the doctor command
pub fn run(
    board_dir: &Path,
    fix: bool,
    _evidence: bool,
    _watch: bool,
    _quick: bool,
    scene: bool,
    status: bool,
) -> Result<()> {
    let _start = Instant::now();

    if status {
        return run_status(board_dir);
    }

    let report = validate(board_dir)?;

    if scene {
        use owo_colors::OwoColorize;
        let passed = report.passed();

        let core_passed = report.mission_checks.iter().all(|c| !c.has_errors())
            && report.pacemaker_checks.iter().all(|c| !c.has_errors());
        let strategy_passed = report.epic_checks.iter().all(|c| !c.has_errors())
            && report.bearing_checks.iter().all(|c| !c.has_errors())
            && report.adr_checks.iter().all(|c| !c.has_errors());
        let execution_passed = report.voyage_checks.iter().all(|c| !c.has_errors())
            && report.story_checks.iter().all(|c| !c.has_errors())
            && report.routine_checks.iter().all(|c| !c.has_errors());
        let flow_passed = report.workflow_checks.iter().all(|c| !c.has_errors())
            && report.delivery_checks.iter().all(|c| !c.has_errors());

        println!(
            "\n    ┌───────────────────────────[ THE LABORATORY ]───────────────────────────┐"
        );

        let mut visual = String::new();
        visual.push_str(
            "    │                                                                        │\n",
        );
        visual.push_str(
            "    │  [ VITAL SIGNS MONITOR ]                                               │\n",
        );
        visual.push_str(&format!(
            "    │  {} │\n",
            ".----------------------------.".pad_to_width(69)
        ));

        if passed {
            let sys_label = format!("SYS: {}", "NOMINAL".green().bold());
            let hr_label = format!("HR:  {}", "72 BPM".green().bold());

            let unit1 = "__/\x1b[1;32m^\x1b[0m\\__".to_string(); // visible width 7
            let ekg1 = format!("{}{}{}{}", unit1, unit1, unit1, unit1);

            let unit2 = "_/    \\".to_string(); // visible width 7
            let ekg2 = format!("{}{}{}{}", unit2, unit2, unit2, unit2);

            let line1_content = format!(" |{}|    {}", ekg1, sys_label);
            let line2_content = format!(" |{}|    {}", ekg2, hr_label);

            visual.push_str(&format!("    │ {} │\n", line1_content.pad_to_width(70)));
            visual.push_str(&format!("    │ {} │\n", line2_content.pad_to_width(70)));
        } else {
            let sys_label = format!("SYS: {}", "CRITICAL".red().bold());
            let hr_label = format!("HR:  {}", "--- BPM".red().bold());

            let line1_content = format!(
                " | {} |    {}",
                "__________________________".red(),
                sys_label
            );
            let line2_content = format!(" |                            |    {}", hr_label);

            visual.push_str(&format!("    │ {} │\n", line1_content.pad_to_width(70)));
            visual.push_str(&format!("    │ {} │\n", line2_content.pad_to_width(70)));
        }

        visual.push_str(&format!(
            "    │  {} │\n",
            "'----------------------------'".pad_to_width(69)
        ));
        visual.push_str(
            "    │                                                                        │\n",
        );

        let status = |p: bool| {
            if p {
                "NOM".green().bold().to_string()
            } else {
                "ERR".red().bold().to_string()
            }
        };
        let subsystems = format!(
            "CORE: [{}]    STRAT: [{}]    EXEC: [{}]    FLOW: [{}]",
            status(core_passed),
            status(strategy_passed),
            status(execution_passed),
            status(flow_passed)
        );
        visual.push_str(&format!("    │       {}        │\n", subsystems));

        visual.push_str(
            "    │                                                                        │\n",
        );
        visual.push_str(
            "    └────────────────────────────────────────────────────────────────────────┘\n",
        );

        println!("{}", visual);

        if passed {
            println!(
                "    \"Patient is responsive. Subsystems are operating within normal parameters.\""
            );
        } else {
            println!("    \"EMERGENCY: Structural flatline detected in laboratory bio-scan.\"");
            println!("    Review the Itemized Pathology report below.");
        }
        return Ok(());
    }

    render::print_report(&report);

    if fix {
        // Invalidate the diagnostics cache to ensure fixes are applied to fresh data
        let cache_path = keel::read_model::diagnostics::cache::cache_path(board_dir);
        if cache_path.exists() {
            let _ = std::fs::remove_file(cache_path);
        }
        run_fixes(board_dir, &report)?;
    }

    let errors = report.total_errors();

    if errors > 0 {
        anyhow::bail!("Board has {} errors", errors);
    }

    Ok(())
}

/// Print a 3-bullet importance summary of the board graph.
///
/// Each bullet is at most 8–10 words with a 3–6 character taxonomy ID,
/// designed for embedding in commit messages.
fn run_status(board_dir: &Path) -> Result<()> {
    use keel::domain::model::{EpicState, StoryState, VoyageState};
    use keel::infrastructure::loader::load_board;
    use keel::read_model::board_graph::build_board_graph;

    let board = load_board(board_dir)?;
    let graph = build_board_graph(&board);
    let report = validate(board_dir)?;

    // --- Mission layer (MSN) ---
    let active_missions = board
        .missions
        .values()
        .filter(|m| !m.status().is_terminal())
        .count();
    let active_epics = board
        .epics
        .values()
        .filter(|e| e.status() != EpicState::Done)
        .count();
    let mission_bullet = if active_missions == 0 {
        "No missions defined on the board".to_string()
    } else {
        let m_word = if active_missions == 1 {
            "mission"
        } else {
            "missions"
        };
        let e_word = if active_epics == 1 { "epic" } else { "epics" };
        format!(
            "{} {} driving {} {} forward",
            active_missions, m_word, active_epics, e_word
        )
    };

    // --- Execution layer (EXC) ---
    let in_progress = board
        .stories
        .values()
        .filter(|s| s.status() == StoryState::InProgress)
        .count();
    let active_voyages = board
        .voyages
        .values()
        .filter(|v| v.status() == VoyageState::InProgress)
        .count();
    let backlog = board
        .stories
        .values()
        .filter(|s| s.status() == StoryState::Backlog)
        .count();
    let exec_bullet = if in_progress == 0 && backlog == 0 {
        "Board idle, no stories queued or active".to_string()
    } else if in_progress == 0 {
        format!("{} stories queued, none actively executing", backlog)
    } else {
        let v_clause = if active_voyages > 0 {
            let v_word = if active_voyages == 1 {
                "voyage"
            } else {
                "voyages"
            };
            format!(" across {} {}", active_voyages, v_word)
        } else {
            String::new()
        };
        let s_word = if in_progress == 1 { "story" } else { "stories" };
        format!("{} {} executing{}", in_progress, s_word, v_clause)
    };

    // --- Health layer (HLT) ---
    let errors = report.total_errors();
    let warnings = report.total_warnings();
    let dangling = graph.dangling_edges().len();
    let health_bullet = if errors == 0 && warnings == 0 && dangling == 0 {
        "All checks passing, graph fully connected".to_string()
    } else if errors == 0 {
        format!("{} warnings, no structural errors detected", warnings)
    } else {
        format!(
            "{} errors and {} warnings need remediation",
            errors, warnings
        )
    };

    println!("• [MSN] {}", mission_bullet);
    println!("• [EXC] {}", exec_bullet);
    println!("• [HLT] {}", health_bullet);

    Ok(())
}
