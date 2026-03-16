//! Health command - subsystem status check and bio-scan (The Med-Bay)

use anyhow::Result;
use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::style::VisualPadding;
use keel::infrastructure::loader::load_board;
use keel::read_model::{diagnostics, flow_status};
use owo_colors::OwoColorize;

/// Run the health command
pub fn run(board_dir: &std::path::Path, scene: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let report = diagnostics::validate_report(board_dir)?;
    let metrics = flow_status::project(&board, chrono::Utc::now());
    let passed = report.passed();
    
    // 1. Kinetic Load (Active work)
    let _kinetic_load = metrics.verification.count + metrics.execution.in_progress_count;
    
    // 2. Strategic Congestion (Top-heavy backlog)
    // Congested if many draft epics (> 5) or many incomplete missions (> 3)
    let draft_epics = board.epics.values().filter(|e| e.status() == keel::domain::model::EpicState::Draft).count();
    let strategic_congested = draft_epics > 5 || metrics.incomplete_missions_count > 3;
    
    // 3. Operational Fatigue (Operational noise)
    // Fatigued if many due routines (> 3) or too much unlinked governance
    let operational_fatigue = metrics.due_routines_count > 3 || metrics.governance.proposed_count > 5;

    let width = get_terminal_width();
    
    if scene {
        render_med_bay_scene(&report, &metrics, strategic_congested, operational_fatigue, width);
    } else {
        println!("\n    {} SUBSYSTEM STATUS REPORT", " HEALTH ".on_green().black().bold());
        
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
        
        if passed && !strategic_congested && !operational_fatigue {
            println!("\n    {} System is 100% healthy.", "✓".green().bold());
        } else if passed {
            println!("\n    ! System is nominal but under pressure.");
        } else {
            println!("\n    {} System has issues. Run `keel doctor` for details.", "✗".red().bold());
        }
    }

    if passed {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn print_category(name: &str, results: &[keel::read_model::diagnostics::types::CheckResult]) {
    let passed = results.iter().all(|c| c.passed);
    let marker = if passed { "✓".green().to_string() } else { "✗".red().to_string() };
    println!("    • {: <10} {}", name, marker);
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
    let _kinetic_load = metrics.verification.count + metrics.execution.in_progress_count;

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

    let mut scene = String::new();
    scene.push_str("             .__________________________________________.\n");
    scene.push_str("             | [ MAIN DIAGNOSTIC MONITOR - HIGH RES ]   |\n");
    scene.push_str("             |                                          |\n");

    // High Res Heartbeat pulse (EKG)
    if !passed || kinetic_load > 15 {
        scene.push_str(&format!(
            "             |   {}{}   |\n",
            "/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\".red().bold(),
            "/\\".red().bold()
        ));
        scene.push_str(&format!(
            "             |   {}   |\n",
            "!! ALARM !! ALARM !! ALARM !! ALARM !!".red().bold()
        ));
    } else if kinetic_load > 7 || strategic_congested {
        scene.push_str(&format!(
            "             |   {}   |\n",
            "_/^\\___/^\\___/^\\___/^\\___/^\\___/^\\___/^\\_".yellow().bold()
        ));
        scene.push_str(&format!(
            "             |   {}   |\n",
            "\\_/   \\_/   \\_/   \\_/   \\_/   \\_/   \\_/   \\".yellow().bold()
        ));
    } else {
        scene.push_str(&format!(
            "             |   {}   |\n",
            "__/\x1b[1;32m^\x1b[0m\\________/\x1b[1;32m^\x1b[0m\\________/\x1b[1;32m^\x1b[0m\\________/\x1b[1;32m^\x1b[0m\\__".green()
        ));
        scene.push_str(&format!(
            "             |  /      \\______/      \\______/      \\____/  |\n"
        ));
    }

    scene.push_str("             |__________________________________________|\n");
    
    let hr_line = format!(" HR: {}   STATUS: {}", heart_rate.bold(), status_label.color(status_color).bold());
    scene.push_str(&format!("             | {} |\n", hr_line.pad_to_width(40)));

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

    let pressure_line = format!(" BP: PRESSURE: {}", pressure_label);
    scene.push_str(&format!("             | {} |\n", pressure_line.pad_to_width(40)));
    
    let state_line = format!(
        " SC: {}   OF: {}", 
        if strategic_congested { "CONGESTED".yellow().bold().to_string() } else { "OPTIMAL".dimmed().to_string() },
        if operational_fatigue { "FATIGUE".red().bold().to_string() } else { "CALM".dimmed().to_string() }
    );
    scene.push_str(&format!("             | {} |\n", state_line.pad_to_width(40)));
    scene.push_str("             '------------------------------------------'\n");

    // Add secondary medical visuals (Life Support / IV)
    scene.push_str("                 | |                            | |\n");
    scene.push_str("              [ LIFE SUPPORT ]               [ NEURAL SCAN ]\n");
    
    let o2_val = if passed { "98% ".green().to_string() } else { "82% ".red().bold().to_string() };
    let state_val = if passed { "SYNC".green().to_string() } else { "FOG ".yellow().to_string() };
    
    let o2_line = format!("|  O2: {} |", o2_val);
    let state_line = format!("|  STATE: {} |", state_val);
    
    scene.push_str(&format!(
        "              {}               {} \n",
        o2_line.pad_to_width(12),
        state_line.pad_to_width(12)
    ));
    scene.push_str("              |  IV: FLOW  |               |  [||||||]   |\n");
    scene.push_str("              '------------'               '------------'\n");

    println!("{}", scene);

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
        let cat_passed = results.iter().all(|c| c.passed);
        let color = if cat_passed { owo_colors::AnsiColors::Green } else { owo_colors::AnsiColors::Red };
        let status = if cat_passed { "NOMINAL" } else { "FAILURE" };
        println!("      - {: <12} [ {} ]", name, status.color(color).bold());
    }

    if passed {
        println!("\n    \"Patient is fit for duty. The garden is in good hands.\"");
    } else {
        println!("\n    \"EMERGENCY: Subsystem failure detected. Scrub in with `keel doctor`.\"");
    }
}
