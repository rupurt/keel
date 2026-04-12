//! Terminal formatting for flow diagnostics

use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

pub use super::capacity::{EpicCapacityReport, WatchCapacityReport};
use crate::cli::presentation::theme::Theme;

const COMPLETED_EPIC_RENDER_LIMIT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepStatus {
    Ready,
    InProgress,
    Blocked,
    VerifyBlocked,
    Inactive,
    Done,
}

pub struct VoyageDepSummary {
    pub voyage_id: String,
    pub stories: Vec<(String, String, DepStatus)>, // id, title, status
}

pub fn pad_to_width(s: &str, target_width: usize) -> String {
    let current_width = keel::infrastructure::utils::visible_width(s);
    if current_width >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - current_width))
    }
}

#[derive(Clone)]
pub struct StoryScopeSummary<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub status: keel::domain::model::StoryState,
    pub index: Option<u32>,
    pub scope: Option<&'a str>,
}

pub fn classify_stories(
    board: &keel::domain::model::Board,
    scope_stories: &[StoryScopeSummary],
    deps: &HashMap<String, Vec<String>>,
    verify_ids: &HashSet<&str>,
) -> Vec<StorySummary> {
    let mut results = Vec::new();

    // Sort stories by: Epic index (asc), Voyage index (asc), Story index (asc)
    let mut sorted_stories = scope_stories.to_vec();
    sorted_stories.sort_by(|a, b| {
        // ... (sorting logic remains same)
        // 1. Epic index (asc)
        let story_a = board.stories.get(a.id);
        let story_b = board.stories.get(b.id);

        let epic_a = story_a
            .and_then(|s| s.epic())
            .and_then(|id| board.epics.get(id));
        let epic_b = story_b
            .and_then(|s| s.epic())
            .and_then(|id| board.epics.get(id));
        let epic_idx_a = epic_a.and_then(|e| e.frontmatter.index).unwrap_or(0);
        let epic_idx_b = epic_b.and_then(|e| e.frontmatter.index).unwrap_or(0);

        let epic_cmp = epic_idx_a.cmp(&epic_idx_b);
        if epic_cmp != std::cmp::Ordering::Equal {
            return epic_cmp;
        }

        // 2. Voyage index (asc)
        let voyage_a = story_a
            .and_then(|s| s.voyage())
            .and_then(|id| board.voyages.get(id));
        let voyage_b = story_b
            .and_then(|s| s.voyage())
            .and_then(|id| board.voyages.get(id));
        let voyage_idx_a = voyage_a.and_then(|v| v.frontmatter.index).unwrap_or(0);
        let voyage_idx_b = voyage_b.and_then(|v| v.frontmatter.index).unwrap_or(0);

        let voyage_cmp = voyage_idx_a.cmp(&voyage_idx_b);
        if voyage_cmp != std::cmp::Ordering::Equal {
            return voyage_cmp;
        }

        // 3. Story index (asc)
        let story_idx_a = a.index.unwrap_or(0);
        let story_idx_b = b.index.unwrap_or(0);

        let story_cmp = story_idx_a.cmp(&story_idx_b);
        if story_cmp != std::cmp::Ordering::Equal {
            return story_cmp;
        }

        // Fallback to ID (asc)
        a.id.cmp(b.id)
    });

    for story in sorted_stories {
        let mut blockers = Vec::new();
        let status = if story.status == keel::domain::model::StoryState::Done {
            DepStatus::Done
        } else if story.status == keel::domain::model::StoryState::InProgress {
            DepStatus::InProgress
        } else if story.status == keel::domain::model::StoryState::Icebox {
            DepStatus::Inactive
        } else if verify_ids.contains(story.id) {
            DepStatus::VerifyBlocked
        } else {
            let story_deps = deps.get(story.id).cloned().unwrap_or_default();
            let unmet: Vec<String> = story_deps
                .iter()
                .filter(|dep_id| {
                    // Dependency is unmet if it's not marked as done in the current scope_stories
                    !scope_stories.iter().any(|s| {
                        s.id == *dep_id && s.status == keel::domain::model::StoryState::Done
                    })
                })
                .cloned()
                .collect();

            if !unmet.is_empty() {
                blockers = unmet;
                DepStatus::Blocked
            } else {
                DepStatus::Ready
            }
        };

        results.push((
            story.id.to_string(),
            story.title.to_string(),
            status,
            blockers,
        ));
    }

    results
}

