//! CHARTER.md goal parsing and readiness utilities

use std::fs;

use super::{CheckId, GapCategory, Problem};
use crate::domain::model::{Board, Mission, MissionArchetype};
use crate::infrastructure::markdown_sections::extract_section;
use crate::infrastructure::validation::structural;

const DEFAULT_HALTING_RULES: [&str; 3] = [
    "do not halt while any mg-* goal has unfinished board work",
    "halt when all mg-* goals with board: verification are satisfied",
    "yield to human when only metric: or manual: goals remain",
];

/// Parse mission archetype from CHARTER.md content
pub fn parse_mission_archetype(content: &str) -> MissionArchetype {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("**Archetype:**") {
            if let Some(archetype) = MissionArchetype::parse(rest.trim()) {
                return archetype;
            }
        }
        if let Some(rest) = trimmed.strip_prefix("Archetype:") {
            if let Some(archetype) = MissionArchetype::parse(rest.trim()) {
                return archetype;
            }
        }
    }
    MissionArchetype::default()
}

/// Mission goal verification type
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum GoalVerification {
    Board(String),
    Metric(String),
    Manual(String),
    Unknown(String),
}

impl GoalVerification {
    pub fn parse(s: &str) -> Self {
        let trimmed = s.trim();
        if let Some(target) = trimmed.strip_prefix("board:") {
            Self::Board(target.trim().to_string())
        } else if let Some(target) = trimmed.strip_prefix("metric:") {
            Self::Metric(target.trim().to_string())
        } else if let Some(target) = trimmed.strip_prefix("manual:") {
            Self::Manual(target.trim().to_string())
        } else {
            Self::Unknown(trimmed.to_string())
        }
    }

    pub fn raw(&self) -> &str {
        match self {
            Self::Board(s) => s,
            Self::Metric(s) => s,
            Self::Manual(s) => s,
            Self::Unknown(s) => s,
        }
    }
}

/// A parsed mission goal from CHARTER.md
#[derive(Debug, Clone, serde::Serialize)]
pub struct ParsedMissionGoal {
    pub id: String,
    pub description: String,
    pub verification: GoalVerification,
}

/// Parse goals from CHARTER.md content
pub fn parse_mission_goals(content: &str) -> Vec<ParsedMissionGoal> {
    let mut goals = Vec::new();
    let Some(section) = extract_section(content, "## Goals") else {
        return goals;
    };

    for line in section.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|')
            || trimmed.contains("ID | Description")
            || trimmed.contains("---|---")
        {
            continue;
        }

        let parts: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
        // | ID | Description | Verification |
        // 0  1       2              3        4
        if parts.len() >= 4 {
            let id = parts[1];
            let description = parts[2];
            let verification_raw = parts[3];

            if !id.is_empty() && !id.starts_with('{') {
                goals.push(ParsedMissionGoal {
                    id: id.to_string(),
                    description: description.to_string(),
                    verification: GoalVerification::parse(verification_raw),
                });
            }
        }
    }

    goals
}

pub fn goal_needs_description(goal: &ParsedMissionGoal) -> bool {
    let description = goal.description.trim();
    description.is_empty() || structural::first_unfilled_placeholder_pattern(description).is_some()
}

pub fn goal_needs_verification(goal: &ParsedMissionGoal) -> bool {
    verification_target_is_placeholder(&goal.verification)
        || matches!(goal.verification, GoalVerification::Unknown(_))
}

pub fn has_authored_constraints(content: &str) -> bool {
    let Some(section) = extract_section(content, "## Constraints") else {
        return false;
    };

    let normalized_lines: Vec<_> = section
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("<!--"))
        .collect();

    !normalized_lines.is_empty()
        && !normalized_lines.iter().all(|line| {
            line.eq_ignore_ascii_case("- (none yet)") || line.eq_ignore_ascii_case("(none yet)")
        })
        && structural::first_unfilled_placeholder_pattern(&section).is_none()
}

pub fn has_authored_halting_rules(content: &str) -> bool {
    let Some(section) = extract_section(content, "## Halting Rules") else {
        return false;
    };

    let bullets = section_bullets(&section);
    !bullets.is_empty()
        && structural::first_unfilled_placeholder_pattern(&section).is_none()
        && bullets != DEFAULT_HALTING_RULES
}

