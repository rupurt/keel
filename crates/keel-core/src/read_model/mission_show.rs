//! Mission show projection

use anyhow::Result;
use std::fs;

use crate::domain::model::{Board, Mission};
use crate::infrastructure::validation::charter;
use crate::infrastructure::validation::charter::ParsedMissionGoal;
use crate::read_model::knowledge_graph::{DriftSurfaceSummary, project_structural_drift_summary};

#[derive(Debug, Clone, serde::Serialize)]
pub struct MissionShowProjection {
    pub id: String,
    pub title: String,
    pub status: String,
    pub archetype: String,
    pub goals: Vec<ParsedMissionGoal>,
    pub child_entities: MissionChildren,
    pub drift: DriftSurfaceSummary,
    pub log_summary: Option<String>,
    pub operator_signal: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MissionChildren {
    pub epics: Vec<EntitySummary>,
    pub bearings: Vec<EntitySummary>,
    pub adrs: Vec<EntitySummary>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EntitySummary {
    pub id: String,
    pub title: String,
    pub status: String,
}

/// Build a mission show projection
pub fn build_projection(board: &Board, mission: &Mission) -> Result<MissionShowProjection> {
    let mission_dir = mission.path.parent().unwrap();

    // Parse goals from CHARTER.md
    let charter_path = mission_dir.join("CHARTER.md");
    let charter_content = fs::read_to_string(&charter_path).unwrap_or_default();
    let goals = charter::parse_mission_goals(&charter_content);

    // Get child entities
    let epics = board
        .epics_for_mission(mission.id())
        .into_iter()
        .map(|e| EntitySummary {
            id: e.id().to_string(),
            title: e.frontmatter.title.clone(),
            status: e.status().to_string(),
        })
        .collect();

    let bearings = board
        .bearings_for_mission(mission.id())
        .into_iter()
        .map(|b| EntitySummary {
            id: b.id().to_string(),
            title: b.frontmatter.title.clone(),
            status: b.status().to_string(),
        })
        .collect();

    let adrs = board
        .adrs_for_mission(mission.id())
        .into_iter()
        .map(|a| EntitySummary {
            id: a.id().to_string(),
            title: a.frontmatter.title.clone(),
            status: a.status().to_string(),
        })
        .collect();

    // Parse latest entry from LOG.md
    let log_path = mission_dir.join("LOG.md");
    let log_content = fs::read_to_string(&log_path).unwrap_or_default();
    let log_summary = extract_latest_log_entry(&log_content);
    let drift = project_structural_drift_summary(board)?;

    Ok(MissionShowProjection {
        id: mission.id().to_string(),
        title: mission.title().to_string(),
        status: mission.status().to_string(),
        archetype: mission.archetype.as_str().to_string(),
        goals,
        child_entities: MissionChildren {
            epics,
            bearings,
            adrs,
        },
        drift,
        log_summary,
        operator_signal: mission.frontmatter.operator_signal.clone(),
    })
}

fn extract_latest_log_entry(content: &str) -> Option<String> {
    // Find the first H2 heading which is the latest entry
    for line in content.lines() {
        if line.starts_with("## ") {
            return Some(line.trim_start_matches("## ").trim().to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission};

    #[test]
    fn test_mission_show_projection_build() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One"))
            .epic(TestEpic::new("E1").mission("M1").title("Epic One"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let mission = board.require_mission("M1").unwrap();

        let projection = build_projection(&board, mission).unwrap();

        assert_eq!(projection.id, "M1");
        assert_eq!(projection.title, "Mission One");
        assert_eq!(projection.child_entities.epics.len(), 1);
        assert_eq!(projection.child_entities.epics[0].id, "E1");
    }

    #[test]
    fn test_parse_mission_goals() {
        let charter = r#"
# Mission Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Complete the task | board: done |
| MG-02 | Measure success | metric: confidence |
"#;
        let goals = charter::parse_mission_goals(charter);
        assert_eq!(goals.len(), 2);
        assert_eq!(goals[0].id, "MG-01");
        assert_eq!(goals[0].description, "Complete the task");
        assert_eq!(goals[0].verification.raw(), "done");
    }

    #[test]
    fn test_extract_latest_log_entry() {
        let log = r#"
# Decision Log

## 2026-03-09T10:00:00 - Initial entry
Description of initial entry.

## 2026-03-09T09:00:00 - Older entry
Description of older entry.
"#;
        let summary = extract_latest_log_entry(log);
        assert_eq!(
            summary.as_deref(),
            Some("2026-03-09T10:00:00 - Initial entry")
        );
    }
}
