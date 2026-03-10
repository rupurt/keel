use std::path::Path;

use super::super::types::*;
use crate::domain::model::{Board, MissionStatus};
use crate::infrastructure::validation::charter::{self, GoalVerification};
use crate::infrastructure::validation::structural;

/// Check mission goal achievement
pub fn check_mission_goals(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for mission in board.missions.values() {
        if mission.status() != MissionStatus::Active {
            continue;
        }

        let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
        let content = match std::fs::read_to_string(&charter_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let goals = charter::parse_mission_goals(&content);
        if goals.is_empty() {
            continue;
        }

        let mut all_board_met = true;
        let mut board_goal_count = 0;

        for goal in &goals {
            if let GoalVerification::Board(target) = &goal.verification {
                board_goal_count += 1;
                if !is_goal_met(board, target) {
                    all_board_met = false;
                }
            }
        }

        if board_goal_count > 0 && all_board_met {
            problems.push(Problem {
                severity: Severity::Info,
                path: mission.path.clone(),
                message: format!(
                    "Mission {} has met all board-verifiable goals. Ready for achievement.",
                    mission.id()
                ),
                fix: None,
                scope: Some(mission.id().to_string()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::MissionGoalAchieved,
            });
        }
    }

    problems
}

struct LogEntry {
    pub raw: String,
}

fn parse_log_entries(content: &str) -> (String, Vec<LogEntry>) {
    let mut header = String::new();
    let mut entries = Vec::new();
    let mut current_entry = String::new();
    let mut in_header = true;

    for line in content.lines() {
        if line.starts_with("## ") {
            in_header = false;
            if !current_entry.is_empty() {
                entries.push(LogEntry {
                    raw: current_entry.trim().to_string(),
                });
            }
            current_entry = line.to_string();
        } else if in_header {
            header.push_str(line);
            header.push('\n');
        } else {
            current_entry.push('\n');
            current_entry.push_str(line);
        }
    }
    if !current_entry.is_empty() {
        entries.push(LogEntry {
            raw: current_entry.trim().to_string(),
        });
    }

    (header, entries)
}

/// Check that completed missions have at least one log entry and at least one child entity
pub fn check_mission_completion_evidence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for mission in board.missions.values() {
        // Only check terminal or near-terminal states
        if !matches!(mission.status(), MissionStatus::Achieved | MissionStatus::Verified) {
            continue;
        }

        // 1. Check for log entries
        let log_path = mission.path.parent().unwrap().join("LOG.md");
        let content = std::fs::read_to_string(&log_path).unwrap_or_default();
        let (_, entries) = parse_log_entries(&content);

        if entries.is_empty() {
            problems.push(Problem {
                severity: Severity::Warning,
                path: mission.path.clone(),
                message: format!(
                    "Mission {} is {} but has no log entries in LOG.md",
                    mission.id(),
                    mission.status()
                ),
                fix: None,
                scope: Some(mission.id().to_string()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::MissionMissingLogEntries,
            });
        }

        // 2. Check for child entities
        let child_count = board.mission_child_count(mission.id());
        if child_count == 0 {
            problems.push(Problem {
                severity: Severity::Warning,
                path: mission.path.clone(),
                message: format!(
                    "Mission {} is {} but has no child entities (epics, bearings, or ADRs)",
                    mission.id(),
                    mission.status()
                ),
                fix: None,
                scope: Some(mission.id().to_string()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::MissionMissingChildren,
            });
        }
    }

    problems
}

fn is_goal_met(board: &Board, target: &str) -> bool {
    let target = target.trim();
    if target.is_empty() || target == "..." {
        return false;
    }

    // Check if it's an epic
    if let Some(epic) = board.epics.get(target) {
        return epic.status() == crate::domain::model::EpicState::Done;
    }

    // Check if it's a voyage
    if let Some(voyage) = board.voyages.get(target) {
        return voyage.status() == crate::domain::state_machine::voyage::VoyageState::Done;
    }

    // Check if it's a story
    if let Some(story) = board.stories.get(target) {
        return story.status == crate::domain::model::StoryState::Done;
    }

    false
}

/// Check for active missions with no in-flight work
pub fn check_mission_active_no_work(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for mission in board.missions.values() {
        if mission.status() != MissionStatus::Active {
            continue;
        }

        let epics = board.epics_for_mission(mission.id());
        let bearings = board.bearings_for_mission(mission.id());

        let has_work = epics
            .iter()
            .any(|e| e.status() != crate::domain::model::EpicState::Done)
            || bearings.iter().any(|b| {
                b.status() != crate::domain::model::BearingStatus::Laid
                    && b.status() != crate::domain::model::BearingStatus::Declined
            });

        if !has_work {
            problems.push(Problem {
                severity: Severity::Warning,
                path: mission.path.clone(),
                message: format!(
                    "Mission {} is Active but has no in-flight work (all epics/bearings terminal).",
                    mission.id()
                ),
                fix: None,
                scope: Some(mission.id().to_string()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::MissionActiveNoWork,
            });
        }
    }

    problems
}

/// Check for orphaned mission lineage
pub fn check_mission_orphans(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        if let Some(mission_id) = &epic.frontmatter.mission {
            if !board.missions.contains_key(mission_id) {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: epic.path.clone(),
                    message: format!(
                        "Epic {} references nonexistent mission '{}'",
                        epic.id(),
                        mission_id
                    ),
                    fix: None,
                    scope: Some(epic.id().to_string()),
                    category: Some(GapCategory::Coherence),
                    check_id: CheckId::MissionOrphanedLineage,
                });
            }
        }
    }

    for bearing in board.bearings.values() {
        if let Some(mission_id) = &bearing.frontmatter.mission {
            if !board.missions.contains_key(mission_id) {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: bearing.path.clone(),
                    message: format!(
                        "Bearing {} references nonexistent mission '{}'",
                        bearing.id(),
                        mission_id
                    ),
                    fix: None,
                    scope: Some(bearing.id().to_string()),
                    category: Some(GapCategory::Coherence),
                    check_id: CheckId::MissionOrphanedLineage,
                });
            }
        }
    }

    for adr in board.adrs.values() {
        if let Some(mission_id) = &adr.frontmatter.mission {
            if !board.missions.contains_key(mission_id) {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: adr.path.clone(),
                    message: format!(
                        "ADR {} references nonexistent mission '{}'",
                        adr.id(),
                        mission_id
                    ),
                    fix: None,
                    scope: Some(adr.id().to_string()),
                    category: Some(GapCategory::Coherence),
                    check_id: CheckId::MissionOrphanedLineage,
                });
            }
        }
    }

    problems
}