pub fn check_mission_charter_readiness(board: &Board, mission: &Mission) -> Vec<Problem> {
    let mut problems = Vec::new();
    let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
    let content = match fs::read_to_string(&charter_path) {
        Ok(content) => content,
        Err(err) => {
            return vec![mission_readiness_problem(
                &charter_path,
                mission,
                format!(
                    "Mission {} cannot be activated: CHARTER.md could not be read ({})",
                    mission.id(),
                    err
                ),
            )];
        }
    };

    let goals = parse_mission_goals(&content);
    if goals.is_empty() {
        problems.push(mission_readiness_problem(
            &charter_path,
            mission,
            format!(
                "Mission {} cannot be activated: CHARTER.md must define at least one goal.",
                mission.id()
            ),
        ));
    }

    let mut valid_board_goals = 0usize;

    for goal in &goals {
        if goal_needs_description(goal) {
            problems.push(mission_readiness_problem(
                &charter_path,
                mission,
                format!(
                    "Mission {} goal {} must include an authored description.",
                    mission.id(),
                    goal.id
                ),
            ));
        }

        match &goal.verification {
            GoalVerification::Board(target) => {
                let target = target.trim();
                if target.is_empty() || target == "..." {
                    problems.push(mission_readiness_problem(
                        &charter_path,
                        mission,
                        format!(
                            "Mission {} goal {} must replace the placeholder board target with a real board entity id.",
                            mission.id(),
                            goal.id
                        ),
                    ));
                    continue;
                }

                match board_target_membership(board, mission.id(), target) {
                    Some(true) => valid_board_goals += 1,
                    Some(false) => problems.push(mission_readiness_problem(
                        &charter_path,
                        mission,
                        format!(
                            "Mission {} goal {} references board target '{}' outside this mission.",
                            mission.id(),
                            goal.id,
                            target
                        ),
                    )),
                    None => problems.push(mission_readiness_problem(
                        &charter_path,
                        mission,
                        format!(
                            "Mission {} goal {} references unknown board target '{}'.",
                            mission.id(),
                            goal.id,
                            target
                        ),
                    )),
                }
            }
            GoalVerification::Metric(value) => {
                if value.trim().is_empty() || value.trim() == "..." {
                    problems.push(mission_readiness_problem(
                        &charter_path,
                        mission,
                        format!(
                            "Mission {} goal {} must provide a concrete metric verification target.",
                            mission.id(),
                            goal.id
                        ),
                    ));
                }
            }
            GoalVerification::Manual(value) => {
                if value.trim().is_empty() || value.trim() == "..." {
                    problems.push(mission_readiness_problem(
                        &charter_path,
                        mission,
                        format!(
                            "Mission {} goal {} must provide a concrete manual verification path.",
                            mission.id(),
                            goal.id
                        ),
                    ));
                }
            }
            GoalVerification::Unknown(value) => problems.push(mission_readiness_problem(
                &charter_path,
                mission,
                format!(
                    "Mission {} goal {} uses unsupported verification '{}'. Use board:, metric:, or manual:.",
                    mission.id(),
                    goal.id,
                    value.trim()
                ),
            )),
        }
    }

    if valid_board_goals == 0 {
        problems.push(mission_readiness_problem(
            &charter_path,
            mission,
            format!(
                "Mission {} cannot be activated: define at least one mission-scoped goal with a valid `board:` verification target.",
                mission.id()
            ),
        ));
    }

    if !has_authored_constraints(&content) {
        problems.push(mission_readiness_problem(
            &charter_path,
            mission,
            format!(
                "Mission {} cannot be activated: `## Constraints` must include authored mission-specific constraints.",
                mission.id()
            ),
        ));
    }

    if !has_authored_halting_rules(&content) {
        problems.push(mission_readiness_problem(
            &charter_path,
            mission,
            format!(
                "Mission {} cannot be activated: `## Halting Rules` must replace the scaffold defaults with mission-specific rules.",
                mission.id()
            ),
        ));
    }

    problems
}

