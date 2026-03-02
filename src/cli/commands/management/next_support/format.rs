//! Terminal formatting for pull-system decisions.

use super::{
    AdrDecision, BlockedDecision, DecomposeDecision, EmptyDecision, NextDecision, ResearchDecision,
    StoryDecision,
};
use owo_colors::OwoColorize;

fn story_header(story: &crate::domain::model::Story) -> String {
    format!(
        "{} {} [{}]",
        crate::cli::style::styled_story_id(story.id()),
        story.title(),
        crate::cli::style::styled_scope(story.scope())
    )
}

fn voyage_header(voyage: &crate::domain::model::Voyage) -> String {
    format!(
        "{} {} [{}]",
        crate::cli::style::styled_voyage_id(voyage.id()),
        voyage.title(),
        crate::cli::style::styled_epic_id(&voyage.epic_id)
    )
}

/// Format a pull-system decision for terminal display.
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
    }
}

fn format_work(d: &StoryDecision) -> String {
    let mut out = String::new();
    if d.is_continuation {
        out.push_str(&format!("{}\n", "Continue underway implementation:".bold()));
    } else {
        out.push_str(&format!("{}\n", "Next available implementation:".bold()));
    }

    out.push_str(&format!("  {}\n", story_header(&d.story)));

    if let Some(warning) = &d.warning {
        out.push_str(&format!("\n{} {}\n", "⚠".yellow(), warning.yellow()));
    }

    // Recommended command line
    let cmd = if d.is_continuation {
        format!("keel story submit {}", d.story.id())
    } else {
        format!("keel story start {}", d.story.id())
    };
    out.push_str(&format!("\nNext step:\n  {}\n", cmd.bold()));

    out
}

fn format_proposed_adrs(d: &AdrDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "Proposed Architecture Decisions need review:".bold()
    ));

    for adr in &d.adrs {
        let scope_styled = crate::cli::style::styled_scope(adr.frontmatter.context.as_deref());
        out.push_str(&format!(
            "  - {} {} [{}]\n",
            crate::cli::style::styled_id(adr.id()),
            adr.title(),
            scope_styled
        ));
    }

    if !d.blocked_stories.is_empty() {
        out.push_str(&format!(
            "\n{} stories are blocked in the backlog by these proposed decisions.\n",
            d.blocked_stories.len().to_string().yellow()
        ));

        for story in &d.blocked_stories {
            let adr_ids = &story.frontmatter.governed_by;
            let blocking_adrs: Vec<_> = d
                .adrs
                .iter()
                .filter(|a| adr_ids.contains(&a.id().to_string()))
                .collect();

            if !blocking_adrs.is_empty() {
                let adr_list = blocking_adrs
                    .iter()
                    .map(|a| format!("{} ({})", crate::cli::style::styled_id(a.id()), a.title()))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!(
                    "  - {} is blocked by: {}\n",
                    story_header(story),
                    adr_list
                ));
            }
        }
    }

    out
}

fn format_accept(d: &super::AcceptDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", "Stories need human acceptance:".bold()));

    for story in &d.stories {
        out.push_str(&format!("  - {}\n", story_header(story)));
    }

    out.push_str(&format!(
        "\nPlease review and {} or {} these stories.",
        "accept".bold(),
        "reject".bold()
    ));
    out
}

fn format_research(d: &ResearchDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "Research pipeline needs attention:".bold()
    ));

    for bearing in &d.bearings {
        out.push_str(&format!(
            "  - {} {} ({})\n",
            crate::cli::style::styled_id(bearing.id()),
            bearing.title(),
            bearing.frontmatter.status.to_string().dimmed(),
        ));
    }

    out.push_str(&format!(
        "\nRun {} to start a discovery session.",
        "keel play".bold()
    ));
    out
}

