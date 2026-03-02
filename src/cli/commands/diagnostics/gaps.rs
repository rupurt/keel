//! Gaps command - identify coverage gaps and missing implementation proof

use anyhow::Result;
use std::path::Path;

use super::doctor;

/// Run the gaps command
pub fn run(board_dir: &Path) -> Result<()> {
    let report = doctor::validate(board_dir)?;
    doctor::print_gap_summary(&report);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn test_gaps_run() {
        let temp = TestBoardBuilder::new().build();
        let result = run(temp.path());
        assert!(result.is_ok());
    }
}