pub fn render_epic_capacities(
    board: &keel::domain::model::Board,
    capacities: &HashMap<String, EpicCapacityReport>,
    theme: &Theme,
) -> String {
    let mut out = String::new();
    let mut draft_epics: Vec<_> = capacities
        .values()
        .filter(|epic| epic.status == keel::domain::model::EpicState::Draft)
        .cloned()
        .collect();
    let mut active_epics: Vec<_> = capacities
        .values()
        .filter(|epic| epic.status == keel::domain::model::EpicState::Active)
        .cloned()
        .collect();
    let mut done_epics: Vec<_> = capacities
        .values()
        .filter(|epic| epic.status == keel::domain::model::EpicState::Done)
        .cloned()
        .collect();

    if draft_epics.is_empty() && active_epics.is_empty() && done_epics.is_empty() {
        return out;
    }

    draft_epics.sort_by_key(|e| e.index);
    active_epics.sort_by_key(|e| e.index);
    done_epics.sort_by_key(|e| e.index);
    done_epics.reverse();

    let all_epics = draft_epics
        .iter()
        .chain(active_epics.iter())
        .chain(done_epics.iter());

    let mut max_width = 15;
    for cap in all_epics {
        let label_width = cap.id.len() + 1 + cap.title.len();
        if label_width > max_width {
            max_width = label_width;
        }
    }
    max_width += 2;
    let status_width = 10;

    let header = format!(
        "     {: <w$} {:<sw$} CAPACITY",
        "EPIC",
        "STATUS",
        w = max_width,
        sw = status_width
    );
    writeln!(out, "{}", header.dimmed()).unwrap();

    // Render Drafts (clipped)
    let draft_len = draft_epics.len();
    if draft_len > COMPLETED_EPIC_RENDER_LIMIT {
        writeln!(
            out,
            "  ... ({} more draft epics)",
            draft_len - COMPLETED_EPIC_RENDER_LIMIT
        )
        .unwrap();
    }
    for epic in draft_epics.iter().take(COMPLETED_EPIC_RENDER_LIMIT) {
        writeln!(
            out,
            "{}",
            render_epic_line(board, epic, max_width, status_width, theme)
        )
        .unwrap();
    }

    // Separator
    if !draft_epics.is_empty() && !active_epics.is_empty() {
        writeln!(out).unwrap();
    }

    // Render Actives (never clipped)
    for epic in &active_epics {
        writeln!(
            out,
            "{}",
            render_epic_line(board, epic, max_width, status_width, theme)
        )
        .unwrap();
    }

    // Separator
    if (!draft_epics.is_empty() || !active_epics.is_empty()) && !done_epics.is_empty() {
        writeln!(out).unwrap();
    }

    // Render Dones (clipped)
    for epic in done_epics.iter().take(COMPLETED_EPIC_RENDER_LIMIT) {
        writeln!(
            out,
            "{}",
            render_epic_line(board, epic, max_width, status_width, theme)
        )
        .unwrap();
    }

    if done_epics.len() > COMPLETED_EPIC_RENDER_LIMIT {
        let ellipsis_width = centered_ellipsis_width(max_width, &header);
        writeln!(out, "{}...", " ".repeat(ellipsis_width)).unwrap();
        writeln!(out).unwrap();
    }

    out
}

