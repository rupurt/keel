//! Workshop command - focus on items requiring human attention

use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::style::VisualPadding;
use anyhow::Result;
use keel::infrastructure::loader::load_board;
use keel::read_model::{workflow_lane_flow, workflow_topology};
use owo_colors::OwoColorize;
use txt_scene::{SceneFrame, SceneLine, visible_width};

const WORKSHOP_SCENE_WIDTH: usize = 75;
const WORKSHOP_FRAME_WIDTH: usize = 69;
const WORKSHOP_SCENE_INDENT: &str = "    ";
const WORKSHOP_FLOOR_INDENT: &str = "     ";
const WORKSHOP_FLOOR_WIDTH: usize = 62;

fn render_workshop_line<F>(build: F) -> String
where
    F: FnOnce(&mut SceneLine),
{
    let mut line = SceneLine::new(WORKSHOP_SCENE_WIDTH);
    build(&mut line);
    line.finish()
}

fn centered_visible(text: impl Into<String>, width: usize) -> String {
    let text = text.into();
    let visible = visible_width(&text);
    let total_padding = width.saturating_sub(visible);
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;

    format!(
        "{}{}{}",
        " ".repeat(left_padding),
        text,
        " ".repeat(right_padding)
    )
}

fn render_workshop_scene(
    board: &keel::domain::model::Board,
    human_items: &[String],
    metrics: &keel::read_model::flow_metrics::FlowMetrics,
    health: &keel::read_model::diagnostics::DoctorReport,
    energized: bool,
) -> String {
    let bench_frame = SceneFrame::new(WORKSHOP_SCENE_INDENT, "|", "|", WORKSHOP_FRAME_WIDTH);
    let floor_frame = SceneFrame::new(WORKSHOP_FLOOR_INDENT, "| |", "| |", WORKSHOP_FLOOR_WIDTH);

    let lamp_color = if energized {
        owo_colors::AnsiColors::Yellow
    } else {
        owo_colors::AnsiColors::White
    };
    let lamp_style = if energized { "(*)" } else { "( )" };
    let lamp_label = if energized {
        "CIRCUIT CLOSED (HEARTBEAT ENERGIZED)"
    } else {
        "CIRCUIT OPEN (RUN keel heartbeat)"
    };

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

    let drill_styled = if operational_fatigue {
        drill_label.red().bold().to_string()
    } else {
        drill_label.to_string()
    };
    let anvil_styled = if operational_fatigue {
        anvil_label.red().bold().to_string()
    } else {
        anvil_label.to_string()
    };

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

    let drift = health.drift_coefficient;
    let mut sawdust_pattern = String::new();
    for i in 0..40 {
        let noise = ((i as f64 * 1.618).sin() + 1.0) / 2.0;
        if noise < drift {
            if i % 3 == 0 {
                sawdust_pattern.push_str(&":".dimmed().to_string());
            } else if i % 2 == 0 {
                sawdust_pattern.push_str(&".".dimmed().to_string());
            } else {
                sawdust_pattern.push_str(&"*".dimmed().to_string());
            }
        } else {
            sawdust_pattern.push(' ');
        }
    }

    let healthy = health.passed();
    let dust_label = if !healthy {
        "SYSTEM UNHEALTHY (BROKEN TOOLS)"
    } else if drift > 0.5 {
        "SHOP ENTROPY (SEVERE)"
    } else {
        "SHOP SAWDUST (DRIFT)"
    };
    let dust_label = centered_visible(dust_label.dimmed().to_string(), 40);

    let mut lines = vec![
        render_workshop_line(|line| {
            line.pad_to(4)
                .push("┌")
                .push("─".repeat(WORKSHOP_FRAME_WIDTH))
                .push("┐");
        }),
        bench_frame.row(|line| {
            line.push(" THE WORKBENCH".bold().to_string());
        }),
        render_workshop_line(|line| {
            line.pad_to(4)
                .push("└")
                .push("─".repeat(WORKSHOP_FRAME_WIDTH))
                .push("┘");
        }),
        render_workshop_line(|line| {
            line.pad_to(13).push("|");
        }),
        render_workshop_line(|line| {
            line.pad_to(11).push("__|__");
        }),
    ];

    if energized {
        lines.push(render_workshop_line(|line| {
            line.pad_to(10)
                .push("/ ")
                .push(lamp_style.color(lamp_color).bold().to_string())
                .push(" \\   <-- ")
                .push(lamp_label.color(lamp_color).bold().to_string());
        }));
        lines.push(render_workshop_line(|line| {
            line.pad_to(9).push("/  '.|.'\\");
        }));
        lines.push(render_workshop_line(|line| {
            line.pad_to(8).push("/    ' '  \\");
        }));
    } else {
        lines.push(render_workshop_line(|line| {
            line.pad_to(10)
                .push("/ ")
                .push(lamp_style.dimmed().to_string())
                .push(" \\   <-- ")
                .push(lamp_label.dimmed().to_string());
        }));
        lines.push(render_workshop_line(|line| {
            line.pad_to(10).push("\\_____/");
        }));
    }

    lines.push(render_workshop_line(|line| {
        line.pad_to(4)
            .push(".")
            .push("_".repeat(WORKSHOP_FRAME_WIDTH))
            .push(".");
    }));
    lines.push(bench_frame.row(|line| {
        line.push(" [ PEGBOARD ]");
    }));
    lines.push(bench_frame.row(|line| {
        const RIGHT_DECOR: &str = ".  .  .  .  .  ";

        line.push("  .  .  .  .  .  .  .  ");
        line.push(&congestion_styled);
        line.pad_to(WORKSHOP_FRAME_WIDTH - visible_width(RIGHT_DECOR));
        line.push(RIGHT_DECOR);
    }));
    lines.push(render_workshop_line(|line| {
        line.pad_to(4)
            .push("|")
            .push("_".repeat(WORKSHOP_FRAME_WIDTH))
            .push("|");
    }));
    lines.push(bench_frame.empty_row());
    lines.push(bench_frame.row(|line| {
        line.push("  ");
        line.push(format!("[ {} ]", drill_styled.pad_to_width(11)));
        line.pad_to(52);
        line.push(format!("[ {} ]", anvil_styled.pad_to_width(7)));
    }));
    lines.push(bench_frame.row(|line| {
        line.push("         _|_");
        line.pad_to(51);
        line.push("_ _");
    }));
    lines.push(bench_frame.row(|line| {
        line.push("        (o o)");
        line.pad_to(50);
        line.push("/   \\");
    }));
    lines.push(bench_frame.row(|line| {
        line.push("   [ ");
        line.push(items_on_bench.yellow().to_string());
        line.push(" ]   <-- BENCH WIP (");
        line.push(human_items.len().to_string());
        line.push(")");
    }));
    lines.push(render_workshop_line(|line| {
        line.pad_to(4)
            .push("|")
            .push("_".repeat(WORKSHOP_FRAME_WIDTH))
            .push("|");
    }));
    lines.push(bench_frame.empty_row());
    lines.push(bench_frame.row(|line| {
        line.push("   ");
        line.push(vice_styled.pad_to_width(24));
        line.push("   ");
        line.push(oil_styled.pad_to_width(36));
    }));
    lines.push(render_workshop_line(|line| {
        line.pad_to(4)
            .push("|")
            .push("_".repeat(WORKSHOP_FRAME_WIDTH))
            .push("|");
    }));
    lines.push(render_workshop_line(|line| {
        line.push(floor_frame.empty_row());
    }));
    lines.push(render_workshop_line(|line| {
        line.push(floor_frame.row(|floor| {
            floor.push("         ");
            floor.push(&sawdust_pattern);
        }));
    }));
    lines.push(render_workshop_line(|line| {
        line.push(floor_frame.row(|floor| {
            floor.push("         ");
            floor.push(dust_label);
        }));
    }));
    lines.push(render_workshop_line(|line| {
        line.pad_to(4).push("_|_|_");
        line.pad_to(69).push("_|_|_");
    }));
    lines.push(render_workshop_line(|line| {
        line.pad_to(3).push("|_____|");
        line.pad_to(68).push("|_____|");
    }));

    lines.join("\n")
}

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
        let energized = super::heartbeat::inspect(board_dir, chrono::Utc::now()).energized;

        println!(
            "\n{}",
            render_workshop_scene(&board, &human_items, &metrics, &health, energized)
        );

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use keel::infrastructure::loader::load_board;
    use keel::test_helpers::TestBoardBuilder;

    #[test]
    fn workshop_scene_has_stable_width_with_colorized_content() {
        let temp = TestBoardBuilder::new().build();
        let board = load_board(temp.path()).unwrap();
        let metrics = keel::read_model::flow_status::project(&board, Utc::now());
        let health = keel::read_model::diagnostics::validate_report(temp.path()).unwrap();

        let scene = render_workshop_scene(
            &board,
            &["Story S1 - Needs review".to_string()],
            &metrics,
            &health,
            true,
        );

        assert!(scene.contains("\x1b["));
        assert!(
            scene
                .lines()
                .all(|line| visible_width(line) == WORKSHOP_SCENE_WIDTH)
        );
    }

    #[test]
    fn workshop_scene_has_stable_width_when_dimmed() {
        let temp = TestBoardBuilder::new().build();
        let board = load_board(temp.path()).unwrap();
        let metrics = keel::read_model::flow_status::project(&board, Utc::now());
        let health = keel::read_model::diagnostics::validate_report(temp.path()).unwrap();

        let scene = render_workshop_scene(&board, &[], &metrics, &health, false);

        assert!(
            scene
                .lines()
                .all(|line| visible_width(line) == WORKSHOP_SCENE_WIDTH)
        );
    }

    #[test]
    fn workshop_scene_lamp_label_tracks_heartbeat_state() {
        let temp = TestBoardBuilder::new().build();
        let board = load_board(temp.path()).unwrap();
        let metrics = keel::read_model::flow_status::project(&board, Utc::now());
        let health = keel::read_model::diagnostics::validate_report(temp.path()).unwrap();

        let energized = render_workshop_scene(&board, &[], &metrics, &health, true);
        let idle = render_workshop_scene(&board, &[], &metrics, &health, false);

        assert!(energized.contains("CIRCUIT CLOSED (HEARTBEAT ENERGIZED)"));
        assert!(idle.contains("CIRCUIT OPEN (RUN keel heartbeat)"));
    }

    #[test]
    fn workshop_scene_right_support_stays_centered_under_floor_posts() {
        let temp = TestBoardBuilder::new().build();
        let board = load_board(temp.path()).unwrap();
        let metrics = keel::read_model::flow_status::project(&board, Utc::now());
        let health = keel::read_model::diagnostics::validate_report(temp.path()).unwrap();

        let scene = render_workshop_scene(&board, &[], &metrics, &health, false);
        let lines: Vec<_> = scene.lines().collect();
        let floor_empty = lines[lines.len() - 5];
        let foot_row = lines[lines.len() - 2];
        let base_row = lines[lines.len() - 1];

        assert_eq!(floor_empty.rfind("| |"), Some(70));
        assert_eq!(foot_row.rfind("_|_|_"), Some(69));
        assert_eq!(base_row.rfind("|_____|"), Some(68));
    }
}
