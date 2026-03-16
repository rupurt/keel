//! Health command - subsystem status check and bio-scan (The Med-Bay)

use anyhow::Result;
use crate::cli::presentation::terminal::get_terminal_width;
use keel::infrastructure::loader::load_board;
use keel::read_model::diagnostics;
use owo_colors::OwoColorize;

/// Run the health command
pub fn run(board_dir: &std::path::Path, scene: bool) -> Result<()> {
    let _board = load_board(board_dir)?;
    let report = diagnostics::validate_report(board_dir)?;
    let passed = report.passed();
    
    let width = get_terminal_width();
    
    if scene {
        render_med_bay_scene(&report, width);
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
        
        if passed {
            println!("\n    {} System is 100% healthy.", "✓".green().bold());
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

fn render_med_bay_scene(report: &keel::read_model::diagnostics::types::DoctorReport, width: usize) {
    println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
    println!("    │ {: <width$} │", "THE MED-BAY (BIO-SCAN)".bold(), width = width.saturating_sub(8));
    println!("    └{}┘", "─".repeat(width.saturating_sub(6)));

    let passed = report.passed();
    let heart_rate = if passed { "72 BPM" } else { "140 BPM" };
    let status_label = if passed { "STABLE" } else { "CRITICAL" };
    let status_color = if passed { owo_colors::AnsiColors::Green } else { owo_colors::AnsiColors::Red };

    let mut scene = String::new();
    scene.push_str("             ___________________________\n");
    scene.push_str("            | [ DIAGNOSTIC MONITOR ]    |\n");
    scene.push_str("            |                           |\n");
    
    // Heartbeat pulse (EKG)
    if passed {
        scene.push_str("            |   __/\\^/\\_      __/\\^/\\_      |\n");
        scene.push_str("            | _/      \\____/      \\____ |\n");
    } else {
        scene.push_str("            |   /\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/  |\n");
        scene.push_str("            | !! ALARM !! ALARM !! ALARM !! |\n");
    }
    
    scene.push_str("            |___________________________|\n");
    scene.push_str(&format!("            | HR: {: <8} STATUS: {: <8} |\n", heart_rate.bold(), status_label.color(status_color).bold()));
    scene.push_str("            '---------------------------'\n");

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
