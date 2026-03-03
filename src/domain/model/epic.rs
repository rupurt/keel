//! Epic data structure

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Entity, FuzzyMatch, VoyageState, deserialize_strict_datetime};

/// Derived states for an epic.
///
/// Epic status is computed from voyage states (never persisted in epic frontmatter):
/// - `Draft`: no voyages, or all voyages are draft
/// - `Done`: at least one voyage exists and all voyages are done
/// - `Active`: any mixed/non-draft in-flight state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum EpicState {
    #[default]
    Draft,
    Active,
    Done,
}

impl std::fmt::Display for EpicState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Active => write!(f, "active"),
            Self::Done => write!(f, "done"),
        }
    }
}

/// Epic frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpicFrontmatter {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    /// Source bearing ID when this epic is laid from a bearing
    #[serde(default)]
    pub bearing: Option<String>,
    /// Ordering within the board (lower = earlier)
    #[serde(default)]
    pub index: Option<u32>,
    /// Creation datetime
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub created_at: Option<NaiveDateTime>,
}

/// An epic with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Epic {
    /// Parsed frontmatter
    pub frontmatter: EpicFrontmatter,
    /// Path to the epic README.md
    pub path: PathBuf,
    /// Derived status from voyages
    pub status: EpicState,
}

impl Epic {
    /// Get the epic ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the epic title
    #[allow(dead_code)] // Accessor for future CLI display
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the epic status (derived from voyages)
    pub fn status(&self) -> EpicState {
        self.status
    }

    /// Update derived status.
    pub(crate) fn set_status(&mut self, status: EpicState) {
        self.status = status;
    }

    /// Derive epic status from voyage states.
    pub fn derive_status(voyage_states: &[VoyageState]) -> EpicState {
        if voyage_states.is_empty() || voyage_states.iter().all(|s| *s == VoyageState::Draft) {
            EpicState::Draft
        } else if voyage_states.iter().all(|s| *s == VoyageState::Done) {
            EpicState::Done
        } else {
            EpicState::Active
        }
    }

    /// Get the index number (for ordering within the board)
    pub fn index(&self) -> Option<u32> {
        self.frontmatter.index
    }

    /// Check if epic matches a pattern (fuzzy match)
    pub fn matches(&self, pattern: &str) -> bool {
        super::fuzzy_match(&self.frontmatter.id, &self.frontmatter.title, pattern)
    }
}

impl Entity for Epic {
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

impl FuzzyMatch for Epic {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }

    fn matches(&self, pattern: &str) -> bool {
        Epic::matches(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epic_frontmatter_deserializes() {
        let yaml = r#"
id: board-cli
title: Board CLI
"#;
        let fm: EpicFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "board-cli");
        assert_eq!(fm.title, "Board CLI");
        assert!(fm.bearing.is_none());
    }

    #[test]
    fn epic_frontmatter_deserializes_bearing_link() {
        let yaml = r#"
id: board-cli
title: Board CLI
bearing: BRG-01
"#;
        let fm: EpicFrontmatter = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(fm.bearing.as_deref(), Some("BRG-01"));
    }

    #[test]
    fn epic_frontmatter_rejects_status_field() {
        let yaml = r#"
id: test
title: Test
status: active
"#;
        let err = serde_yaml::from_str::<EpicFrontmatter>(yaml).unwrap_err();
        assert!(err.to_string().contains("status"));
    }

    #[test]
    fn epic_frontmatter_rejects_completed_at_field() {
        let yaml = r#"
id: test
title: Test
completed_at: 2026-01-01T00:00:00
"#;
        let err = serde_yaml::from_str::<EpicFrontmatter>(yaml).unwrap_err();
        assert!(err.to_string().contains("completed_at"));
    }

    #[test]
    fn epic_matches() {
        let epic = Epic {
            frontmatter: EpicFrontmatter {
                id: "board-cli".to_string(),
                title: "Board CLI".to_string(),
                description: None,
                bearing: None,
                index: None,
                created_at: None,
            },
            path: PathBuf::from("test"),
            status: EpicState::Draft,
        };

        assert!(epic.matches("board-cli"));
        assert!(epic.matches("board"));
        assert!(epic.matches("CLI"));
        assert!(!epic.matches("web"));
    }

    #[test]
    fn derive_status_matches_voyage_states() {
        assert_eq!(Epic::derive_status(&[]), EpicState::Draft);
        assert_eq!(
            Epic::derive_status(&[VoyageState::Draft, VoyageState::Draft]),
            EpicState::Draft
        );
        assert_eq!(
            Epic::derive_status(&[VoyageState::Done, VoyageState::Done]),
            EpicState::Done
        );
        assert_eq!(
            Epic::derive_status(&[VoyageState::InProgress]),
            EpicState::Active
        );
        assert_eq!(
            Epic::derive_status(&[VoyageState::Draft, VoyageState::Done]),
            EpicState::Active
        );
    }

    #[test]
    fn test_deserialize_strict_datetime() {
        let yaml_dt = "created_at: 2024-01-01T12:00:00\n";
        let fm_dt: EpicFrontmatter =
            serde_yaml::from_str(&format!("id: test\ntitle: test\n{}", yaml_dt)).unwrap();
        assert!(fm_dt.created_at.is_some());

        let yaml_d = "created_at: 2024-01-01\n";
        let res_d: Result<EpicFrontmatter, _> =
            serde_yaml::from_str(&format!("id: test\ntitle: test\n{}", yaml_d));
        assert!(res_d.is_err(), "Date-only should be rejected");

        let yaml_invalid = "created_at: invalid\n";
        let res: Result<EpicFrontmatter, _> =
            serde_yaml::from_str(&format!("id: test\ntitle: test\n{}", yaml_invalid));
        assert!(res.is_err());
    }
}
