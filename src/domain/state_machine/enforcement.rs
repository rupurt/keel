//! Unified transition-enforcement service.
//!
//! This module composes transition legality checks with existing gate
//! evaluators and classifies findings according to caller policy.

use crate::domain::model::{Board, Story, Voyage};
use crate::domain::state_machine::gating::{
    VoyageCompletionPolicy, evaluate_story_transition, evaluate_voyage_completion,
    evaluate_voyage_transition,
};
use crate::domain::state_machine::story::StoryTransition;
use crate::domain::state_machine::voyage::VoyageTransition;
use crate::infrastructure::validation::{CheckId, Problem, Severity};

/// How enforcement findings should be classified for blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockingMode {
    /// All findings block transitions (warnings included).
    Strict,
    /// Only error findings block transitions.
    Runtime,
    /// Findings are reporting-only and never block transitions.
    Reporting,
}

/// Policy controls for transition-enforcement behavior.
#[derive(Debug, Clone, Copy)]
pub struct EnforcementPolicy {
    /// Blocking classification mode.
    pub blocking_mode: BlockingMode,
    /// Whether voyage transition checks should enforce requirement coverage.
    pub require_requirements_coverage: bool,
    /// Whether story acceptance should block manual verification criteria unless human override is used.
    pub require_human_review_for_manual_acceptance: bool,
    /// Completion-gate behavior for voyage completion transitions.
    pub completion_policy: VoyageCompletionPolicy,
}

impl EnforcementPolicy {
    /// Strict runtime mode where warnings also block.
    pub const STRICT: EnforcementPolicy = EnforcementPolicy {
        blocking_mode: BlockingMode::Strict,
        require_requirements_coverage: true,
        require_human_review_for_manual_acceptance: true,
        completion_policy: VoyageCompletionPolicy::RUNTIME,
    };

    /// Normal runtime mode where only errors block.
    pub const RUNTIME: EnforcementPolicy = EnforcementPolicy {
        blocking_mode: BlockingMode::Runtime,
        require_requirements_coverage: true,
        require_human_review_for_manual_acceptance: true,
        completion_policy: VoyageCompletionPolicy::RUNTIME,
    };

    /// Reporting mode for doctor/diagnostic flows.
    pub const REPORTING: EnforcementPolicy = EnforcementPolicy {
        blocking_mode: BlockingMode::Reporting,
        require_requirements_coverage: false,
        require_human_review_for_manual_acceptance: false,
        completion_policy: VoyageCompletionPolicy::REPORTING,
    };

    fn blocks(&self, problem: &Problem) -> bool {
        match self.blocking_mode {
            BlockingMode::Strict => true,
            BlockingMode::Runtime => problem.blocks_runtime(false),
            BlockingMode::Reporting => false,
        }
    }
}

impl Default for EnforcementPolicy {
    fn default() -> Self {
        Self::RUNTIME
    }
}

/// Transition intent for the enforcement API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionIntent {
    Story(StoryTransition),
    Voyage(VoyageTransition),
}

impl TransitionIntent {
    fn name(self) -> &'static str {
        match self {
            TransitionIntent::Story(transition) => match transition {
                StoryTransition::Start => "start",
                StoryTransition::Submit => "submit",
                StoryTransition::Accept => "accept",
                StoryTransition::Reject => "reject",
                StoryTransition::Restart => "restart",
                StoryTransition::Ice => "ice",
                StoryTransition::Thaw => "thaw",
                StoryTransition::SubmitDone => "submit-done",
            },
            TransitionIntent::Voyage(transition) => match transition {
                VoyageTransition::Plan => "plan",
                VoyageTransition::Start => "start",
                VoyageTransition::Complete => "complete",
            },
        }
    }
}

