//! Terminal formatting for pull-system decisions.

use super::{
    AcceptDecision, AdrDecision, BlockedDecision, DecomposeDecision, EmptyDecision, NextDecision,
    ResearchDecision, StoryDecision, VerifyMissionDecision,
};
use keel::domain::model::Entity;
use keel::read_model::mission_stack::{MissionStackExecutionReason, MissionStackExecutionStatus};
use owo_colors::OwoColorize;

fn story_header(story: &keel::domain::model::Story) -> String {
    format!(
        "{} {}",
        crate::cli::style::styled_story_id(story.id()),
        story.title().bold()
    )
}

/// Format a single next-step decision for terminal output.
pub fn format_decision(decision: &NextDecision) -> String {
    match decision {
        NextDecision::Work(d) => format_work(d),
        NextDecision::Decision(d) => format_proposed_adrs(d),
        NextDecision::Accept(d) => format_accept(d),
        NextDecision::Research(d) => format_research(d),
        NextDecision::Empty(d) => format_empty(d),
        NextDecision::Blocked(d) => format_blocked(d),
        NextDecision::NeedsStories(d) => format_needs_stories(d),
        NextDecision::NeedsPlanning(d) => format_needs_planning(d),
        NextDecision::NeedsPRD(d) => format_needs_prd(d),
        NextDecision::Mission(d) => format_mission(d),
        NextDecision::Missions(d) => format_missions(d),
        NextDecision::VerifyMission(d) => format_verify_mission(d),
        NextDecision::Diagnostics {
            report,
            suggested_command,
        } => format_diagnostics(report, suggested_command),
        NextDecision::StackYield(d) => format_stack_gate(d, MissionStackExecutionStatus::Yield),
        NextDecision::StackBlocked(d) => format_stack_gate(d, MissionStackExecutionStatus::Blocked),
    }
}

fn format_stack_gate(
    decision: &super::MissionStackDecision,
    expected_status: MissionStackExecutionStatus,
) -> String {
    let mut out = String::new();
    let label = match expected_status {
        MissionStackExecutionStatus::Yield => "Mission Stack Yield".bold().yellow().to_string(),
        MissionStackExecutionStatus::Blocked => "Mission Stack Blocked".bold().red().to_string(),
        MissionStackExecutionStatus::Allowed => "Mission Stack".bold().to_string(),
    };

    out.push_str(&format!(
        "{}: {} on {}\n",
        label, decision.stack.id, decision.stack.branch
    ));
    out.push_str(&format!(
        "  Local member: {} ({:?})\n",
        decision.stack.local_repo, decision.stack.local_member.role
    ));
    out.push_str(&format!("  Mode: {}", decision.stack.mode_label().bold()));
    if !decision.gate.active_repos.is_empty() {
        out.push_str(&format!(
            " | active repos: {}",
            decision.gate.active_repos.join(", ")
        ));
    }
    out.push('\n');

    if let Some(reason) = decision.gate.reason {
        out.push_str(&format!("  Reason: {}\n", stack_gate_reason(reason)));
    }

    if let Some(checkpoint) = &decision.gate.checkpoint {
        out.push_str(&format!("  Checkpoint: {checkpoint}\n"));
    }

    if !decision.gate.checkpoint_waiting_on.is_empty() {
        out.push_str(&format!(
            "  Waiting on: {}\n",
            decision.gate.checkpoint_waiting_on.join(", ")
        ));
    }

    if !decision.gate.waiting_for_receipts_from.is_empty() {
        out.push_str(&format!(
            "  Waiting for receipts from: {}\n",
            decision.gate.waiting_for_receipts_from.join(", ")
        ));
    }

    if let Some(state) = decision.gate.foreign_execution_state {
        out.push_str(&format!("  Foreign execution state: {:?}\n", state));
    }

    out.push_str("  Run `keel mission next --status` for full stack coordination state.");
    out
}

