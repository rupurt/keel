#![allow(dead_code)]
//! Domain gate evaluators used to enforce transition and completion preconditions.
//!
//! Gates are deterministic, side-effect free checks that can be reused by both
//! the transition engine and command/doctor validation paths.

use std::collections::HashMap;
use std::fs;

use crate::domain::model::{Board, Story, StoryState, Voyage};
use crate::domain::state_machine::invariants;
use crate::domain::state_machine::story::StoryTransition;
use crate::domain::state_machine::voyage::VoyageTransition;
use crate::infrastructure::validation::{CheckId, Problem, Severity};
use crate::infrastructure::verification::parse_verify_annotations;
use crate::infrastructure::verification::parser::RequirementPhase;

/// Whether and how voyage completion gates should evaluate evidence.
#[derive(Debug, Clone, Copy)]
pub struct VoyageCompletionPolicy {
    /// If true, incomplete evidence blocks completion checks.
    pub strict: bool,
    /// If true, require all stories to be done (including candidate) before running completion checks.
    pub require_all_stories_done: bool,
    /// If true, enforce evidence chain checks.
    pub require_evidence: bool,
}

impl VoyageCompletionPolicy {
    /// Policy for completion at runtime (e.g. `story accept`, `voyage done`).
    pub const RUNTIME: VoyageCompletionPolicy = VoyageCompletionPolicy {
        strict: true,
        require_all_stories_done: true,
        require_evidence: true,
    };

    /// Policy for doctor checks (reporting only).
    pub const REPORTING: VoyageCompletionPolicy = VoyageCompletionPolicy {
        strict: false,
        require_all_stories_done: false,
        require_evidence: true,
    };
}

/// Internal evidence accumulator used by evidence-chain checks.
#[derive(Debug, Default)]
struct RequirementEvidence {
    has_start: bool,
    has_end: bool,
    has_start_end: bool,
    has_continues: bool,
}

/// Find a voyage by scope (`epic/voyage`).
pub fn voyage_for_scope<'a>(board: &'a Board, scope: &str) -> Option<&'a Voyage> {
    board.voyages.values().find(|v| v.scope_path() == scope)
}

/// Convert blocking policy for runtime paths.
pub fn problem_blocks_runtime(problem: &Problem, strict: bool) -> bool {
    strict || matches!(problem.severity, Severity::Error)
}

/// Return runtime-blocking gate problems for a story transition.
pub fn evaluate_story_transition(
    board: &Board,
    story: &Story,
    transition: StoryTransition,
    require_human_review_for_manual_acceptance: bool,
) -> Vec<Problem> {
    match transition {
        StoryTransition::Start => evaluate_story_start(board, story),
        StoryTransition::Thaw => evaluate_story_thaw(story),
        StoryTransition::Submit => evaluate_story_submit(board, story),
        StoryTransition::Accept => {
            evaluate_story_accept(board, story, require_human_review_for_manual_acceptance)
        }
        _ => Vec::new(),
    }
}