/// Entity handle for transition-enforcement.
pub enum TransitionEntity<'a> {
    Story(&'a Story),
    Voyage(&'a Voyage),
}

/// Structured result of transition-enforcement evaluation.
#[derive(Debug, Clone)]
pub struct EnforcementResult {
    /// Transition intent that was evaluated.
    #[allow(dead_code)]
    pub intent: TransitionIntent,
    /// Policy used for classification.
    #[allow(dead_code)]
    pub policy: EnforcementPolicy,
    /// Transition legality findings.
    #[allow(dead_code)]
    pub legality_problems: Vec<Problem>,
    /// Gate evaluator findings.
    #[allow(dead_code)]
    pub gate_problems: Vec<Problem>,
    /// Full set of findings.
    #[allow(dead_code)]
    pub problems: Vec<Problem>,
    /// Policy-classified blocking findings.
    pub blocking_problems: Vec<Problem>,
}

impl EnforcementResult {
    /// Returns true when transition execution should proceed.
    pub fn allows_transition(&self) -> bool {
        self.blocking_problems.is_empty()
    }
}

/// Classify findings into policy-specific blocking set.
pub fn classify_findings(policy: EnforcementPolicy, problems: &[Problem]) -> Vec<Problem> {
    problems
        .iter()
        .filter(|problem| policy.blocks(problem))
        .cloned()
        .collect()
}

/// Format blocking findings using the shared gate formatter.
pub fn format_enforcement_error(
    entity_label: &str,
    intent: TransitionIntent,
    problems: &[Problem],
) -> String {
    super::formatting::format_transition_error(entity_label, intent.name(), problems)
}

/// Enforce a transition by composing legality checks and gate evaluators.
pub fn enforce_transition(
    board: &Board,
    entity: TransitionEntity<'_>,
    intent: TransitionIntent,
    policy: EnforcementPolicy,
) -> EnforcementResult {
    let (legality_problems, gate_problems) = match (entity, intent) {
        (TransitionEntity::Story(story), TransitionIntent::Story(transition)) => {
            enforce_story_transition(board, story, transition, policy)
        }
        (TransitionEntity::Voyage(voyage), TransitionIntent::Voyage(transition)) => {
            enforce_voyage_transition(board, voyage, transition, policy)
        }
        (TransitionEntity::Story(story), TransitionIntent::Voyage(transition)) => (
            vec![incompatible_intent_problem_for_story(story, transition)],
            Vec::new(),
        ),
        (TransitionEntity::Voyage(voyage), TransitionIntent::Story(transition)) => (
            vec![incompatible_intent_problem_for_voyage(voyage, transition)],
            Vec::new(),
        ),
    };

    let mut problems = legality_problems.clone();
    problems.extend(gate_problems.clone());
    let blocking_problems = classify_findings(policy, &problems);

    EnforcementResult {
        intent,
        policy,
        legality_problems,
        gate_problems,
        problems,
        blocking_problems,
    }
}

fn enforce_story_transition(
    board: &Board,
    story: &Story,
    transition: StoryTransition,
    policy: EnforcementPolicy,
) -> (Vec<Problem>, Vec<Problem>) {
    let legality_problems = validate_story_legality(story, transition);
    if !legality_problems.is_empty() {
        return (legality_problems, Vec::new());
    }

    let mut gate_problems = evaluate_story_transition(
        board,
        story,
        transition,
        policy.require_human_review_for_manual_acceptance,
    );

    if transition == StoryTransition::Accept
        && let Some(scope) = story.scope()
        && let Some(voyage) = board.voyages.values().find(|v| v.scope_path() == scope)
    {
        let stories = board.stories_for_voyage(voyage);
        let remaining = stories
            .iter()
            .filter(|s| s.id() != story.id())
            .filter(|s| s.stage != crate::domain::model::StoryState::Done)
            .count();

        if remaining == 0 {
            gate_problems.extend(evaluate_voyage_completion(
                board,
                voyage,
                Some(story.id()),
                policy.completion_policy,
            ));
        }
    }

    (Vec::new(), gate_problems)
}

fn enforce_voyage_transition(
    board: &Board,
    voyage: &Voyage,
    transition: VoyageTransition,
    policy: EnforcementPolicy,
) -> (Vec<Problem>, Vec<Problem>) {
    let legality_problems = validate_voyage_legality(voyage, transition);
    if !legality_problems.is_empty() {
        return (legality_problems, Vec::new());
    }

    let mut gate_problems = evaluate_voyage_transition(
        board,
        voyage,
        transition,
        policy.require_requirements_coverage,
    );

    if transition == VoyageTransition::Complete {
        gate_problems.extend(evaluate_voyage_completion(
            board,
            voyage,
            None,
            policy.completion_policy,
        ));
    }

    (Vec::new(), gate_problems)
}

fn validate_story_legality(story: &Story, transition: StoryTransition) -> Vec<Problem> {
    if transition.valid_from().contains(&story.stage) {
        return Vec::new();
    }

    let valid_states = transition
        .valid_from()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" or ");

    vec![Problem {
        severity: Severity::Error,
        path: story.path.clone(),
        scope: story.scope().map(str::to_string),
        message: format!(
            "Cannot {} story '{}' from '{}' state (must be {})",
            TransitionIntent::Story(transition).name(),
            story.id(),
            story.stage,
            valid_states
        ),
        fix: None,
        category: None,
        check_id: CheckId::Unknown,
    }]
}

