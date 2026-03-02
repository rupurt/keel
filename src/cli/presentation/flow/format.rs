//! Terminal formatting for flow diagnostics

use owo_colors::OwoColorize;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

pub use super::capacity::EpicCapacityReport;
use super::theme::Theme;

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
    let current_width = crate::infrastructure::utils::visible_width(s);
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
    pub stage: crate::domain::model::StoryState,
    pub index: Option<u32>,
    pub scope: Option<&'a str>,
}

pub fn classify_stories(
    board: &crate::domain::model::Board,
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
        let status = if story.stage == crate::domain::model::StoryState::Done {
            DepStatus::Done
        } else if story.stage == crate::domain::model::StoryState::InProgress {
            DepStatus::InProgress
        } else if story.stage == crate::domain::model::StoryState::Icebox {
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
                        s.id == *dep_id && s.stage == crate::domain::model::StoryState::Done
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
    capacities: &HashMap<String, EpicCapacityReport>,
    theme: &Theme,
) -> String {
    let mut sorted: Vec<_> = capacities.values().collect();
    sorted.sort_by_key(|c| c.id.clone());

    // Calculate max label width for alignment
    let mut max_width = 15; // default minimum
    for cap in &sorted {
        let label_width = cap.id.len() + 1 + cap.title.len();
        if label_width > max_width {
            max_width = label_width;
        }
    }
    max_width += 2; // buffer

    let mut output = String::new();
    writeln!(output, "     {: <w$} CAPACITY", "EPIC", w = max_width).unwrap();

    for cap in sorted {
        let emoji = match cap.charge_state {
            crate::cli::presentation::flow::capacity::ChargeState::Blocked => "🔴",
            crate::cli::presentation::flow::capacity::ChargeState::Discharged => "⚪",
            crate::cli::presentation::flow::capacity::ChargeState::Trickle => "💡",
            crate::cli::presentation::flow::capacity::ChargeState::Charged => "🔋",
            crate::cli::presentation::flow::capacity::ChargeState::Supercharged => "⚡",
            crate::cli::presentation::flow::capacity::ChargeState::Overloaded => "🔥",
        };

        let total = cap.capacity.done
            + cap.capacity.ready
            + cap.capacity.in_flight
            + cap.capacity.blocked
            + cap.capacity.inactive;

        let bar = crate::cli::style::capacity_progress_bar(
            cap.capacity.done,
            cap.capacity.in_flight,
            total,
            15,
            Some(theme),
        );

        let id_styled = crate::cli::style::styled_epic_id(&cap.id);
        let epic_label = format!("{} {}", id_styled, cap.title);
        let epic_padded = pad_to_width(&epic_label, max_width);

        writeln!(
            output,
            "  {} {} {} [D:{:>2} R:{:>2} F:{:>2} B:{:>2} I:{:>2}]",
            emoji,
            epic_padded,
            bar,
            cap.capacity.done,
            cap.capacity.ready,
            cap.capacity.in_flight,
            cap.capacity.blocked,
            cap.capacity.inactive
        )
        .unwrap();
    }

    output
}

type StorySummary = (String, String, DepStatus, Vec<String>);

pub fn render_dependency_chains(
    board: &crate::domain::model::Board,
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
                "{}  {}  {} {} ({})",
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
    use crate::domain::model::StoryState;

    #[test]
    fn test_classify_stories() {
        let stories = vec![
            StoryScopeSummary {
                id: "S1",
                title: "Story 1",
                stage: StoryState::Done,
                index: Some(1),
                scope: Some("epic1"),
            },
            StoryScopeSummary {
                id: "S2",
                title: "Story 2",
                stage: StoryState::InProgress,
                index: Some(2),
                scope: Some("epic1"),
            },
            StoryScopeSummary {
                id: "S3",
                title: "Story 3",
                stage: StoryState::Backlog,
                index: Some(3),
                scope: Some("epic1"),
            },
        ];
        let mut deps = HashMap::new();
        deps.insert("S3".to_string(), vec!["S2".to_string()]);
        let verify_ids = HashSet::new();
        let board = crate::domain::model::Board::default();

        let results = classify_stories(&board, &stories, &deps, &verify_ids);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].2, DepStatus::Done);
        assert_eq!(results[1].2, DepStatus::InProgress);
        assert_eq!(results[2].2, DepStatus::Blocked);
        assert_eq!(results[2].3, vec!["S2".to_string()]);
    }

    #[test]
    fn test_render_epic_capacities() {
        let mut capacities = HashMap::new();
        capacities.insert(
            "epic1".to_string(),
            EpicCapacityReport {
                id: "epic1".to_string(),
                title: "Epic 1".to_string(),
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
        let rendered = render_epic_capacities(&capacities, &theme);
        assert!(rendered.contains("epic1"));
        assert!(rendered.contains("█"));
        assert!(rendered.contains("▒"));
    }

    #[test]
    fn test_render_dependency_chains() {
        let mut board = crate::domain::model::Board::default();
        let story = crate::test_helpers::StoryFactory::new("S1")
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
    }
}
