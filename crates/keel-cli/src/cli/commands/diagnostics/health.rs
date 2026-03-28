//! Health command - subsystem status check and bio-scan (The Med-Bay)

use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::style::VisualPadding;
use anyhow::Result;
use keel::infrastructure::loader::load_board;
use keel::read_model::diagnostics::types::CheckResult;
use keel::read_model::{diagnostics, flow_status};
use owo_colors::OwoColorize;
use txt_scene::{SceneFrame, SceneLine};

const HEALTH_SCENE_WIDTH: usize = 75;
const HEALTH_SCENE_MONITOR_INDENT: &str = "             ";
const HEALTH_SCENE_MONITOR_WIDTH: usize = 42;
const HEALTH_SCENE_POD_INDENT: usize = 14;
const HEALTH_SCENE_POD_GAP_COLUMN: usize = 47;
const HEALTH_SCENE_POD_WIDTH: usize = 13;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CategoryStatus {
    Nominal,
    Warning,
    Failure,
}

impl CategoryStatus {
    fn marker(self) -> String {
        match self {
            Self::Nominal => "✓".green().to_string(),
            Self::Warning => "!".yellow().to_string(),
            Self::Failure => "✗".red().to_string(),
        }
    }

    fn scene_label(self) -> &'static str {
        match self {
            Self::Nominal => "NOMINAL",
            Self::Warning => "WATCH",
            Self::Failure => "FAILURE",
        }
    }

    fn scene_color(self) -> owo_colors::AnsiColors {
        match self {
            Self::Nominal => owo_colors::AnsiColors::Green,
            Self::Warning => owo_colors::AnsiColors::Yellow,
            Self::Failure => owo_colors::AnsiColors::Red,
        }
    }
}

fn category_status(results: &[CheckResult]) -> CategoryStatus {
    if results.iter().any(CheckResult::has_errors) {
        CategoryStatus::Failure
    } else if results.iter().any(CheckResult::has_warnings) {
        CategoryStatus::Warning
    } else {
        CategoryStatus::Nominal
    }
}

/// Run the health command
pub fn run(board_dir: &std::path::Path, scene: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let report = diagnostics::validate_report(board_dir)?;
    let metrics = flow_status::project(&board, chrono::Utc::now());
    let passed = report.passed();
    let warning_active = report.total_warnings() > 0;

    // 1. Strategic Congestion (Top-heavy backlog)
    // Congested if many draft epics (> 5) or many incomplete missions (> 3)
    let draft_epics = board
        .epics
        .values()
        .filter(|e| e.status() == keel::domain::model::EpicState::Draft)
        .count();
    let strategic_congested = draft_epics > 5 || metrics.incomplete_missions_count > 3;

    // 3. Operational Fatigue (Operational noise)
    // Fatigued if many due routines (> 3) or too much unlinked governance
    let operational_fatigue =
        metrics.due_routines_count > 3 || metrics.governance.proposed_count > 5;

    let width = get_terminal_width();

    if scene {
        render_med_bay_scene(
            &report,
            &metrics,
            strategic_congested,
            operational_fatigue,
            width,
        );
    } else {
        println!(
            "\n    {} SUBSYSTEM STATUS REPORT",
            " HEALTH ".on_green().black().bold()
        );

        print_category("Stories", &report.story_checks);
        print_category("Voyages", &report.voyage_checks);
        print_category("Epics", &report.epic_checks);
        print_category("Bearings", &report.bearing_checks);
        print_category("ADRs", &report.adr_checks);
        print_category("Missions", &report.mission_checks);
        print_category("Routines", &report.routine_checks);
        print_category("Workflow", &report.workflow_checks);
        print_category("Pacemaker", &report.pacemaker_checks);
        print_category("Delivery", &report.delivery_checks);

        if strategic_congested {
            println!("    ! {: <10} {}", "Strategy", "CONGESTED".yellow().bold());
        }
        if operational_fatigue {
            println!("    ! {: <10} {}", "Ops", "FATIGUE".red().bold());
        }

        if passed && !warning_active && !strategic_congested && !operational_fatigue {
            println!("\n    {} System is 100% healthy.", "✓".green().bold());
        } else if passed {
            println!("\n    ! System is stable but under observation.");
        } else {
            println!(
                "\n    {} System has issues. Run `keel doctor` for details.",
                "✗".red().bold()
            );
        }
    }

    if passed {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn print_category(name: &str, results: &[CheckResult]) {
    println!("    • {: <10} {}", name, category_status(results).marker());
}

fn render_health_line<F>(build: F) -> String
where
    F: FnOnce(&mut SceneLine),
{
    let mut line = SceneLine::new(HEALTH_SCENE_WIDTH);
    build(&mut line);
    line.finish()
}

fn render_monitor_row<F>(monitor_frame: &SceneFrame<'_>, build: F) -> String
where
    F: FnOnce(&mut SceneLine),
{
    render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_MONITOR_INDENT.len())
            .push(monitor_frame.row(build));
    })
}

