//! Doctor command - board health diagnostics and automated fixing

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod catalog;
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

fn configured_check(
    doctor_config: &crate::infrastructure::config::DoctorConfig,
    id: &'static str,
    name: &'static str,
    evaluations: usize,
    problems: Vec<crate::infrastructure::validation::Problem>,
) -> CheckResult {
    let disabled = doctor_config.is_disabled(id);
    let passed = problems.is_empty();
    CheckResult {
        id,
        name,
        evaluations,
        problems: if disabled { Vec::new() } else { problems },
        duration: Duration::from_millis(0),
        passed: disabled || passed,
        disabled,
    }
}

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
    let (config, _) = crate::infrastructure::config::load_config();
    let doctor_config = &config.doctor;

    let mut story_checks = Vec::new();
    let mut voyage_checks = Vec::new();
    let mut epic_checks = Vec::new();
    let mut adr_checks = Vec::new();
    let mut bearing_checks = Vec::new();

    // 1. Story Checks
    let (story_file_problems, story_count) = checks::stories::scan_story_files(board_dir)?;
    story_checks.push(configured_check(
        doctor_config,
        "story-id-uniqueness",
        "ID uniqueness",
        story_count,
        story_file_problems,
    ));

    let consistency_problems = checks::stories::check_filename_frontmatter_consistency(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-id-consistency",
        "ID consistency",
        board.stories.len(),
        consistency_problems,
    ));

    let story_title_problems = checks::stories::check_story_title_case(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-title-convention",
        "Title convention",
        board.stories.len(),
        story_title_problems,
    ));

    let ac_problems = checks::stories::check_acceptance_criteria_complete(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-acceptance-criteria-completion",
        "Acceptance criteria completion",
        board.stories.len(),
        ac_problems,
    ));

    let verify_problems = checks::stories::check_verification_annotations(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-verification-annotations",
        "Verification annotations",
        board.stories.len(),
        verify_problems,
    ));

    let traceability_problems = checks::stories::check_srs_traceability(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-srs-traceability",
        "SRS traceability",
        board.stories.len(),
        traceability_problems,
    ));

    let dependency_cycle_problems = checks::stories::check_story_dependency_cycles(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-implementation-dependency-cycles",
        "Implementation dependency cycles",
        board.stories.len(),
        dependency_cycle_problems,
    ));

    let parallel_conflict_problems = checks::stories::check_parallel_conflict_coherence(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-parallel-conflict-coherence",
        "Parallel conflict coherence",
        board.stories.len(),
        parallel_conflict_problems,
    ));

    let scoped_evidence_problems = checks::stories::check_scoped_story_evidence(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-scoped-evidence-coverage",
        "Scoped evidence coverage",
        board.stories.len(),
        scoped_evidence_problems,
    ));

    let story_drift_problems = checks::stories::check_deprecated_frontmatter_fields(board_dir);
    story_checks.push(configured_check(
        doctor_config,
        "story-frontmatter-drift",
        "Story frontmatter drift",
        story_count,
        story_drift_problems,
    ));

    let reflection_problems = checks::stories::check_reflection_coherence(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-reflection-coherence",
        "Reflection coherence",
        board.stories.len(),
        reflection_problems,
    ));

    let active_coherence_problems = checks::stories::check_active_story_coherence(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-active-story-coherence",
        "Active story coherence",
        board.stories.len(),
        active_coherence_problems,
    ));

    let terminal_coherence_problems = checks::stories::check_terminal_story_coherence(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-terminal-artifact-coherence",
        "Terminal artifact coherence",
        board.stories.len(),
        terminal_coherence_problems,
    ));

    let knowledge_manifest_problems =
        checks::stories::check_knowledge_manifest_integrity(board_dir);
    story_checks.push(configured_check(
        doctor_config,
        "story-knowledge-catalog-integrity",
        "Knowledge catalog integrity",
        board.stories.len(),
        knowledge_manifest_problems,
    ));

    let manifest_problems = checks::stories::check_verification_manifests(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-verification-manifest-integrity",
        "Verification manifest integrity",
        board.stories.len(),
        manifest_problems,
    ));

    let story_date_problems = checks::stories::check_story_dates(&board);
    story_checks.push(configured_check(
        doctor_config,
        "story-date-consistency",
        "Story date consistency",
        board.stories.len(),
        story_date_problems,
    ));

    // 2. Voyage Checks
    let (voyage_file_problems, voyage_count) = checks::voyages::scan_voyage_files(board_dir)?;
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-structure",
        "Voyage structure",
        voyage_count,
        voyage_file_problems,
    ));

    let voyage_duplicate_problems = checks::voyages::check_voyage_duplicates(board_dir);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-id-uniqueness",
        "ID uniqueness",
        voyage_count,
        voyage_duplicate_problems,
    ));

    let voyage_id_problems = checks::voyages::check_voyage_id_consistency(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-id-consistency",
        "ID consistency",
        board.voyages.len(),
        voyage_id_problems,
    ));

    let voyage_title_problems = checks::voyages::check_voyage_title_case(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-title-convention",
        "Title convention",
        board.voyages.len(),
        voyage_title_problems,
    ));

    let voyage_drift_problems = checks::voyages::check_voyage_status_drift(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-status-drift",
        "Voyage status drift",
        board.voyages.len(),
        voyage_drift_problems,
    ));

    let voyage_scope_content_problems = checks::voyages::check_scope_authored_content(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-scope-authored-content",
        "Scope authored content",
        board.voyages.len(),
        voyage_scope_content_problems,
    ));

    let voyage_srs_requirements_problems = checks::voyages::check_srs_authored_requirements(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-srs-authored-requirements",
        "SRS authored requirements",
        board.voyages.len(),
        voyage_srs_requirements_problems,
    ));

    let voyage_sdd_content_problems = checks::voyages::check_sdd_authored_content(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-sdd-authored-content",
        "SDD authored content",
        board.voyages.len(),
        voyage_sdd_content_problems,
    ));

    let voyage_prd_lineage_problems = checks::voyages::check_prd_lineage_coherence(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-prd-lineage-coherence",
        "PRD lineage coherence",
        board.voyages.len(),
        voyage_prd_lineage_problems,
    ));

    let voyage_scope_lineage_problems = checks::voyages::check_scope_lineage_coherence(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-scope-lineage-coherence",
        "Scope lineage coherence",
        board.voyages.len(),
        voyage_scope_lineage_problems,
    ));

    let voyage_legacy_scope_problems = checks::voyages::check_legacy_scope_headings(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-legacy-scope-headings",
        "Legacy scope headings",
        board.voyages.len(),
        voyage_legacy_scope_problems,
    ));

    let voyage_artifact_problems = checks::voyages::check_voyage_press_release_artifacts(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-artifact-contract",
        "Voyage artifact contract",
        board.voyages.len(),
        voyage_artifact_problems,
    ));

    let evidence_problems = checks::evidence::check_evidence_chains(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-evidence-chains",
        "Evidence chains",
        board.voyages.len(),
        evidence_problems,
    ));

    let voyage_date_problems = checks::voyages::check_voyage_dates(&board);
    voyage_checks.push(configured_check(
        doctor_config,
        "voyage-date-consistency",
        "Voyage date consistency",
        board.voyages.len(),
        voyage_date_problems,
    ));

    // 3. Epic Checks
    let (epic_file_problems, epic_count) = checks::epics::scan_epic_files(board_dir)?;
    epic_checks.push(configured_check(
        doctor_config,
        "epic-structure",
        "Epic structure",
        epic_count,
        epic_file_problems,
    ));

    let epic_duplicate_problems = checks::epics::check_epic_duplicates(board_dir);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-id-uniqueness",
        "ID uniqueness",
        epic_count,
        epic_duplicate_problems,
    ));

    let epic_id_problems = checks::epics::check_epic_id_consistency(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-id-consistency",
        "ID consistency",
        board.epics.len(),
        epic_id_problems,
    ));

    let epic_title_problems = checks::epics::check_epic_title_case(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-title-convention",
        "Title convention",
        board.epics.len(),
        epic_title_problems,
    ));

    let epic_drift_problems = checks::epics::check_epic_status_drift(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-status-drift",
        "Epic status drift",
        board.epics.len(),
        epic_drift_problems,
    ));

    let epic_done_problems = checks::epics::check_epic_done_gates(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-completion-gates",
        "Epic completion gates",
        board.epics.len(),
        epic_done_problems,
    ));

    let epic_goal_lineage_problems = checks::epics::check_epic_goal_lineage_coherence(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-goal-lineage-coherence",
        "Goal lineage coherence",
        board.epics.len(),
        epic_goal_lineage_problems,
    ));

    let epic_pr_problems = checks::epics::check_epic_press_release(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-press-release-coherence",
        "Press release coherence (optional)",
        board.epics.len(),
        epic_pr_problems,
    ));

    let epic_date_problems = checks::epics::check_epic_dates(&board);
    epic_checks.push(configured_check(
        doctor_config,
        "epic-date-consistency",
        "Epic date consistency",
        board.epics.len(),
        epic_date_problems,
    ));

    // 4. Bearing Checks
    let (bearing_file_problems, bearing_count) = checks::bearings::scan_bearing_files(board_dir)?;
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-structure",
        "Bearing structure",
        bearing_count,
        bearing_file_problems,
    ));

    let bearing_id_problems = checks::bearings::check_bearing_id_consistency(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-id-consistency",
        "ID consistency",
        board.bearings.len(),
        bearing_id_problems,
    ));

    let bearing_id_format_problems = checks::bearings::check_bearing_id_format(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-id-format",
        "ID format",
        board.bearings.len(),
        bearing_id_format_problems,
    ));

    let bearing_duplicate_problems = checks::bearings::check_bearing_duplicates(board_dir);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-id-uniqueness",
        "ID uniqueness",
        bearing_count,
        bearing_duplicate_problems,
    ));

    let bearing_title_problems = checks::bearings::check_bearing_title_case(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-title-convention",
        "Title convention",
        board.bearings.len(),
        bearing_title_problems,
    ));

    let bearing_coherence_problems = checks::bearings::check_bearing_state_coherence(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-coherence",
        "Bearing coherence",
        board.bearings.len(),
        bearing_coherence_problems,
    ));

    let bearing_content_problems = checks::bearings::check_bearing_content_sections(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-content-completion",
        "Bearing content completion",
        board.bearings.len(),
        bearing_content_problems,
    ));

    let bearing_epic_problems = checks::bearings::check_bearing_epic_coherence(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-epic-coherence",
        "Bearing-Epic coherence",
        board.bearings.len(),
        bearing_epic_problems,
    ));

    let bearing_date_problems = checks::bearings::check_bearing_dates(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-date-consistency",
        "Bearing date consistency",
        board.bearings.len(),
        bearing_date_problems,
    ));

    let bearing_recommendation_problems =
        checks::bearings::check_bearing_assessment_recommendation(&board, board_dir);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-recommendation",
        "Bearing decision readiness",
        board.bearings.len(),
        bearing_recommendation_problems,
    ));

    // 5. ADR Checks
    let (adr_file_problems, adr_count) = checks::adrs::scan_adr_files(board_dir)?;
    adr_checks.push(configured_check(
        doctor_config,
        "adr-structure",
        "ADR structure",
        adr_count,
        adr_file_problems,
    ));

    let adr_duplicate_problems = checks::adrs::check_adr_duplicates(board_dir);
    adr_checks.push(configured_check(
        doctor_config,
        "adr-id-uniqueness",
        "ID uniqueness",
        adr_count,
        adr_duplicate_problems,
    ));

    let adr_id_problems = checks::adrs::check_adr_id_consistency(&board);
    adr_checks.push(configured_check(
        doctor_config,
        "adr-id-consistency",
        "ID consistency",
        board.adrs.len(),
        adr_id_problems,
    ));

    let adr_title_problems = checks::adrs::check_adr_title_case(&board);
    adr_checks.push(configured_check(
        doctor_config,
        "adr-title-convention",
        "Title convention",
        board.adrs.len(),
        adr_title_problems,
    ));

    let adr_warning_problems = checks::adrs::check_proposed_adr_warning(&board);
    adr_checks.push(configured_check(
        doctor_config,
        "adr-proposed-usage",
        "Proposed ADR usage",
        board.adrs.len(),
        adr_warning_problems,
    ));

    let adr_content_problems = checks::adrs::check_adr_content_sections(&board);
    adr_checks.push(configured_check(
        doctor_config,
        "adr-content-completion",
        "ADR content completion",
        board.adrs.len(),
        adr_content_problems,
    ));

    let adr_date_problems = checks::adrs::check_adr_dates(&board);
    adr_checks.push(configured_check(
        doctor_config,
        "adr-date-consistency",
        "ADR date consistency",
        board.adrs.len(),
        adr_date_problems,
    ));

    Ok(DoctorReport {
        story_checks,
        voyage_checks,
        epic_checks,
        adr_checks,
        bearing_checks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::validation::{CheckId, Severity};
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn with_current_dir<T>(path: &Path, f: impl FnOnce() -> T) -> T {
        let original = std::env::current_dir().unwrap();
        std::env::set_current_dir(path).unwrap();
        let result = f();
        std::env::set_current_dir(original).unwrap();
        result
    }

    fn write_prd(temp: &tempfile::TempDir, epic_id: &str, content: &str) {
        fs::write(temp.path().join(format!("epics/{epic_id}/PRD.md")), content).unwrap();
    }

    #[test]
    fn validate_returns_all_story_checks() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001").status(StoryState::Backlog))
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
                    .status(StoryState::NeedsHumanVerification)
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
                    .status(StoryState::Done)
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
                    .status(StoryState::NeedsHumanVerification)
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
                    .status(StoryState::Backlog)
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

    #[test]
    fn validate_marks_disabled_doctor_checks() {
        let srs = r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Requirement 1 | FR-01 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("planned")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] covered"),
            )
            .build();
        write_prd(
            &temp,
            "e1",
            r#"# PRD

## Scope

### In Scope
- [SCOPE-01] Ship the planned slice.

### Out of Scope
- [SCOPE-02] Leave follow-on hardening for later.

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Requirement 1 | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );
        fs::write(
            temp.path().join("keel.toml"),
            r#"[doctor.checks.voyage-scope-authored-content]
disabled = true
"#,
        )
        .unwrap();

        let report = with_current_dir(temp.path(), || validate(temp.path()).unwrap());
        let scope_check = report
            .voyage_checks
            .iter()
            .find(|check| check.id == "voyage-scope-authored-content")
            .unwrap();

        assert!(scope_check.disabled);
        assert!(scope_check.passed);
        assert!(scope_check.problems.is_empty());
    }
}
