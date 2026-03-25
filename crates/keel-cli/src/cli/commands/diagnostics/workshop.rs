//! Workshop command - focus on items requiring human attention

use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::style::VisualPadding;
use anyhow::Result;
use chrono::Timelike;
use keel::infrastructure::loader::load_board;
use keel::read_model::{workflow_lane_flow, workflow_topology};
use owo_colors::OwoColorize;

/// Run the workshop command
pub fn run(board_dir: &std::path::Path, scene: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let topology = workflow_topology::load_for_board(board_dir)?;
    let lane_flow = workflow_lane_flow::project(&board, &topology);

    // Get metrics for "sawdust" (drift)
    let metrics = keel::read_model::flow_status::project(&board, chrono::Utc::now());
    let health = keel::read_model::diagnostics::validate_report(board_dir)?;

    let mut human_items = Vec::new();
    for lane in &lane_flow.lanes {
        if lane.total_count > 0 {
            for source in &lane.source_counts {
                if source.count > 0 {
                    let items: Vec<_> = match source.source.as_str() {
                        "story.needs-human-verification" => board
                            .stories
                            .values()
                            .filter(|s| {
                                s.status == keel::domain::model::StoryState::NeedsHumanVerification
                            })
                            .map(|s| format!("Story {} - {}", s.id().yellow(), s.title()))
                            .collect(),
                        "story.in-progress" => board
                            .stories
                            .values()
                            .filter(|s| s.status == keel::domain::model::StoryState::InProgress)
                            .map(|s| format!("Active Story {} - {}", s.id().cyan(), s.title()))
                            .collect(),
                        "mission.achieved" => board
                            .missions
                            .values()
                            .filter(|m| m.status() == keel::domain::model::MissionStatus::Achieved)
                            .map(|m| format!("Mission {} - {}", m.id().cyan(), m.title()))
                            .collect(),
                        "voyage.draft" => board
                            .voyages
                            .values()
                            .filter(|v| {
                                v.status()
                                    == keel::domain::state_machine::voyage::VoyageState::Draft
                            })
                            .map(|v| format!("Voyage {} - {}", v.id().magenta(), v.title()))
                            .collect(),
                        "bearing.exploring" | "bearing.evaluating" | "bearing.ready" => board
                            .bearings
                            .values()
                            .filter(|b| !b.is_complete())
                            .map(|b| format!("Bearing {} - {}", b.id().green(), b.title()))
                            .collect(),
                        _ => vec![],
                    };
                    human_items.extend(items);
                }
            }
        }
    }

    // Draft epics are planning work — surface them even if not in a lane source
    let draft_epics: Vec<_> = board
        .epics
        .values()
        .filter(|e| e.status() == keel::domain::model::EpicState::Draft)
        .map(|e| format!("Epic {} - {} (draft)", e.id().bright_blue(), e.title()))
        .collect();
    for item in draft_epics {
        if !human_items.contains(&item) {
            human_items.push(item);
        }
    }

    let width = get_terminal_width();

    if scene {
        let healthy = health.passed();
        let (config, _) = keel::infrastructure::config::load_config();
        let current_hour = chrono::Local::now().hour() as u8;
        let within_working_hours = current_hour >= config.workflow.working_hours_start
            && current_hour < config.workflow.working_hours_end;
        let energized = config.workflow.open_for_work && within_working_hours;

        println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
        println!(
            "    │ {: <width$} │",
            "THE WORKBENCH".bold(),
            width = width.saturating_sub(8)
        );
        println!("    └{}┘", "─".repeat(width.saturating_sub(6)));

        // Work Lamp visual
        let lamp_color = if energized {
            owo_colors::AnsiColors::Yellow
        } else {
            owo_colors::AnsiColors::White
        };
        let lamp_style = if energized { "(*)" } else { "( )" };
        let lamp_label = if energized {
            "CIRCUIT CLOSED (ON THE CLOCK)"
        } else {
            "CIRCUIT OPEN (OFF THE CLOCK)"
        };

        let mut visual = String::new();
        visual.push_str("             | \n");
        visual.push_str("           __|__\n");
        if energized {
            visual.push_str(&format!(
                "          / {: <3} \\   <-- {}\n",
                lamp_style.color(lamp_color).bold(),
                lamp_label.color(lamp_color).bold()
            ));
            visual.push_str("         /  '.|.'  \\\n");
            visual.push_str("        /    ' '    \\\n");
        } else {
            visual.push_str(&format!(
                "          / {: <3} \\   <-- {}\n",
                lamp_style.dimmed(),
                lamp_label.dimmed()
            ));
            visual.push_str("          \\_____/\n");
        }

        // Ultra High Fidelity Workbench visual
        visual.push_str(
            "    ._____________________________________________________________________.\n",
        );
        visual.push_str(
            "    | [ PEGBOARD ]                                                        |\n",
        );

        let draft_epic_count = board
            .epics
            .values()
            .filter(|e| e.status() == keel::domain::model::EpicState::Draft)
            .count();
        let done_epic_count = board
            .epics
            .values()
            .filter(|e| e.status() == keel::domain::model::EpicState::Done)
            .count();
        let active_epic_count = board.epics.len() - draft_epic_count - done_epic_count;
        let congestion_ratio = if active_epic_count > 0 {
            draft_epic_count as f64 / active_epic_count as f64
        } else {
            draft_epic_count as f64
        };

        let pegboard = format!(
            "M:{} E:{}/{} B:{} A:{}",
            board.missions.len(),
            draft_epic_count,
            active_epic_count,
            board.bearings.len(),
            board.adrs.len()
        );

        let congestion_styled = if draft_epic_count > 0 || congestion_ratio > 2.0 {
            pegboard.yellow().bold().to_string()
        } else {
            pegboard.dimmed().to_string()
        };

        visual.push_str(&format!(
            "    |  .  .  .  .  .  .  .  {}  .  .  .  .  .  |\n",
            congestion_styled.pad_to_width(33)
        ));
        visual.push_str(
            "    |_____________________________________________________________________|\n",
        );
        visual.push_str(
            "    |                                                                     |\n",
        );

        let operational_fatigue = metrics.governance.proposed_count > 5;
        let drill_label = if operational_fatigue {
            "NOISY"
        } else {
            "DRILL PRESS"
        };
        let anvil_label = if operational_fatigue {
            "CLANGING"
        } else {
            "ANVIL"
        };

        visual.push_str(&format!(
            "    |  [ {: <11} ]                                     [ {: <7} ]      |\n",
            if operational_fatigue {
                drill_label.red().bold().to_string()
            } else {
                drill_label.to_string()
            },
            if operational_fatigue {
                anvil_label.red().bold().to_string()
            } else {
                anvil_label.to_string()
            }
        ));
        visual.push_str(
            "    |         _|_                                            _ _          |\n",
        );
        visual.push_str(
            "    |        (o o)                                          /   \\         |\n",
        );

        let occupancy = (human_items.len() as f64 / 10.0).min(1.0);
        let bar_width = 40;
        let filled = (occupancy * bar_width as f64) as usize;
        let mut items_on_bench = String::new();
        for i in 0..bar_width {
            if i < filled {
                items_on_bench.push('█');
            } else {
                items_on_bench.push(' ');
            }
        }

        let bench_line = format!(
            "[ {} ]   <-- BENCH WIP ({})",
            items_on_bench.yellow(),
            human_items.len()
        );
        visual.push_str(&format!("    |   {} |\n", bench_line.pad_to_width(65)));
        visual.push_str(
            "    |_____________________________________________________________________|\n",
        );
        visual.push_str(
            "    |                                                                     |\n",
        );

        let blocked = metrics.execution.backlog_blocked_count;
        let remediation = health.estimated_remediation_hours;
        let vice_label = format!("[ VICE ]  {} BLOCKED", blocked);
        let oil_label = format!("[ OIL CAN ]  {:.1}h REMEDIATION", remediation);

        let vice_styled = if blocked > 0 {
            vice_label.red().bold().to_string()
        } else {
            vice_label.dimmed().to_string()
        };
        let oil_styled = if remediation > 0.0 {
            oil_label.yellow().bold().to_string()
        } else {
            oil_label.dimmed().to_string()
        };

        visual.push_str(&format!(
            "    |   {}        {}   |\n",
            vice_styled.pad_to_width(28),
            oil_styled.pad_to_width(25)
        ));
        visual.push_str(
            "    |_____________________________________________________________________|\n",
        );
        visual.push_str(
            "      | |                                                               | |\n",
        );

        // Deterministic sawdust based on drift
        let drift = health.drift_coefficient;
        let mut floor = String::from("      | |         ");
        for i in 0..40 {
            // Pseudo-random pattern based on i and drift
            let noise = ((i as f64 * 1.618).sin() + 1.0) / 2.0;
            if noise < drift {
                if i % 3 == 0 {
                    floor.push_str(&":".dimmed().to_string());
                } else if i % 2 == 0 {
                    floor.push_str(&".".dimmed().to_string());
                } else {
                    floor.push_str(&"*".dimmed().to_string());
                }
            } else {
                floor.push(' ');
            }
        }
        floor.push_str("      | |\n");

        visual.push_str(&floor);
        let label = if !healthy {
            "SYSTEM UNHEALTHY (BROKEN TOOLS)"
        } else if drift > 0.5 {
            "SHOP ENTROPY (SEVERE)"
        } else {
            "SHOP SAWDUST (DRIFT)"
        };
        visual.push_str(&format!(
            "      | |         {: ^40}      | |\n",
            label.dimmed()
        ));
        visual.push_str(
            "     _|_|_                                                           _|_|_\n",
        );
        visual.push_str(
            "    |_____|                                                         |_____|\n",
        );

        println!("{}", visual);

        if !human_items.is_empty() {
            println!("    REQUIRED DECISIONS:");
            for item in human_items.iter().take(5) {
                println!("      - {}", item);
            }
            if human_items.len() > 5 {
                println!("      ... and {} more", human_items.len() - 5);
            }
        } else {
            println!("    (The bench is clean)");
        }

        // Entropy details
        println!("\n    ENTROPY DETAILS:");
        let drift_color = if health.drift_coefficient > 0.5 {
            owo_colors::AnsiColors::Red
        } else if health.drift_coefficient > 0.2 {
            owo_colors::AnsiColors::Yellow
        } else {
            owo_colors::AnsiColors::Green
        };

        println!(
            "      - Structural Drift:  {}",
            format!("{:.2}", health.drift_coefficient)
                .color(drift_color)
                .bold()
        );

        let unlinked_knowledge = metrics.governance.proposed_count;
        if unlinked_knowledge > 0 {
            println!(
                "      - Floor Mess:        {} unlinked knowledge units",
                unlinked_knowledge.yellow()
            );
        }

        let mut all_problems = Vec::new();
        all_problems.extend(&health.story_checks);
        all_problems.extend(&health.voyage_checks);
        all_problems.extend(&health.epic_checks);
        all_problems.extend(&health.adr_checks);
        all_problems.extend(&health.bearing_checks);
        all_problems.extend(&health.mission_checks);
        all_problems.extend(&health.routine_checks);
        all_problems.extend(&health.workflow_checks);

        let blocking_problems: usize = all_problems
            .iter()
            .map(|r| {
                r.problems
                    .iter()
                    .filter(|p| p.severity == keel::infrastructure::validation::Severity::Error)
                    .count()
            })
            .sum();

        if blocking_problems > 0 {
            println!(
                "      - Broken Tools:      {} doctor errors",
                blocking_problems.red().bold()
            );
        }

        println!("\n    \"A broken workshop is a messy workshop!\"");
        return Ok(());
    }

    println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
    println!(
        "    │ {: <width$} │",
        "THE WORKBENCH".bold(),
        width = width.saturating_sub(8)
    );
    println!("    └{}┘", "─".repeat(width.saturating_sub(6)));

    // 1. Bench Occupancy
    let occupancy = (human_items.len() as f64 / 5.0).min(1.0);
    let bar_width = 20;
    let filled = (occupancy * bar_width as f64) as usize;
    let bar = format!(
        "{}{}",
        "█".repeat(filled).yellow(),
        "░".repeat(bar_width - filled).dimmed()
    );

    println!(
        "\n    BENCH OCCUPANCY: [ {} ] {} items",
        bar,
        human_items.len()
    );

    if human_items.is_empty() {
        println!("    (The bench is clean)");
    } else {
        for item in human_items {
            println!("      - {}", item);
        }
    }

    // 2. The Sawdust (Drift & Entropy)
    println!("\n    SHOP ENTROPY (SAWDUST):");

    let drift_color = if health.drift_coefficient > 0.5 {
        owo_colors::AnsiColors::Red
    } else if health.drift_coefficient > 0.2 {
        owo_colors::AnsiColors::Yellow
    } else {
        owo_colors::AnsiColors::Green
    };

    println!(
        "      - {: <18} {}",
        "Structural Drift:",
        format!("{:.2}", health.drift_coefficient)
            .color(drift_color)
            .bold()
    );

    let unlinked_knowledge = metrics.governance.proposed_count; // Simplified proxy for mess
    if unlinked_knowledge > 0 {
        println!(
            "      - {: <18} {} unlinked knowledge units",
            "Floor Mess:",
            unlinked_knowledge.yellow()
        );
    }

    // Check all section results for errors
    let mut all_problems = Vec::new();
    all_problems.extend(&health.story_checks);
    all_problems.extend(&health.voyage_checks);
    all_problems.extend(&health.epic_checks);
    all_problems.extend(&health.adr_checks);
    all_problems.extend(&health.bearing_checks);
    all_problems.extend(&health.mission_checks);
    all_problems.extend(&health.routine_checks);
    all_problems.extend(&health.workflow_checks);

    let blocking_problems: usize = all_problems
        .iter()
        .map(|r| {
            r.problems
                .iter()
                .filter(|p| p.severity == keel::infrastructure::validation::Severity::Error)
                .count()
        })
        .sum();

    if blocking_problems > 0 {
        println!(
            "      - {: <18} {} broken parts (doctor errors)",
            "Broken Tools:",
            blocking_problems.red().bold()
        );
    }

    println!("\n    \"A broken workshop is a messy workshop!\"");

    Ok(())
}
