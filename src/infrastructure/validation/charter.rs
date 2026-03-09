//! CHARTER.md goal parsing utilities

use crate::infrastructure::markdown_sections::extract_section;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
