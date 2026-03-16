//! Terminal rendering for doctor reports

use keel::read_model::diagnostics::types::{CheckResult, DoctorReport, Severity};
use owo_colors::OwoColorize;
use std::time::Duration;

/// Print the full doctor report to the terminal
pub fn print_report(report: &DoctorReport) {
    println!("{}", "Doctor's Laboratory Report".bold().underline());
    println!();

    render_pathology_summary(report);
    println!();

    println!("{}", "[ ITEMIZED PATHOLOGY ]".bold().dimmed());
    println!();

    print_section("Vital Subsystem (Missions)", &report.mission_checks);
    print_section("Pacemaker (Heartbeat)", &report.pacemaker_checks);
    print_section("Strategic Subsystem (Epics)", &report.epic_checks);
    print_section("Sensory Subsystem (Bearings)", &report.bearing_checks);
    print_section("Skeletal Subsystem (ADRs)", &report.adr_checks);
    print_section("Motor Subsystem (Voyages)", &report.voyage_checks);
    print_section("Neural Subsystem (Stories)", &report.story_checks);
    print_section("Autonomic Subsystem (Routines)", &report.routine_checks);
    print_section("Circulatory Subsystem (Workflow)", &report.workflow_checks);
    print_section("Kinetic Subsystem (Delivery)", &report.delivery_checks);
}

fn render_pathology_summary(report: &DoctorReport) {
    println!("{}", "[ SYSTEMS PATHOLOGY ]".bold().cyan());
    
    // Define the higher-level subsystem groups
    let core_passed = report.mission_checks.iter().all(|c| !c.has_errors()) && report.pacemaker_checks.iter().all(|c| !c.has_errors());
    let strategy_passed = report.epic_checks.iter().all(|c| !c.has_errors()) && report.bearing_checks.iter().all(|c| !c.has_errors()) && report.adr_checks.iter().all(|c| !c.has_errors());
    let execution_passed = report.voyage_checks.iter().all(|c| !c.has_errors()) && report.story_checks.iter().all(|c| !c.has_errors()) && report.routine_checks.iter().all(|c| !c.has_errors());
    let flow_passed = report.workflow_checks.iter().all(|c| !c.has_errors()) && report.delivery_checks.iter().all(|c| !c.has_errors());

    let get_status = |passed: bool| if passed { "NOMINAL".green().bold().to_string() } else { "CRITICAL".red().bold().to_string() };

    println!("  ┌─ CORE      [{}] (Vital, Pacemaker)", get_status(core_passed));
    println!("  │");
    println!("  ├─ STRATEGY  [{}] (Strategic, Sensory, Skeletal)", get_status(strategy_passed));
    println!("  │");
    println!("  ├─ EXECUTION [{}] (Motor, Neural, Autonomic)", get_status(execution_passed));
    println!("  │");
    println!("  └─ FLOW      [{}] (Circulatory, Kinetic)", get_status(flow_passed));
}

fn print_section(name: &str, checks: &[CheckResult]) {
    println!("{}:", name.bold().cyan());
    for check in checks {
        if check.disabled {
            println!(
                "  {} {} ({} checks) {}",
                "⊝".bright_black(),
                check.name,
                check.evaluations,
                "[disabled]".bright_black()
            );
            continue;
        }

        let status = if check.has_errors() {
            "✗".red().to_string()
        } else if check.has_warnings() {
            "!".yellow().to_string()
        } else {
            "✓".green().to_string()
        };
        
        println!("  {} {} ({} checks)", status, check.name, check.evaluations);
        for prob in &check.problems {
            let sev = match prob.severity {
                Severity::Error => "Error:".red().to_string(),
                Severity::Warning => "Warn: ".yellow().to_string(),
                Severity::Info => "Info: ".blue().to_string(),
            };
            println!("    {} {}", sev, prob.message);
            println!("      Path: {}", prob.path.display().dimmed());
        }
    }
    println!();
}

/// Calculate total duration of checks
#[allow(dead_code)]
pub fn sum_check_durations(checks: &[CheckResult]) -> Duration {
    checks.iter().map(|c| c.duration).sum()
}
