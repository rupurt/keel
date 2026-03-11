//! Doctor command - board health diagnostics and automated fixing

pub mod render;

use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use keel::read_model::diagnostics::fixes::run_fixes;
use keel::read_model::diagnostics::validate;

/// Run the doctor command
pub fn run(board_dir: &Path, fix: bool, _evidence: bool, _watch: bool, _quick: bool) -> Result<()> {
    let _start = Instant::now();
    let report = validate(board_dir)?;

    render::print_report(&report);

    if fix {
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
