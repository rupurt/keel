//! Topology presentation helpers.

use owo_colors::OwoColorize;

use crate::cli::presentation::planning_lineage;
use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::read_model::planning_show::ScopeDriftRow;
use crate::read_model::topology::{
    EpicTopologyEpic, EpicTopologyProjection, HorizonCommentary, HorizonCommentaryKind,
    KnowledgeAnnotationKind, StoryTopologyNode, TopologyKnowledgeAnnotation,
};

const EMPTY_TOPOLOGY_PLACEHOLDER: &str = "(no voyages or stories visible for this epic)";
const EMPTY_STORIES_PLACEHOLDER: &str = "(no stories visible)";
const EMPTY_KNOWLEDGE_PLACEHOLDER: &str = "(no scoped knowledge surfaced)";
const EMPTY_HORIZON_PLACEHOLDER: &str = "(no approaching risks detected)";

#[derive(Debug, Clone, Copy)]
struct LayoutHints {
    epic_title_limit: usize,
    voyage_title_limit: usize,
    story_title_limit: usize,
    knowledge_title_limit: usize,
    horizon_message_limit: usize,
    requirement_limit: usize,
    hotspot_limit: usize,
}

impl LayoutHints {
    fn for_width(width: usize) -> Self {
        if width < 90 {
            Self {
                epic_title_limit: 28,
                voyage_title_limit: 24,
                story_title_limit: 22,
                knowledge_title_limit: 20,
                horizon_message_limit: 46,
                requirement_limit: 2,
                hotspot_limit: 1,
            }
        } else if width < 120 {
            Self {
                epic_title_limit: 40,
                voyage_title_limit: 30,
                story_title_limit: 28,
                knowledge_title_limit: 28,
                horizon_message_limit: 70,
                requirement_limit: 3,
                hotspot_limit: 2,
            }
        } else {
            Self {
                epic_title_limit: 56,
                voyage_title_limit: 42,
                story_title_limit: 36,
                knowledge_title_limit: 40,
                horizon_message_limit: 96,
                requirement_limit: 4,
                hotspot_limit: 4,
            }
        }
    }
}

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
    let layout = LayoutHints::for_width(width);
    topology.push_lines(render_topology_lines(projection, layout));

    let mut knowledge = ShowSection::new("Knowledge");
    knowledge.push_lines(render_knowledge_lines(projection, layout));

    let mut horizon = ShowSection::new("Horizon");
    horizon.push_lines(render_horizon_lines(projection, layout));

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    document.push_section(topology);
    document.push_section(knowledge);
    document.push_section(horizon);
    document.render()
}

fn render_topology_lines(projection: &EpicTopologyProjection, layout: LayoutHints) -> Vec<String> {
    let mut lines = vec![render_epic_line(&projection.epic, layout)];

    if projection.voyages.is_empty() {
        lines.push(format!("  {}", EMPTY_TOPOLOGY_PLACEHOLDER.dimmed()));
        return lines;
    }

    for (voyage_index, voyage) in projection.voyages.iter().enumerate() {
        let voyage_is_last = voyage_index + 1 == projection.voyages.len();
        let voyage_connector = if voyage_is_last { "└─" } else { "├─" };
        let voyage_hotspots = summarize_hotspots(voyage_hotspots(voyage), layout.hotspot_limit);

        lines.push(format!(
            "  {} {} {} ({}){}",
            voyage_connector,
            style::styled_voyage_id(&voyage.id),
            style::styled_inline_markdown(&truncate_text(&voyage.title, layout.voyage_title_limit)),
            style::styled_voyage_stage(&voyage.status),
            render_annotation_suffix(&voyage_hotspots)
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
                render_story_line(story, layout)
            ));
        }
    }

    lines
}

fn render_epic_line(epic: &EpicTopologyEpic, layout: LayoutHints) -> String {
    let epic_hotspots = summarize_hotspots(epic_hotspots(epic), layout.hotspot_limit);
    format!(
        "  {} {} ({}){}",
        style::styled_epic_id(&epic.id),
        style::styled_inline_markdown(&truncate_text(&epic.title, layout.epic_title_limit)),
        style::styled_epic_stage(&epic.status),
        render_annotation_suffix(&epic_hotspots)
    )
}

fn render_story_line(story: &StoryTopologyNode, layout: LayoutHints) -> String {
    let requirement_refs = summarize_annotations(
        story
            .requirement_refs
            .iter()
            .map(|requirement| style::styled_requirement_id(requirement))
            .collect(),
        layout.requirement_limit,
    );
    let story_hotspots = summarize_hotspots(story_hotspots(story), layout.hotspot_limit);

    let mut fragments = vec![
        style::styled_story_id(&story.id),
        style::styled_inline_markdown(&truncate_text(&story.title, layout.story_title_limit)),
        format!("({})", style::styled_story_status(&story.status)),
    ];

    if !requirement_refs.is_empty() {
        fragments.push(format!("[{}]", requirement_refs.join(", ")));
    }

    let mut rendered = fragments.join(" ");
    rendered.push_str(&render_annotation_suffix(&story_hotspots));
    rendered
}

fn render_knowledge_lines(projection: &EpicTopologyProjection, layout: LayoutHints) -> Vec<String> {
    let mut lines = Vec::new();

    for annotation in &projection.recent_insights {
        lines.push(render_knowledge_line(annotation, layout));
    }
    for annotation in &projection.pending_knowledge {
        lines.push(render_knowledge_line(annotation, layout));
    }

    if lines.is_empty() {
        vec![format!("  {}", EMPTY_KNOWLEDGE_PLACEHOLDER.dimmed())]
    } else {
        lines
    }
}

