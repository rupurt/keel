//! Terminal rendering for doctor reports

use super::types::{DoctorReport, Severity};
use owo_colors::OwoColorize;
use std::time::Duration;

/// Print the full doctor report to the terminal
pub fn print_report(report: &DoctorReport) {
    println!("{}", "Board Health Report".bold().underline());
    println!();

    print_section("Stories", &report.story_checks);
    print_section("Voyages", &report.voyage_checks);
    print_section("Epics", &report.epic_checks);
    print_section("Bearings", &report.bearing_checks);
    print_section("ADRs", &report.adr_checks);
}

fn print_section(name: &str, checks: &[super::types::CheckResult]) {
    println!("{}:", name.bold().cyan());
    for check in checks {
        let status = if check.passed {
            "✓".green().to_string()
        } else {
            "✗".red().to_string()
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

/// Print a summary of coverage gaps
pub fn print_gap_summary(report: &DoctorReport) {
    let mut total_errors = 0;
    let mut total_warnings = 0;

    for check in report
        .story_checks
        .iter()
        .chain(&report.voyage_checks)
        .chain(&report.epic_checks)
        .chain(&report.adr_checks)
        .chain(&report.bearing_checks)
    {
        for prob in &check.problems {
            match prob.severity {
                Severity::Error => total_errors += 1,
                Severity::Warning => total_warnings += 1,
                _ => {}
            }
        }
    }

    println!("Board Health Summary:");
    println!("  Errors:   {}", total_errors.red());
    println!("  Warnings: {}", total_warnings.yellow());
}

/// Calculate total duration of checks
pub fn sum_check_durations(checks: &[super::types::CheckResult]) -> Duration {
    checks.iter().map(|c| c.duration).sum()
}
