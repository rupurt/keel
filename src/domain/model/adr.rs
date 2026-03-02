//! ADR (Architecture Decision Record) data structure
//!
//! ADRs document architectural decisions for the project. They live in `.keel/adrs/`.

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Entity, FuzzyMatch, deserialize_strict_datetime};

/// ADR status lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdrStatus {
    /// Proposed decision awaiting review
    #[default]
    Proposed,
    /// Decision has been accepted and is in effect
    Accepted,
    /// Decision was rejected with a reason
    Rejected,
    /// Decision is no longer recommended
    Deprecated,
    /// Decision has been replaced by another ADR
    Superseded,
}

impl std::fmt::Display for AdrStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdrStatus::Proposed => write!(f, "proposed"),
            AdrStatus::Accepted => write!(f, "accepted"),
            AdrStatus::Rejected => write!(f, "rejected"),
            AdrStatus::Deprecated => write!(f, "deprecated"),
            AdrStatus::Superseded => write!(f, "superseded"),
        }
    }
}

impl std::str::FromStr for AdrStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(AdrStatus::Proposed),
            "accepted" => Ok(AdrStatus::Accepted),
            "rejected" => Ok(AdrStatus::Rejected),
            "deprecated" => Ok(AdrStatus::Deprecated),
            "superseded" => Ok(AdrStatus::Superseded),
            _ => Err(format!("Invalid ADR status: {}", s)),
        }
    }
}

/// ADR frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrFrontmatter {
    /// Unique identifier (e.g., "ADR-0001")
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Current status in lifecycle
    #[serde(default)]
    pub status: AdrStatus,
    /// Primary bounded context this ADR applies to (e.g., "work-management")
    #[serde(default)]
    pub context: Option<String>,
    /// Additional scopes this ADR applies to (e.g., ["all"], ["state-machines"])
    #[serde(default, rename = "applies-to")]
    pub applies_to: Vec<String>,
    /// ADR IDs that this decision supersedes
    #[serde(default)]
    pub supersedes: Vec<String>,
    /// ADR ID that superseded this decision (if superseded)
    #[serde(default, rename = "superseded-by")]
    pub superseded_by: Option<String>,
    /// Reason for rejection (if rejected)
    #[serde(default, rename = "rejection-reason")]
    pub rejection_reason: Option<String>,
    /// Reason for deprecation (if deprecated)
    #[serde(default, rename = "deprecation-reason")]
    pub deprecation_reason: Option<String>,
    /// Date the decision was made
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub decided_at: Option<NaiveDateTime>,
    /// Ordering within the board (lower = earlier)
    #[serde(default)]
    pub index: Option<u32>,
}

/// An ADR with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Adr {
    /// Parsed frontmatter
    pub frontmatter: AdrFrontmatter,
    /// Path to the ADR file
    #[allow(dead_code)] // Available for file operations
    pub path: PathBuf,
}

impl Adr {
    /// Get the ADR ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the ADR title
    #[allow(dead_code)] // Accessor for future CLI display
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the index number (for ordering within the board)
    pub fn index(&self) -> Option<u32> {
        self.frontmatter.index
    }

    /// Get the ADR status
    #[allow(dead_code)] // Used by future doctor checks (SRS-05)
    pub fn status(&self) -> AdrStatus {
        self.frontmatter.status
    }

    /// Get the primary bounded context
    #[allow(dead_code)] // Used by future doctor checks (SRS-05)
    pub fn context(&self) -> Option<&str> {
        self.frontmatter.context.as_deref()
    }

    /// Check if this ADR is active (accepted and not superseded)
    #[allow(dead_code)] // Used by future doctor checks
    pub fn is_active(&self) -> bool {
        self.frontmatter.status == AdrStatus::Accepted
    }

    /// Check if this ADR is blocking (proposed status blocks new work in its context)
    #[allow(dead_code)] // Used by future doctor checks (SRS-05)
    pub fn is_blocking(&self) -> bool {
        self.frontmatter.status == AdrStatus::Proposed
    }

    /// Check if ADR matches a pattern (fuzzy match)
    pub fn matches(&self, pattern: &str) -> bool {
        super::fuzzy_match(&self.frontmatter.id, &self.frontmatter.title, pattern)
    }
}

impl Entity for Adr {
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

impl FuzzyMatch for Adr {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }

