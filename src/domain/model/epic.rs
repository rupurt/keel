//! Epic data structure

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize};

use super::{Entity, FuzzyMatch, deserialize_strict_datetime};

/// Strategic states for an epic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum EpicState {
    /// High-level goal defined, but no tactical execution yet (all voyages in draft)
    #[default]
    Strategic,
    /// Tactical execution has begun (at least one voyage is planned or in-progress)
    Tactical,
    /// Strategic goal met (all voyages done and success criteria verified)
    Done,
}

impl<'de> Deserialize<'de> for EpicState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "strategic" => Ok(EpicState::Strategic),
            "tactical" => Ok(EpicState::Tactical),
            "done" => Ok(EpicState::Done),
            "planned" | "draft" => Err(serde::de::Error::custom(format!(
                "legacy epic status `{s}` is no longer supported; use `strategic`"
            ))),
            "in-progress" | "active" => Err(serde::de::Error::custom(format!(
                "legacy epic status `{s}` is no longer supported; use `tactical`"
            ))),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["strategic", "tactical", "done"],
            )),
        }
    }
}

impl std::fmt::Display for EpicState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Strategic => write!(f, "strategic"),
            Self::Tactical => write!(f, "tactical"),
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
    pub status: EpicState,
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
    /// Completion datetime
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub completed_at: Option<NaiveDateTime>,
}

/// An epic with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Epic {
    /// Parsed frontmatter
    pub frontmatter: EpicFrontmatter,
    /// Path to the epic README.md
    pub path: PathBuf,
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

    /// Get the epic status
    pub fn status(&self) -> EpicState {
        self.frontmatter.status
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
status: strategic
"#;
        let fm: EpicFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "board-cli");
        assert_eq!(fm.title, "Board CLI");
        assert_eq!(fm.status, EpicState::Strategic);
        assert!(fm.bearing.is_none());
    }

    #[test]
    fn epic_frontmatter_handles_defaults() {
        let yaml = r#"
id: web-ui
title: Web UI
"#;
        let fm: EpicFrontmatter = serde_yaml::from_str(yaml).unwrap();

        // Default is Strategic
        assert_eq!(fm.status, EpicState::Strategic);
        assert!(fm.completed_at.is_none());
    }

    #[test]
    fn epic_frontmatter_deserializes_bearing_link() {
        let yaml = r#"
id: board-cli
title: Board CLI
status: strategic
bearing: BRG-01
"#;
        let fm: EpicFrontmatter = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(fm.bearing.as_deref(), Some("BRG-01"));
    }

    #[test]
    fn epic_frontmatter_rejects_legacy_completed_field() {
        let yaml = r#"
id: test
title: Test
status: done
completed: 2026-01-01
"#;
        let err = serde_yaml::from_str::<EpicFrontmatter>(yaml).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("completed"));
    }

    #[test]
    fn epic_frontmatter_rejects_legacy_names_with_replacements() {
        for (legacy, replacement) in [
            ("planned", "strategic"),
            ("draft", "strategic"),
            ("in-progress", "tactical"),
            ("active", "tactical"),
        ] {
            let yaml = format!("id: test\ntitle: test\nstatus: {legacy}\n");
            let err = serde_yaml::from_str::<EpicFrontmatter>(&yaml).unwrap_err();
            let message = err.to_string();
            assert!(
                message.contains(legacy),
                "missing legacy token in: {message}"
            );
            assert!(
                message.contains(replacement),
                "missing canonical replacement in: {message}"
            );
        }
    }

    #[test]
    fn epic_matches() {
        let epic = Epic {
            frontmatter: EpicFrontmatter {
                id: "board-cli".to_string(),
                title: "Board CLI".to_string(),
                status: EpicState::Strategic,
                description: None,
                bearing: None,
                index: None,
                created_at: None,
                completed_at: None,
            },
            path: PathBuf::from("test"),
        };

        assert!(epic.matches("board-cli"));
        assert!(epic.matches("board"));
        assert!(epic.matches("CLI"));
        assert!(!epic.matches("web"));
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