pub fn render_watch_capacities(
    board: &keel::domain::model::Board,
    capacities: &[WatchCapacityReport],
    _theme: &Theme,
) -> String {
    let mut out = String::new();
    if capacities.is_empty() {
        return out;
    }

    let mut max_width = 15;
    for cap in capacities {
        let label_width = cap.id.len() + 1 + cap.title.len();
        if label_width > max_width {
            max_width = label_width;
        }
    }
    max_width += 2;
    let status_width = 10;

    let header = format!(
        "     {: <w$} {:<sw$} CAPACITY",
        "WATCH",
        "STATUS",
        w = max_width,
        sw = status_width
    );
    writeln!(out, "{}", header.dimmed()).unwrap();

    for cap in capacities {
        writeln!(
            out,
            "{}",
            render_watch_line(board, cap, max_width, status_width)
        )
        .unwrap();
    }

    out
}

fn render_epic_line(
    board: &keel::domain::model::Board,
    cap: &EpicCapacityReport,
    epic_width: usize,
    status_width: usize,
    _theme: &Theme,
) -> String {
    let emoji = charge_icon(cap.charge_state);

    let epic = board.epics.get(&cap.id).unwrap();
    let status_str = epic.status().to_string();
    let status_styled = {
        let stage = epic.status();
        match stage {
            keel::domain::model::EpicState::Active => {
                if cap.capacity.in_flight > 0 {
                    format!("{}", status_str.green())
                } else {
                    format!("{}", status_str.yellow())
                }
            }
            _ => format!("{}", status_str.dimmed()),
        }
    };

    let id_styled = crate::cli::style::styled_epic_id(&cap.id);
    let epic_label = format!("{} {}", id_styled, cap.title);
    let epic_padded = pad_to_width(&epic_label, epic_width);

    let bar = crate::cli::presentation::progress::render_capacity_bar(
        cap.capacity.done,
        cap.capacity.in_flight,
        cap.capacity.ready,
        15,
        if cap.capacity.in_flight > 0 {
            Some(owo_colors::AnsiColors::Green)
        } else if cap.capacity.ready > 0 {
            Some(owo_colors::AnsiColors::Yellow)
        } else {
            None
        },
    );

    let status_padded = pad_to_width(&status_styled, status_width);

    format!("  {} {} {} {}", emoji, epic_padded, status_padded, bar,)
}

fn render_watch_line(
    board: &keel::domain::model::Board,
    cap: &WatchCapacityReport,
    watch_width: usize,
    status_width: usize,
) -> String {
    let emoji = charge_icon(cap.charge_state);
    let watch = board.watches.get(&cap.id).unwrap();

    let id_styled = crate::cli::style::styled_watch_id(&cap.id);
    let watch_label = format!("{} {}", id_styled, watch.title());
    let watch_padded = pad_to_width(&watch_label, watch_width);
    let status_padded = pad_to_width(&format!("{}", "watch".dimmed()), status_width);

    let bar = crate::cli::presentation::progress::render_capacity_bar(
        cap.capacity.done,
        cap.capacity.in_flight,
        cap.capacity.ready,
        15,
        if cap.capacity.in_flight > 0 {
            Some(owo_colors::AnsiColors::Green)
        } else if cap.capacity.ready > 0 {
            Some(owo_colors::AnsiColors::Yellow)
        } else {
            None
        },
    );

    format!("  {} {} {} {}", emoji, watch_padded, status_padded, bar)
}

fn charge_icon(
    charge_state: crate::cli::presentation::flow::capacity::ChargeState,
) -> &'static str {
    match charge_state {
        crate::cli::presentation::flow::capacity::ChargeState::Blocked => "🔴",
        crate::cli::presentation::flow::capacity::ChargeState::Discharged => "⚪",
        crate::cli::presentation::flow::capacity::ChargeState::Trickle => "💡",
        crate::cli::presentation::flow::capacity::ChargeState::Charged => "🔋",
        crate::cli::presentation::flow::capacity::ChargeState::Supercharged => "⚡",
        crate::cli::presentation::flow::capacity::ChargeState::Overloaded => "🔥",
    }
}