fn render_knowledge_line(annotation: &TopologyKnowledgeAnnotation, layout: LayoutHints) -> String {
    let label = match annotation.kind {
        KnowledgeAnnotationKind::RecentInsight => "recent insight",
        KnowledgeAnnotationKind::PendingKnowledge => "pending knowledge",
    };

    let mut fragments = vec![
        format!("{label}:"),
        format!("[{}]", annotation.id.cyan()),
        style::styled_inline_markdown(&truncate_text(
            &annotation.title,
            layout.knowledge_title_limit,
        )),
        format!("({})", annotation.category),
    ];
    if annotation.scope.is_some() {
        fragments.push(format!(
            "[{}]",
            style::styled_scope(annotation.scope.as_deref())
        ));
    }

    format!("  - {}", fragments.join(" "))
}

fn render_horizon_lines(projection: &EpicTopologyProjection, layout: LayoutHints) -> Vec<String> {
    if projection.horizon.is_empty() {
        return vec![format!("  {}", EMPTY_HORIZON_PLACEHOLDER.dimmed())];
    }

    projection
        .horizon
        .iter()
        .map(|entry| render_horizon_line(entry, layout))
        .collect()
}

fn render_horizon_line(entry: &HorizonCommentary, layout: LayoutHints) -> String {
    let prefix = match entry.kind {
        HorizonCommentaryKind::Risk => "risk",
        HorizonCommentaryKind::Advisory => "advisory",
    };

    format!(
        "  - {prefix}: {}",
        truncate_text(&entry.message, layout.horizon_message_limit)
    )
}

fn epic_hotspots(epic: &EpicTopologyEpic) -> Vec<String> {
    let mut hotspots: Vec<String> = epic
        .show
        .scope_drift
        .iter()
        .filter(|row| row.voyage_id.is_none())
        .map(format_scope_drift_hotspot)
        .collect();

    hotspots.extend(
        epic.show
            .requirement_coverage
            .iter()
            .filter(|row| !row.is_covered())
            .map(|row| {
                format!(
                    "coverage gap: uncovered PRD requirements {}",
                    style::styled_requirement_id(&row.id)
                )
            }),
    );

    hotspots
}

fn voyage_hotspots(voyage: &crate::read_model::topology::VoyageTopologyNode) -> Vec<String> {
    let mut hotspots: Vec<String> = voyage
        .show
        .scope_drift
        .iter()
        .map(format_scope_drift_hotspot)
        .collect();

    hotspots.extend(
        voyage
            .show
            .requirements
            .iter()
            .filter(|row| row.linked_stories.is_empty())
            .map(|row| {
                format!(
                    "coverage gap: uncovered SRS requirements {}",
                    style::styled_requirement_id(&row.id)
                )
            }),
    );

    hotspots
}

fn story_hotspots(story: &StoryTopologyNode) -> Vec<String> {
    let mut hotspots = Vec::new();

    if !story.unmet_dependencies.is_empty() {
        hotspots.push(format!(
            "dependency block: blocked by {}",
            render_story_id_list(&story.unmet_dependencies)
        ));
    } else if !story.dependencies.is_empty() {
        hotspots.push(format!(
            "dependency block: depends on {}",
            render_story_id_list(&story.dependencies)
        ));
    }

    if !story.show.evidence.missing_proofs.is_empty() {
        hotspots.push(format!(
            "verification gap: missing proofs {}",
            story.show.evidence.missing_proofs.join(", ")
        ));
    }

    if story.show.evidence.items.is_empty() {
        hotspots.push("verification gap: no verification coverage".to_string());
    }

    hotspots
}

fn format_scope_drift_hotspot(row: &ScopeDriftRow) -> String {
    format!(
        "scope drift: {}",
        style::styled_inline_markdown(&planning_lineage::format_scope_drift_row(row))
    )
}

fn summarize_annotations(items: Vec<String>, limit: usize) -> Vec<String> {
    if items.len() <= limit {
        return items;
    }

    let total = items.len();
    let mut summary = items.into_iter().take(limit).collect::<Vec<_>>();
    let more = total.saturating_sub(limit);
    summary.push(format!("+{more} more"));
    summary
}

fn summarize_hotspots(items: Vec<String>, limit: usize) -> Vec<String> {
    if items.len() <= limit {
        return items;
    }

    let mut categories = Vec::new();
    for item in &items {
        let category = item
            .split_once(':')
            .map(|(category, _)| category.trim())
            .unwrap_or(item.as_str());
        if !categories.iter().any(|existing| existing == category) {
            categories.push(category.to_string());
        }
    }

    vec![format!(
        "{} hotspot(s): {}",
        items.len(),
        categories.join("; ")
    )]
}

fn render_annotation_suffix(items: &[String]) -> String {
    if items.is_empty() {
        String::new()
    } else {
        format!(" [{}]", items.join(" | "))
    }
}

fn render_story_id_list(ids: &[String]) -> String {
    ids.iter()
        .map(|dependency| style::styled_story_id(dependency))
        .collect::<Vec<_>>()
        .join(", ")
}

fn truncate_text(value: &str, max_chars: usize) -> String {
    let char_count = value.chars().count();
    if char_count <= max_chars {
        return value.to_string();
    }

    let head = value
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    format!("{}…", head)
}
