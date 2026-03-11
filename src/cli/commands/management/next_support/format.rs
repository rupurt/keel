//! Terminal formatting for pull-system decisions.

use super::{
    AcceptDecision, AdrDecision, BlockedDecision, DecomposeDecision, EmptyDecision, NextDecision,
    ResearchDecision, StoryDecision,
};
use owo_colors::OwoColorize;

fn story_header(story: &crate::domain::model::Story) -> String {
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
    }
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
    out.push_str(&format!("  {}: {}\n", "Status".dimmed(), d.mission.status()));
    out.push_str(&format!(
        "  {}: {}\n",
        "Unmet Goals".dimmed(),
        d.unmet_goals.len()
    ));
    out.push_str(&format!(
        "  {}: {}\n",
        "Next".bold().yellow(),
        d.suggestion
    ));

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::TestStory;

    #[test]
    fn test_format_work() {
        let story = TestStory::new("S1").title("Story 1").build();
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
        let story = TestStory::new("S1").title("Story 1").build();
        let decision = NextDecision::Work(StoryDecision {
            story,
            is_continuation: true,
            warning: Some("Warning!".to_string()),
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Continue work on existing story"));
        assert!(formatted.contains("Warning!"));
    }

    #[test]
    fn test_format_proposed_adrs() {
        let adr = crate::domain::model::Adr {
            frontmatter: crate::domain::model::AdrFrontmatter {
                id: "ADR1".to_string(),
                title: "ADR 1".to_string(),
                status: crate::domain::model::AdrStatus::Proposed,
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
        let story = TestStory::new("S1").status(StoryState::NeedsHumanVerification).build();
        let decision = NextDecision::Accept(super::AcceptDecision {
            stories: vec![story],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Verification"));
        assert!(formatted.contains("S1"));
    }

    #[test]
    fn test_format_research() {
        let bearing = crate::domain::model::Bearing {
            frontmatter: crate::domain::model::BearingFrontmatter {
                id: "B1".to_string(),
                title: "Bearing 1".to_string(),
                status: crate::domain::model::BearingStatus::Exploring,
                index: None,
                created_at: None,
                decline_reason: None,
                laid_at: None,
                epic: None,
                mission: None,
                goals: None,
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
        let story = TestStory::new("S1").build();
        let decision = NextDecision::Blocked(BlockedDecision {
            story,
            count: 5,
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("System Blocked"));
        assert!(formatted.contains("5 items"));
        assert!(formatted.contains("S1"));
    }

    #[test]
    fn test_format_needs_stories() {
        let voyage = crate::domain::model::Voyage {
            id: "V1".to_string(),
            title: "Voyage 1".to_string(),
            epic_id: "E1".to_string(),
            status: crate::domain::state_machine::voyage::VoyageState::Draft,
            path: std::path::PathBuf::from("path"),
            created_at: None,
            started_at: None,
            updated_at: None,
            completed_at: None,
        };
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
        let voyage = crate::domain::model::Voyage {
            id: "V1".to_string(),
            title: "Voyage 1".to_string(),
            epic_id: "E1".to_string(),
            status: crate::domain::state_machine::voyage::VoyageState::Draft,
            path: std::path::PathBuf::from("path"),
            created_at: None,
            started_at: None,
            updated_at: None,
            completed_at: None,
        };
        let decision = NextDecision::NeedsPlanning(DecomposeDecision {
            voyages: vec![voyage],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Strategic Gap"));
        assert!(formatted.contains("Voyage(s) need authored requirements and design"));
        assert!(formatted.contains("V1"));
    }
}
