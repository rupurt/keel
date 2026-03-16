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
) -> Result<()> {
    let _start = Instant::now();
    let report = validate(board_dir)?;

    if scene {
        use owo_colors::OwoColorize;
        let passed = report.passed();
        
        let core_passed = report.mission_checks.iter().all(|c| !c.has_errors()) && report.pacemaker_checks.iter().all(|c| !c.has_errors());
        let strategy_passed = report.epic_checks.iter().all(|c| !c.has_errors()) && report.bearing_checks.iter().all(|c| !c.has_errors()) && report.adr_checks.iter().all(|c| !c.has_errors());
        let execution_passed = report.voyage_checks.iter().all(|c| !c.has_errors()) && report.story_checks.iter().all(|c| !c.has_errors()) && report.routine_checks.iter().all(|c| !c.has_errors());
        let flow_passed = report.workflow_checks.iter().all(|c| !c.has_errors()) && report.delivery_checks.iter().all(|c| !c.has_errors());

        println!("\n    ┌───────────────────────────[ THE LABORATORY ]───────────────────────────┐");
        
        let mut visual = String::new();
        visual.push_str("    │                                                                        │\n");
        visual.push_str("    │  [ VITAL SIGNS MONITOR ]                                               │\n");
        visual.push_str("    │  .-----------------------.                                             │\n");
        
        if passed {
            let sys_label = format!("SYS: {}", "NOMINAL".green().bold());
            let hr_label = format!("HR:  {}", "72 BPM".green().bold());
            
            let line1_content = format!("  |   {}/\\{}__{}/\\{}__{}/\\{} |  {}", "_".green(), "^".green(), "_".green(), "^".green(), "_".green(), "^".green(), sys_label);
            let line2_content = format!("  | {}/    \\{}/    \\{}/    \\ |  {}", "_".green(), "_".green(), "_".green(), hr_label);
            
            visual.push_str(&format!("    │ {} │\n", line1_content.pad_to_width(68)));
            visual.push_str(&format!("    │ {} │\n", line2_content.pad_to_width(68)));
        } else {
            let sys_label = format!("SYS: {}", "CRITICAL".red().bold());
            let hr_label = format!("HR:  {}", "--- BPM".red().bold());
            
            let line1_content = format!("  | {} |  {}", "_______________________".red(), sys_label);
            let line2_content = format!("  |                       |  {}", hr_label);
            
            visual.push_str(&format!("    │ {} │\n", line1_content.pad_to_width(68)));
            visual.push_str(&format!("    │ {} │\n", line2_content.pad_to_width(68)));
        }
        
        visual.push_str("    │  '-----------------------'                                             │\n");
        visual.push_str("    │                                                                        │\n");
        
        let status = |p: bool| if p { "NOM".green().bold().to_string() } else { "ERR".red().bold().to_string() };
        visual.push_str(&format!("    │    CORE: [{}]  STRAT: [{}]  EXEC: [{}]  FLOW: [{}]    │\n", 
            status(core_passed), status(strategy_passed), status(execution_passed), status(flow_passed)));
        
        visual.push_str("    │                                                                        │\n");
        visual.push_str("    └────────────────────────────────────────────────────────────────────────┘\n");
        
        println!("{}", visual);

        if passed {
            println!("    \"Patient is responsive. Subsystems are operating within normal parameters.\"");
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