fn render_monitor_empty_row(monitor_frame: &SceneFrame<'_>) -> String {
    render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_MONITOR_INDENT.len())
            .push(monitor_frame.empty_row());
    })
}

fn render_med_bay_scene(
    report: &keel::read_model::diagnostics::types::DoctorReport,
    metrics: &keel::read_model::flow_metrics::FlowMetrics,
    strategic_congested: bool,
    operational_fatigue: bool,
    width: usize,
) {
    println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
    println!(
        "    │ {: <width$} │",
        "THE MED-BAY (ULTRA-RES BIO-SCAN)".bold(),
        width = width.saturating_sub(8)
    );
    println!("    └{}┘", "─".repeat(width.saturating_sub(6)));

    let passed = report.passed();

    println!(
        "{}",
        build_med_bay_scene(
            report,
            metrics,
            strategic_congested,
            operational_fatigue,
            true,
        )
    );

    println!("    SCANNING SUBSYSTEMS...");

    let categories = [
        ("NEURAL", &report.story_checks),
        ("MOTOR", &report.voyage_checks),
        ("STRATEGIC", &report.epic_checks),
        ("SENSORY", &report.bearing_checks),
        ("SKELETAL", &report.adr_checks),
        ("VITAL", &report.mission_checks),
        ("AUTONOMIC", &report.routine_checks),
        ("CIRCULATORY", &report.workflow_checks),
        ("PACEMAKER", &report.pacemaker_checks),
        ("KINETIC", &report.delivery_checks),
    ];

    for (name, results) in categories {
        let status = category_status(results);
        println!(
            "      - {: <12} [ {} ]",
            name,
            status.scene_label().color(status.scene_color()).bold()
        );
    }

    let warning_active = report.total_warnings() > 0;
    if passed && warning_active {
        println!(
            "\n    \"Patient is fit for duty. Monitor flagged subsystems for warning-level drift.\""
        );
    } else if passed {
        println!("\n    \"Patient is fit for duty. The garden is in good hands.\"");
    } else {
        println!("\n    \"EMERGENCY: Subsystem failure detected. Scrub in with `keel doctor`.\"");
    }
}

