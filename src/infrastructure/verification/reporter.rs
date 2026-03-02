//! Reporting of verification results

use serde::Serialize;

/// A single verification result for reporting
#[derive(Debug, Clone, Serialize)]
pub struct VerificationResult {
    /// The acceptance criterion text
    pub criterion: String,
    /// Whether verification passed
    pub passed: bool,
    /// Expected value (for error messages)
    pub expected: String,
    /// Actual value (for error messages)
    pub actual: String,
    /// Whether this requires manual verification
    pub requires_human_review: bool,
}

/// Full report for a story verification
#[derive(Debug, Clone, Serialize)]
pub struct VerificationReport {
    pub story_id: String,
    pub results: Vec<VerificationResult>,
}

impl VerificationReport {
    pub fn passed(&self) -> bool {
        // Verification passes if all automated checks pass.
        // If there are manual checks, it's NOT fully "passed" in terms of auto-completion,
        // but it doesn't "fail" either.
        self.results
            .iter()
            .all(|r| r.passed || r.requires_human_review)
    }

    pub fn requires_human_review(&self) -> bool {
        self.results.iter().any(|r| r.requires_human_review)
    }
}

/// Summary of verification results
#[derive(Debug, Clone, Default)]
pub struct VerificationSummary {
    pub passed: usize,
    pub failed: usize,
    pub manual: usize,
}

/// Format verification results for terminal output
pub fn print_terminal_report(report: &VerificationReport) {
    use owo_colors::OwoColorize;

    for result in &report.results {
        let status = if result.requires_human_review {
            "○".blue().to_string()
        } else if result.passed {
            "✓".green().to_string()
        } else {
            "✗".red().to_string()
        };

        println!("  {} {}", status, result.criterion);

        if !result.passed && !result.requires_human_review {
            println!("    {}", format!("expected: {}", result.expected).red());
            println!("    {}", format!("actual:   {}", result.actual).red());
        }
    }

    if report.passed() && !report.requires_human_review() {
        println!("  {}", "Verification passed".green());
    }
}

/// Backward-compatible terminal formatter shim.
pub fn format_terminal(_results: &[VerificationResult]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_passed(criterion: &str) -> VerificationResult {
        VerificationResult {
            criterion: criterion.to_string(),
            passed: true,
            expected: "ok".to_string(),
            actual: "ok".to_string(),
            requires_human_review: false,
        }
    }

    #[test]
    fn report_passed_when_all_pass() {
        let report = VerificationReport {
            story_id: "S1".to_string(),
            results: vec![make_passed("AC1")],
        };
        assert!(report.passed());
    }

    #[test]
    fn report_requires_human_review() {
        let report = VerificationReport {
            story_id: "S1".to_string(),
            results: vec![VerificationResult {
                criterion: "manual".into(),
                passed: false,
                expected: "".into(),
                actual: "".into(),
                requires_human_review: true,
            }],
        };
        assert!(report.passed()); // manual review doesn't count as "failed"
        assert!(report.requires_human_review());
    }

    #[test]
    fn test_print_terminal_report() {
        let report = VerificationReport {
            story_id: "S1".to_string(),
            results: vec![
                make_passed("AC1"),
                VerificationResult {
                    criterion: "failed".into(),
                    passed: false,
                    expected: "exp".into(),
                    actual: "act".into(),
                    requires_human_review: false,
                },
            ],
        };
        print_terminal_report(&report);
    }
}
