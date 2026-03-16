//! Board health diagnostics engine

pub mod cache;
pub mod catalog;
pub mod checks;
pub mod fixes;
pub mod types;

use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;
use std::time::Duration;

use crate::domain::model::AC_REQ_RE;
use crate::domain::model::StoryState;
use crate::infrastructure::loader::load_board;
pub use types::{CheckResult, DoctorReport};

// Legacy constants for compatibility with existing check modules
pub static CRITERIA_RE: &LazyLock<Regex> = &AC_REQ_RE;
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
        id: id.to_string(),
        name: name.to_string(),
        evaluations,
        problems: if disabled { Vec::new() } else { problems },
        duration: Duration::from_millis(0),
        passed: disabled || passed,
        disabled,
    }
}

/// Run all health checks and return a full report with a freshness signal
pub fn validate(board_dir: &Path) -> Result<(DoctorReport, bool)> {
    // Prefer config from the board directory itself or its parent (project root)
    let (config, source) = crate::infrastructure::config::load_config_from(board_dir);
    if source != crate::infrastructure::config::ConfigSource::Defaults {
        return validate_with_config_internal(board_dir, &config);
    }

    let project_root = board_dir.parent().unwrap_or(board_dir);
    let (config, _) = crate::infrastructure::config::load_config_from(project_root);
    validate_with_config_internal(board_dir, &config)
}

/// Legacy compatibility for single report return
pub fn validate_report(board_dir: &Path) -> Result<DoctorReport> {
    Ok(validate(board_dir)?.0)
}

/// Run all health checks with a specific configuration
pub fn validate_with_config(
    board_dir: &Path,
    config: &crate::infrastructure::config::Config,
) -> Result<DoctorReport> {
    Ok(validate_with_config_internal(board_dir, config)?.0)
}