fn stack_gate_reason(reason: MissionStackExecutionReason) -> &'static str {
    match reason {
        MissionStackExecutionReason::ExclusiveLeaseHeldElsewhere => {
            "another repo currently holds the exclusive execution baton"
        }
        MissionStackExecutionReason::SharedWindowClosed => {
            "the local repo is outside the current shared execution window"
        }
        MissionStackExecutionReason::CheckpointActive => {
            "the stack is paused at a checkpoint and local execution is sealed"
        }
        MissionStackExecutionReason::PendingNegotiation => {
            "the local member is still waiting for reactor negotiation"
        }
        MissionStackExecutionReason::WaitingForReceipts => {
            "the local member is waiting on pushed receipts from another repo"
        }
        MissionStackExecutionReason::ForeignExecutionRequired => {
            "execution must happen from the managed foreign worktree"
        }
    }
}

fn format_diagnostics(
    report: &keel::read_model::diagnostics::DoctorReport,
    suggested_command: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: Board has health issues that must be resolved\n",
        "Diagnostics".bold().red()
    ));

    let errors = report.total_errors();
    let warnings = report.total_warnings();

    if errors > 0 {
        out.push_str(&format!("  - {} error(s) detected\n", errors.red().bold()));

        // Highlight the first error specifically
        if let Some(first_error) = report
            .all_problems()
            .iter()
            .find(|p| p.severity == keel::infrastructure::validation::Severity::Error)
        {
            out.push_str(&format!(
                "\nCritical Issue: {}\nLocation: {}\n",
                first_error.message.bold(),
                first_error.path.display().italic().dimmed()
            ));
        }
    }
    if warnings > 0 {
        out.push_str(&format!("  - {} warning(s) detected\n", warnings.yellow()));

        if errors == 0 {
            // If no errors, highlight the first warning
            if let Some(first_warning) = report
                .all_problems()
                .iter()
                .find(|p| p.severity == keel::infrastructure::validation::Severity::Warning)
            {
                out.push_str(&format!(
                    "\nIssue: {}\nLocation: {}\n",
                    first_warning.message.bold(),
                    first_warning.path.display().italic().dimmed()
                ));
            }
        }
    }

    out.push_str(&format!(
        "\nRun `{}` to resolve.",
        suggested_command.bold().white()
    ));
    out
}

fn format_verify_mission(d: &VerifyMissionDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} mission(s) ready for final verification\n",
        "Verification".bold().green(),
        d.missions.len()
    ));

    for mission in &d.missions {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(Entity::id(mission)),
            mission.title().bold()
        ));
    }

    out.push_str("\nReview the log and reports, then run `keel mission verify <ID>`.");
    out
}

fn format_missions(d: &super::MissionsDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "Multiple active missions require attention:".bold()
    ));

    for m in &d.missions {
        out.push_str(&format!(
            "  - {} {} ({})\n",
            crate::cli::style::styled_story_id(m.mission.id()),
            m.mission.title(),
            m.suggestion.yellow()
        ));
    }

    out.push_str("\nUse `keel mission show <ID>` to inspect goals.");
    out
}

fn format_needs_prd(d: &super::NeedsPRDDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} Epic(s) need authored PRD content\n",
        "Strategic Gap".bold().yellow(),
        d.epics.len()
    ));

    for epic in &d.epics {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(epic.id()),
            epic.title().bold()
        ));
    }

    out.push_str("\nUse `keel epic list --status draft` to identify epics needing authoring.");
    out
}

fn format_work(d: &StoryDecision) -> String {
    let mut out = String::new();
    if d.is_continuation {
        out.push_str(&format!(
            "{} work on existing story:\n  {}",
            "Continue".bold().cyan(),
            story_header(&d.story)
        ));
    } else {
        out.push_str(&format!(
            "Next available implementation:\n  {}",
            story_header(&d.story)
        ));
    }

    if let Some(warning) = &d.warning {
        out.push_str(&format!("\n\n{}", warning.yellow()));
    }

    out
}

