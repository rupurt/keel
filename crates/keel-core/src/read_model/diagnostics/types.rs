//! Types for board health diagnostics

use std::time::Duration;

pub use crate::infrastructure::validation::{CheckId, Fix, GapCategory, Problem, Severity};

/// Full board health report
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DoctorReport {
    pub story_checks: Vec<CheckResult>,
    pub voyage_checks: Vec<CheckResult>,
    pub epic_checks: Vec<CheckResult>,
    pub adr_checks: Vec<CheckResult>,
    pub bearing_checks: Vec<CheckResult>,
    pub mission_checks: Vec<CheckResult>,
    pub routine_checks: Vec<CheckResult>,
    pub workflow_checks: Vec<CheckResult>,
    pub pacemaker_checks: Vec<CheckResult>,
    pub delivery_checks: Vec<CheckResult>,
    /// Simulation drift coefficient (0.0 to 1.0)
    pub drift_coefficient: f64,
    /// Estimated hours to reach zero drift
    pub estimated_remediation_hours: f64,
    /// When this report was generated
    pub last_checked_at: std::time::SystemTime,
}

impl DoctorReport {
    pub fn passed(&self) -> bool {
        !self.story_checks.iter().any(|c| c.has_errors())
            && !self.voyage_checks.iter().any(|c| c.has_errors())
            && !self.epic_checks.iter().any(|c| c.has_errors())
            && !self.adr_checks.iter().any(|c| c.has_errors())
            && !self.bearing_checks.iter().any(|c| c.has_errors())
            && !self.mission_checks.iter().any(|c| c.has_errors())
            && !self.routine_checks.iter().any(|c| c.has_errors())
            && !self.workflow_checks.iter().any(|c| c.has_errors())
            && !self.pacemaker_checks.iter().any(|c| c.has_errors())
            && !self.delivery_checks.iter().any(|c| c.has_errors())
    }

    pub fn total_errors(&self) -> usize {
        self.all_problems()
            .iter()
            .filter(|p| p.severity == Severity::Error)
            .count()
    }

    pub fn total_warnings(&self) -> usize {
        self.all_problems()
            .iter()
            .filter(|p| p.severity == Severity::Warning)
            .count()
    }

    pub fn all_problems(&self) -> Vec<&Problem> {
        self.story_checks
            .iter()
            .chain(&self.voyage_checks)
            .chain(&self.epic_checks)
            .chain(&self.adr_checks)
            .chain(&self.bearing_checks)
            .chain(&self.mission_checks)
            .chain(&self.routine_checks)
            .chain(&self.workflow_checks)
            .chain(&self.pacemaker_checks)
            .chain(&self.delivery_checks)
            .flat_map(|c| &c.problems)
            .collect()
    }

    /// Calculate drift based on problem volume and severity.
    pub fn calculate_drift(&mut self) {
        let errors = self.total_errors();
        let warnings = self.total_warnings();

        // Heuristic: Each error adds 0.05, each warning adds 0.01
        // Capped at 1.0 (Full Chaos)
        self.drift_coefficient = ((errors as f64 * 0.05) + (warnings as f64 * 0.01)).min(1.0);

        // Heuristic: Each error takes 1 hour, each warning takes 0.2 hours
        self.estimated_remediation_hours = (errors as f64 * 1.0) + (warnings as f64 * 0.2);
    }
}

/// Result of a single health check category
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CheckResult {
    pub id: String,
    pub name: String,
    pub problems: Vec<Problem>,
    pub evaluations: usize,
    pub duration: Duration,
    pub passed: bool,
    pub disabled: bool,
}

impl CheckResult {
    pub fn has_errors(&self) -> bool {
        self.problems.iter().any(|p| p.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.problems.iter().any(|p| p.severity == Severity::Warning)
    }
}

/// Summary result for a section of checks
pub struct SectionResult {
    pub total_problems: usize,
    pub errors: usize,
    pub warnings: usize,
    pub checks: Vec<CheckResult>,
}
