#![allow(dead_code)]
//! Mission data structure
//!
//! Missions are top-level entities that represent autonomous long-running harness
//! sessions. They live in .keel/missions/<id>/ and contain README.md, CHARTER.md,
//! and LOG.md.

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Entity, deserialize_strict_datetime};
use crate::domain::state_machine::mission::MissionStatus;

/// Mission archetype category
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum MissionArchetype {
    /// Large-scale value shifts or architectural foundations
    #[default]
    Strategic,
    /// Purely focused on reaching/maintaining Zero Drift
    Maintenance,
    /// Dominated by Bearings and Play; focuses on reducing the fog of war
    Exploratory,
    /// Explicitly graduates conclusive research into Planned implementation work
    Bridging,
}

impl MissionArchetype {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Strategic => "Strategic",
            Self::Maintenance => "Maintenance",
            Self::Exploratory => "Exploratory",
            Self::Bridging => "Bridging",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let lower = s.to_ascii_lowercase();
        if lower.contains("strategic") {
            Some(Self::Strategic)
        } else if lower.contains("maintenance") || lower.contains("healing") {
            Some(Self::Maintenance)
        } else if lower.contains("exploratory") || lower.contains("discovery") {
            Some(Self::Exploratory)
        } else if lower.contains("bridging") || lower.contains("realization") {
            Some(Self::Bridging)
        } else {
            None
        }
    }
}

/// Mission frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MissionFrontmatter {
    /// Unique identifier (same as directory name)
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Current status in lifecycle
    #[serde(default)]
    pub status: MissionStatus,
    /// Creation date
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub created_at: Option<NaiveDateTime>,
    /// Last update date
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub updated_at: Option<NaiveDateTime>,
    /// Date mission was activated (Defining → Active)
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub activated_at: Option<NaiveDateTime>,
    /// Date mission was achieved (Active → Achieved)
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub achieved_at: Option<NaiveDateTime>,
    /// Date mission was verified (Achieved → Verified)
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub verified_at: Option<NaiveDateTime>,
    /// Optional signal from the operator
    #[serde(default, rename = "operator-signal")]
    pub operator_signal: Option<String>,
}

/// A mission with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Mission {
    /// Parsed frontmatter from README.md
    pub frontmatter: MissionFrontmatter,
    /// Path to the mission README.md
    pub path: PathBuf,
    /// Mission archetype
    pub archetype: MissionArchetype,
    /// Whether CHARTER.md exists in the mission directory
    pub has_charter: bool,
    /// Whether LOG.md exists in the mission directory
    pub has_log: bool,
}

impl Mission {
    /// Create a new Mission
    pub fn new(
        frontmatter: MissionFrontmatter,
        path: impl Into<PathBuf>,
        archetype: MissionArchetype,
        has_charter: bool,
        has_log: bool,
    ) -> Self {
        Self {
            frontmatter,
            path: path.into(),
            archetype,
            has_charter,
            has_log,
        }
    }

    /// Get the mission ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the mission title
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the mission status
    pub fn status(&self) -> MissionStatus {
        self.frontmatter.status
    }
}

impl Entity for Mission {
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

#[cfg(test)]
mod tests {
    use super::*;

    // ============ MissionFrontmatter Tests (SRS-01) ============

    #[test]
    fn mission_frontmatter_has_all_fields() {
        // SRS-01/AC-01: MissionFrontmatter has id, title, status, created_at,
        // updated_at, activated_at, achieved_at, verified_at fields
        let yaml = r#"
id: mission-01
title: Port Migration
status: active
created_at: 2026-03-01T10:00:00
updated_at: 2026-03-05T14:30:00
activated_at: 2026-03-02T09:00:00
achieved_at: 2026-03-08T16:00:00
verified_at: 2026-03-09T11:00:00
"#;
        let fm: MissionFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "mission-01");
        assert_eq!(fm.title, "Port Migration");
        assert_eq!(fm.status, MissionStatus::Active);
        assert!(fm.created_at.is_some());
        assert!(fm.updated_at.is_some());
        assert!(fm.activated_at.is_some());
        assert!(fm.achieved_at.is_some());
        assert!(fm.verified_at.is_some());
    }

    #[test]
    fn mission_frontmatter_defaults() {
        let yaml = r#"
id: test-mission
title: Test Mission
"#;
        let fm: MissionFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "test-mission");
        assert_eq!(fm.title, "Test Mission");
        assert_eq!(fm.status, MissionStatus::Defining);
        assert!(fm.created_at.is_none());
        assert!(fm.updated_at.is_none());
        assert!(fm.activated_at.is_none());
        assert!(fm.achieved_at.is_none());
        assert!(fm.verified_at.is_none());
    }

    #[test]
    fn mission_frontmatter_rejects_unknown_fields() {
        let yaml = r#"
id: test
title: Test
unknown_field: value
"#;
        let result: Result<MissionFrontmatter, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn mission_frontmatter_rejects_date_only() {
        // Strict datetime format: must include time component
        let yaml = r#"
id: test
title: Test
created_at: 2026-03-01
"#;
        let result: Result<MissionFrontmatter, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn mission_frontmatter_roundtrips() {
        let yaml = r#"
id: roundtrip-test
title: Roundtrip Test
status: defining
"#;
        let fm: MissionFrontmatter = serde_yaml::from_str(yaml).unwrap();
        let serialized = serde_yaml::to_string(&fm).unwrap();
        let fm2: MissionFrontmatter = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(fm.id, fm2.id);
        assert_eq!(fm.title, fm2.title);
        assert_eq!(fm.status, fm2.status);
    }

    // ============ Mission Struct Tests (SRS-04) ============

    #[test]
    fn mission_struct_has_entity_fields_and_artifact_flags() {
        // SRS-04/AC-01: Mission struct implements Entity trait with has_charter and has_log fields
        let fm = MissionFrontmatter {
            id: "test-mission".to_string(),
            title: "Test Mission".to_string(),
            status: MissionStatus::Defining,
            created_at: None,
            updated_at: None,
            activated_at: None,
            achieved_at: None,
            verified_at: None,
            operator_signal: None,
        };

        let mission = Mission::new(
            fm,
            "/test/missions/test-mission/README.md",
            MissionArchetype::Strategic,
            true,
            false,
        );

        assert_eq!(mission.id(), "test-mission");
        assert_eq!(mission.title(), "Test Mission");
        assert_eq!(mission.status(), MissionStatus::Defining);
        assert_eq!(mission.archetype, MissionArchetype::Strategic);
        assert!(mission.has_charter);
        assert!(!mission.has_log);
    }

    #[test]
    fn mission_entity_trait_path() {
        let fm = MissionFrontmatter {
            id: "m1".to_string(),
            title: "Mission One".to_string(),
            status: MissionStatus::Active,
            created_at: None,
            updated_at: None,
            activated_at: None,
            achieved_at: None,
            verified_at: None,
            operator_signal: None,
        };

        let mission = Mission::new(
            fm,
            "/board/missions/m1/README.md",
            MissionArchetype::Strategic,
            true,
            true,
        );
        assert_eq!(
            Entity::path(&mission),
            Path::new("/board/missions/m1/README.md")
        );
    }
}