fn build_med_bay_scene(
    report: &keel::read_model::diagnostics::types::DoctorReport,
    metrics: &keel::read_model::flow_metrics::FlowMetrics,
    strategic_congested: bool,
    operational_fatigue: bool,
    use_color: bool,
) -> String {
    let passed = report.passed();
    let kinetic_load = metrics.verification.count + metrics.execution.in_progress_count;
    let (heart_rate, status_label, status_color) = if !passed {
        ("140 BPM", "CRITICAL", owo_colors::AnsiColors::Red)
    } else if kinetic_load > 15 || (strategic_congested && kinetic_load > 8) {
        ("120 BPM", "OVERLOAD", owo_colors::AnsiColors::Red)
    } else if kinetic_load > 7 || strategic_congested || operational_fatigue {
        ("100 BPM", "ELEVATED", owo_colors::AnsiColors::Yellow)
    } else if kinetic_load > 3 {
        ("85 BPM", "STRESSED", owo_colors::AnsiColors::Cyan)
    } else {
        ("72 BPM", "STABLE", owo_colors::AnsiColors::Green)
    };
    let monitor_frame = SceneFrame::new("", "|", "|", HEALTH_SCENE_MONITOR_WIDTH);
    let pod_frame = SceneFrame::new("", "|", "|", HEALTH_SCENE_POD_WIDTH);

    let paint = |text: String, status: CategoryStatus| -> String {
        if !use_color {
            return text;
        }

        match status {
            CategoryStatus::Nominal => text.green().bold().to_string(),
            CategoryStatus::Warning => text.yellow().bold().to_string(),
            CategoryStatus::Failure => text.red().bold().to_string(),
        }
    };

    // High Res Heartbeat pulse (EKG)
    let mut lines = vec![
        render_health_line(|line| {
            line.pad_to(HEALTH_SCENE_MONITOR_INDENT.len())
                .push(".")
                .push("_".repeat(HEALTH_SCENE_MONITOR_WIDTH))
                .push(".");
        }),
        render_monitor_row(&monitor_frame, |line| {
            line.push(" [ MAIN DIAGNOSTIC MONITOR - HIGH RES ]");
        }),
        render_monitor_empty_row(&monitor_frame),
    ];

    if !passed || kinetic_load > 15 {
        lines.push(render_monitor_row(&monitor_frame, |line| {
            let pulse = if use_color {
                format!(
                    "{}{}",
                    "/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\"
                        .red()
                        .bold(),
                    "/\\".red().bold()
                )
            } else {
                "/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\".to_string()
            };
            line.push("   ");
            line.push(pulse);
        }));
        lines.push(render_monitor_row(&monitor_frame, |line| {
            let alarm = if use_color {
                "!! ALARM !! ALARM !! ALARM !! ALARM !!"
                    .red()
                    .bold()
                    .to_string()
            } else {
                "!! ALARM !! ALARM !! ALARM !! ALARM !!".to_string()
            };
            line.push("   ");
            line.push(alarm);
            line.push("   ");
        }));
    } else if kinetic_load > 7 || strategic_congested {
        lines.push(render_monitor_row(&monitor_frame, |line| {
            let upper = if use_color {
                "_/^\\___/^\\___/^\\___/^\\___/^\\___/^\\___/^\\_"
                    .yellow()
                    .bold()
                    .to_string()
            } else {
                "_/^\\___/^\\___/^\\___/^\\___/^\\___/^\\___/^\\_".to_string()
            };
            line.push("   ");
            line.push(upper);
            line.push("   ");
        }));
        lines.push(render_monitor_row(&monitor_frame, |line| {
            let lower = if use_color {
                "\\_/   \\_/   \\_/   \\_/   \\_/   \\_/   \\_/   \\"
                    .yellow()
                    .bold()
                    .to_string()
            } else {
                "\\_/   \\_/   \\_/   \\_/   \\_/   \\_/   \\_/   \\".to_string()
            };
            line.push("   ");
            line.push(lower);
            line.push("   ");
        }));
    } else {
        lines.push(render_monitor_row(&monitor_frame, |line| {
            let upper = if use_color {
                "__/^\\________/^\\________/^\\________/^\\__"
                    .green()
                    .bold()
                    .to_string()
            } else {
                "__/^\\________/^\\________/^\\________/^\\__".to_string()
            };
            line.push("   ");
            line.push(upper);
            line.push("   ");
        }));
        lines.push(render_monitor_row(&monitor_frame, |line| {
            line.push("  /      \\______/      \\______/      \\____/  ");
        }));
    }

    lines.push(render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_MONITOR_INDENT.len())
            .push("|")
            .push("_".repeat(HEALTH_SCENE_MONITOR_WIDTH))
            .push("|");
    }));

    let hr_line = format!(
        "HR: {}   STATUS: {}",
        heart_rate.bold(),
        status_label.color(status_color).bold()
    );
    lines.push(render_monitor_row(&monitor_frame, |line| {
        line.push(" ");
        line.push(hr_line.pad_to_width(40));
        line.push(" ");
    }));

    let pressure_label = if kinetic_load > 15 {
        "MAXIMUM".red().bold().to_string()
    } else if kinetic_load > 7 {
        "HIGH".yellow().bold().to_string()
    } else if kinetic_load > 3 {
        "MODERATE".cyan().to_string()
    } else if kinetic_load > 0 {
        "NORMAL".green().to_string()
    } else {
        "IDLE".dimmed().to_string()
    };

    let pressure_line = format!("BP: PRESSURE: {}", pressure_label);
    lines.push(render_monitor_row(&monitor_frame, |line| {
        line.push(" ");
        line.push(pressure_line.pad_to_width(40));
        line.push(" ");
    }));

    let system_state_line = format!(
        "SC: {}   OF: {}",
        if strategic_congested {
            "CONGESTED".yellow().bold().to_string()
        } else {
            "OPTIMAL".dimmed().to_string()
        },
        if operational_fatigue {
            "FATIGUE".red().bold().to_string()
        } else {
            "CALM".dimmed().to_string()
        }
    );
    lines.push(render_monitor_row(&monitor_frame, |line| {
        line.push(" ");
        line.push(system_state_line.pad_to_width(40));
        line.push(" ");
    }));
    lines.push(render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_MONITOR_INDENT.len())
            .push("'")
            .push("-".repeat(HEALTH_SCENE_MONITOR_WIDTH))
            .push("'");
    }));

    lines.push(render_health_line(|line| {
        line.pad_to(17).push("| |");
        line.pad_to(45).push("| |");
    }));
    lines.push(render_health_line(|line| {
        line.pad_to(14).push("[ LIFE SUPPORT ]");
        line.pad_to(HEALTH_SCENE_POD_GAP_COLUMN)
            .push("[ NEURAL SCAN ]");
    }));

    let o2_val = if passed {
        paint("98%".to_string(), CategoryStatus::Nominal)
    } else {
        paint("82%".to_string(), CategoryStatus::Failure)
    };
    let state_val = if passed {
        paint("SYNC".to_string(), CategoryStatus::Nominal)
    } else {
        paint("FOG".to_string(), CategoryStatus::Warning)
    };

    let o2_line = format!("  O2: {}", o2_val);
    let state_line = format!("  STATE: {}", state_val);
    lines.push(render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_POD_INDENT)
            .push(pod_frame.row(|pod| {
                pod.push(o2_line.pad_to_width(HEALTH_SCENE_POD_WIDTH));
            }));
        line.pad_to(HEALTH_SCENE_POD_GAP_COLUMN)
            .push(pod_frame.row(|pod| {
                pod.push(state_line.pad_to_width(HEALTH_SCENE_POD_WIDTH));
            }));
    }));
    lines.push(render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_POD_INDENT)
            .push(pod_frame.row(|pod| {
                pod.push("  IV: FLOW");
            }));
        line.pad_to(HEALTH_SCENE_POD_GAP_COLUMN)
            .push(pod_frame.row(|pod| {
                pod.push("  [||||||]");
            }));
    }));
    lines.push(render_health_line(|line| {
        line.pad_to(HEALTH_SCENE_POD_INDENT)
            .push("'")
            .push("-".repeat(HEALTH_SCENE_POD_WIDTH))
            .push("'");
        line.pad_to(HEALTH_SCENE_POD_GAP_COLUMN)
            .push("'")
            .push("-".repeat(HEALTH_SCENE_POD_WIDTH))
            .push("'");
    }));

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{CategoryStatus, HEALTH_SCENE_WIDTH, build_med_bay_scene, category_status};
    use chrono::Utc;
    use keel::read_model::diagnostics::types::{CheckResult, DoctorReport, Problem, Severity};
    use keel::read_model::flow_status;
    use keel::test_helpers::TestBoardBuilder;
    use std::path::PathBuf;
    use std::time::Duration;
    use txt_scene::visible_width;

    fn check_with_problems(problems: Vec<Problem>) -> CheckResult {
        CheckResult {
            id: "check".to_string(),
            name: "Check".to_string(),
            problems,
            evaluations: 1,
            duration: Duration::from_millis(0),
            passed: false,
            disabled: false,
        }
    }

    fn problem(severity: Severity) -> Problem {
        Problem {
            severity,
            path: PathBuf::from(".keel/test"),
            message: "problem".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: keel::read_model::diagnostics::types::CheckId::Unknown,
        }
    }

    #[test]
    fn category_status_is_nominal_without_problems() {
        let results = vec![check_with_problems(Vec::new())];
        assert_eq!(category_status(&results), CategoryStatus::Nominal);
    }

    #[test]
    fn category_status_is_warning_for_warning_only_checks() {
        let results = vec![check_with_problems(vec![problem(Severity::Warning)])];
        assert_eq!(category_status(&results), CategoryStatus::Warning);
    }

    #[test]
    fn category_status_is_failure_when_any_error_exists() {
        let results = vec![
            check_with_problems(vec![problem(Severity::Warning)]),
            check_with_problems(vec![problem(Severity::Error)]),
        ];
        assert_eq!(category_status(&results), CategoryStatus::Failure);
    }

    fn nominal_report() -> DoctorReport {
        let empty = vec![check_with_problems(Vec::new())];
        DoctorReport {
            story_checks: empty.clone(),
            voyage_checks: empty.clone(),
            epic_checks: empty.clone(),
            adr_checks: empty.clone(),
            bearing_checks: empty.clone(),
            mission_checks: empty.clone(),
            routine_checks: empty.clone(),
            workflow_checks: empty.clone(),
            pacemaker_checks: empty.clone(),
            delivery_checks: empty,
            drift_coefficient: 0.0,
            estimated_remediation_hours: 0.0,
            last_checked_at: std::time::SystemTime::now(),
        }
    }

    #[test]
    fn med_bay_scene_has_stable_width_with_colorized_content() {
        let temp = TestBoardBuilder::new().build();
        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let metrics = flow_status::project(&board, Utc::now());
        let report = nominal_report();

        let scene = build_med_bay_scene(&report, &metrics, false, false, true);

        assert!(scene.contains("\x1b["));
        assert!(
            scene
                .lines()
                .all(|line| visible_width(line) == HEALTH_SCENE_WIDTH)
        );
    }

    #[test]
    fn med_bay_scene_has_stable_width_without_color() {
        let temp = TestBoardBuilder::new().build();
        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let metrics = flow_status::project(&board, Utc::now());
        let report = nominal_report();

        let scene = build_med_bay_scene(&report, &metrics, false, false, false);

        assert!(
            scene
                .lines()
                .all(|line| visible_width(line) == HEALTH_SCENE_WIDTH)
        );
    }
}
