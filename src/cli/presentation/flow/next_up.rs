//! NEXT UP detail section for flow dashboard
//!
//! Shows priority items for both human and agent queues below the flow boxes.

use crate::domain::model::{Bearing, BearingStatus, Board, Story, StoryState, Voyage, VoyageState};

/// Item to display in the NEXT UP section
#[derive(Debug, Clone)]
pub struct NextUpItem {
    /// Item ID (story or voyage ID)
    pub id: String,
    /// Display title
    pub title: String,
    /// Category label (e.g., "accept", "start", "plan", "decompose", "backlog", "wip")
    pub category: String,
    /// Suggested next command (optional)
    pub command: Option<String>,
}

/// NEXT UP section data for both queues
#[derive(Debug, Clone)]
pub struct NextUpSection {
    /// Human queue next items (ordered by priority)
    pub human_items: Vec<NextUpItem>,
    /// Agent queue next items (ordered by priority)
    pub agent_items: Vec<NextUpItem>,
}

/// Calculate NEXT UP items from board state
pub fn calculate_next_up(board: &Board) -> NextUpSection {
    let human_items = build_human_next_up(board);
    let agent_items = build_agent_next_up(board);

    NextUpSection {
        human_items,
        agent_items,
    }
}

/// Extract epic name from scope string (e.g., "role-based-cli/voyage-1" -> "role-based-cli")
fn epic_from_scope(scope: &Option<String>) -> Option<String> {
    scope
        .as_ref()
        .and_then(|s| s.split('/').next())
        .map(|e| e.to_string())
}

/// Find the most recently active epic based on story submissions
/// Returns epics with recent activity, prioritized by recency then by draft voyages needing work
#[allow(clippy::collapsible_if)]
fn find_most_recent_epic(board: &Board, draft_voyages: &[&Voyage]) -> Option<String> {
    // Collect all epics with recent activity and their most recent timestamp
    let mut epic_activity: std::collections::HashMap<String, chrono::NaiveDateTime> =
        std::collections::HashMap::new();

    for story in board.stories.values() {
        if story.stage == StoryState::NeedsHumanVerification {
            if let Some(submitted) = story.frontmatter.submitted_at {
                if let Some(epic) = epic_from_scope(&story.frontmatter.scope) {
                    epic_activity
                        .entry(epic)
                        .and_modify(|t| {
                            if submitted > *t {
                                *t = submitted
                            }
                        })
                        .or_insert(submitted);
                }
            }
        }
    }

    // Also check in-progress stories (current work)
    for story in board.stories.values() {
        if story.stage == StoryState::InProgress {
            if let Some(updated) = story.frontmatter.updated_at {
                if let Some(epic) = epic_from_scope(&story.frontmatter.scope) {
                    epic_activity
                        .entry(epic)
                        .and_modify(|t| {
                            if updated > *t {
                                *t = updated
                            }
                        })
                        .or_insert(updated);
                }
            }
        }
    }

    // Find the maximum timestamp
    let max_time = epic_activity.values().max().copied();

    // Get all epics with the most recent timestamp (tiebreaker needed)
    let most_recent_epics: Vec<_> = epic_activity
        .iter()
        .filter(|(_, t)| Some(**t) == max_time)
        .map(|(e, _)| e.clone())
        .collect();

    if most_recent_epics.is_empty() {
        return None;
    }

    if most_recent_epics.len() == 1 {
        return Some(most_recent_epics[0].clone());
    }

    // Tiebreaker 1: prefer epics that have draft voyages needing decomposition
    let epics_with_draft_voyages: Vec<_> = most_recent_epics
        .iter()
        .filter(|epic| draft_voyages.iter().any(|v| &v.epic_id == *epic))
        .cloned()
        .collect();

    let candidates = if epics_with_draft_voyages.is_empty() {
        most_recent_epics
    } else {
        epics_with_draft_voyages
    };

    if candidates.len() == 1 {
        return Some(candidates[0].clone());
    }

    // Tiebreaker 2: prefer epic with MORE stories awaiting verification (more momentum)
    let mut epic_story_counts: Vec<(String, usize)> = candidates
        .iter()
        .map(|epic| {
            let count = board
                .stories
                .values()
                .filter(|s| s.stage == StoryState::NeedsHumanVerification)
                .filter(|s| epic_from_scope(&s.frontmatter.scope).as_ref() == Some(epic))
                .count();
            (epic.clone(), count)
        })
        .collect();

    // Sort by count descending, then alphabetically for final tiebreaker
    epic_story_counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    epic_story_counts.first().map(|(epic, _)| epic.clone())
}

