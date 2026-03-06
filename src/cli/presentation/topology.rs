//! Topology presentation helpers.

use owo_colors::OwoColorize;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::read_model::topology::{EpicTopologyProjection, StoryTopologyNode};

const EMPTY_TOPOLOGY_PLACEHOLDER: &str = "(no voyages or stories visible for this epic)";
const EMPTY_STORIES_PLACEHOLDER: &str = "(no stories visible)";

/// Render an epic topology projection for terminal output.
pub fn render_topology(
    projection: &EpicTopologyProjection,
    include_done: bool,
    width: usize,
) -> String {
    let story_count = projection
        .voyages
        .iter()
        .map(|voyage| voyage.stories.len())
        .sum::<usize>();
    let visibility = if include_done {
        "all entities (including done)"
    } else {
        "focused (planned + in-progress)"
    };

    let metadata = ShowKeyValues::new()
        .with_min_label_width(11)
        .row("Title:", format!("{}", projection.epic.title.bold()))
        .row("Epic:", style::styled_epic_id(&projection.epic.id))
        .row("Status:", style::styled_epic_stage(&projection.epic.status))
        .row("Visibility:", visibility)
        .row("Voyages:", projection.voyages.len().to_string())
        .row("Stories:", story_count.to_string());

    let mut topology = ShowSection::new("Topology");
    topology.push_lines(render_topology_lines(projection));

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    document.push_section(topology);
    document.render()
}

fn render_topology_lines(projection: &EpicTopologyProjection) -> Vec<String> {
    let mut lines = vec![format!(
        "  {} {} ({})",
        style::styled_epic_id(&projection.epic.id),
        style::styled_inline_markdown(&projection.epic.title),
        style::styled_epic_stage(&projection.epic.status)
    )];

    if projection.voyages.is_empty() {
        lines.push(format!("  {}", EMPTY_TOPOLOGY_PLACEHOLDER.dimmed()));
        return lines;
    }

    for (voyage_index, voyage) in projection.voyages.iter().enumerate() {
        let voyage_is_last = voyage_index + 1 == projection.voyages.len();
        let voyage_connector = if voyage_is_last { "└─" } else { "├─" };
        lines.push(format!(
            "  {} {} {} ({})",
            voyage_connector,
            style::styled_voyage_id(&voyage.id),
            style::styled_inline_markdown(&voyage.title),
            style::styled_voyage_stage(&voyage.status)
        ));

        let story_prefix = if voyage_is_last { "   " } else { "│  " };
        if voyage.stories.is_empty() {
            lines.push(format!(
                "  {} {}",
                story_prefix,
                EMPTY_STORIES_PLACEHOLDER.dimmed()
            ));
            continue;
        }

        for (story_index, story) in voyage.stories.iter().enumerate() {
            let story_is_last = story_index + 1 == voyage.stories.len();
            let story_connector = if story_is_last { "└─" } else { "├─" };
            lines.push(format!(
                "  {} {} {}",
                story_prefix,
                story_connector,
                render_story_line(story)
            ));
        }
    }

    lines
}

fn render_story_line(story: &StoryTopologyNode) -> String {
    let mut fragments = vec![
        style::styled_story_id(&story.id),
        style::styled_inline_markdown(&story.title),
        format!("({})", style::styled_story_status(&story.status)),
    ];

    if !story.requirement_refs.is_empty() {
        let requirements = story
            .requirement_refs
            .iter()
            .map(|requirement| style::styled_requirement_id(requirement))
            .collect::<Vec<_>>()
            .join(", ");
        fragments.push(format!("[{}]", requirements));
    }

    if !story.dependencies.is_empty() {
        let dependencies = story
            .dependencies
            .iter()
            .map(|dependency| style::styled_story_id(dependency))
            .collect::<Vec<_>>()
            .join(", ");
        fragments.push(format!("deps: {}", dependencies));
    }

    fragments.join(" ")
}