    fn matches(&self, pattern: &str) -> bool {
        Adr::matches(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adr_status_serializes_lowercase() {
        assert_eq!(
            serde_yaml::to_string(&AdrStatus::Proposed).unwrap().trim(),
            "proposed"
        );
        assert_eq!(
            serde_yaml::to_string(&AdrStatus::Accepted).unwrap().trim(),
            "accepted"
        );
        assert_eq!(
            serde_yaml::to_string(&AdrStatus::Deprecated)
                .unwrap()
                .trim(),
            "deprecated"
        );
        assert_eq!(
            serde_yaml::to_string(&AdrStatus::Superseded)
                .unwrap()
                .trim(),
            "superseded"
        );
    }

    #[test]
    fn adr_status_deserializes() {
        let yaml = "proposed";
        let status: AdrStatus = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(status, AdrStatus::Proposed);

        let yaml = "accepted";
        let status: AdrStatus = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(status, AdrStatus::Accepted);
    }

    #[test]
    fn adr_frontmatter_deserializes() {
        let yaml = r#"
id: ADR-0001
title: 2-Queue Pull System
status: accepted
context: work-management
applies-to: [all]
supersedes: []
superseded-by: null
decided_at: 2025-01-15T00:00:00
"#;
        let fm: AdrFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "ADR-0001");
        assert_eq!(fm.title, "2-Queue Pull System");
        assert_eq!(fm.status, AdrStatus::Accepted);
        assert_eq!(fm.context, Some("work-management".to_string()));
        assert_eq!(fm.applies_to, vec!["all".to_string()]);
        assert!(fm.supersedes.is_empty());
        assert!(fm.superseded_by.is_none());
        assert!(fm.decided_at.is_some());
    }

    #[test]
    fn adr_frontmatter_handles_defaults() {
        let yaml = r#"
id: ADR-0002
title: Test ADR
"#;
        let fm: AdrFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.status, AdrStatus::Proposed);
        assert!(fm.context.is_none());
        assert!(fm.applies_to.is_empty());
        assert!(fm.supersedes.is_empty());
        assert!(fm.superseded_by.is_none());
        assert!(fm.decided_at.is_none());
    }

    #[test]
    fn adr_matches() {
        let adr = Adr {
            frontmatter: AdrFrontmatter {
                id: "ADR-0001".to_string(),
                title: "2-Queue Pull System".to_string(),
                status: AdrStatus::Accepted,
                context: Some("work-management".to_string()),
                applies_to: vec!["all".to_string()],
                supersedes: vec![],
                superseded_by: None,
                rejection_reason: None,
                deprecation_reason: None,
                decided_at: None,
                index: Some(1),
            },
            path: PathBuf::from("test"),
        };

        assert!(adr.matches("ADR-0001"));
        assert!(adr.matches("0001"));
        assert!(adr.matches("queue"));
        assert!(adr.matches("pull"));
        assert!(!adr.matches("nonexistent"));
    }

    #[test]
    fn adr_display_status() {
        assert_eq!(AdrStatus::Proposed.to_string(), "proposed");
        assert_eq!(AdrStatus::Accepted.to_string(), "accepted");
        assert_eq!(AdrStatus::Rejected.to_string(), "rejected");
        assert_eq!(AdrStatus::Deprecated.to_string(), "deprecated");
        assert_eq!(AdrStatus::Superseded.to_string(), "superseded");
    }

    #[test]
    fn adr_status_from_str() {
        assert_eq!(
            "proposed".parse::<AdrStatus>().unwrap(),
            AdrStatus::Proposed
        );
        assert_eq!(
            "accepted".parse::<AdrStatus>().unwrap(),
            AdrStatus::Accepted
        );
        assert_eq!(
            "deprecated".parse::<AdrStatus>().unwrap(),
            AdrStatus::Deprecated
        );
        assert_eq!(
            "superseded".parse::<AdrStatus>().unwrap(),
            AdrStatus::Superseded
        );
        // Case insensitive
        assert_eq!(
            "PROPOSED".parse::<AdrStatus>().unwrap(),
            AdrStatus::Proposed
        );
        // Invalid
        assert!("invalid".parse::<AdrStatus>().is_err());
    }

    #[test]
    fn adr_is_active() {
        let accepted = Adr {
            frontmatter: AdrFrontmatter {
                id: "ADR-0001".to_string(),
                title: "Test".to_string(),
                status: AdrStatus::Accepted,
                context: None,
                applies_to: vec![],
                supersedes: vec![],
                superseded_by: None,
                rejection_reason: None,
                deprecation_reason: None,
                decided_at: None,
                index: Some(1),
            },
            path: PathBuf::from("test"),
        };
        assert!(accepted.is_active());

        let proposed = Adr {
            frontmatter: AdrFrontmatter {
                id: "ADR-0002".to_string(),
                title: "Test".to_string(),
                status: AdrStatus::Proposed,
                context: None,
                applies_to: vec![],
                supersedes: vec![],
                superseded_by: None,
                rejection_reason: None,
                deprecation_reason: None,
                decided_at: None,
                index: Some(2),
            },
            path: PathBuf::from("test"),
        };
        assert!(!proposed.is_active());
    }

    #[test]
    fn adr_is_blocking() {
        let proposed = Adr {
            frontmatter: AdrFrontmatter {
                id: "ADR-0001".to_string(),
                title: "Test".to_string(),
                status: AdrStatus::Proposed,
                context: None,
                applies_to: vec![],
                supersedes: vec![],
                superseded_by: None,
                rejection_reason: None,
                deprecation_reason: None,
                decided_at: None,
                index: Some(1),
            },
            path: PathBuf::from("test"),
        };
        assert!(proposed.is_blocking());

        let accepted = Adr {
            frontmatter: AdrFrontmatter {
                id: "ADR-0002".to_string(),
                title: "Test".to_string(),
                status: AdrStatus::Accepted,
                context: None,
                applies_to: vec![],
                supersedes: vec![],
                superseded_by: None,
                rejection_reason: None,
                deprecation_reason: None,
                decided_at: None,
                index: Some(2),
            },
            path: PathBuf::from("test"),
        };
        assert!(!accepted.is_blocking());
    }
}