/// Find the single most recently active epic for prioritization
/// Only includes the epic with the most recent submission, not all active epics
#[allow(clippy::collapsible_if)]
fn find_active_epics(
    board: &Board,
    draft_voyages: &[&Voyage],
) -> std::collections::HashSet<String> {
    let mut active = std::collections::HashSet::new();

    // Only the MOST recently active epic gets priority
    // This ensures we focus on the epic with the latest work, not just any active epic
    if let Some(epic) = find_most_recent_epic(board, draft_voyages) {
        active.insert(epic);
    }

    active
}

fn build_human_next_up(board: &Board) -> Vec<NextUpItem> {
    let mut items = Vec::new();

    // Collect draft voyages for momentum calculations
    let draft_voyages: Vec<&Voyage> = board
        .voyages
        .values()
        .filter(|v| v.status() == VoyageState::Draft)
        .collect();

    // Identify epics with recent activity for momentum-based prioritization
    let active_epics = find_active_epics(board, &draft_voyages);

    // 1. Next story to accept (needs-human-verification, oldest first)
    let mut to_accept: Vec<&Story> = board
        .stories
        .values()
        .filter(|s| s.stage == StoryState::NeedsHumanVerification)
        .collect();
    to_accept.sort_by_key(|s| s.frontmatter.submitted_at);
    if let Some(story) = to_accept.first() {
        items.push(NextUpItem {
            id: story.id().to_string(),
            title: story.title().to_string(),
            category: "accept".to_string(),
            command: Some(format!("keel story accept {}", story.id().to_uppercase())),
        });
    }

    // 2. Next voyage to start (planned status, prefer active epics)
    let mut to_start: Vec<&Voyage> = board
        .voyages
        .values()
        .filter(|v| v.status() == VoyageState::Planned)
        .collect();
    to_start.sort_by(|a, b| {
        let a_active = active_epics.contains(&a.epic_id);
        let b_active = active_epics.contains(&b.epic_id);
        // Active epics first, then by ID for determinism
        b_active.cmp(&a_active).then_with(|| a.id().cmp(b.id()))
    });
    if let Some(voyage) = to_start.first() {
        items.push(NextUpItem {
            id: voyage.id().to_string(),
            title: voyage.title().to_string(),
            category: "start".to_string(),
            command: Some(format!("keel voyage start {}", voyage.id())),
        });
    }

    // 3. Next item to decompose: epics needing voyages first, then draft voyages
    // Check for epics with 0 voyages first (higher priority)
    let mut epics_needing_voyages: Vec<_> = board
        .epics
        .values()
        .filter(|epic| {
            // Count voyages for this epic
            let voyage_count = board
                .voyages
                .values()
                .filter(|v| v.frontmatter.epic.as_deref() == Some(epic.id()))
                .count();
            voyage_count == 0
        })
        .collect();
    epics_needing_voyages.sort_by_key(|e| e.id());

    if let Some(epic) = epics_needing_voyages.first() {
        items.push(NextUpItem {
            id: epic.id().to_string(),
            title: epic.title().to_string(),
            category: "decompose".to_string(),
            command: Some(format!("keel voyage new \"<name>\" --epic {}", epic.id())),
        });
    } else {
        // Fall back to draft voyages needing stories, preferring active epics
        let mut to_decompose: Vec<&Voyage> = board
            .voyages
            .values()
            .filter(|v| v.status() == VoyageState::Draft)
            .collect();
        to_decompose.sort_by(|a, b| {
            let a_active = active_epics.contains(&a.epic_id);
            let b_active = active_epics.contains(&b.epic_id);
            // Active epics first, then by ID for determinism
            b_active.cmp(&a_active).then_with(|| a.id().cmp(b.id()))
        });
        if let Some(voyage) = to_decompose.first() {
            items.push(NextUpItem {
                id: voyage.id().to_string(),
                title: voyage.title().to_string(),
                category: "plan".to_string(),
                command: Some(format!("keel voyage plan {}", voyage.id())),
            });
        }
    }

    // 4. Next bearing needing attention (research flashlight)
    if let Some((bearing, action)) = find_next_bearing_action(board) {
        let command = match action.as_str() {
            "lay" => format!("keel bearing lay {}", bearing.id()),
            "survey" => format!("keel bearing survey {}", bearing.id()),
            "assess" => format!("keel bearing assess {}", bearing.id()),
            _ => format!("keel bearing show {}", bearing.id()),
        };
        items.push(NextUpItem {
            id: bearing.id().to_string(),
            title: bearing.title().to_string(),
            category: action,
            command: Some(command),
        });
    }

    items
}