fn format_proposed_adrs(d: &AdrDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} Proposed ADR(s) need review\n",
        "Architectural Decision".bold().yellow(),
        d.adrs.len()
    ));

    for adr in &d.adrs {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(adr.id()),
            adr.title().bold()
        ));
    }

    if !d.blocked_stories.is_empty() {
        out.push_str(&format!(
            "\nThese ADRs are blocking {} story/stories in the backlog.",
            d.blocked_stories.len()
        ));
    }

    out
}

fn format_accept(d: &AcceptDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} story/stories ready for human acceptance\n",
        "Verification".bold().green(),
        d.stories.len()
    ));

    for story in &d.stories {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(story.id()),
            story.title().bold()
        ));
    }

    out
}

fn format_research(d: &ResearchDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} research bearing(s) in progress\n",
        "Technical Research".bold().magenta(),
        d.bearings.len()
    ));

    for bearing in &d.bearings {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(bearing.id()),
            bearing.title().bold()
        ));
    }

    out
}

fn format_empty(d: &EmptyDecision) -> String {
    let mut out = String::new();
    out.push_str("No immediately workable items found.\n");
    if !d.suggestions.is_empty() {
        out.push_str("\nSuggestions:");
        for suggestion in &d.suggestions {
            out.push_str(&format!("\n  - {suggestion}"));
        }
    }
    out
}

fn format_blocked(d: &BlockedDecision) -> String {
    format!(
        "{}: Verification queue has reached its limit ({} items).\nAccept {} first to unblock the flow.",
        "System Blocked".bold().red(),
        d.count,
        crate::cli::style::styled_story_id(d.story.id())
    )
}

fn format_needs_stories(d: &DecomposeDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} Draft voyage(s) need implementable stories\n",
        "Strategic Gap".bold().yellow(),
        d.voyages.len()
    ));

    for voyage in &d.voyages {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(voyage.id()),
            voyage.title().bold()
        ));
    }

    out
}

fn format_needs_planning(d: &DecomposeDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {} Voyage(s) need authored requirements and design\n",
        "Strategic Gap".bold().yellow(),
        d.voyages.len()
    ));

    for voyage in &d.voyages {
        out.push_str(&format!(
            "  - {} {}\n",
            crate::cli::style::styled_story_id(voyage.id()),
            voyage.title().bold()
        ));
    }

    out
}