/// Check for duplicate mission IDs
pub fn check_mission_duplicates(board_dir: &Path) -> Vec<Problem> {
    crate::infrastructure::duplicate_ids::duplicate_id_problems(
        board_dir,
        crate::infrastructure::duplicate_ids::DuplicateEntity::Mission,
    )
}

/// Check mission date consistency
pub fn check_mission_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for mission in board.missions.values() {
        problems.extend(structural::check_date_consistency(
            &mission.path,
            CheckId::MissionDateConsistency,
        ));
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission};
    use std::fs;

    #[test]
    fn test_check_mission_goals_achieved() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        let charter = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: E1 |
"#;
        fs::write(charter_path, charter).unwrap();

        // E1 is NOT done, so it shouldn't be achieved
        let board = load_board(temp.path()).unwrap();
        let problems = check_mission_goals(&board);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_check_mission_orphans() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1").mission("NONEXISTENT"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_mission_orphans(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].check_id, CheckId::MissionOrphanedLineage);
        assert!(problems[0].message.contains("NONEXISTENT"));
    }

    #[test]
    fn test_check_mission_active_no_work() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            // No epics or bearings linked to M1
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_mission_active_no_work(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].check_id, CheckId::MissionActiveNoWork);
    }

    #[test]
    fn test_check_mission_completion_evidence() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("achieved"))
            .build();
        
        let board = load_board(temp.path()).unwrap();
        let problems = check_mission_completion_evidence(&board);
        
        // Should have 2 problems: missing log and missing children
        assert_eq!(problems.len(), 2);
        assert!(problems.iter().any(|p| p.check_id == CheckId::MissionMissingLogEntries));
        assert!(problems.iter().any(|p| p.check_id == CheckId::MissionMissingChildren));

        // Fix children
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("achieved"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();
        let board = load_board(temp.path()).unwrap();
        let problems = check_mission_completion_evidence(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].check_id, CheckId::MissionMissingLogEntries);
    }
}
