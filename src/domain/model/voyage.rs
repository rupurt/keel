//! Voyage data structure

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Entity, FuzzyMatch, VoyageState, deserialize_strict_datetime};

/// Voyage frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoyageFrontmatter {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub goal: Option<String>,
    #[serde(default)]
    pub status: VoyageState,
    #[serde(default)]
    pub epic: Option<String>,
    /// Ordering within the epic (lower = earlier)
    #[serde(default)]
    pub index: Option<u32>,
    /// Creation datetime
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub created_at: Option<NaiveDateTime>,
    /// Last update datetime
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub updated_at: Option<NaiveDateTime>,
    /// First start datetime
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub started_at: Option<NaiveDateTime>,
    /// Completion datetime
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub completed_at: Option<NaiveDateTime>,
}

/// A voyage with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Voyage {
    /// Parsed frontmatter
    pub frontmatter: VoyageFrontmatter,
    /// Path to the voyage README.md
    pub path: PathBuf,
    /// Parent epic ID (extracted from path)
    pub epic_id: String,
}

impl Voyage {
    /// Get the voyage ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the voyage title
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the voyage status
    pub fn status(&self) -> VoyageState {
        self.frontmatter.status
    }

    /// Get the index number (for ordering within the epic)
    pub fn index(&self) -> Option<u32> {
        self.frontmatter.index
    }

    /// Build the scope path for matching stories
    pub fn scope_path(&self) -> String {
        format!("{}/{}", self.epic_id, self.frontmatter.id)
    }

    /// Check if voyage matches a pattern (fuzzy match)
    pub fn matches(&self, pattern: &str) -> bool {
        super::fuzzy_match(&self.frontmatter.id, &self.frontmatter.title, pattern)
    }
}

impl Entity for Voyage {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }
    fn title(&self) -> &str {
        &self.frontmatter.title
    }
    fn path(&self) -> &Path {
        &self.path
    }
}

impl FuzzyMatch for Voyage {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }

    fn matches(&self, pattern: &str) -> bool {
        Voyage::matches(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voyage_frontmatter_deserializes() {
        let yaml = r#"
id: 01-core-infrastructure
title: Core Infrastructure
goal: Build the core
status: in-progress
epic: board-cli
"#;
        let fm: VoyageFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "01-core-infrastructure");
        assert_eq!(fm.title, "Core Infrastructure");
        assert_eq!(fm.goal, Some("Build the core".to_string()));
        assert_eq!(fm.status, VoyageState::InProgress);
        assert_eq!(fm.epic, Some("board-cli".to_string()));
    }

    #[test]
    fn voyage_frontmatter_handles_defaults() {
        let yaml = r#"
id: 02-generate
title: Generate Command
"#;
        let fm: VoyageFrontmatter = serde_yaml::from_str(yaml).unwrap();

        // Default status is now Draft (voyage being planned)
        assert_eq!(fm.status, VoyageState::Draft);
        assert!(fm.epic.is_none());
        assert!(fm.started_at.is_none());
        assert!(fm.completed_at.is_none());
    }

    #[test]
    fn voyage_scope_path() {
        let v = Voyage {
            frontmatter: VoyageFrontmatter {
                id: "01-core".to_string(),
                title: "Core".to_string(),
                goal: None,
                status: VoyageState::Planned,
                epic: None,
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
            },
            path: PathBuf::from("test"),
            epic_id: "board-cli".to_string(),
        };

        assert_eq!(v.scope_path(), "board-cli/01-core");
    }

    #[test]
    fn voyage_matches() {
        let v = Voyage {
            frontmatter: VoyageFrontmatter {
                id: "01-core-infrastructure".to_string(),
                title: "Core Infrastructure".to_string(),
                goal: None,
                status: VoyageState::Planned,
                epic: None,
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
            },
            path: PathBuf::from("test"),
            epic_id: "board-cli".to_string(),
        };

        assert!(v.matches("01-core"));
        assert!(v.matches("infrastructure"));
        assert!(v.matches("Core"));
        assert!(!v.matches("generate"));
    }

    #[test]
    fn test_deserialize_strict_datetime() {
        let yaml_dt = "created_at: 2024-01-01T12:00:00\n";
        let fm_dt: VoyageFrontmatter =
            serde_yaml::from_str(&format!("id: test\ntitle: test\n{}", yaml_dt)).unwrap();
        assert!(fm_dt.created_at.is_some());

        let yaml_d = "created_at: 2024-01-01\n";
        let res_d: Result<VoyageFrontmatter, _> =
            serde_yaml::from_str(&format!("id: test\ntitle: test\n{}", yaml_d));
        assert!(res_d.is_err(), "Date-only should be rejected");

        let yaml_invalid = "created_at: invalid\n";
        let res: Result<VoyageFrontmatter, _> =
            serde_yaml::from_str(&format!("id: test\ntitle: test\n{}", yaml_invalid));
        assert!(res.is_err());
    }
}
