//! Health command - subsystem status check and bio-scan (The Med-Bay)

use anyhow::Result;
use crate::cli::presentation::terminal::get_terminal_width;
use keel::infrastructure::loader::load_board;
use keel::read_model::{diagnostics, flow_status, queue_policy};
use owo_colors::OwoColorize;

/// Run the health command
pub fn run(board_dir: &std::path::Path, scene: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let report = diagnostics::validate_report(board_dir)?;
    let metrics = flow_status::project(&board, chrono::Utc::now());
    let queue_snapshot = queue_policy::project(&metrics);
    let passed = report.passed();
    
    let width = get_terminal_width();
    
    if scene {
        render_med_bay_scene(&report, &queue_snapshot, width);
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

fn render_med_bay_scene(report: &keel::read_model::diagnostics::types::DoctorReport, queue: &keel::read_model::queue_policy::QueuePolicySnapshot, width: usize) {
    println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
    println!("    │ {: <width$} │", "THE MED-BAY (BIO-SCAN)".bold(), width = width.saturating_sub(8));
    println!("    └{}┘", "─".repeat(width.saturating_sub(6)));

    let passed = report.passed();
    let pressure = queue.verification;
    
    let (heart_rate, status_label, status_color) = if !passed {
        ("140 BPM", "CRITICAL", owo_colors::AnsiColors::Red)
    } else {
        match pressure {
            keel::domain::policy::queue::VerificationQueueCategory::FlowBlocked => ("120 BPM", "OVERLOAD", owo_colors::AnsiColors::Red),
            keel::domain::policy::queue::VerificationQueueCategory::HumanBlocked => ("100 BPM", "ELEVATED", owo_colors::AnsiColors::Yellow),
            keel::domain::policy::queue::VerificationQueueCategory::Attention => ("85 BPM", "STRESSED", owo_colors::AnsiColors::Cyan),
            _ => ("72 BPM", "STABLE", owo_colors::AnsiColors::Green),
        }
    };

    let mut scene = String::new();
    scene.push_str("             ___________________________\n");
    scene.push_str("            | [ DIAGNOSTIC MONITOR ]    |\n");
    scene.push_str("            |                           |\n");
    
    // Heartbeat pulse (EKG) - speed based on heart rate
    if !passed || pressure.blocks_flow() {
        scene.push_str("            |   /\\^/\\/\\^/\\/\\^/\\/\\^/\\/\\^/\\/  |\n");
        scene.push_str("            | !! ALARM !! ALARM !! ALARM !! |\n");
    } else if pressure.blocks_human_next() {
        scene.push_str("            |  _/\\^/\\_  _/\\^/\\_  _/\\^/\\_    |\n");
        scene.push_str("            | /      \\/      \\/      \\_____ |\n");
    } else {
        scene.push_str("            |   __/\\^/\\_      __/\\^/\\_      |\n");
        scene.push_str("            | _/      \\____/      \\____ |\n");
    }
    
    scene.push_str("            |___________________________|\n");
    scene.push_str(&format!("            | HR: {: <8} STATUS: {: <8} |\n", heart_rate.bold(), status_label.color(status_color).bold()));
    
    let pressure_label = match pressure {
        keel::domain::policy::queue::VerificationQueueCategory::Empty => "IDLE".dimmed().to_string(),
        keel::domain::policy::queue::VerificationQueueCategory::Attention => "MODERATE".cyan().to_string(),
        keel::domain::policy::queue::VerificationQueueCategory::HumanBlocked => "HIGH".yellow().bold().to_string(),
        keel::domain::policy::queue::VerificationQueueCategory::FlowBlocked => "MAXIMUM".red().bold().to_string(),
    };
    
    let pressure_plain = match pressure {
        keel::domain::policy::queue::VerificationQueueCategory::Empty => "IDLE",
        keel::domain::policy::queue::VerificationQueueCategory::Attention => "MODERATE",
        keel::domain::policy::queue::VerificationQueueCategory::HumanBlocked => "HIGH",
        keel::domain::policy::queue::VerificationQueueCategory::FlowBlocked => "MAXIMUM",
    };
    
    let bp_total_width: usize = 16;
    let bp_padding = bp_total_width.saturating_sub(pressure_plain.len());
    
    scene.push_str(&format!("            | BP: PRESSURE: {}{} |\n", pressure_label, " ".repeat(bp_padding)));
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