fn validate_with_config_internal(
    board_dir: &Path,
    config: &crate::infrastructure::config::Config,
) -> Result<(DoctorReport, bool)> {
    let current_hash = cache::calculate_board_hash(board_dir)?;
    if let Some((cached, _)) = cache::load_cache(board_dir, &current_hash) {
        return Ok((cached, true));
    }

    let board = load_board(board_dir)?;
    let doctor_config = &config.doctor;

    let mut story_checks = Vec::new();
    let mut voyage_checks = Vec::new();
    let mut epic_checks = Vec::new();
    let mut adr_checks = Vec::new();
    let mut bearing_checks = Vec::new();
    let mut mission_checks = Vec::new();
    let mut workflow_checks = Vec::new();
    let mut pacemaker_checks = Vec::new();
    let mut delivery_checks = Vec::new();

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

    let bearing_dependency_problems = checks::bearings::check_bearing_dependencies(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-dependencies",
        "Bearing dependency integrity",
        board.bearings.len(),
        bearing_dependency_problems,
    ));

    let bearing_lineage_epic_problems = checks::bearings::check_bearing_lineage_epic(&board);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-lineage-epic",
        "Bearing lineage epic",
        board.bearings.len(),
        bearing_lineage_epic_problems,
    ));

    let bearing_lineage_goals_problems =
        checks::bearings::check_bearing_lineage_goals(&board, board_dir);
    bearing_checks.push(configured_check(
        doctor_config,
        "bearing-lineage-goals",
        "Bearing lineage goals",
        board.bearings.len(),
        bearing_lineage_goals_problems,
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

    // 6. Mission Checks
    let mission_duplicate_problems = checks::missions::check_mission_duplicates(board_dir);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-id-uniqueness",
        "ID uniqueness",
        board.missions.len(),
        mission_duplicate_problems,
    ));

    let mission_goal_problems = checks::missions::check_mission_goals(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-goal-achievement",
        "Goal achievement",
        board.missions.len(),
        mission_goal_problems,
    ));

    let mission_definition_problems = checks::missions::check_mission_definition_readiness(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-definition-readiness",
        "Mission definition readiness",
        board.missions.len(),
        mission_definition_problems,
    ));

    let mission_work_problems = checks::missions::check_mission_active_no_work(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-active-work-coherence",
        "Active mission work coherence",
        board.missions.len(),
        mission_work_problems,
    ));

    let mission_orphan_problems = checks::missions::check_mission_orphans(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-orphaned-lineage",
        "Orphaned mission lineage",
        board.epics.len() + board.bearings.len() + board.adrs.len(),
        mission_orphan_problems,
    ));

    let mission_date_problems = checks::missions::check_mission_dates(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-date-consistency",
        "Mission date consistency",
        board.missions.len(),
        mission_date_problems,
    ));

    let mission_evidence_problems = checks::missions::check_mission_completion_evidence(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-completion-evidence",
        "Mission completion evidence",
        board.missions.len(),
        mission_evidence_problems,
    ));

    let mission_watch_problems = checks::missions::check_mission_watches(&board);
    mission_checks.push(configured_check(
        doctor_config,
        "mission-watch-constraint",
        "Watch constraints",
        board.missions.len(),
        mission_watch_problems,
    ));

    let mut routine_checks = Vec::new();

    // 7. Routine Checks
    let routine_cadence_problems = checks::routines::check_routine_cadence(&board);
    routine_checks.push(configured_check(
        doctor_config,
        "routine-cadence",
        "Routine cadence",
        board.routines.len(),
        routine_cadence_problems,
    ));

    let routine_id_problems = checks::routines::check_routine_id_consistency(&board);
    routine_checks.push(configured_check(
        doctor_config,
        "routine-id-consistency",
        "ID consistency",
        board.routines.len(),
        routine_id_problems,
    ));

    let routine_scope_problems = checks::routines::check_routine_scope_coherence(&board);
    routine_checks.push(configured_check(
        doctor_config,
        "routine-scope-coherence",
        "Scope coherence",
        board.routines.len(),
        routine_scope_problems,
    ));

    // 8. Workflow Checks
    let graph_integrity_problems = checks::graph::check_workflow_graph_integrity(&board);
    workflow_checks.push(configured_check(
        doctor_config,
        "workflow-graph-integrity",
        "Graph integrity",
        board.missions.len()
            + board.epics.len()
            + board.bearings.len()
            + board.adrs.len()
            + board.voyages.len()
            + board.stories.len()
            + board.routines.len()
            + 1, // Pacemaker
        graph_integrity_problems,
    ));

    let topology_problems = checks::workflow::check_workflow_topology(board_dir);
    workflow_checks.push(configured_check(
        doctor_config,
        "workflow-topology",
        "Workflow topology",
        1, // Single topology to check
        topology_problems,
    ));

    let overload_problems =
        checks::stories::check_circuit_overload(&board, config.workflow.max_battery_packs);
    workflow_checks.push(configured_check(
        doctor_config,
        "workflow-circuit-overload",
        "Circuit Overload (Max Battery Packs)",
        board
            .stories
            .values()
            .filter(|s| s.status == StoryState::Backlog)
            .count(),
        overload_problems,
    ));

    // 9. Pacemaker Checks
    let pacemaker_problems = checks::pacemaker::check_pacemaker_stability(&board);
    pacemaker_checks.push(configured_check(
        doctor_config,
        "pacemaker-stability",
        "Pacemaker stability",
        1,
        pacemaker_problems,
    ));

    // 10. Delivery Checks
    let liquidity_problems = checks::delivery::check_delivery_liquidity(&board);
    delivery_checks.push(configured_check(
        doctor_config,
        "delivery-liquidity",
        "Backlog liquidity",
        board
            .stories
            .values()
            .filter(|s| s.status == StoryState::Backlog)
            .count(),
        liquidity_problems,
    ));

    let blockage_problems = checks::delivery::check_delivery_blockages(&board);
    delivery_checks.push(configured_check(
        doctor_config,
        "delivery-blockages",
        "Active blockages",
        board
            .stories
            .values()
            .filter(|s| s.status == StoryState::InProgress)
            .count(),
        blockage_problems,
    ));

    let mut report = DoctorReport {
        story_checks,
        voyage_checks,
        epic_checks,
        adr_checks,
        bearing_checks,
        mission_checks,
        routine_checks,
        workflow_checks,
        pacemaker_checks,
        delivery_checks,
        drift_coefficient: 0.0,
        estimated_remediation_hours: 0.0,
        last_checked_at: std::time::SystemTime::now(),
    };

    report.calculate_drift();

    // Save to cache
    let _ = cache::save_cache(board_dir, current_hash, report.clone());

    Ok((report, false))
}