fn evaluate_story_start(board: &Board, story: &Story) -> Vec<Problem> {
    let mut problems = Vec::new();

    let Some(scope) = story.scope() else {
        return problems;
    };

    let Some(voyage) = voyage_for_scope(board, scope) else {
        return problems;
    };

    // 1. Check Voyage documentation
    problems.extend(check_voyage_documents_complete(voyage));

    // 2. Check Epic documentation
    if let Some(epic) = board.epics.get(&voyage.epic_id) {
        problems.extend(check_epic_documents_complete(epic));
    }

    if !voyage.status().is_ready_for_work() {
        problems.push(Problem {
            severity: Severity::Error,
            path: voyage.path.clone(),
            scope: Some(scope.to_string()),
            message: format!(
                "Cannot start story {} while voyage '{}' (scope: {}) is '{}'; must be 'planned' or 'in-progress'",
                story.id(),
                voyage.id(),
                voyage.scope_path(),
                voyage.status()
            ),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}

fn evaluate_story_thaw(story: &Story) -> Vec<Problem> {
    if story.scope().is_none() {
        return Vec::new();
    }
    check_story_ready_for_backlog_transition(story)
}

fn check_epic_documents_complete(epic: &crate::domain::model::Epic) -> Vec<Problem> {
    let mut problems = Vec::new();
    let epic_dir = epic.path.parent().unwrap_or(&epic.path);
    let prd_path = epic_dir.join("PRD.md");

    let files = [("README.md", "README"), ("PRD.md", "PRD")];

    for (filename, label) in files {
        let path = epic_dir.join(filename);
        if !path.exists() {
            problems.push(Problem {
                severity: Severity::Error,
                path: path.clone(),
                scope: Some(epic.id().to_string()),
                message: format!("Epic {} is missing {}", epic.id(), label),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            continue;
        }

        if let Ok(content) = fs::read_to_string(&path)
            && crate::infrastructure::validation::structural::is_placeholder_unfilled(&content)
        {
            problems.push(Problem {
                severity: Severity::Error,
                path: path.clone(),
                scope: Some(epic.id().to_string()),
                message: format!("Epic {} has unfilled placeholders in {}", epic.id(), label),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    if prd_path.exists() {
        let mut prd_problems =
            crate::infrastructure::validation::structural::check_epic_prd_authored_content(
                &prd_path,
            );
        for problem in &mut prd_problems {
            if problem.scope.is_none() {
                problem.scope = Some(epic.id().to_string());
            }
        }
        problems.extend(prd_problems);
    }

    let press_release_path = epic_dir.join("PRESS_RELEASE.md");
    if press_release_path.exists()
        && let Ok(content) = fs::read_to_string(&press_release_path)
        && crate::infrastructure::validation::structural::is_placeholder_unfilled(&content)
    {
        problems.push(Problem {
            severity: Severity::Error,
            path: press_release_path,
            scope: Some(epic.id().to_string()),
            message: format!(
                "Epic {} has unfilled placeholders in optional Press Release",
                epic.id()
            ),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}

fn evaluate_story_submit(_board: &Board, story: &Story) -> Vec<Problem> {
    let mut problems = Vec::new();

    // 1. Check for evidence bundle structural requirements
    let bundle_dir = match story.path.parent() {
        Some(p) => p,
        None => return problems,
    };

    let evidence_dir = bundle_dir.join("EVIDENCE");
    if !evidence_dir.exists() {
        problems.push(
            Problem::error(story.path.clone(), "EVIDENCE directory missing in bundle")
                .with_check_id(CheckId::Unknown),
        );
    }

    let reflect_path = bundle_dir.join("REFLECT.md");
    if !reflect_path.exists() {
        problems.push(
            Problem::error(story.path.clone(), "REFLECT.md missing in bundle")
                .with_check_id(CheckId::Unknown),
        );
    }

    // 2. Check for SRS refs
    let content = match fs::read_to_string(&story.path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    let ac_report = crate::infrastructure::validation::parse_acceptance_criteria(&content);
    if !ac_report.is_complete() {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!(
                "{} unchecked acceptance criteria",
                ac_report.unchecked.len()
            ),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    // 2. Check for verification annotations & SRS traceability
    for criterion in ac_report.checked.iter().chain(ac_report.unchecked.iter()) {
        let has_verify = criterion.contains("verify:") && criterion.contains("<!--");
        if !has_verify {
            problems.push(Problem {
                severity: Severity::Error,
                path: story.path.clone(),
                scope: story.scope().map(String::from),
                message: "missing verification annotations".to_string(),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            break;
        }

        if !crate::cli::style::AC_REQ_RE.is_match(criterion) {
            problems.push(Problem {
                severity: Severity::Error,
                path: story.path.clone(),
                scope: story.scope().map(String::from),
                message: "missing SRS refs".to_string(),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            break;
        }
    }

    // 3. Check for evidence chain phase markers
    let annotations = parse_verify_annotations(&content);
    let has_phases = annotations.iter().any(|a| a.requirement.is_some());
    if !has_phases && !ac_report.checked.is_empty() {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: "missing evidence chain phase markers (e.g. :start, :end)".to_string(),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems.extend(check_story_bundle_coherence(story, bundle_dir));

    problems
}

fn evaluate_story_accept(
    _board: &Board,
    story: &Story,
    require_human_review_for_manual_acceptance: bool,
) -> Vec<Problem> {
    let mut problems = Vec::new();

    if require_human_review_for_manual_acceptance
        && let Ok(content) = fs::read_to_string(&story.path)
        && crate::infrastructure::validation::parse_acceptance_criteria(&content).requires_manual()
    {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!(
                "Story {} has manual acceptance criteria. Please use --human to confirm manual verification.",
                story.id()
            ),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    let bundle_dir = match story.path.parent() {
        Some(p) => p,
        None => return problems,
    };

    // 1. Ensure REFLECT.md exists and has content
    let reflect_path = bundle_dir.join("REFLECT.md");
    if !reflect_path.exists() {
        problems.push(
            Problem::error(story.path.clone(), "REFLECT.md missing in bundle")
                .with_check_id(CheckId::Unknown),
        );
    }

    // 2. Ensure EVIDENCE/ directory exists
    let evidence_dir = bundle_dir.join("EVIDENCE");
    if !evidence_dir.exists() {
        problems.push(
            Problem::error(story.path.clone(), "EVIDENCE directory missing in bundle")
                .with_check_id(CheckId::Unknown),
        );
    }

    problems.extend(check_story_bundle_coherence(story, bundle_dir));

    problems
}

fn check_story_bundle_coherence(story: &Story, bundle_dir: &std::path::Path) -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Ok(content) = fs::read_to_string(&story.path)
        && let Some(pattern) =
            crate::infrastructure::validation::structural::first_unfilled_placeholder_pattern(
                &content,
            )
    {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!(
                "README has unresolved scaffold/default text (pattern: {})",
                pattern
            ),
            fix: None,
            category: None,
            check_id: CheckId::StoryTerminalScaffold,
        });
    }

    let reflect_path = bundle_dir.join("REFLECT.md");
    if reflect_path.exists()
        && let Ok(content) = fs::read_to_string(&reflect_path)
    {
        if let Some(pattern) =
            crate::infrastructure::validation::structural::first_unfilled_placeholder_pattern(
                &content,
            )
        {
            problems.push(Problem {
                severity: Severity::Error,
                path: reflect_path.clone(),
                scope: story.scope().map(String::from),
                message: format!(
                    "REFLECT has unresolved scaffold/default text (pattern: {})",
                    pattern
                ),
                fix: None,
                category: None,
                check_id: CheckId::StoryTerminalScaffold,
            });
        }

        for issue in crate::read_model::knowledge::scanner::validate_knowledge_content(&content) {
            problems.push(Problem {
                severity: Severity::Error,
                path: reflect_path.clone(),
                scope: story.scope().map(String::from),
                message: format!(
                    "REFLECT has invalid knowledge unit {}: {}",
                    issue.id, issue.reason
                ),
                fix: None,
                category: None,
                check_id: CheckId::StoryTerminalScaffold,
            });
        }
    }

    problems
}

/// Return gate problems for voyage lifecycle transition checks.
pub fn evaluate_voyage_transition(
    board: &Board,
    voyage: &Voyage,
    transition: VoyageTransition,
    require_requirements_coverage: bool,
) -> Vec<Problem> {
    let stories = board.stories_for_voyage(voyage);
    match transition {
        VoyageTransition::Plan => {
            evaluate_voyage_plan(board, voyage, &stories, require_requirements_coverage)
        }
        VoyageTransition::Start => evaluate_voyage_start(board, voyage, &stories),
        VoyageTransition::Complete => evaluate_voyage_start_completion(voyage, board, &stories),
    }
}

fn evaluate_voyage_plan(
    board: &Board,
    voyage: &Voyage,
    stories: &[&Story],
    require_requirements_coverage: bool,
) -> Vec<Problem> {
    let mut problems = Vec::new();

    if stories.is_empty() {
        problems.push(Problem {
            severity: Severity::Error,
            path: voyage.path.clone(),
            scope: Some(voyage.scope_path()),
            message: format!("Voyage {} has no stories", voyage.id()),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
        return problems;
    }

    // Check for unfilled placeholders in planning documents
    problems.extend(check_voyage_documents_complete(voyage));

    // Stories thawed to backlog by `voyage plan` must be actionable.
    // Reject scaffold/default story content before allowing the transition.
    for story in stories {
        problems.extend(check_story_ready_for_backlog_transition(story));
    }

    if require_requirements_coverage {
        problems.extend(prd_lineage_gate_problems(voyage, board));
        problems.extend(
            uncovered_requirements_gate_problems(voyage, board)
                .into_iter()
                .map(|message| Problem {
                    severity: Severity::Error,
                    path: voyage.path.parent().unwrap_or(&voyage.path).join("SRS.md"),
                    scope: Some(voyage.scope_path()),
                    message,
                    fix: None,
                    category: None,
                    check_id: CheckId::Unknown,
                }),
        );
    }

    problems
}

fn check_story_ready_for_backlog_transition(story: &Story) -> Vec<Problem> {
    let mut problems = Vec::new();
    let content = match fs::read_to_string(&story.path) {
        Ok(content) => content,
        Err(err) => {
            problems.push(Problem {
                severity: Severity::Error,
                path: story.path.clone(),
                scope: story.scope().map(String::from),
                message: format!("Story {} README cannot be read: {}", story.id(), err),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            return problems;
        }
    };

    let criteria = crate::infrastructure::validation::parse_acceptance_criteria(&content);
    let criteria_count = criteria.checked.len() + criteria.unchecked.len();
    if !criteria.has_section {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!("Story {} has no acceptance criteria section", story.id()),
            fix: None,
            category: None,
            check_id: CheckId::StoryIncompleteAcceptance,
        });
    } else if criteria_count == 0 {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!(
                "Story {} has no acceptance criteria checklist items",
                story.id()
            ),
            fix: None,
            category: None,
            check_id: CheckId::StoryIncompleteAcceptance,
        });
    }

    let missing_refs = crate::infrastructure::validation::missing_srs_references(&criteria);
    if !missing_refs.is_empty() {
        let list = missing_refs
            .iter()
            .map(|criterion| format!("      - {}", criterion))
            .collect::<Vec<_>>()
            .join("\n");
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!(
                "Story {} has {} acceptance criteria missing SRS refs:\n{}",
                story.id(),
                missing_refs.len(),
                list
            ),
            fix: None,
            category: None,
            check_id: CheckId::StoryMissingSrsRef,
        });
    }

    if let Some(pattern) =
        crate::infrastructure::validation::structural::first_unfilled_placeholder_pattern(&content)
    {
        problems.push(Problem {
            severity: Severity::Error,
            path: story.path.clone(),
            scope: story.scope().map(String::from),
            message: format!(
                "Story {} README has unresolved scaffold/default text (pattern: {})",
                story.id(),
                pattern
            ),
            fix: None,
            category: None,
            check_id: CheckId::StoryPlanningScaffold,
        });
    }

    problems
}

fn evaluate_voyage_start(_board: &Board, voyage: &Voyage, stories: &[&Story]) -> Vec<Problem> {
    let mut problems = Vec::new();

    // Check for unfilled placeholders in planning documents (MUST be filled to start)
    problems.extend(check_voyage_documents_complete(voyage));

    let mut invalid_stories = Vec::new();
    for story in stories {
        if !matches!(story.stage, StoryState::Backlog | StoryState::Icebox) {
            invalid_stories.push(format!("{} ({})", story.id(), story.stage));
        }
    }

    if !invalid_stories.is_empty() {
        problems.push(Problem {
            severity: Severity::Error,
            path: voyage.path.clone(),
            scope: Some(voyage.scope_path()),
            message: format!(
                "Voyage {} cannot start: {} story(ies) are not in allowed states (backlog/icebox)",
                voyage.id(),
                invalid_stories.join(", ")
            ),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}

fn check_voyage_documents_complete(voyage: &Voyage) -> Vec<Problem> {
    let mut problems = Vec::new();
    let voyage_dir = voyage.path.parent().unwrap_or(&voyage.path);

    let files = [
        ("README.md", "README"),
        ("SRS.md", "SRS"),
        ("SDD.md", "SDD"),
    ];

    for (filename, label) in files {
        let path = voyage_dir.join(filename);
        if !path.exists() {
            problems.push(Problem {
                severity: Severity::Error,
                path: path.clone(),
                scope: Some(voyage.scope_path()),
                message: format!("Voyage {} is missing {}", voyage.id(), label),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            continue;
        }

        if let Ok(content) = fs::read_to_string(&path)
            && crate::infrastructure::validation::structural::is_placeholder_unfilled(&content)
        {
            problems.push(Problem {
                severity: Severity::Error,
                path: path.clone(),
                scope: Some(voyage.scope_path()),
                message: format!(
                    "Voyage {} has unfilled placeholders in {}",
                    voyage.id(),
                    label
                ),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
}

fn evaluate_voyage_start_completion(
    _voyage: &Voyage,
    _board: &Board,
    _stories: &[&Story],
) -> Vec<Problem> {
    Vec::new()
}

fn uncovered_requirements_gate_problems(voyage: &Voyage, board: &Board) -> Vec<String> {
    invariants::uncovered_requirements_for_voyage(voyage, board)
        .into_iter()
        .map(|id| format!("{} requirements not covered by stories", id))
        .collect()
}

fn prd_lineage_gate_problems(voyage: &Voyage, board: &Board) -> Vec<Problem> {
    let srs_path = voyage.path.parent().unwrap_or(&voyage.path).join("SRS.md");
    invariants::evaluate_prd_srs_lineage(voyage, board)
        .into_iter()
        .map(|issue| Problem {
            severity: Severity::Error,
            path: srs_path.clone(),
            scope: Some(voyage.scope_path()),
            message: issue.message(),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        })
        .collect()
}

/// Evaluate completion/evidence gates for a voyage.
pub fn evaluate_voyage_completion(
    board: &Board,
    voyage: &Voyage,
    candidate_story_done: Option<&str>,
    policy: VoyageCompletionPolicy,
) -> Vec<Problem> {
    let stories = board.stories_for_voyage(voyage);
    let mut problems = Vec::new();

    // 1. Check documentation completeness (MUST be fully filled to complete)
    if policy.require_all_stories_done {
        problems.extend(check_voyage_documents_complete(voyage));
    }

    if policy.require_all_stories_done {
        let all_done = stories.iter().all(|s| {
            if Some(s.id()) == candidate_story_done {
                true
            } else {
                s.stage == StoryState::Done
            }
        });

        if !all_done {
            let done_count = stories
                .iter()
                .filter(|s| s.stage == StoryState::Done)
                .count();
            let next_count = if candidate_story_done.is_some() {
                done_count + 1
            } else {
                done_count
            };
            problems.push(Problem {
                severity: Severity::Error,
                path: voyage.path.clone(),
                scope: Some(voyage.scope_path()),
                message: format!(
                    "Voyage {} is not complete: {}/{} stories done",
                    voyage.id(),
                    next_count.min(stories.len()),
                    stories.len()
                ),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            return problems;
        }
    }

    if !policy.require_evidence {
        return problems;
    }

    let srs_path = voyage.path.parent().map(|p| p.join("SRS.md"));
    let Some(srs_path) = srs_path else {
        return problems;
    };

    let requirements = invariants::parse_requirements(&srs_path);
    if requirements.is_empty() {
        return problems;
    }

    let mut evidence_by_req: HashMap<String, RequirementEvidence> = HashMap::new();
    for story in &stories {
        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for annotation in parse_verify_annotations(&content) {
            if let Some(req_ref) = annotation.requirement {
                let entry = evidence_by_req.entry(req_ref.id).or_default();
                match req_ref.phase {
                    RequirementPhase::Start => entry.has_start = true,
                    RequirementPhase::Continues => entry.has_continues = true,
                    RequirementPhase::End => entry.has_end = true,
                    RequirementPhase::StartEnd => entry.has_start_end = true,
                }
            }
        }
    }

    if evidence_by_req.is_empty() {
        if !policy.strict && !policy.require_all_stories_done {
            return problems;
        }

        problems.push(Problem {
            severity: if policy.strict || policy.require_all_stories_done {
                Severity::Error
            } else {
                Severity::Warning
            },
            path: srs_path.clone(),
            scope: Some(voyage.scope_path()),
            message: format!("0/{}/requirements have evidence chains", requirements.len()),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
        return problems;
    }

    let _has_complete_chain = evidence_by_req
        .values()
        .any(|ev| ev.has_start_end || (ev.has_start && ev.has_end));
    let enforce_completeness = policy.strict || policy.require_all_stories_done;

    for req_id in requirements {
        match evidence_by_req.get(&req_id) {
            Some(ev) => {
                if ev.has_start_end {
                    continue;
                }
                if ev.has_start && ev.has_end {
                    continue;
                }

                if policy.strict {
                    if ev.has_start && !ev.has_end {
                        problems.push(Problem {
                            severity: Severity::Error,
                            path: srs_path.clone(),
                            scope: Some(voyage.scope_path()),
                            message: format!("{} has start but missing end", req_id),
                            fix: None,
                            category: None,
                            check_id: CheckId::Unknown,
                        });
                    }
                    if ev.has_end && !ev.has_start {
                        problems.push(Problem {
                            severity: Severity::Error,
                            path: srs_path.clone(),
                            scope: Some(voyage.scope_path()),
                            message: format!("{} has end but missing start", req_id),
                            fix: None,
                            category: None,
                            check_id: CheckId::Unknown,
                        });
                    }
                    if ev.has_continues && !ev.has_start {
                        problems.push(Problem {
                            severity: Severity::Error,
                            path: srs_path.clone(),
                            scope: Some(voyage.scope_path()),
                            message: format!("{} has continues but missing start", req_id),
                            fix: None,
                            category: None,
                            check_id: CheckId::Unknown,
                        });
                    }
                } else {
                    if ev.has_start && !ev.has_end {
                        problems.push(Problem {
                            severity: Severity::Warning,
                            path: srs_path.clone(),
                            scope: Some(voyage.scope_path()),
                            message: format!("{} has start but missing end", req_id),
                            fix: None,
                            category: None,
                            check_id: CheckId::Unknown,
                        });
                    }
                    if ev.has_end && !ev.has_start {
                        problems.push(Problem {
                            severity: Severity::Warning,
                            path: srs_path.clone(),
                            scope: Some(voyage.scope_path()),
                            message: format!("{} has end but missing start", req_id),
                            fix: None,
                            category: None,
                            check_id: CheckId::Unknown,
                        });
                    }
                    if ev.has_continues && !ev.has_start {
                        problems.push(Problem {
                            severity: Severity::Warning,
                            path: srs_path.clone(),
                            scope: Some(voyage.scope_path()),
                            message: format!("{} has continues but missing start", req_id),
                            fix: None,
                            category: None,
                            check_id: CheckId::Unknown,
                        });
                    }
                }
            }
            None => {
                if enforce_completeness {
                    problems.push(Problem {
                        severity: Severity::Error,
                        path: srs_path.clone(),
                        scope: Some(voyage.scope_path()),
                        message: format!("{} has no evidence chain", req_id),
                        fix: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
        }
    }

    problems
}

/// Evaluate gates for completing an epic.
pub fn evaluate_epic_done(board: &Board, epic: &crate::domain::model::Epic) -> Vec<Problem> {
    let mut problems = Vec::new();

    // 1. Check all voyages are Done
    let voyages = board.voyages_for_epic(epic);
    let incomplete_voyages: Vec<_> = voyages
        .iter()
        .filter(|v| v.status() != crate::domain::state_machine::voyage::VoyageState::Done)
        .map(|v| v.id())
        .collect();

    if !incomplete_voyages.is_empty() {
        problems.push(Problem {
            severity: Severity::Error,
            path: epic.path.clone(),
            scope: Some(epic.id().to_string()),
            message: format!(
                "Epic {} cannot be completed: {} incomplete voyage(s) ({})",
                epic.id(),
                incomplete_voyages.len(),
                incomplete_voyages.join(", ")
            ),
            fix: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    // 2. Check documentation completeness
    problems.extend(check_epic_documents_complete(epic));

    // 3. Check Success Criteria are all checked
    if let Ok(prd_content) =
        fs::read_to_string(epic.path.parent().unwrap_or(&epic.path).join("PRD.md"))
    {
        let criteria = crate::infrastructure::validation::parse_acceptance_criteria(&prd_content);
        if !criteria.is_complete() {
            problems.push(Problem {
                severity: Severity::Error,
                path: epic.path.clone(),
                scope: Some(epic.id().to_string()),
                message: format!(
                    "Epic {} has {} unchecked success criteria in PRD.md",
                    epic.id(),
                    criteria.unchecked.len()
                ),
                fix: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
}

/// Format a non-empty set of blocking gate problems into a command error string.
pub fn format_gate_error(entity: &str, transition: &str, problems: &[Problem]) -> String {
    super::formatting::format_transition_error(entity, transition, problems)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use std::path::PathBuf;

    fn has_message(problems: &[Problem], needle: &str) -> bool {
        problems
            .iter()
            .any(|problem| problem.message.contains(needle))
    }

    fn story_terminal_scaffold_problem<'a>(
        problems: &'a [Problem],
        needle: &str,
    ) -> Option<&'a Problem> {
        problems.iter().find(|problem| {
            problem.check_id == CheckId::StoryTerminalScaffold && problem.message.contains(needle)
        })
    }

    fn write_prd(temp: &tempfile::TempDir, epic_id: &str, content: &str) {
        fs::write(temp.path().join(format!("epics/{epic_id}/PRD.md")), content).unwrap();
    }

    #[test]
    fn evaluate_voyage_transition_plan_skips_legality_checks() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-planned", "test-epic").status("planned"))
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-planned")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Summary\n\nReady for work.\n\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Planned item",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-planned").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, true);

        assert!(
            problems.is_empty(),
            "legality should be enforced by state_machine::enforcement"
        );
    }

    #[test]
    fn evaluate_voyage_transition_plan_requires_requirements_when_requested() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Requirement 1 | FR-01 | test |
| SRS-02 | Requirement 2 | FR-02 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Requirement 1 covered <!-- verify: cargo test SRS-01:start:end -->",
                    ),
            )
            .build();
        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Requirement 1 | must | test |
| FR-02 | Requirement 2 | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, true);

        assert_eq!(
            problems.len(),
            1,
            "uncovered SRS requirements should fail plan gate"
        );
        assert!(has_message(&problems, "SRS-02"));
        assert!(problems[0].severity == Severity::Error);
    }

    #[test]
    fn evaluate_voyage_transition_plan_bypasses_requirements_when_not_requested() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
| SRS-02 | Requirement 2 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Requirement 1 covered <!-- verify: cargo test SRS-01:start:end -->",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, false);

        assert!(problems.is_empty(), "coverage should be skippable");
    }

    #[test]
    fn evaluate_voyage_transition_plan_rejects_story_scaffold_text() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Summary\n\nImplemented details pending.\n\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Define acceptance criteria for this slice",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, false);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryPlanningScaffold);
        assert!(has_message(
            &problems,
            "PLAN01 README has unresolved scaffold/default text"
        ));
    }

    #[test]
    fn evaluate_voyage_transition_plan_requires_acceptance_criteria_items() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog)
                    .body("## Summary\n\nReady.\n\n## Acceptance Criteria\n\nTBD"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, false);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryIncompleteAcceptance);
        assert!(has_message(
            &problems,
            "PLAN01 has no acceptance criteria checklist items"
        ));
    }

    #[test]
    fn prd_lineage_gate_errors_are_actionable() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Requirement 1 | PRD-01 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Requirement 1 covered <!-- verify: cargo test SRS-01:start:end -->",
                    ),
            )
            .build();
        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Requirement 1 | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, true);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(
            problems[0].path,
            temp.path().join("epics/test-epic/voyages/01-draft/SRS.md")
        );
        assert!(has_message(&problems, "SRS-01"));
        assert!(has_message(&problems, "PRD-01"));
        assert!(has_message(&problems, "SRS.md"));
        assert!(has_message(&problems, "FR-* or NFR-*"));
    }

    #[test]
    fn evaluate_voyage_completion_runtime_requires_all_stories_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-inprogress", "test-epic").status("in-progress"))
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-inprogress")
                    .stage(StoryState::Done),
            )
            .story(
                TestStory::new("STORY02")
                    .scope("test-epic/01-inprogress")
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-inprogress").unwrap();

        let policy = VoyageCompletionPolicy {
            strict: true,
            require_all_stories_done: true,
            require_evidence: false,
        };
        let problems = evaluate_voyage_completion(&board, voyage, None, policy);

        assert_eq!(problems.len(), 1);
        assert!(has_message(&problems, "is not complete"));
        assert_eq!(problems[0].severity, Severity::Error);
    }

    #[test]
    fn evaluate_voyage_completion_accepts_reporting_policy_without_full_story_completion() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-inprogress", "test-epic")
                    .status("in-progress")
                    .srs_content(
                        r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
                    "#,
                    ),
            )
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-inprogress")
                    .stage(StoryState::Backlog)
                    .body("- [ ] [SRS-01/AC-01] started <!-- verify: cargo test, SRS-01:start -->"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-inprogress").unwrap();

        let policy = VoyageCompletionPolicy {
            strict: false,
            require_all_stories_done: false,
            require_evidence: true,
        };
        let problems = evaluate_voyage_completion(&board, voyage, None, policy);

        assert_eq!(problems.len(), 1);
        assert!(has_message(&problems, "start but missing end"));
        assert_eq!(problems[0].severity, Severity::Warning);
    }

    #[test]
    fn test_evaluate_story_start_voyage_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("S1")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S1").unwrap();

        let problems = evaluate_story_start(&board, story);
        assert_eq!(problems.len(), 1);
        assert!(has_message(&problems, "must be 'planned' or 'in-progress'"));
    }

    #[test]
    fn test_evaluate_story_start_requires_authored_epic_prd() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-planned", "test-epic").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("test-epic/01-planned")
                    .stage(StoryState::Backlog),
            )
            .build();
        fs::write(
            temp.path().join("epics/test-epic/PRD.md"),
            r#"# PRD
## Problem Statement

## Goals & Objectives
| Goal | Success Metric | Target |
|------|----------------|--------|
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
<!-- END FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN SUCCESS_CRITERIA -->
<!-- END SUCCESS_CRITERIA -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S1").unwrap();
        let problems = evaluate_story_start(&board, story);

        assert!(problems.iter().any(|problem| {
            problem.check_id == CheckId::EpicPrdAuthoredContent
                && problem.message.contains("Problem Statement")
        }));
    }

    #[test]
    fn test_evaluate_story_submit_checks() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").title("Story 1").body(
                "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] unchecked <!-- verify: manual -->",
            ))
            .build();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S1").unwrap();

        let problems = evaluate_story_submit(&board, story);
        // Should find unchecked AC
        assert!(has_message(&problems, "1 unchecked acceptance criteria"));
    }

    #[test]
    fn evaluate_story_submit_blocks_unresolved_readme_scaffold() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S-SUBMIT-README").title("Story 1").body(
                "TODO: finish this\n\n## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] implemented <!-- verify: cargo test, SRS-01:start:end -->",
            ))
            .build();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-SUBMIT-README").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Submit, true);
        let problem = story_terminal_scaffold_problem(
            &problems,
            "README has unresolved scaffold/default text",
        )
        .expect("expected README scaffold violation");
        assert_eq!(problem.severity, Severity::Error);
        assert_eq!(problem.check_id, CheckId::StoryTerminalScaffold);
        assert!(
            problems
                .iter()
                .filter(|candidate| candidate.check_id == CheckId::StoryTerminalScaffold)
                .all(|candidate| candidate.severity == Severity::Error),
            "terminal scaffold gate must never downgrade to warnings"
        );
    }

    #[test]
    fn evaluate_story_submit_blocks_unresolved_reflect_scaffold() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S-SUBMIT-REFLECT").title("Story 1").body(
                "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] implemented <!-- verify: cargo test, SRS-01:start:end -->",
            ))
            .build();
        fs::write(
            temp.path().join("stories/S-SUBMIT-REFLECT/REFLECT.md"),
            "# Reflection\n\n### L-01: Keep reflection\n\nTODO: replace this",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-SUBMIT-REFLECT").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Submit, true);
        let problem = story_terminal_scaffold_problem(
            &problems,
            "REFLECT has unresolved scaffold/default text",
        )
        .expect("expected REFLECT scaffold violation");
        assert_eq!(problem.severity, Severity::Error);
        assert_eq!(problem.check_id, CheckId::StoryTerminalScaffold);
        assert!(
            problems
                .iter()
                .filter(|candidate| candidate.check_id == CheckId::StoryTerminalScaffold)
                .all(|candidate| candidate.severity == Severity::Error),
            "terminal scaffold gate must never downgrade to warnings"
        );
    }

    #[test]
    fn evaluate_story_submit_blocks_invalid_knowledge_entry() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S-SUBMIT-KNOWLEDGE").title("Story 1").body(
                "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] implemented <!-- verify: cargo test, SRS-01:start:end -->",
            ))
            .build();
        fs::write(
            temp.path().join("stories/S-SUBMIT-KNOWLEDGE/REFLECT.md"),
            "# Reflection\n\n## Knowledge\n\n### L001: Implementation Insight\n\n| Field | Value |\n|-------|-------|\n| **Insight** | |\n| **Suggested Action** | |\n",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-SUBMIT-KNOWLEDGE").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Submit, true);
        assert!(
            has_message(&problems, "REFLECT has invalid knowledge unit"),
            "invalid knowledge entries should be hard-blocked on submit"
        );
    }

    #[test]
    fn evaluate_story_submit_allows_reflect_without_knowledge_entries() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S-SUBMIT-OPTIONAL-KNOWLEDGE").title("Story 1").body(
                "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] implemented <!-- verify: cargo test, SRS-01:start:end -->",
            ))
            .build();
        fs::write(
            temp.path()
                .join("stories/S-SUBMIT-OPTIONAL-KNOWLEDGE/REFLECT.md"),
            "# Reflection\n\n## Knowledge\n\n## Observations\n\nNo novel reusable insight in this slice.\n",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-SUBMIT-OPTIONAL-KNOWLEDGE").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Submit, true);
        assert!(
            !has_message(&problems, "No knowledge units found"),
            "knowledge capture should be optional when no reusable insights are present"
        );
        assert!(
            !has_message(&problems, "REFLECT has invalid knowledge unit"),
            "empty knowledge section should not be treated as invalid"
        );
    }

    #[test]
    fn evaluate_story_accept_requires_human_override_for_manual_checks() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-ACCEPT")
                    .stage(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Manual validation <!-- verify: manual -->",
                    ),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-ACCEPT").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Accept, true);
        assert!(has_message(&problems, "manual acceptance criteria"));
        assert!(problems.iter().any(|p| p.severity == Severity::Error));
    }

    #[test]
    fn evaluate_story_accept_allows_manual_checks_when_human_override_is_set() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-ACCEPT2")
                    .stage(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Manual validation <!-- verify: manual -->",
                    ),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-ACCEPT2").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Accept, false);
        assert!(
            !has_message(&problems, "manual acceptance criteria"),
            "manual requirement should be bypassed when human override is set"
        );
    }

    #[test]
    fn evaluate_story_accept_blocks_unresolved_reflect_scaffold() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-ACCEPT-REFLECT")
                    .stage(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Done <!-- verify: cargo test, SRS-01:start:end -->",
                    ),
            )
            .build();
        fs::write(
            temp.path().join("stories/S-ACCEPT-REFLECT/REFLECT.md"),
            "# Reflection\n\n### L-01: Captured\n\nTODO: pending cleanup",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-ACCEPT-REFLECT").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Accept, false);
        let problem = story_terminal_scaffold_problem(
            &problems,
            "REFLECT has unresolved scaffold/default text",
        )
        .expect("expected REFLECT scaffold violation");
        assert_eq!(problem.severity, Severity::Error);
        assert_eq!(problem.check_id, CheckId::StoryTerminalScaffold);
        assert!(
            problems
                .iter()
                .filter(|candidate| candidate.check_id == CheckId::StoryTerminalScaffold)
                .all(|candidate| candidate.severity == Severity::Error),
            "terminal scaffold gate must never downgrade to warnings"
        );
    }

    #[test]
    fn evaluate_story_accept_blocks_invalid_knowledge_entry() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-ACCEPT-KNOWLEDGE")
                    .stage(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Done <!-- verify: cargo test, SRS-01:start:end -->",
                    ),
            )
            .build();
        fs::write(
            temp.path().join("stories/S-ACCEPT-KNOWLEDGE/REFLECT.md"),
            "# Reflection\n\n## Knowledge\n\n### L001: Title\n\n| Field | Value |\n|-------|-------|\n| **Insight** | Placeholder insight |\n| **Suggested Action** | |\n",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-ACCEPT-KNOWLEDGE").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Accept, false);
        assert!(
            has_message(&problems, "REFLECT has invalid knowledge unit"),
            "invalid knowledge entries should be hard-blocked on accept"
        );
    }

    #[test]
    fn evaluate_story_accept_allows_reflect_without_knowledge_entries() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-ACCEPT-OPTIONAL-KNOWLEDGE")
                    .stage(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Done <!-- verify: cargo test, SRS-01:start:end -->",
                    ),
            )
            .build();
        fs::write(
            temp.path()
                .join("stories/S-ACCEPT-OPTIONAL-KNOWLEDGE/REFLECT.md"),
            "# Reflection\n\n## Knowledge\n\n## Observations\n\nNo reusable insight captured.\n",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-ACCEPT-OPTIONAL-KNOWLEDGE").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Accept, false);
        assert!(
            !has_message(&problems, "No knowledge units found"),
            "knowledge capture should be optional at accept time"
        );
        assert!(
            !has_message(&problems, "REFLECT has invalid knowledge unit"),
            "empty knowledge section should be allowed"
        );
    }

    #[test]
    fn evaluate_story_accept_ignores_generated_manifest_for_scaffold_gate() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S-ACCEPT-MANIFEST")
                    .stage(StoryState::NeedsHumanVerification)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Done <!-- verify: cargo test, SRS-01:start:end -->",
                    ),
            )
            .build();
        fs::write(
            temp.path().join("stories/S-ACCEPT-MANIFEST/manifest.yaml"),
            "artifact: TODO: generated report content",
        )
        .unwrap();
        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("S-ACCEPT-MANIFEST").unwrap();

        let problems = evaluate_story_transition(&board, story, StoryTransition::Accept, false);
        assert!(
            !has_message(&problems, "unresolved scaffold/default text"),
            "only README and REFLECT should be checked for unresolved scaffold markers"
        );
    }

    #[test]
    fn test_format_gate_error() {
        let problem = Problem::error(PathBuf::from("test"), "Some error");
        let formatted = format_gate_error("story", "start", &[problem]);
        assert!(formatted.contains("Cannot start story"));
        assert!(formatted.contains("- Some error"));
    }
}
