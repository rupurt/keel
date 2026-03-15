//! Doctor command - board health diagnostics and automated fixing

pub mod render;

use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use keel::read_model::diagnostics::fixes::run_fixes;
use keel::read_model::diagnostics::validate_report as validate;

/// Run the doctor command
pub fn run(board_dir: &Path, fix: bool, _evidence: bool, _watch: bool, _quick: bool, scene: bool) -> Result<()> {
    let _start = Instant::now();
    let report = validate(board_dir)?;

    let errors = report.total_errors();
    let warnings = report.total_warnings();

    if scene {
        use owo_colors::OwoColorize;
        let passed = errors == 0 && warnings == 0;
        if passed {
            let ekg = r#"
    /\         /\         /\    
 __/  \  _  __/  \  _  __/  \  _
       \/         \/         \/ 
"#;
            println!("{}", ekg.green());
        } else {
            let ekg = r#"
                                
 _______________________________
                                
"#;
            println!("{}", ekg.red());
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
    let warnings = report.total_warnings();

    if errors > 0 {
        anyhow::bail!("Board has {} errors", errors);
    }

    if warnings > 0 {
        // We use a special error message that main.rs can recognize if we want specific exit codes
        anyhow::bail!("Board has {} warnings", warnings);
    }

    Ok(())
}