fn validate_voyage_legality(voyage: &Voyage, transition: VoyageTransition) -> Vec<Problem> {
    if transition.valid_from().contains(&voyage.status()) {
        return Vec::new();
    }

    let valid_states = transition
        .valid_from()
        .iter()
        .map(|state| state.to_string())
        .collect::<Vec<_>>()
        .join(" or ");

    vec![Problem {
        severity: Severity::Error,
        path: voyage.path.clone(),
        scope: Some(voyage.scope_path()),
        message: format!(
            "Cannot {} voyage '{}' from '{}' state (must be {})",
            TransitionIntent::Voyage(transition).name(),
            voyage.id(),
            voyage.status(),
            valid_states,
        ),
        fix: None,
        category: None,
        check_id: CheckId::Unknown,
    }]
}

fn incompatible_intent_problem_for_story(story: &Story, transition: VoyageTransition) -> Problem {
    Problem {
        severity: Severity::Error,
        path: story.path.clone(),
        scope: story.scope().map(str::to_string),
        message: format!(
            "Cannot apply voyage transition '{}' to story {}",
            TransitionIntent::Voyage(transition).name(),
            story.id()
        ),
        fix: None,
        category: None,
        check_id: CheckId::Unknown,
    }
}

fn incompatible_intent_problem_for_voyage(voyage: &Voyage, transition: StoryTransition) -> Problem {
    Problem {
        severity: Severity::Error,
        path: voyage.path.clone(),
        scope: Some(voyage.scope_path()),
        message: format!(
            "Cannot apply story transition '{}' to voyage {}",
            TransitionIntent::Story(transition).name(),
            voyage.id()
        ),
        fix: None,
        category: None,
        check_id: CheckId::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    fn problem_fingerprints(problems: &[Problem]) -> Vec<String> {
        let mut fingerprints = problems
            .iter()
            .map(|problem| format!("{:?}:{}", problem.severity, problem.message))
            .collect::<Vec<_>>();
        fingerprints.sort_unstable();
        fingerprints
    }

    fn submit_transition_with_missing_bundle(policy: EnforcementPolicy) -> EnforcementResult {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("STORY-RUNTIME").stage(StoryState::InProgress))
            .build();

        fs::remove_file(temp.path().join("stories/STORY-RUNTIME/REFLECT.md")).unwrap();
        fs::remove_dir_all(temp.path().join("stories/STORY-RUNTIME/EVIDENCE")).unwrap();

        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("STORY-RUNTIME").unwrap();

        enforce_transition(
            &board,
            TransitionEntity::Story(story),
            TransitionIntent::Story(StoryTransition::Submit),
            policy,
        )
    }

    fn voyage_start_with_invalid_story_states(policy: EnforcementPolicy) -> EnforcementResult {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-planned", "test-epic").status("planned"))
            .story(
                TestStory::new("ACTIVE-STORY")
                    .scope("test-epic/01-planned")
                    .stage(StoryState::InProgress),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-planned").unwrap();

        enforce_transition(
            &board,
            TransitionEntity::Voyage(voyage),
            TransitionIntent::Voyage(VoyageTransition::Start),
            policy,
        )
    }

    #[test]
    fn story_transition_composes_legality_and_gate_results() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("STORY1").stage(StoryState::InProgress))
            .build();

        fs::remove_file(temp.path().join("stories/STORY1/REFLECT.md")).unwrap();
        fs::remove_dir_all(temp.path().join("stories/STORY1/EVIDENCE")).unwrap();

        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("STORY1").unwrap();

        let result = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            TransitionIntent::Story(StoryTransition::Submit),
            EnforcementPolicy::STRICT,
        );

        assert!(result.legality_problems.is_empty());
        assert!(matches!(
            result.intent,
            TransitionIntent::Story(StoryTransition::Submit)
        ));
        assert_eq!(result.policy.blocking_mode, BlockingMode::Strict);
        assert!(
            result
                .gate_problems
                .iter()
                .any(|p| p.message.contains("REFLECT.md missing")),
            "expected REFLECT gate problem"
        );
        assert!(
            result
                .gate_problems
                .iter()
                .any(|p| p.message.contains("EVIDENCE directory missing")),
            "expected EVIDENCE gate problem"
        );
        assert!(!result.blocking_problems.is_empty());
        assert!(!result.allows_transition());
    }

    #[test]
    fn story_transition_legality_problem_is_structured() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("STORY2").stage(StoryState::Backlog))
            .build();

        let board = load_board(temp.path()).unwrap();
        let story = board.require_story("STORY2").unwrap();

        let result = enforce_transition(
            &board,
            TransitionEntity::Story(story),
            TransitionIntent::Story(StoryTransition::Submit),
            EnforcementPolicy::STRICT,
        );

        assert_eq!(result.legality_problems.len(), 1);
        assert!(result.gate_problems.is_empty());
        assert_eq!(result.problems.len(), 1);
        assert!(
            result.legality_problems[0]
                .message
                .contains("must be in-progress"),
            "unexpected legality message: {}",
            result.legality_problems[0].message
        );
    }

    #[test]
    fn voyage_transition_legality_problem_is_structured() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("VOYAGE1", "test-epic").status("done"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("VOYAGE1").unwrap();

        let result = enforce_transition(
            &board,
            TransitionEntity::Voyage(voyage),
            TransitionIntent::Voyage(VoyageTransition::Start),
            EnforcementPolicy::STRICT,
        );

        assert_eq!(result.legality_problems.len(), 1);
        assert!(result.gate_problems.is_empty());
        assert!(
            result.legality_problems[0]
                .message
                .contains("Cannot start voyage")
        );
        assert!(
            result.legality_problems[0]
                .message
                .contains("must be planned")
        );
    }

    #[test]
    fn strict_policy_blocks_warning_findings_for_voyage_complete() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-inprogress", "test-epic")
                    .status("in-progress")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY3")
                    .scope("test-epic/01-inprogress")
                    .stage(StoryState::Backlog)
                    .body(
                        "- [x] [SRS-01/AC-01] Partial chain <!-- verify: manual, SRS-01:start -->",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-inprogress").unwrap();

        let strict_policy = EnforcementPolicy {
            require_requirements_coverage: false,
            completion_policy: VoyageCompletionPolicy::REPORTING,
            ..EnforcementPolicy::STRICT
        };

        let result = enforce_transition(
            &board,
            TransitionEntity::Voyage(voyage),
            TransitionIntent::Voyage(VoyageTransition::Complete),
            strict_policy,
        );

        assert!(
            result
                .gate_problems
                .iter()
                .any(|p| p.severity == Severity::Warning),
            "expected warning gate problem"
        );
        assert!(
            result
                .blocking_problems
                .iter()
                .any(|p| p.severity == Severity::Warning),
            "strict policy should block warnings"
        );
    }

    #[test]
    fn reporting_policy_keeps_warning_findings_non_blocking() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-inprogress", "test-epic")
                    .status("in-progress")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY4")
                    .scope("test-epic/01-inprogress")
                    .stage(StoryState::Backlog)
                    .body(
                        "- [x] [SRS-01/AC-01] Partial chain <!-- verify: manual, SRS-01:start -->",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-inprogress").unwrap();

        let result = enforce_transition(
            &board,
            TransitionEntity::Voyage(voyage),
            TransitionIntent::Voyage(VoyageTransition::Complete),
            EnforcementPolicy::REPORTING,
        );

        assert!(
            result
                .gate_problems
                .iter()
                .any(|p| p.severity == Severity::Warning),
            "expected warning gate problem"
        );
        assert!(matches!(
            result.intent,
            TransitionIntent::Voyage(VoyageTransition::Complete)
        ));
        assert_eq!(result.policy.blocking_mode, BlockingMode::Reporting);
        assert!(!result.problems.is_empty());
        assert!(
            result.blocking_problems.is_empty(),
            "reporting policy should not block findings"
        );
        assert!(result.allows_transition());
    }

    #[test]
    fn runtime_mode_blocks_errors_for_story_and_voyage_transitions() {
        let story_result = submit_transition_with_missing_bundle(EnforcementPolicy::RUNTIME);
        assert!(
            story_result
                .gate_problems
                .iter()
                .any(|problem| problem.severity == Severity::Error),
            "runtime story gates should produce blocking errors"
        );
        assert!(!story_result.blocking_problems.is_empty());
        assert!(!story_result.allows_transition());

        let voyage_result = voyage_start_with_invalid_story_states(EnforcementPolicy::RUNTIME);
        assert!(
            voyage_result
                .gate_problems
                .iter()
                .any(|problem| problem.message.contains("cannot start")),
            "runtime voyage gates should include start-state violations"
        );
        assert!(!voyage_result.blocking_problems.is_empty());
        assert!(!voyage_result.allows_transition());
    }

    #[test]
    fn reporting_mode_surfaces_non_blocking_findings_for_same_story_and_voyage() {
        let story_result = submit_transition_with_missing_bundle(EnforcementPolicy::REPORTING);
        assert!(
            story_result
                .gate_problems
                .iter()
                .any(|problem| problem.severity == Severity::Error),
            "reporting mode should still surface submit gate errors"
        );
        assert!(story_result.blocking_problems.is_empty());
        assert!(story_result.allows_transition());

        let voyage_result = voyage_start_with_invalid_story_states(EnforcementPolicy::REPORTING);
        assert!(
            voyage_result
                .gate_problems
                .iter()
                .any(|problem| problem.message.contains("cannot start")),
            "reporting mode should still surface voyage start violations"
        );
        assert!(voyage_result.blocking_problems.is_empty());
        assert!(voyage_result.allows_transition());
    }

    #[test]
    fn runtime_and_reporting_share_gate_outputs_for_story_and_voyage() {
        let runtime_story = submit_transition_with_missing_bundle(EnforcementPolicy::RUNTIME);
        let reporting_story = submit_transition_with_missing_bundle(EnforcementPolicy::REPORTING);
        assert_eq!(
            problem_fingerprints(&runtime_story.gate_problems),
            problem_fingerprints(&reporting_story.gate_problems),
            "story submit gates should come from one shared rule source"
        );

        let runtime_voyage = voyage_start_with_invalid_story_states(EnforcementPolicy::RUNTIME);
        let reporting_voyage = voyage_start_with_invalid_story_states(EnforcementPolicy::REPORTING);
        assert_eq!(
            problem_fingerprints(&runtime_voyage.gate_problems),
            problem_fingerprints(&reporting_voyage.gate_problems),
            "voyage start gates should come from one shared rule source"
        );
    }

    #[test]
    fn formatting_uses_shared_gate_formatter() {
        let problem = Problem::error(std::path::PathBuf::from("story.md"), "boom");
        let problems = std::slice::from_ref(&problem);
        let message = format_enforcement_error(
            "story STORY5",
            TransitionIntent::Story(StoryTransition::Start),
            problems,
        );

        assert_eq!(
            message,
            crate::domain::state_machine::format_transition_error(
                "story STORY5",
                "start",
                problems
            )
        );
        assert_eq!(message, "Cannot start story STORY5:\n- boom");
    }

    #[test]
    fn formatting_structure_is_consistent_across_key_transitions() {
        let problem = Problem::error(std::path::PathBuf::from("story.md"), "blocked");

        let start = format_enforcement_error(
            "story STORY6",
            TransitionIntent::Story(StoryTransition::Start),
            std::slice::from_ref(&problem),
        );
        let submit = format_enforcement_error(
            "story STORY6",
            TransitionIntent::Story(StoryTransition::Submit),
            std::slice::from_ref(&problem),
        );
        let complete = format_enforcement_error(
            "voyage VOYAGE6",
            TransitionIntent::Voyage(VoyageTransition::Complete),
            std::slice::from_ref(&problem),
        );

        assert_eq!(start, "Cannot start story STORY6:\n- blocked");
        assert_eq!(submit, "Cannot submit story STORY6:\n- blocked");
        assert_eq!(complete, "Cannot complete voyage VOYAGE6:\n- blocked");
    }
}