/// Find the next bearing that needs action, returning (bearing, action_category)
fn find_next_bearing_action(board: &Board) -> Option<(&Bearing, String)> {
    let mut active_bearings: Vec<&Bearing> = board
        .bearings
        .values()
        .filter(|b| {
            matches!(
                b.status(),
                BearingStatus::Exploring | BearingStatus::Evaluating | BearingStatus::Ready
            )
        })
        .collect();

    // Sort by priority: most actionable first (Ready > Evaluating > Exploring),
    // then by progress (more artifacts = clearer next step), then alphabetically.
    active_bearings.sort_by(|a, b| a.priority_key().cmp(&b.priority_key()));

    // Get the first bearing needing attention
    if let Some(bearing) = active_bearings.first() {
        // Determine what action is needed
        let action = if bearing.status() == BearingStatus::Ready {
            "lay".to_string()
        } else if !bearing.has_survey {
            "survey".to_string()
        } else if !bearing.has_assessment {
            "assess".to_string()
        } else {
            // Has survey and assessment but still exploring - might need review
            "review".to_string()
        };

        return Some((*bearing, action));
    }

    None
}

fn build_agent_next_up(board: &Board) -> Vec<NextUpItem> {
    let mut items = Vec::new();

    // Get stories in progress to determine current epic(s)
    let in_progress: Vec<&Story> = board
        .stories
        .values()
        .filter(|s| s.stage == StoryState::InProgress)
        .collect();

    // Extract epic names from WIP stories (epic = first segment of scope)
    let wip_epics: std::collections::HashSet<String> = in_progress
        .iter()
        .filter_map(|s| s.frontmatter.scope.as_ref())
        .filter_map(|scope| scope.split('/').next())
        .map(|s| s.to_string())
        .collect();

    // 1. Next unstarted story (backlog), preferring same epic as WIP
    let mut backlog: Vec<&Story> = board
        .stories
        .values()
        .filter(|s| s.stage == StoryState::Backlog)
        .collect();

    // Sort with epic momentum: same-epic first, then by index
    backlog.sort_by(|a, b| {
        let a_epic = a
            .frontmatter
            .scope
            .as_ref()
            .and_then(|s| s.split('/').next());
        let b_epic = b
            .frontmatter
            .scope
            .as_ref()
            .and_then(|s| s.split('/').next());

        let a_same_epic = a_epic.map(|e| wip_epics.contains(e)).unwrap_or(false);
        let b_same_epic = b_epic.map(|e| wip_epics.contains(e)).unwrap_or(false);

        // Same-epic stories come first (true > false when reversed)
        b_same_epic
            .cmp(&a_same_epic)
            .then_with(|| a.index().cmp(&b.index()))
            .then_with(|| a.id().cmp(b.id()))
    });

    if let Some(story) = backlog.first() {
        items.push(NextUpItem {
            id: story.id().to_string(),
            title: story.title().to_string(),
            category: "backlog".to_string(),
            command: Some(format!("keel story start {}", story.id().to_uppercase())),
        });
    }

    // 2. All stories currently in progress (sorted by index)
    let mut in_progress_sorted = in_progress;
    in_progress_sorted.sort_by_key(|s| (s.index(), s.id()));
    for story in in_progress_sorted {
        items.push(NextUpItem {
            id: story.id().to_string(),
            title: story.title().to_string(),
            category: "wip".to_string(),
            command: None,
        });
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{
        BearingFrontmatter, StoryFrontmatter, StoryState, StoryType, VoyageFrontmatter,
    };
    use std::path::PathBuf;

    fn make_story(id: &str, title: &str, stage: StoryState, index: Option<u32>) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: id.to_string(),
                title: title.to_string(),
                story_type: StoryType::Feat,
                status: stage,
                scope: None,
                milestone: None,
                created_at: None,
                updated_at: None,
                completed_at: None,
                submitted_at: None,
                index,
                governed_by: vec![],
                role: None,
            },
            path: PathBuf::from(format!("{}.md", id)),
            stage,
        }
    }

    fn make_voyage(id: &str, title: &str, status: VoyageState) -> Voyage {
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
                completed_at: None,
            },
            path: PathBuf::from(format!("{}/README.md", id)),
            epic_id: "test-epic".to_string(),
        }
    }

    #[test]
    fn next_up_includes_story_to_accept() {
        let mut board = Board::new(PathBuf::from("test"));
        let story = make_story(
            "s1",
            "Test Story",
            StoryState::NeedsHumanVerification,
            Some(1),
        );
        board.stories.insert("s1".to_string(), story);

        let next_up = calculate_next_up(&board);

        assert!(
            next_up
                .human_items
                .iter()
                .any(|i| i.category == "accept" && i.id == "s1")
        );
    }

    #[test]
    fn next_up_includes_voyage_to_start() {
        let mut board = Board::new(PathBuf::from("test"));
        let voyage = make_voyage("v1", "Test Voyage", VoyageState::Planned);
        board.voyages.insert("v1".to_string(), voyage);

        let next_up = calculate_next_up(&board);

        assert!(
            next_up
                .human_items
                .iter()
                .any(|i| i.category == "start" && i.id == "v1")
        );
    }

    #[test]
    fn next_up_includes_voyage_to_decompose() {
        let mut board = Board::new(PathBuf::from("test"));
        let voyage = make_voyage("v2", "Draft Voyage", VoyageState::Draft);
        board.voyages.insert("v2".to_string(), voyage);

        let next_up = calculate_next_up(&board);

        assert!(
            next_up
                .human_items
                .iter()
                .any(|i| i.category == "plan" && i.id == "v2")
        );
    }

    #[test]
    fn next_up_includes_backlog_story() {
        let mut board = Board::new(PathBuf::from("test"));
        let story = make_story("s2", "Backlog Story", StoryState::Backlog, Some(1));
        board.stories.insert("s2".to_string(), story);

        let next_up = calculate_next_up(&board);

        assert!(
            next_up
                .agent_items
                .iter()
                .any(|i| i.category == "backlog" && i.id == "s2")
        );
    }

    #[test]
    fn next_up_includes_wip_stories() {
        let mut board = Board::new(PathBuf::from("test"));
        let story1 = make_story("s3", "WIP Story 1", StoryState::InProgress, Some(1));
        let story2 = make_story("s4", "WIP Story 2", StoryState::InProgress, Some(2));
        board.stories.insert("s3".to_string(), story1);
        board.stories.insert("s4".to_string(), story2);

        let next_up = calculate_next_up(&board);

        let wip_items: Vec<_> = next_up
            .agent_items
            .iter()
            .filter(|i| i.category == "wip")
            .collect();
        assert_eq!(wip_items.len(), 2);
    }

    #[test]
    fn human_items_ordered_by_priority() {
        let mut board = Board::new(PathBuf::from("test"));
        // Add items in reverse order to test sorting
        board.stories.insert(
            "s1".to_string(),
            make_story("s1", "Accept", StoryState::NeedsHumanVerification, Some(1)),
        );
        board.voyages.insert(
            "v1".to_string(),
            make_voyage("v1", "Start", VoyageState::Planned),
        );
        board.voyages.insert(
            "v2".to_string(),
            make_voyage("v2", "Decompose", VoyageState::Draft),
        );

        let next_up = calculate_next_up(&board);

        // Should be: accept, start, plan
        assert_eq!(next_up.human_items[0].category, "accept");
        assert_eq!(next_up.human_items[1].category, "start");
        assert_eq!(next_up.human_items[2].category, "plan");
    }

    fn make_bearing(id: &str, title: &str, has_survey: bool, has_assessment: bool) -> Bearing {
        Bearing {
            frontmatter: BearingFrontmatter {
                id: id.to_string(),
                title: title.to_string(),
                status: BearingStatus::Exploring,
                index: None,
                decline_reason: None,
                created_at: None,
                laid_at: None,
            },
            path: PathBuf::from(format!("bearings/{}/BRIEF.md", id)),
            has_survey,
            has_assessment,
        }
    }

    #[test]
    fn next_up_includes_bearing_needing_survey() {
        let mut board = Board::new(PathBuf::from("test"));
        let bearing = make_bearing("test-bearing", "Test Bearing", false, false);
        board.bearings.insert("test-bearing".to_string(), bearing);

        let next_up = calculate_next_up(&board);

        assert!(
            next_up
                .human_items
                .iter()
                .any(|i| i.category == "survey" && i.id == "test-bearing")
        );
    }

    #[test]
    fn next_up_includes_bearing_needing_assessment() {
        let mut board = Board::new(PathBuf::from("test"));
        let bearing = make_bearing("test-bearing", "Test Bearing", true, false);
        board.bearings.insert("test-bearing".to_string(), bearing);

        let next_up = calculate_next_up(&board);

        assert!(
            next_up
                .human_items
                .iter()
                .any(|i| i.category == "assess" && i.id == "test-bearing")
        );
    }

    fn make_bearing_with_status(
        id: &str,
        status: BearingStatus,
        has_survey: bool,
        has_assessment: bool,
    ) -> Bearing {
        Bearing {
            frontmatter: BearingFrontmatter {
                id: id.to_string(),
                title: id.to_string(),
                status,
                index: None,
                decline_reason: None,
                created_at: None,
                laid_at: None,
            },
            path: PathBuf::from(format!("bearings/{}/BRIEF.md", id)),
            has_survey,
            has_assessment,
        }
    }

    #[test]
    fn next_up_selects_bearing_by_priority_not_alphabetically() {
        let mut board = Board::new(PathBuf::from("test"));

        // "alpha" is alphabetically first but is Exploring (least mature)
        board.bearings.insert(
            "alpha".to_string(),
            make_bearing_with_status("alpha", BearingStatus::Exploring, false, false),
        );
        // "zulu" is alphabetically last but is Evaluating with a survey (most mature)
        board.bearings.insert(
            "zulu".to_string(),
            make_bearing_with_status("zulu", BearingStatus::Evaluating, true, false),
        );

        let next_up = calculate_next_up(&board);

        // The first bearing item should be zulu (higher priority), not alpha
        let bearing_item = next_up
            .human_items
            .iter()
            .find(|i| i.id == "zulu" || i.id == "alpha")
            .expect("should have a bearing item");
        assert_eq!(
            bearing_item.id, "zulu",
            "Should select evaluating bearing over alphabetically-first exploring bearing"
        );
    }

    // Epic momentum tests

    fn make_story_with_scope(
        id: &str,
        title: &str,
        stage: StoryState,
        index: Option<u32>,
        scope: &str,
    ) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: id.to_string(),
                title: title.to_string(),
                story_type: StoryType::Feat,
                status: stage,
                scope: Some(scope.to_string()),
                milestone: None,
                created_at: None,
                updated_at: None,
                completed_at: None,
                submitted_at: None,
                index,
                governed_by: vec![],
                role: None,
            },
            path: PathBuf::from(format!("{}.md", id)),
            stage,
        }
    }

    #[test]
    fn next_backlog_prefers_same_epic_as_wip() {
        let mut board = Board::new(PathBuf::from("test"));

        // WIP story in "keel" epic
        board.stories.insert(
            "wip1".to_string(),
            make_story_with_scope(
                "wip1",
                "WIP in Keel",
                StoryState::InProgress,
                Some(1),
                "keel/voyage-1",
            ),
        );

        // Backlog story in "dojo" epic (lower index - would normally be picked)
        board.stories.insert(
            "backlog1".to_string(),
            make_story_with_scope(
                "backlog1",
                "Backlog in Dojo",
                StoryState::Backlog,
                Some(1),
                "dojo/voyage-1",
            ),
        );

        // Backlog story in "keel" epic (higher index but same epic as WIP)
        board.stories.insert(
            "backlog2".to_string(),
            make_story_with_scope(
                "backlog2",
                "Backlog in Keel",
                StoryState::Backlog,
                Some(2),
                "keel/voyage-2",
            ),
        );

        let next_up = calculate_next_up(&board);

        // Should pick backlog2 (same epic as WIP) over backlog1 (different epic)
        let backlog_item = next_up.agent_items.iter().find(|i| i.category == "backlog");
        assert!(backlog_item.is_some(), "Should have a backlog item");
        assert_eq!(
            backlog_item.unwrap().id,
            "backlog2",
            "Should prefer same-epic story"
        );
    }

    #[test]
    fn next_backlog_falls_back_when_no_same_epic_available() {
        let mut board = Board::new(PathBuf::from("test"));

        // WIP story in "keel" epic
        board.stories.insert(
            "wip1".to_string(),
            make_story_with_scope(
                "wip1",
                "WIP in Keel",
                StoryState::InProgress,
                Some(1),
                "keel/voyage-1",
            ),
        );

        // Only backlog story is in "dojo" epic (different from WIP)
        board.stories.insert(
            "backlog1".to_string(),
            make_story_with_scope(
                "backlog1",
                "Backlog in Dojo",
                StoryState::Backlog,
                Some(1),
                "dojo/voyage-1",
            ),
        );

        let next_up = calculate_next_up(&board);

        // Should still pick backlog1 since it's the only option
        let backlog_item = next_up.agent_items.iter().find(|i| i.category == "backlog");
        assert!(backlog_item.is_some(), "Should have a backlog item");
        assert_eq!(
            backlog_item.unwrap().id,
            "backlog1",
            "Should fall back to any backlog"
        );
    }

    // Decompose momentum tests

    fn make_voyage_with_epic(id: &str, title: &str, status: VoyageState, epic: &str) -> Voyage {
        Voyage {
            frontmatter: VoyageFrontmatter {
                id: id.to_string(),
                title: title.to_string(),
                goal: None,
                status,
                epic: Some(epic.to_string()),
                index: None,
                created_at: None,
                updated_at: None,
                completed_at: None,
            },
            path: PathBuf::from(format!("{}/README.md", id)),
            epic_id: epic.to_string(),
        }
    }

    #[test]
    fn decompose_prefers_epic_with_recent_activity() {
        use chrono::NaiveDateTime;

        let mut board = Board::new(PathBuf::from("test"));

        // Recently submitted story in "role-based-cli" epic (awaiting verification)
        let mut submitted_story = make_story_with_scope(
            "submitted1",
            "Submitted in RoleCli",
            StoryState::NeedsHumanVerification,
            Some(1),
            "role-based-cli/01-taxonomy-parser",
        );
        submitted_story.frontmatter.submitted_at =
            NaiveDateTime::parse_from_str("2026-02-04T10:00:00", "%Y-%m-%dT%H:%M:%S").ok();
        board
            .stories
            .insert("submitted1".to_string(), submitted_story);

        // Draft voyage in "mobile" epic (alphabetically first: m < r)
        board.voyages.insert(
            "02-ios-mobile-app".to_string(),
            make_voyage_with_epic(
                "02-ios-mobile-app",
                "iOS Mobile App",
                VoyageState::Draft,
                "mobile",
            ),
        );

        // Draft voyage in "role-based-cli" epic (has recent activity)
        board.voyages.insert(
            "02-story-role-requirements".to_string(),
            make_voyage_with_epic(
                "02-story-role-requirements",
                "Story Role Requirements",
                VoyageState::Draft,
                "role-based-cli",
            ),
        );

        let next_up = calculate_next_up(&board);

        // Should pick role-based-cli voyage (epic with recent activity) over mobile
        let plan_item = next_up.human_items.iter().find(|i| i.category == "plan");
        assert!(plan_item.is_some(), "Should have a plan item");
        assert_eq!(
            plan_item.unwrap().id,
            "02-story-role-requirements",
            "Should prefer voyage in epic with recent activity"
        );
    }

    #[test]
    fn decompose_with_multiple_epics_same_date_prefers_one_with_more_stories() {
        use chrono::NaiveDateTime;

        let mut board = Board::new(PathBuf::from("test"));

        // 4 stories in role-based-cli epic (more momentum)
        for i in 1..=4 {
            let mut story = make_story_with_scope(
                &format!("s{}", i),
                &format!("Story {}", i),
                StoryState::NeedsHumanVerification,
                Some(i as u32),
                "role-based-cli/01-taxonomy-parser",
            );
            story.frontmatter.submitted_at =
                NaiveDateTime::parse_from_str("2026-02-04T12:00:00", "%Y-%m-%dT%H:%M:%S").ok();
            board.stories.insert(format!("s{}", i), story);
        }

        // 1 story in evals epic (less momentum)
        let mut story_evals = make_story_with_scope(
            "s5",
            "Story 5",
            StoryState::NeedsHumanVerification,
            Some(1),
            "evals/01-performance-evaluation",
        );
        story_evals.frontmatter.submitted_at =
            NaiveDateTime::parse_from_str("2026-02-04T12:00:00", "%Y-%m-%dT%H:%M:%S").ok();
        board.stories.insert("s5".to_string(), story_evals);

        // Draft voyage in role-based-cli
        board.voyages.insert(
            "02-story-role-requirements".to_string(),
            make_voyage_with_epic(
                "02-story-role-requirements",
                "Story Role Requirements",
                VoyageState::Draft,
                "role-based-cli",
            ),
        );

        // Draft voyage in evals
        board.voyages.insert(
            "02-long-term-studies".to_string(),
            make_voyage_with_epic(
                "02-long-term-studies",
                "Long-term Studies",
                VoyageState::Draft,
                "evals",
            ),
        );

        let next_up = calculate_next_up(&board);

        let plan_item = next_up.human_items.iter().find(|i| i.category == "plan");

        assert!(plan_item.is_some(), "Should have a plan item");
        // role-based-cli has 4 stories vs evals' 1, so role-based-cli should win
        assert_eq!(
            plan_item.unwrap().id,
            "02-story-role-requirements",
            "Should prefer epic with more stories awaiting verification"
        );
    }
}