fn centered_ellipsis_width(max_width: usize, header: &str) -> usize {
    let sample_row = format!("  ⚪ {} [D: 0 R: 0 F: 0 B: 0 I: 0]", " ".repeat(max_width));
    let target_width = keel::infrastructure::utils::visible_width(header)
        .max(keel::infrastructure::utils::visible_width(&sample_row));
    target_width.saturating_sub(3) / 2
}

fn visible_epic_capacities<'a>(sorted: &[&'a EpicCapacityReport]) -> Vec<&'a EpicCapacityReport> {
    let completed_total = sorted.iter().filter(|cap| is_completed_epic(cap)).count();
    let completed_to_skip = completed_total.saturating_sub(COMPLETED_EPIC_RENDER_LIMIT);
    let mut skipped_completed = 0;
    let mut visible = Vec::with_capacity(
        sorted.len().min(
            completed_to_skip
                .saturating_add(COMPLETED_EPIC_RENDER_LIMIT)
                .saturating_add(1),
        ),
    );

    for cap in sorted {
        if is_completed_epic(cap) && skipped_completed < completed_to_skip {
            skipped_completed += 1;
            continue;
        }
        visible.push(*cap);
    }

    visible
}

fn clipped_completed_epics(sorted: &[&EpicCapacityReport]) -> bool {
    sorted.iter().filter(|cap| is_completed_epic(cap)).count() > COMPLETED_EPIC_RENDER_LIMIT
}

fn is_completed_epic(cap: &EpicCapacityReport) -> bool {
    cap.capacity.done > 0
        && cap.capacity.ready == 0
        && cap.capacity.in_flight == 0
        && cap.capacity.blocked == 0
        && cap.capacity.inactive == 0
}

type StorySummary = (String, String, DepStatus, Vec<String>);

