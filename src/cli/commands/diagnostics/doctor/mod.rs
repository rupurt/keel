//! Doctor command - board health diagnostics and automated fixing

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod checks;
pub mod fixes;
pub mod render;
pub mod types;

use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use crate::infrastructure::loader::load_board;
pub use types::{CheckResult, DoctorReport};

// Legacy constants for compatibility with existing check modules
pub static CRITERIA_RE: &LazyLock<Regex> = &crate::cli::style::AC_REQ_RE;
pub static AC_REQ_RE: &LazyLock<Regex> = &crate::cli::style::AC_REQ_RE;
pub static EVIDENCE_PHASE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bSRS-[A-Z0-9-]+:([a-z:]+)\b").unwrap());

/// Run the doctor command
pub fn run(board_dir: &Path, fix: bool, _evidence: bool, _watch: bool, _quick: bool) -> Result<()> {
    let _start = Instant::now();
    let report = validate(board_dir)?;

    render::print_report(&report);

    if fix {
        fixes::run_fixes(board_dir, &report)?;
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

/// Run all health checks and return a full report
pub fn validate(board_dir: &Path) -> Result<DoctorReport> {
    let board = load_board(board_dir)?;

    let mut story_checks = Vec::new();
    let mut voyage_checks = Vec::new();
    let mut epic_checks = Vec::new();
    let mut adr_checks = Vec::new();
    let mut bearing_checks = Vec::new();

    // 1. Story Checks
    let (story_file_problems, story_count) = checks::stories::scan_story_files(board_dir)?;
    story_checks.push(CheckResult {
        name: "ID uniqueness",
        evaluations: story_count,
        passed: story_file_problems.is_empty(),
        problems: story_file_problems,
        duration: Duration::from_millis(0),
    });

    let consistency_problems = checks::stories::check_filename_frontmatter_consistency(&board);
    story_checks.push(CheckResult {
        name: "ID consistency",
        evaluations: board.stories.len(),
        passed: consistency_problems.is_empty(),
        problems: consistency_problems,
        duration: Duration::from_millis(0),
    });

    let story_title_problems = checks::stories::check_story_title_case(&board);
    story_checks.push(CheckResult {
        name: "Title convention",
        evaluations: board.stories.len(),
        passed: story_title_problems.is_empty(),
        problems: story_title_problems,
        duration: Duration::from_millis(0),
    });

    let ac_problems = checks::stories::check_acceptance_criteria_complete(&board);
    story_checks.push(CheckResult {
        name: "Acceptance criteria completion",
        evaluations: board.stories.len(),
        passed: ac_problems.is_empty(),
        problems: ac_problems,
        duration: Duration::from_millis(0),
    });

    let verify_problems = checks::stories::check_verification_annotations(&board);
    story_checks.push(CheckResult {
        name: "Verification annotations",
        evaluations: board.stories.len(),
        passed: verify_problems.is_empty(),
        problems: verify_problems,
        duration: Duration::from_millis(0),
    });

    let traceability_problems = checks::stories::check_srs_traceability(&board);
    story_checks.push(CheckResult {
        name: "SRS traceability",
        evaluations: board.stories.len(),
        passed: traceability_problems.is_empty(),
        problems: traceability_problems,
        duration: Duration::from_millis(0),
    });

    let dependency_cycle_problems = checks::stories::check_story_dependency_cycles(&board);
    story_checks.push(CheckResult {
        name: "Implementation dependency cycles",
        evaluations: board.stories.len(),
        passed: dependency_cycle_problems.is_empty(),
        problems: dependency_cycle_problems,
        duration: Duration::from_millis(0),
    });

    let parallel_conflict_problems = checks::stories::check_parallel_conflict_coherence(&board);
    story_checks.push(CheckResult {
        name: "Parallel conflict coherence",
        evaluations: board.stories.len(),
        passed: parallel_conflict_problems.is_empty(),
        problems: parallel_conflict_problems,
        duration: Duration::from_millis(0),
    });

    let scoped_evidence_problems = checks::stories::check_scoped_story_evidence(&board);
    story_checks.push(CheckResult {
        name: "Scoped evidence coverage",
        evaluations: board.stories.len(),
        passed: scoped_evidence_problems.is_empty(),
        problems: scoped_evidence_problems,
        duration: Duration::from_millis(0),
    });

    let story_drift_problems = checks::stories::check_deprecated_frontmatter_fields(board_dir);
    story_checks.push(CheckResult {
        name: "Story frontmatter drift",
        evaluations: story_count,
        passed: story_drift_problems.is_empty(),
        problems: story_drift_problems,
        duration: Duration::from_millis(0),
    });

    let reflection_problems = checks::stories::check_reflection_coherence(&board);
    story_checks.push(CheckResult {
        name: "Reflection coherence",
        evaluations: board.stories.len(),
        passed: reflection_problems.is_empty(),
        problems: reflection_problems,
        duration: Duration::from_millis(0),
    });

    let active_coherence_problems = checks::stories::check_active_story_coherence(&board);
    story_checks.push(CheckResult {
        name: "Active story coherence",
        evaluations: board.stories.len(),
        passed: active_coherence_problems.is_empty(),
        problems: active_coherence_problems,
        duration: Duration::from_millis(0),
    });

    let terminal_coherence_problems = checks::stories::check_terminal_story_coherence(&board);
    story_checks.push(CheckResult {
        name: "Terminal artifact coherence",
        evaluations: board.stories.len(),
        passed: terminal_coherence_problems.is_empty(),
        problems: terminal_coherence_problems,
        duration: Duration::from_millis(0),
    });

    let knowledge_manifest_problems =
        checks::stories::check_knowledge_manifest_integrity(board_dir);
    story_checks.push(CheckResult {
        name: "Knowledge catalog integrity",
        evaluations: board.stories.len(),
        passed: knowledge_manifest_problems.is_empty(),
        problems: knowledge_manifest_problems,
        duration: Duration::from_millis(0),
    });

    let manifest_problems = checks::stories::check_verification_manifests(&board);
    story_checks.push(CheckResult {
        name: "Verification manifest integrity",
        evaluations: board.stories.len(),
        passed: manifest_problems.is_empty(),
        problems: manifest_problems,
        duration: Duration::from_millis(0),
    });

    let story_date_problems = checks::stories::check_story_dates(&board);
    story_checks.push(CheckResult {
        name: "Story date consistency",
        evaluations: board.stories.len(),
        passed: story_date_problems.is_empty(),
        problems: story_date_problems,
        duration: Duration::from_millis(0),
    });

    // 2. Voyage Checks
    let (voyage_file_problems, voyage_count) = checks::voyages::scan_voyage_files(board_dir)?;
    voyage_checks.push(CheckResult {
        name: "Voyage structure",
        evaluations: voyage_count,
        passed: voyage_file_problems.is_empty(),
        problems: voyage_file_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_duplicate_problems = checks::voyages::check_voyage_duplicates(board_dir);
    voyage_checks.push(CheckResult {
        name: "ID uniqueness",
        evaluations: voyage_count,
        passed: voyage_duplicate_problems.is_empty(),
        problems: voyage_duplicate_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_id_problems = checks::voyages::check_voyage_id_consistency(&board);
    voyage_checks.push(CheckResult {
        name: "ID consistency",
        evaluations: board.voyages.len(),
        passed: voyage_id_problems.is_empty(),
        problems: voyage_id_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_title_problems = checks::voyages::check_voyage_title_case(&board);
    voyage_checks.push(CheckResult {
        name: "Title convention",
        evaluations: board.voyages.len(),
        passed: voyage_title_problems.is_empty(),
        problems: voyage_title_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_drift_problems = checks::voyages::check_voyage_status_drift(&board);
    voyage_checks.push(CheckResult {
        name: "Voyage status drift",
        evaluations: board.voyages.len(),
        passed: voyage_drift_problems.is_empty(),
        problems: voyage_drift_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_prd_lineage_problems = checks::voyages::check_prd_lineage_coherence(&board);
    voyage_checks.push(CheckResult {
        name: "PRD lineage coherence",
        evaluations: board.voyages.len(),
        passed: voyage_prd_lineage_problems.is_empty(),
        problems: voyage_prd_lineage_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_artifact_problems = checks::voyages::check_voyage_press_release_artifacts(&board);
    voyage_checks.push(CheckResult {
        name: "Voyage artifact contract",
        evaluations: board.voyages.len(),
        passed: voyage_artifact_problems.is_empty(),
        problems: voyage_artifact_problems,
        duration: Duration::from_millis(0),
    });

    let evidence_problems = checks::evidence::check_evidence_chains(&board);
    voyage_checks.push(CheckResult {
        name: "Evidence chains",
        evaluations: board.voyages.len(),
        passed: evidence_problems.is_empty(),
        problems: evidence_problems,
        duration: Duration::from_millis(0),
    });

    let voyage_date_problems = checks::voyages::check_voyage_dates(&board);
    voyage_checks.push(CheckResult {
        name: "Voyage date consistency",
        evaluations: board.voyages.len(),
        passed: voyage_date_problems.is_empty(),
        problems: voyage_date_problems,
        duration: Duration::from_millis(0),
    });

    // 3. Epic Checks
    let (epic_file_problems, epic_count) = checks::epics::scan_epic_files(board_dir)?;
    epic_checks.push(CheckResult {
        name: "Epic structure",
        evaluations: epic_count,
        passed: epic_file_problems.is_empty(),
        problems: epic_file_problems,
        duration: Duration::from_millis(0),
    });

    let epic_duplicate_problems = checks::epics::check_epic_duplicates(board_dir);
    epic_checks.push(CheckResult {
        name: "ID uniqueness",
        evaluations: epic_count,
        passed: epic_duplicate_problems.is_empty(),
        problems: epic_duplicate_problems,
        duration: Duration::from_millis(0),
    });

    let epic_id_problems = checks::epics::check_epic_id_consistency(&board);
    epic_checks.push(CheckResult {
        name: "ID consistency",
        evaluations: board.epics.len(),
        passed: epic_id_problems.is_empty(),
        problems: epic_id_problems,
        duration: Duration::from_millis(0),
    });

    let epic_title_problems = checks::epics::check_epic_title_case(&board);
    epic_checks.push(CheckResult {
        name: "Title convention",
        evaluations: board.epics.len(),
        passed: epic_title_problems.is_empty(),
        problems: epic_title_problems,
        duration: Duration::from_millis(0),
    });

    let epic_drift_problems = checks::epics::check_epic_status_drift(&board);
    epic_checks.push(CheckResult {
        name: "Epic status drift",
        evaluations: board.epics.len(),
        passed: epic_drift_problems.is_empty(),
        problems: epic_drift_problems,
        duration: Duration::from_millis(0),
    });

    let epic_done_problems = checks::epics::check_epic_done_gates(&board);
    epic_checks.push(CheckResult {
        name: "Epic completion gates",
        evaluations: board.epics.len(),
        passed: epic_done_problems.is_empty(),
        problems: epic_done_problems,
        duration: Duration::from_millis(0),
    });

    let epic_pr_problems = checks::epics::check_epic_press_release(&board);
    epic_checks.push(CheckResult {
        name: "Press release coherence (optional)",
        evaluations: board.epics.len(),
        passed: epic_pr_problems.is_empty(),
        problems: epic_pr_problems,
        duration: Duration::from_millis(0),
    });

    let epic_date_problems = checks::epics::check_epic_dates(&board);
    epic_checks.push(CheckResult {
        name: "Epic date consistency",
        evaluations: board.epics.len(),
        passed: epic_date_problems.is_empty(),
        problems: epic_date_problems,
        duration: Duration::from_millis(0),
    });

    // 4. Bearing Checks
    let (bearing_file_problems, bearing_count) = checks::bearings::scan_bearing_files(board_dir)?;
    bearing_checks.push(CheckResult {
        name: "Bearing structure",
        evaluations: bearing_count,
        passed: bearing_file_problems.is_empty(),
        problems: bearing_file_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_id_problems = checks::bearings::check_bearing_id_consistency(&board);
    bearing_checks.push(CheckResult {
        name: "ID consistency",
        evaluations: board.bearings.len(),
        passed: bearing_id_problems.is_empty(),
        problems: bearing_id_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_id_format_problems = checks::bearings::check_bearing_id_format(&board);
    bearing_checks.push(CheckResult {
        name: "ID format",
        evaluations: board.bearings.len(),
        passed: bearing_id_format_problems.is_empty(),
        problems: bearing_id_format_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_duplicate_problems = checks::bearings::check_bearing_duplicates(board_dir);
    bearing_checks.push(CheckResult {
        name: "ID uniqueness",
        evaluations: bearing_count,
        passed: bearing_duplicate_problems.is_empty(),
        problems: bearing_duplicate_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_title_problems = checks::bearings::check_bearing_title_case(&board);
    bearing_checks.push(CheckResult {
        name: "Title convention",
        evaluations: board.bearings.len(),
        passed: bearing_title_problems.is_empty(),
        problems: bearing_title_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_coherence_problems = checks::bearings::check_bearing_state_coherence(&board);
    bearing_checks.push(CheckResult {
        name: "Bearing coherence",
        evaluations: board.bearings.len(),
        passed: bearing_coherence_problems.is_empty(),
        problems: bearing_coherence_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_content_problems = checks::bearings::check_bearing_content_sections(&board);
    bearing_checks.push(CheckResult {
        name: "Bearing content completion",
        evaluations: board.bearings.len(),
        passed: bearing_content_problems.is_empty(),
        problems: bearing_content_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_epic_problems = checks::bearings::check_bearing_epic_coherence(&board);
    bearing_checks.push(CheckResult {
        name: "Bearing-Epic coherence",
        evaluations: board.bearings.len(),
        passed: bearing_epic_problems.is_empty(),
        problems: bearing_epic_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_date_problems = checks::bearings::check_bearing_dates(&board);
    bearing_checks.push(CheckResult {
        name: "Bearing date consistency",
        evaluations: board.bearings.len(),
        passed: bearing_date_problems.is_empty(),
        problems: bearing_date_problems,
        duration: Duration::from_millis(0),
    });

    let bearing_recommendation_problems =
        checks::bearings::check_bearing_assessment_recommendation(&board, board_dir);
    bearing_checks.push(CheckResult {
        name: "Bearing recommendation",
        evaluations: board.bearings.len(),
        passed: bearing_recommendation_problems.is_empty(),
        problems: bearing_recommendation_problems,
        duration: Duration::from_millis(0),
    });

    // 5. ADR Checks
    let (adr_file_problems, adr_count) = checks::adrs::scan_adr_files(board_dir)?;
    adr_checks.push(CheckResult {
        name: "ADR structure",
        evaluations: adr_count,
        passed: adr_file_problems.is_empty(),
        problems: adr_file_problems,
        duration: Duration::from_millis(0),
    });

    let adr_duplicate_problems = checks::adrs::check_adr_duplicates(board_dir);
    adr_checks.push(CheckResult {
        name: "ID uniqueness",
        evaluations: adr_count,
        passed: adr_duplicate_problems.is_empty(),
        problems: adr_duplicate_problems,
        duration: Duration::from_millis(0),
    });

    let adr_id_problems = checks::adrs::check_adr_id_consistency(&board);
    adr_checks.push(CheckResult {
        name: "ID consistency",
        evaluations: board.adrs.len(),
        passed: adr_id_problems.is_empty(),
        problems: adr_id_problems,
        duration: Duration::from_millis(0),
    });

    let adr_title_problems = checks::adrs::check_adr_title_case(&board);
    adr_checks.push(CheckResult {
        name: "Title convention",
        evaluations: board.adrs.len(),
        passed: adr_title_problems.is_empty(),
        problems: adr_title_problems,
        duration: Duration::from_millis(0),
    });

    let adr_warning_problems = checks::adrs::check_proposed_adr_warning(&board);
    adr_checks.push(CheckResult {
        name: "Proposed ADR usage",
        evaluations: board.adrs.len(),
        passed: adr_warning_problems.is_empty(),
        problems: adr_warning_problems,
        duration: Duration::from_millis(0),
    });

    let adr_content_problems = checks::adrs::check_adr_content_sections(&board);
    adr_checks.push(CheckResult {
        name: "ADR content completion",
        evaluations: board.adrs.len(),
        passed: adr_content_problems.is_empty(),
        problems: adr_content_problems,
        duration: Duration::from_millis(0),
    });

    let adr_date_problems = checks::adrs::check_adr_dates(&board);
    adr_checks.push(CheckResult {
        name: "ADR date consistency",
        evaluations: board.adrs.len(),
        passed: adr_date_problems.is_empty(),
        problems: adr_date_problems,
        duration: Duration::from_millis(0),
    });

    Ok(DoctorReport {
        story_checks,
        voyage_checks,
        epic_checks,
        adr_checks,
        bearing_checks,
    })
}

/// Print a summary of coverage gaps (used by `keel gaps`)
pub fn print_gap_summary(report: &DoctorReport) {
    render::print_gap_summary(report);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::validation::{CheckId, Severity};
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn validate_returns_all_story_checks() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001").stage(StoryState::Backlog))
            .build();

        let report = validate(temp.path()).unwrap();
        assert!(!report.story_checks.is_empty());
    }

    #[test]
    fn validate_detects_duplicate_story_ids() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Manual setup for duplicates - MUST include frontmatter or scan fails early
        let s1_dir = root.join("stories/S1");
        let s2_dir = root.join("stories/S2");
        fs::create_dir_all(&s1_dir).unwrap();
        fs::create_dir_all(&s2_dir).unwrap();

        fs::write(s1_dir.join("README.md"), "---\nid: DUP1\ntitle: T1\ntype: feat\nstatus: backlog\n---\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] t1 <!-- verify: manual SRS-01:start:end -->\n").unwrap();
        fs::write(s2_dir.join("README.md"), "---\nid: DUP1\ntitle: T2\ntype: feat\nstatus: backlog\n---\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] t2 <!-- verify: manual SRS-01:start:end -->\n").unwrap();

        let report = validate(root).unwrap();
        let mut found = false;
        for check in &report.story_checks {
            for prob in &check.problems {
                if prob.message.contains("duplicate story ID") {
                    found = true;
                }
            }
        }

        assert!(found, "Should detect duplicate story IDs");
    }

    #[test]
    fn acceptance_criteria_detects_unchecked_in_ready_for_acceptance() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Unchecked <!-- verify: manual SRS-01:start:end -->")
            )
            .build();

        let report = validate(temp.path()).unwrap();
        let mut found = false;
        for check in &report.story_checks {
            for prob in &check.problems {
                if prob.message.contains("unchecked acceptance criteria") {
                    found = true;
                }
            }
        }

        assert!(found);
    }

    #[test]
    fn acceptance_criteria_detects_unchecked_in_done() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Unchecked <!-- verify: manual SRS-01:start:end -->")
            )
            .build();

        let report = validate(temp.path()).unwrap();
        let mut found = false;
        for check in &report.story_checks {
            for prob in &check.problems {
                if prob.message.contains("unchecked acceptance criteria")
                    || prob.message.contains("incomplete criteria")
                {
                    found = true;
                }
            }
        }

        assert!(found);
    }

    #[test]
    fn validate_detects_terminal_story_scaffold_text() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Summary\n\nTODO: Describe the story\n\n## Acceptance Criteria\n\n- [x] [SRS-02/AC-01] done <!-- verify: manual, SRS-02:start:end -->"),
            )
            .build();

        let report = validate(temp.path()).unwrap();
        let terminal_scaffold_problems: Vec<_> = report
            .story_checks
            .iter()
            .filter(|check| check.name == "Terminal artifact coherence")
            .flat_map(|check| check.problems.iter())
            .filter(|problem| problem.check_id == CheckId::StoryTerminalScaffold)
            .collect();

        assert!(
            !terminal_scaffold_problems.is_empty(),
            "expected terminal artifact coherence check to flag unresolved scaffold text"
        );
        assert!(
            terminal_scaffold_problems
                .iter()
                .all(|problem| problem.severity == Severity::Error),
            "terminal scaffold violations must be hard errors"
        );
        assert!(
            terminal_scaffold_problems.iter().any(|problem| problem
                .message
                .contains("README has unresolved scaffold/default text")),
            "expected README scaffold violation message"
        );
    }

    #[test]
    fn validate_detects_active_story_scaffold_text() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Summary\n\nTODO: Describe the story\n\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Define acceptance criteria for this slice",
                    ),
            )
            .build();

        let report = validate(temp.path()).unwrap();
        let active_scaffold_problems: Vec<_> = report
            .story_checks
            .iter()
            .filter(|check| check.name == "Active story coherence")
            .flat_map(|check| check.problems.iter())
            .filter(|problem| problem.check_id == CheckId::StoryPlanningScaffold)
            .collect();

        assert!(
            !active_scaffold_problems.is_empty(),
            "expected active story coherence check to flag unresolved scaffold text"
        );
        assert!(
            active_scaffold_problems
                .iter()
                .all(|problem| problem.severity == Severity::Error),
            "active story scaffold violations must be hard errors"
        );
    }
}