fn format_mission(d: &super::MissionDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}: {}\n",
        "Mission Steering".bold().cyan(),
        d.mission.title()
    ));
    out.push_str(&format!(
        "  {}: {}\n",
        "Status".dimmed(),
        d.mission.status()
    ));
    out.push_str(&format!(
        "  {}: {}\n",
        "Unmet Goals".dimmed(),
        d.unmet_goals.len()
    ));
    out.push_str(&format!("  {}: {}\n", "Next".bold().yellow(), d.suggestion));

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::{
        Story, StoryFrontmatter, StoryState, StoryType, Voyage, VoyageFrontmatter, VoyageState,
    };
    use std::path::PathBuf;

    fn make_test_story(id: &str, title: &str, status: StoryState) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: id.to_string(),
                title: title.to_string(),
                story_type: StoryType::Feat,
                status,
                scope: None,
                milestone: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
                submitted_at: None,
                index: None,
                governed_by: Vec::new(),
                blocked_by: Vec::new(),
                role: None,
                operator_signal: None,
            },
            path: PathBuf::from("test.md"),
            materialization_key: None,
        }
    }

    fn make_test_voyage(id: &str, title: &str, status: VoyageState) -> Voyage {
        Voyage {
            frontmatter: VoyageFrontmatter {
                id: id.to_string(),
                title: title.to_string(),
                goal: None,
                status,
                epic: None,
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
                operator_signal: None,
            },
            path: PathBuf::from("test.md"),
            epic_id: "epic1".to_string(),
        }
    }

    #[test]
    fn test_format_work() {
        let story = make_test_story("S1", "Story 1", StoryState::Backlog);
        let decision = NextDecision::Work(StoryDecision {
            story,
            is_continuation: false,
            warning: None,
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Next available implementation"));
        assert!(formatted.contains("S1"));
        assert!(formatted.contains("Story 1"));
    }

    #[test]
    fn test_format_continuation() {
        let story = make_test_story("S1", "Story 1", StoryState::InProgress);
        let decision = NextDecision::Work(StoryDecision {
            story,
            is_continuation: true,
            warning: Some("Warning!".to_string()),
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Continue"));
        assert!(formatted.contains("work on existing story"));
        assert!(formatted.contains("Warning!"));
    }

    #[test]
    fn test_format_proposed_adrs() {
        let adr = keel::domain::model::Adr {
            frontmatter: keel::domain::model::AdrFrontmatter {
                id: "ADR1".to_string(),
                title: "ADR 1".to_string(),
                status: keel::domain::model::AdrStatus::Proposed,
                context: None,
                applies_to: vec![],
                mission: None,
                supersedes: vec![],
                superseded_by: None,
                rejection_reason: None,
                deprecation_reason: None,
                decided_at: None,
                index: None,
            },
            path: std::path::PathBuf::from("path"),
        };
        let decision = NextDecision::Decision(super::AdrDecision {
            adrs: vec![adr],
            blocked_stories: vec![],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Architectural Decision"));
        assert!(formatted.contains("ADR1"));
    }

    #[test]
    fn test_format_accept() {
        let story = make_test_story("S1", "Story 1", StoryState::NeedsHumanVerification);
        let decision = NextDecision::Accept(super::AcceptDecision {
            stories: vec![story],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Verification"));
        assert!(formatted.contains("S1"));
    }

    #[test]
    fn test_format_research() {
        let bearing = keel::domain::model::Bearing {
            frontmatter: keel::domain::model::BearingFrontmatter {
                id: "B1".to_string(),
                title: "Bearing 1".to_string(),
                status: keel::domain::model::BearingStatus::Exploring,
                index: None,
                created_at: None,
                decline_reason: None,
                laid_at: None,
                epic: None,
                mission: None,
                goals: None,
                depends_on: None,
            },
            path: std::path::PathBuf::from("path"),
            has_evidence: false,
            has_assessment: false,
        };
        let decision = NextDecision::Research(super::ResearchDecision {
            bearings: vec![bearing],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Technical Research"));
        assert!(formatted.contains("B1"));
    }

    #[test]
    fn test_format_empty() {
        let decision = NextDecision::Empty(EmptyDecision {
            suggestions: vec!["Suggest 1".to_string()],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("No immediately workable items found"));
        assert!(formatted.contains("Suggest 1"));
    }

    #[test]
    fn test_format_blocked() {
        let story = make_test_story("S1", "Story 1", StoryState::Backlog);
        let decision = NextDecision::Blocked(BlockedDecision { story, count: 5 });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("System Blocked"));
        assert!(formatted.contains("5 items"));
        assert!(formatted.contains("S1"));
    }

    #[test]
    fn test_format_needs_stories() {
        let voyage = make_test_voyage("V1", "Voyage 1", VoyageState::Draft);
        let decision = NextDecision::NeedsStories(DecomposeDecision {
            voyages: vec![voyage],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Strategic Gap"));
        assert!(formatted.contains("Draft voyage(s) need implementable stories"));
        assert!(formatted.contains("V1"));
    }

    #[test]
    fn test_format_needs_planning() {
        let voyage = make_test_voyage("V1", "Voyage 1", VoyageState::Draft);
        let decision = NextDecision::NeedsPlanning(DecomposeDecision {
            voyages: vec![voyage],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Strategic Gap"));
        assert!(formatted.contains("Voyage(s) need authored requirements and design"));
        assert!(formatted.contains("V1"));
    }
}