fn mission_readiness_problem(
    path: &std::path::Path,
    mission: &Mission,
    message: String,
) -> Problem {
    Problem::error(path.to_path_buf(), message)
        .with_scope(mission.id())
        .with_category(GapCategory::Coherence)
        .with_check_id(CheckId::MissionDefinitionReadiness)
}

fn verification_target_is_placeholder(verification: &GoalVerification) -> bool {
    match verification {
        GoalVerification::Board(target)
        | GoalVerification::Metric(target)
        | GoalVerification::Manual(target) => {
            let target = target.trim();
            target.is_empty() || target == "..."
        }
        GoalVerification::Unknown(_) => true,
    }
}

fn board_target_membership(board: &Board, mission_id: &str, target: &str) -> Option<bool> {
    if board.epics.contains_key(target) {
        return Some(board.is_epic_in_mission(target, mission_id));
    }
    if let Some(voyage) = board.voyages.get(target) {
        return Some(board.is_voyage_in_mission(voyage, mission_id));
    }
    if let Some(story) = board.stories.get(target) {
        return Some(board.is_story_in_mission(story, mission_id));
    }
    if let Some(bearing) = board.bearings.get(target) {
        return Some(board.is_bearing_in_mission(bearing, mission_id));
    }
    if let Some(adr) = board.adrs.get(target) {
        return Some(board.is_adr_in_mission(adr, mission_id));
    }
    None
}

fn section_bullets(section: &str) -> Vec<String> {
    section
        .lines()
        .map(str::trim)
        .filter_map(|line| line.strip_prefix("- "))
        .map(normalize_bullet)
        .collect()
}

fn normalize_bullet(line: &str) -> String {
    line.trim()
        .trim_end_matches('.')
        .replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission};
    use std::fs;

    #[test]
    fn test_parse_mission_goals() {
        let content = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Goal One | board: E1 |
| MG-02 | Goal Two | metric: 100% |
| MG-03 | Goal Three | manual: review |
"#;
        let goals = parse_mission_goals(content);
        assert_eq!(goals.len(), 3);
        assert_eq!(goals[0].id, "MG-01");
        assert_eq!(
            goals[0].verification,
            GoalVerification::Board("E1".to_string())
        );
        assert_eq!(
            goals[1].verification,
            GoalVerification::Metric("100%".to_string())
        );
        assert_eq!(
            goals[2].verification,
            GoalVerification::Manual("review".to_string())
        );
    }

    #[test]
    fn test_parse_mission_goals_empty() {
        assert!(parse_mission_goals("# No goals").is_empty());
        assert!(
            parse_mission_goals(
                "## Goals
Empty"
            )
            .is_empty()
        );
    }

    #[test]
    fn test_has_authored_halting_rules_rejects_scaffold_defaults() {
        let content = r#"
## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work.
- HALT when all MG-* goals with `board:` verification are satisfied.
- YIELD to human when only `metric:` or `manual:` goals remain.
"#;

        assert!(!has_authored_halting_rules(content));
    }

    #[test]
    fn test_check_mission_charter_readiness_flags_placeholder_board_target_and_defaults() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let mission = board.require_mission("M1").unwrap();
        let problems = check_mission_charter_readiness(&board, mission);

        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("placeholder board target"))
        );
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("mission-specific rules"))
        );
        assert!(
            problems
                .iter()
                .all(|problem| { problem.check_id == CheckId::MissionDefinitionReadiness })
        );
    }

    #[test]
    fn test_check_mission_charter_readiness_accepts_authored_charter() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("defining"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        fs::write(
            charter_path,
            r#"# Mission One - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Ship the first planning slice. | board: E1 |

## Constraints

- Keep the work scoped to one implementation slice at a time.

## Halting Rules

- Halt after the first epic is decomposed into ready voyages and stories.
- Yield to human review if a routine model requires changing the public storage contract.
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let mission = board.require_mission("M1").unwrap();
        let problems = check_mission_charter_readiness(&board, mission);
        assert!(problems.is_empty(), "expected clean charter: {problems:#?}");
    }
}
