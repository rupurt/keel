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

fn check_epic_documents_complete(epic: &crate::domain::model::Epic) -> Vec<Problem> {
    let mut problems = Vec::new();
    let epic_dir = epic.path.parent().unwrap_or(&epic.path);

    let files = [
        ("README.md", "README"),
        ("PRD.md", "PRD"),
        ("PRESS_RELEASE.md", "Press Release"),
    ];

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
    } else if let Ok(reflect_content) = fs::read_to_string(&reflect_path)
        && !reflect_content.contains("### L")
        && !reflect_content.contains("### ML")
    {
        problems.push(
            Problem::error(
                reflect_path,
                "No knowledge units found in REFLECT.md (Institutional memory is mandatory)",
            )
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
    } else if let Ok(content) = fs::read_to_string(&reflect_path)
        && !content.contains("### L")
        && !content.contains("### ML")
    {
        problems.push(
            Problem::error(reflect_path, "No knowledge units found in REFLECT.md")
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
        && let Some(pattern) =
            crate::infrastructure::validation::structural::first_unfilled_placeholder_pattern(
                &content,
            )
    {
        problems.push(Problem {
            severity: Severity::Error,
            path: reflect_path,
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

    if require_requirements_coverage {
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

    #[test]
    fn evaluate_voyage_transition_plan_skips_legality_checks() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-planned", "test-epic").status("planned"))
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-planned")
                    .stage(StoryState::Backlog),
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
                        "- [ ] [SRS-01/AC-01] Requirement 1 covered <!-- verify: cargo test SRS-01:start:end -->",
                    ),
            )
            .build();

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
                        "- [ ] [SRS-01/AC-01] Requirement 1 covered <!-- verify: cargo test SRS-01:start:end -->",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();

        let problems = evaluate_voyage_transition(&board, voyage, VoyageTransition::Plan, false);

        assert!(problems.is_empty(), "coverage should be skippable");
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