fn format_empty(d: &EmptyDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "No immediately workable items found.".bold()
    ));

    if !d.suggestions.is_empty() {
        out.push_str("\nSuggestions:\n");
        for suggestion in &d.suggestions {
            out.push_str(&format!("  - {}\n", suggestion));
        }
    }

    out
}

fn format_blocked(d: &BlockedDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "System is blocked on verification:".bold().red()
    ));
    out.push_str(&format!(
        "  {} stories are awaiting human acceptance.\n",
        d.count.to_string().yellow()
    ));
    out.push_str(&format!("  Next up: {}\n", story_header(&d.story)));
    out.push_str(&format!(
        "\nPlease review and {} or {} these stories.",
        "accept".bold(),
        "reject".bold()
    ));
    out
}

fn format_needs_stories(d: &DecomposeDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "Strategic Gap: Voyages need decomposition:".bold()
    ));

    for voyage in &d.voyages {
        out.push_str(&format!("  - {}\n", voyage_header(voyage)));
    }

    out.push_str("\nCreate stories for these voyages to refuel the execution pipeline.");
    out
}

fn format_needs_planning(d: &DecomposeDecision) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{}\n",
        "Strategic Gap: Voyages need planning:".bold()
    ));

    for voyage in &d.voyages {
        out.push_str(&format!("  - {}\n", voyage_header(voyage)));
    }

    out.push_str(&format!(
        "\nReview and move these voyages to 'planned' state:\n  {} voyage plan <id>",
        "keel".bold()
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{AdrFactory, BearingFactory, StoryFactory, VoyageFactory};

    #[test]
    fn test_format_work() {
        let story = StoryFactory::new("S1").title("Story 1").build();
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
        let story = StoryFactory::new("S1").title("Story 1").build();
        let decision = NextDecision::Work(StoryDecision {
            story,
            is_continuation: true,
            warning: Some("stale".to_string()),
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Continue underway implementation"));
        assert!(formatted.contains("stale"));
    }

    #[test]
    fn test_format_proposed_adrs() {
        let adr = AdrFactory::new("ADR-1").title("ADR 1").build();
        let decision = NextDecision::Decision(AdrDecision {
            adrs: vec![adr],
            blocked_stories: vec![],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Proposed Architecture Decisions"));
        assert!(formatted.contains("ADR-1"));
    }

    #[test]
    fn test_format_research() {
        let bearing = BearingFactory::new("B1").title("Bearing 1").build();
        let decision = NextDecision::Research(ResearchDecision {
            bearings: vec![bearing],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Research pipeline needs attention"));
        assert!(formatted.contains("B1"));
    }

    #[test]
    fn test_format_empty() {
        let decision = NextDecision::Empty(EmptyDecision {
            suggestions: vec!["Refuel".to_string()],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("No immediately workable items found"));
        assert!(formatted.contains("Refuel"));
    }

    #[test]
    fn test_format_blocked() {
        let story = StoryFactory::new("S1").title("Story 1").build();
        let decision = NextDecision::Blocked(BlockedDecision { story, count: 5 });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("System is blocked on verification"));
        assert!(formatted.contains("5"));
        assert!(formatted.contains("stories are awaiting human acceptance"));
    }

    #[test]
    fn test_format_needs_stories() {
        let voyage = VoyageFactory::new("V1", "E1").title("Voyage 1").build();
        let decision = NextDecision::NeedsStories(DecomposeDecision {
            voyages: vec![voyage],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Strategic Gap"));
        assert!(formatted.contains("Voyages need decomposition"));
        assert!(formatted.contains("V1"));
    }

    #[test]
    fn test_format_needs_planning() {
        let voyage = VoyageFactory::new("V1", "E1").title("Voyage 1").build();
        let decision = NextDecision::NeedsPlanning(DecomposeDecision {
            voyages: vec![voyage],
        });
        let formatted = format_decision(&decision);
        assert!(formatted.contains("Strategic Gap"));
        assert!(formatted.contains("Voyages need planning"));
        assert!(formatted.contains("V1"));
    }
}