pub fn render_dependency_chains(
    board: &keel::domain::model::Board,
    summaries: &[StorySummary],
    next_up_ids: &HashSet<String>,
    theme: &Theme,
) -> String {
    let mut output = String::new();

    // Group by scope
    let mut scope_map: HashMap<String, Vec<&StorySummary>> = HashMap::new();
    for summary in summaries {
        let story_id = &summary.0;
        let story = board.stories.get(story_id).unwrap();
        let scope = story.scope().unwrap_or("unscoped").to_string();
        scope_map.entry(scope).or_default().push(summary);
    }

    let mut sorted_scopes: Vec<_> = scope_map.keys().collect();
    sorted_scopes.sort();

    let mut max_id_width = 10;
    for (id, _, _, _) in summaries {
        if id.len() > max_id_width {
            max_id_width = id.len();
        }
    }
    max_id_width += 2;

    for scope in sorted_scopes {
        let stories = scope_map.get(scope).unwrap();
        if scope != "unscoped" {
            writeln!(
                output,
                "    {}",
                crate::cli::style::styled_scope(Some(scope))
            )
            .unwrap();
        } else {
            writeln!(output, "    {}", "unscoped".dimmed()).unwrap();
        }

        for (i, (id, title, status, blockers)) in stories.iter().enumerate() {
            if *status == DepStatus::Done {
                continue;
            }

            let is_last = i == stories.len() - 1;
            let connector = if is_last { "└── " } else { "├── " };

            let prefix = if next_up_ids.contains(id) {
                "→ ".bold().yellow().to_string()
            } else {
                "  ".to_string()
            };

            let status_text = match status {
                DepStatus::Ready => format!("{}ready{}", theme.agent, theme.reset),
                DepStatus::InProgress => format!("{}in-progress{}", theme.human, theme.reset),
                DepStatus::Blocked => {
                    let blocker_ids: Vec<_> = blockers
                        .iter()
                        .map(|b| crate::cli::style::styled_id(b))
                        .collect();
                    format!(
                        "{}blocked by {}{}",
                        theme.warning,
                        blocker_ids.join(", "),
                        theme.reset
                    )
                }
                DepStatus::VerifyBlocked => {
                    format!("{}verify-blocked{}", theme.warning, theme.reset)
                }
                DepStatus::Inactive => format!("{}inactive{}", theme.muted, theme.reset),
                DepStatus::Done => format!("{}done{}", theme.muted, theme.reset),
            };

            let id_styled = crate::cli::style::styled_id(id);
            let id_padded = pad_to_width(&id_styled, max_id_width);

            writeln!(
                output,
                "{}  {}{} {} ({})",
                prefix, connector, id_padded, title, status_text
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    output
}

pub struct QueueItemDisplay {
    pub label: String,
    pub count: usize,
    pub age_days: Option<usize>,
    pub secondary_count: Option<usize>,
}

impl QueueItemDisplay {
    pub fn from_item(item: crate::cli::presentation::flow::bottleneck::QueueItem) -> Self {
        Self {
            label: item.label,
            count: item.count,
            age_days: item.age_days,
            secondary_count: item.secondary_count,
        }
    }

    pub fn render_to_string(&self, width: usize, theme: &Theme) -> String {
        let mut label_part = format!("  {:<15} {:>3} ", self.label, self.count);
        if let Some(age) = self.age_days {
            label_part.push_str(&format!("({}d) ", age));
        }

        // Volume bar: 1 block per item, max 10
        let bar_width = 10;
        let filled = self.count.min(bar_width);
        let empty = bar_width - filled;

        let bar = if let Some(secondary) = self.secondary_count {
            // Agent queue: secondary is InProgress (▒), rest is Backlog (█)
            let in_progress = secondary.min(filled);
            let backlog = filled - in_progress;
            format!(
                "{}{}{}{}{}",
                theme.agent,
                "█".repeat(backlog),
                "▒".repeat(in_progress),
                theme.muted,
                "░".repeat(empty)
            )
        } else {
            // Human queue: use human color
            format!(
                "{}{}{}{}",
                theme.human,
                "█".repeat(filled),
                theme.muted,
                "░".repeat(empty)
            )
        };

        let res = format!("{}{}{}", label_part, bar, theme.reset);
        pad_to_width(&res, width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::StoryState;

    #[test]
    fn test_classify_stories() {
        let stories = vec![
            StoryScopeSummary {
                id: "S1",
                title: "Story 1",
                status: StoryState::Done,
                index: Some(1),
                scope: Some("epic1"),
            },
            StoryScopeSummary {
                id: "S2",
                title: "Story 2",
                status: StoryState::InProgress,
                index: Some(2),
                scope: Some("epic1"),
            },
            StoryScopeSummary {
                id: "S3",
                title: "Story 3",
                status: StoryState::Backlog,
                index: Some(3),
                scope: Some("epic1"),
            },
        ];
        let mut deps = HashMap::new();
        deps.insert("S3".to_string(), vec!["S2".to_string()]);
        let verify_ids = HashSet::new();
        let board = keel::domain::model::Board::default();

        let results = classify_stories(&board, &stories, &deps, &verify_ids);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].2, DepStatus::Done);
        assert_eq!(results[1].2, DepStatus::InProgress);
        assert_eq!(results[2].2, DepStatus::Blocked);
        assert_eq!(results[2].3, vec!["S2".to_string()]);
    }

    fn build_epic(id: &str, status: keel::domain::model::EpicState) -> keel::domain::model::Epic {
        keel::domain::model::Epic {
            frontmatter: keel::domain::model::EpicFrontmatter {
                id: id.to_string(),
                title: id.to_string(),
                description: None,
                bearing: None,
                mission: None,
                index: None,
                created_at: Some(chrono::Utc::now().naive_utc()),
            },
            path: std::path::PathBuf::from(format!("epics/{}/README.md", id)),
            status,
        }
    }

    #[test]
    fn test_render_epic_capacities() {
        let mut board = keel::domain::model::Board::default();
        board.epics.insert(
            "epic1".to_string(),
            build_epic("epic1", keel::domain::model::EpicState::Active),
        );
        let mut capacities = HashMap::new();
        capacities.insert(
            "epic1".to_string(),
            EpicCapacityReport {
                index: Some(1),
                id: "epic1".to_string(),
                title: "Epic 1".to_string(),
                status: keel::domain::model::EpicState::Active,
                charge_state: crate::cli::presentation::flow::capacity::ChargeState::Charged,
                capacity: crate::cli::presentation::flow::capacity::EpicCapacity {
                    ready: 1,
                    in_flight: 1,
                    blocked: 0,
                    inactive: 0,
                    done: 1,
                },
            },
        );
        let theme = Theme::default();
        let rendered = render_epic_capacities(&board, &capacities, &theme);
        assert!(rendered.contains("epic1"));
        assert!(rendered.contains("active"));
        assert!(rendered.contains("▓"));
        assert!(rendered.contains("▒"));
        assert!(rendered.contains("░"));
    }

    #[test]
    fn render_epic_capacities_aligns_capacity_bar_right_edges() {
        let theme = Theme::default();
        let capacities = HashMap::from([
            (
                "epic1".to_string(),
                EpicCapacityReport {
                    index: Some(1),
                    id: "epic1".to_string(),
                    title: "Split Work".to_string(),
                    status: keel::domain::model::EpicState::Active,
                    charge_state: crate::cli::presentation::flow::capacity::ChargeState::Blocked,
                    capacity: crate::cli::presentation::flow::capacity::EpicCapacity {
                        ready: 0,
                        in_flight: 1,
                        blocked: 0,
                        inactive: 0,
                        done: 1,
                    },
                },
            ),
            (
                "epic2".to_string(),
                EpicCapacityReport {
                    index: Some(2),
                    id: "epic2".to_string(),
                    title: "Completed Work".to_string(),
                    status: keel::domain::model::EpicState::Done,
                    charge_state: crate::cli::presentation::flow::capacity::ChargeState::Discharged,
                    capacity: crate::cli::presentation::flow::capacity::EpicCapacity {
                        ready: 0,
                        in_flight: 0,
                        blocked: 0,
                        inactive: 0,
                        done: 2,
                    },
                },
            ),
        ]);

        let mut board = keel::domain::model::Board::default();
        board.epics.insert(
            "epic1".to_string(),
            build_epic("epic1", keel::domain::model::EpicState::Active),
        );
        board.epics.insert(
            "epic2".to_string(),
            build_epic("epic2", keel::domain::model::EpicState::Done),
        );

        let rendered = render_epic_capacities(&board, &capacities, &theme);
        let bracket_columns: Vec<_> = rendered
            .lines()
            .filter_map(|line| {
                let plain = ansi_escape_sequences::strip_ansi(line);
                plain.rfind(']').map(|right_edge| {
                    keel::infrastructure::utils::visible_width(&plain[..=right_edge])
                })
            })
            .collect();

        assert_eq!(bracket_columns.len(), 2);
        assert!(bracket_columns.windows(2).all(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn render_epic_capacities_clips_completed_epics_to_last_three() {
        let theme = Theme::default();
        let capacities = HashMap::from([
            (
                "epic1".to_string(),
                epic_capacity("epic1", "Epic 1", 2, 0, 0, 0, 0),
            ),
            (
                "epic2".to_string(),
                epic_capacity("epic2", "Epic 2", 4, 0, 0, 0, 0),
            ),
            (
                "epic3".to_string(),
                epic_capacity("epic3", "Epic 3", 6, 0, 0, 0, 0),
            ),
            (
                "epic4".to_string(),
                epic_capacity("epic4", "Epic 4", 8, 0, 0, 0, 0),
            ),
            (
                "epic5".to_string(),
                epic_capacity("epic5", "Epic 5", 10, 0, 0, 0, 0),
            ),
        ]);

        let mut board = keel::domain::model::Board::default();
        board.epics.insert(
            "epic1".to_string(),
            build_epic("epic1", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic2".to_string(),
            build_epic("epic2", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic3".to_string(),
            build_epic("epic3", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic4".to_string(),
            build_epic("epic4", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic5".to_string(),
            build_epic("epic5", keel::domain::model::EpicState::Done),
        );
        let rendered = render_epic_capacities(&board, &capacities, &theme);

        assert!(!rendered.contains("epic1"));
        assert!(!rendered.contains("epic2"));
        assert!(rendered.contains("epic3"));
        assert!(rendered.contains("epic4"));
        assert!(rendered.contains("epic5"));
        let lines: Vec<_> = rendered.lines().collect();
        let ellipsis_index = lines
            .iter()
            .position(|line| line.trim() == "...")
            .expect("expected ellipsis line");
        let ellipsis_line = lines[ellipsis_index];
        let leading_spaces = ellipsis_line.chars().take_while(|ch| *ch == ' ').count();
        assert!(
            leading_spaces > 4,
            "expected centered ellipsis, got: {ellipsis_line:?}"
        );
        assert_eq!(
            lines.get(ellipsis_index + 1).copied(),
            Some(""),
            "expected blank spacer below ellipsis"
        );
    }

    #[test]
    fn render_epic_capacities_keeps_incomplete_epics_even_when_completed_are_clipped() {
        let theme = Theme::default();
        let capacities = HashMap::from([
            (
                "epic1".to_string(),
                epic_capacity("epic1", "Epic 1", 2, 0, 0, 0, 0),
            ),
            (
                "epic2".to_string(),
                epic_capacity("epic2", "Epic 2", 4, 0, 0, 0, 0),
            ),
            (
                "epic3".to_string(),
                epic_capacity("epic3", "Epic 3", 6, 0, 0, 0, 0),
            ),
            (
                "epic4".to_string(),
                epic_capacity("epic4", "Epic 4", 8, 0, 0, 0, 0),
            ),
            (
                "epic5".to_string(),
                epic_capacity("epic5", "Epic 5", 10, 0, 0, 0, 0),
            ),
            (
                "epic6".to_string(),
                epic_capacity("epic6", "Epic 6", 0, 1, 0, 0, 0),
            ),
            (
                "epic7".to_string(),
                epic_capacity("epic7", "Epic 7", 0, 0, 0, 0, 0),
            ),
            (
                "epic8".to_string(),
                epic_capacity("epic8", "Epic 8", 1, 0, 0, 1, 0),
            ),
        ]);

        let mut board = keel::domain::model::Board::default();
        board.epics.insert(
            "epic1".to_string(),
            build_epic("epic1", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic2".to_string(),
            build_epic("epic2", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic3".to_string(),
            build_epic("epic3", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic4".to_string(),
            build_epic("epic4", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic5".to_string(),
            build_epic("epic5", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic6".to_string(),
            build_epic("epic6", keel::domain::model::EpicState::Active),
        );
        board.epics.insert(
            "epic7".to_string(),
            build_epic("epic7", keel::domain::model::EpicState::Active),
        );
        board.epics.insert(
            "epic8".to_string(),
            build_epic("epic8", keel::domain::model::EpicState::Active),
        );
        let rendered = render_epic_capacities(&board, &capacities, &theme);

        assert!(!rendered.contains("epic1"));
        assert!(!rendered.contains("epic2"));
        assert!(rendered.contains("epic3"));
        assert!(rendered.contains("epic4"));
        assert!(rendered.contains("epic5"));
        assert!(rendered.contains("epic6"));
        assert!(rendered.contains("epic7"));
        assert!(rendered.contains("epic8"));
    }

    #[test]
    fn render_epic_capacities_skips_ellipsis_when_completed_epics_fit() {
        let theme = Theme::default();
        let capacities = HashMap::from([
            (
                "epic1".to_string(),
                epic_capacity("epic1", "Epic 1", 2, 0, 0, 0, 0),
            ),
            (
                "epic2".to_string(),
                epic_capacity("epic2", "Epic 2", 4, 0, 0, 0, 0),
            ),
            (
                "epic3".to_string(),
                epic_capacity("epic3", "Epic 3", 6, 0, 0, 0, 0),
            ),
            (
                "epic4".to_string(),
                epic_capacity("epic4", "Epic 4", 0, 1, 0, 0, 0),
            ),
        ]);

        let mut board = keel::domain::model::Board::default();
        board.epics.insert(
            "epic1".to_string(),
            build_epic("epic1", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic2".to_string(),
            build_epic("epic2", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic3".to_string(),
            build_epic("epic3", keel::domain::model::EpicState::Done),
        );
        board.epics.insert(
            "epic4".to_string(),
            build_epic("epic4", keel::domain::model::EpicState::Active),
        );
        let rendered = render_epic_capacities(&board, &capacities, &theme);

        assert!(rendered.contains("epic1"));
        assert!(rendered.contains("epic2"));
        assert!(rendered.contains("epic3"));
        assert!(rendered.contains("epic4"));
        assert!(rendered.lines().all(|line| line.trim() != "..."));
    }

    #[test]
    fn render_watch_capacities_shows_watch_pressure() {
        let theme = Theme::default();
        let mut board = keel::domain::model::Board::default();
        board.watches.insert(
            "W1".to_string(),
            keel::domain::model::Watch::new(
                keel::domain::model::WatchFrontmatter {
                    id: "W1".to_string(),
                    title: "Standard Operations".to_string(),
                    limit_hours: 12,
                },
                std::path::PathBuf::from("watches/W1/README.md"),
            ),
        );

        let rendered = render_watch_capacities(
            &board,
            &[WatchCapacityReport {
                id: "W1".to_string(),
                title: "Standard Operations".to_string(),
                charge_state: crate::cli::presentation::flow::capacity::ChargeState::Charged,
                capacity: crate::cli::presentation::flow::capacity::EpicCapacity {
                    ready: 3,
                    in_flight: 0,
                    blocked: 0,
                    inactive: 0,
                    done: 0,
                },
            }],
            &theme,
        );

        assert!(rendered.contains("WATCH"));
        assert!(rendered.contains("W1"));
        assert!(rendered.contains("Standard Operations"));
        assert!(rendered.contains("watch"));
        assert!(rendered.contains("["));
        assert!(rendered.contains("]"));
    }

    fn epic_capacity(
        id: &str,
        title: &str,
        done: usize,
        ready: usize,
        in_flight: usize,
        blocked: usize,
        inactive: usize,
    ) -> EpicCapacityReport {
        let charge_state = crate::cli::presentation::flow::capacity::ChargeState::Discharged;
        let index = id.replace("epic", "").parse::<u32>().ok();

        let status = if done > 0 && ready == 0 && in_flight == 0 && blocked == 0 && inactive == 0 {
            keel::domain::model::EpicState::Done
        } else if ready == 0 && in_flight == 0 && blocked == 0 && inactive == 0 && done == 0 {
            keel::domain::model::EpicState::Draft
        } else {
            keel::domain::model::EpicState::Active
        };

        EpicCapacityReport {
            index,
            id: id.to_string(),
            title: title.to_string(),
            status,
            charge_state,
            capacity: crate::cli::presentation::flow::capacity::EpicCapacity {
                ready,
                in_flight,
                blocked,
                inactive,
                done,
            },
        }
    }

    #[test]
    fn test_render_dependency_chains() {
        let mut board = keel::domain::model::Board::default();
        let story = keel::test_helpers::StoryFactory::new("S1")
            .title("Story 1")
            .build();
        board.stories.insert("S1".to_string(), story);

        let summaries = vec![(
            "S1".to_string(),
            "Story 1".to_string(),
            DepStatus::Ready,
            vec![],
        )];
        let mut next_up_ids = HashSet::new();
        next_up_ids.insert("S1".to_string());
        let theme = Theme::default();
        let rendered = render_dependency_chains(&board, &summaries, &next_up_ids, &theme);
        assert!(rendered.contains("→"));
        assert!(rendered.contains("S1"));
        assert!(rendered.contains("ready"));
        assert!(!rendered.contains("└──  S1"));
    }
}
