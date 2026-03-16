//! Show watch command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use keel::domain::model::Board;
use keel::infrastructure::loader::load_board;

/// Show watch details
pub fn run(id: &str) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let watch = board.require_watch(id)?;

    build_document(&board, watch, crate::cli::presentation::terminal::get_terminal_width()).print();

    Ok(())
}

fn build_document(board: &Board, watch: &keel::domain::model::Watch, width: usize) -> ShowDocument {
    let metadata = ShowKeyValues::new()
        .with_min_label_width(11)
        .row("Title:", format!("{}", watch.title().bold()))
        .row("Limit:", format!("{} hours", watch.limit_hours()));

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));

    let mut sections = Vec::new();

    // Visual Section
    let mut visual_section = ShowSection::new("Metaphor");
    // For the watch itself, we just show it at 0 or maybe we can calculate something.
    // For now, 0h.
    let watch_scene = crate::cli::presentation::scene::render_watch(0);
    visual_section.push_lines(watch_scene.lines().map(|s| s.to_string()));
    sections.push(visual_section);

    // Missions Section
    let mut missions_section = ShowSection::new("Constrained Missions");
    let missions = board.missions_for_watch(watch.id());
    if missions.is_empty() {
        missions_section.push_text_block("  (no missions constrained by this watch)");
    } else {
        for mission in missions {
            let elapsed = mission.frontmatter.activated_at
                .map(|a| (chrono::Utc::now().naive_utc() - a).num_hours() as u32)
                .unwrap_or(0);

            missions_section.push_lines([format!(
                "  {} - {} ({}h elapsed)",
                style::styled_story_id(mission.id()),
                mission.title(),
                elapsed
            )]);
        }
    }
    sections.push(missions_section);

    document.push_sections_spaced(sections);
    document
}
