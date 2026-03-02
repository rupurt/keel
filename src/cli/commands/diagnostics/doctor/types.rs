//! Types for board health diagnostics

use serde::Serialize;
use std::time::Duration;

pub use crate::infrastructure::validation::{CheckId, Fix, GapCategory, Problem, Severity};

/// Full board health report
#[derive(Debug, Serialize, Clone)]
pub struct DoctorReport {
    pub story_checks: Vec<CheckResult>,
    pub voyage_checks: Vec<CheckResult>,
    pub epic_checks: Vec<CheckResult>,
    pub adr_checks: Vec<CheckResult>,
    pub bearing_checks: Vec<CheckResult>,
}

impl DoctorReport {
    pub fn passed(&self) -> bool {
        self.story_checks.iter().all(|c| c.passed)
            && self.voyage_checks.iter().all(|c| c.passed)
            && self.epic_checks.iter().all(|c| c.passed)
    }

    pub fn total_errors(&self) -> usize {
        self.story_checks
            .iter()
            .chain(&self.voyage_checks)
            .chain(&self.epic_checks)
            .chain(&self.adr_checks)
            .chain(&self.bearing_checks)
            .flat_map(|c| &c.problems)
            .filter(|p| p.severity == Severity::Error)
            .count()
    }

    pub fn total_warnings(&self) -> usize {
        self.story_checks
            .iter()
            .chain(&self.voyage_checks)
            .chain(&self.epic_checks)
            .chain(&self.adr_checks)
            .chain(&self.bearing_checks)
            .flat_map(|c| &c.problems)
            .filter(|p| p.severity == Severity::Warning)
            .count()
    }
}

/// Result of a single health check category
#[derive(Debug, Serialize, Clone)]
pub struct CheckResult {
    pub name: &'static str,
    pub problems: Vec<Problem>,
    pub evaluations: usize,
    pub duration: Duration,
    pub passed: bool,
}

/// Summary result for a section of checks
pub struct SectionResult {
    pub total_problems: usize,
    pub errors: usize,
    pub warnings: usize,
    pub checks: Vec<CheckResult>,
}
