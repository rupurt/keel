#![allow(dead_code)]
//! Bearing data structure
//!
//! Bearings are research entities that represent directions worth investigating
//! before committing to an epic. They live in .keel/bearings/<name>/.

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Entity, FuzzyMatch, deserialize_strict_datetime};

/// Bearing status lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BearingStatus {
    /// Initial research phase
    #[default]
    Exploring,
    /// Actively gathering data (has SURVEY.md)
    Evaluating,
    /// Ready for decision (has ASSESSMENT.md)
    Ready,
    /// Graduated to epic via `lay` command
    Laid,
    /// Shelved for later
    Parked,
    /// Rejected with reason
    Declined,
}

impl std::fmt::Display for BearingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BearingStatus::Exploring => write!(f, "exploring"),
            BearingStatus::Evaluating => write!(f, "evaluating"),
            BearingStatus::Ready => write!(f, "ready"),
            BearingStatus::Laid => write!(f, "laid"),
            BearingStatus::Parked => write!(f, "parked"),
            BearingStatus::Declined => write!(f, "declined"),
        }
    }
}

impl std::str::FromStr for BearingStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "exploring" => Ok(BearingStatus::Exploring),
            "evaluating" => Ok(BearingStatus::Evaluating),
            "ready" => Ok(BearingStatus::Ready),
            "laid" => Ok(BearingStatus::Laid),
            "parked" => Ok(BearingStatus::Parked),
            "declined" => Ok(BearingStatus::Declined),
            _ => Err(format!("Invalid bearing status: {}", s)),
        }
    }
}

/// Bearing frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearingFrontmatter {
    /// Unique identifier (same as directory name)
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Current status in lifecycle
    #[serde(default)]
    pub status: BearingStatus,
    /// Ordering within the board (lower = earlier)
    #[serde(default)]
    pub index: Option<u32>,
    /// Creation date
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub created_at: Option<NaiveDateTime>,
    /// Decline reason (if status is declined)
    #[serde(default)]
    pub decline_reason: Option<String>,
    /// Date bearing was laid (graduated to epic)
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub laid_at: Option<NaiveDateTime>,
}

/// A bearing with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Bearing {
    /// Parsed frontmatter from BRIEF.md
    pub frontmatter: BearingFrontmatter,
    /// Path to the bearing BRIEF.md
    #[allow(dead_code)] // Available for file operations
    pub path: PathBuf,
    /// Whether SURVEY.md exists
    pub has_survey: bool,
    /// Whether ASSESSMENT.md exists
    pub has_assessment: bool,
}

impl Bearing {
    /// Get the bearing ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the bearing title
    #[allow(dead_code)] // Accessor for future CLI display
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the bearing status
    pub fn status(&self) -> BearingStatus {
        self.frontmatter.status
    }

    /// Sort key for bearing prioritization.
    ///
    /// Orders bearings by actionability: Ready (closest to generating work) first,
    /// then by progress within status (more artifacts = clearer next step), then
    /// alphabetically for determinism.
    ///
    /// Used by both `keel next` and `keel flow` to recommend the same bearing.
    pub fn priority_key(&self) -> (u8, u8, &str) {
        let status_rank = match self.frontmatter.status {
            BearingStatus::Ready => 0,
            BearingStatus::Evaluating => 1,
            BearingStatus::Exploring => 2,
            // Terminal states sort last (shouldn't appear in active lists)
            _ => 3,
        };
        let progress_rank = match (self.has_survey, self.has_assessment) {
            (true, true) => 0,
            (true, false) => 1,
            _ => 2,
        };
        (status_rank, progress_rank, self.id())
    }

    /// Check if this bearing is in a terminal state (laid, parked, or declined)
    pub fn is_complete(&self) -> bool {
        matches!(
            self.frontmatter.status,
            BearingStatus::Laid | BearingStatus::Parked | BearingStatus::Declined
        )
    }

    /// Check if bearing matches a pattern (fuzzy match)
    pub fn matches(&self, pattern: &str) -> bool {
        super::fuzzy_match(&self.frontmatter.id, &self.frontmatter.title, pattern)
    }
}

impl Entity for Bearing {
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

impl FuzzyMatch for Bearing {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }

    fn matches(&self, pattern: &str) -> bool {
        Bearing::matches(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearing_status_serializes_kebab_case() {
        assert_eq!(
            serde_yaml::to_string(&BearingStatus::Exploring)
                .unwrap()
                .trim(),
            "exploring"
        );
        assert_eq!(
            serde_yaml::to_string(&BearingStatus::Evaluating)
                .unwrap()
                .trim(),
            "evaluating"
        );
        assert_eq!(
            serde_yaml::to_string(&BearingStatus::Ready).unwrap().trim(),
            "ready"
        );
        assert_eq!(
            serde_yaml::to_string(&BearingStatus::Laid).unwrap().trim(),
            "laid"
        );
        assert_eq!(
            serde_yaml::to_string(&BearingStatus::Parked)
                .unwrap()
                .trim(),
            "parked"
        );
        assert_eq!(
            serde_yaml::to_string(&BearingStatus::Declined)
                .unwrap()
                .trim(),
            "declined"
        );
    }

    #[test]
    fn bearing_status_deserializes() {
        let yaml = "exploring";
        let status: BearingStatus = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(status, BearingStatus::Exploring);

        let yaml = "evaluating";
        let status: BearingStatus = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(status, BearingStatus::Evaluating);
    }

    #[test]
    fn bearing_frontmatter_deserializes() {
        let yaml = r#"
id: ai-powered-search
title: AI-Powered Search
status: exploring
created_at: 2026-01-28T12:00:00
"#;
        let fm: BearingFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "ai-powered-search");
        assert_eq!(fm.title, "AI-Powered Search");
        assert_eq!(fm.status, BearingStatus::Exploring);
        assert!(fm.created_at.is_some());
    }

    #[test]
    fn bearing_frontmatter_deserializes_laid_at() {
        let yaml = r#"
id: ai-powered-search
title: AI-Powered Search
status: laid
laid_at: 2026-01-28T12:00:00
"#;
        let fm: BearingFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.status, BearingStatus::Laid);
        assert!(fm.laid_at.is_some());
        assert_eq!(
            fm.laid_at,
            Some(
                NaiveDateTime::parse_from_str("2026-01-28T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap()
            )
        );
    }

    #[test]
    fn bearing_frontmatter_handles_defaults() {
        let yaml = r#"
id: test
title: Test Bearing
"#;
        let fm: BearingFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.status, BearingStatus::Exploring);
        assert!(fm.created_at.is_none());
        assert!(fm.decline_reason.is_none());
    }

    #[test]
    fn bearing_matches() {
        let b = Bearing {
            frontmatter: BearingFrontmatter {
                id: "ai-powered-search".to_string(),
                title: "AI-Powered Search".to_string(),
                status: BearingStatus::Exploring,
                index: None,
                created_at: None,
                decline_reason: None,
                laid_at: None,
            },
            path: PathBuf::from("test"),
            has_survey: false,
            has_assessment: false,
        };

        assert!(b.matches("ai-powered"));
        assert!(b.matches("search"));
        assert!(b.matches("AI"));
        assert!(b.matches("powered"));
        assert!(!b.matches("voyages"));
    }

    #[test]
    fn bearing_display_status() {
        assert_eq!(BearingStatus::Exploring.to_string(), "exploring");
        assert_eq!(BearingStatus::Declined.to_string(), "declined");
    }

    #[test]
    fn bearing_status_from_str() {
        assert_eq!(
            "exploring".parse::<BearingStatus>().unwrap(),
            BearingStatus::Exploring
        );
        assert_eq!(
            "evaluating".parse::<BearingStatus>().unwrap(),
            BearingStatus::Evaluating
        );
        assert_eq!(
            "ready".parse::<BearingStatus>().unwrap(),
            BearingStatus::Ready
        );
        assert_eq!(
            "laid".parse::<BearingStatus>().unwrap(),
            BearingStatus::Laid
        );
        assert_eq!(
            "parked".parse::<BearingStatus>().unwrap(),
            BearingStatus::Parked
        );
        assert_eq!(
            "declined".parse::<BearingStatus>().unwrap(),
            BearingStatus::Declined
        );
        // Case insensitive
        assert_eq!(
            "EXPLORING".parse::<BearingStatus>().unwrap(),
            BearingStatus::Exploring
        );
        // Invalid
        assert!("invalid".parse::<BearingStatus>().is_err());
    }

    fn make_bearing(id: &str, status: BearingStatus, survey: bool, assessment: bool) -> Bearing {
        Bearing {
            frontmatter: BearingFrontmatter {
                id: id.to_string(),
                title: id.to_string(),
                status,
                index: None,
                created_at: None,
                decline_reason: None,
                laid_at: None,
            },
            path: PathBuf::from("test"),
            has_survey: survey,
            has_assessment: assessment,
        }
    }

    #[test]
    fn priority_key_ready_before_evaluating_before_exploring() {
        let ready = make_bearing("a", BearingStatus::Ready, true, true);
        let evaluating = make_bearing("a", BearingStatus::Evaluating, true, true);
        let exploring = make_bearing("a", BearingStatus::Exploring, true, true);

        assert!(ready.priority_key() < evaluating.priority_key());
        assert!(evaluating.priority_key() < exploring.priority_key());
    }

    #[test]
    fn priority_key_progress_within_same_status() {
        let both = make_bearing("a", BearingStatus::Evaluating, true, true);
        let survey_only = make_bearing("a", BearingStatus::Evaluating, true, false);
        let neither = make_bearing("a", BearingStatus::Evaluating, false, false);

        assert!(both.priority_key() < survey_only.priority_key());
        assert!(survey_only.priority_key() < neither.priority_key());
    }

    #[test]
    fn priority_key_alphabetical_tiebreaker() {
        let alpha = make_bearing("alpha", BearingStatus::Exploring, false, false);
        let beta = make_bearing("beta", BearingStatus::Exploring, false, false);

        assert!(alpha.priority_key() < beta.priority_key());
    }

    #[test]
    fn priority_key_status_trumps_progress() {
        // A Ready bearing with no artifacts beats an Exploring bearing with both
        let ready_empty = make_bearing("z", BearingStatus::Ready, false, false);
        let exploring_full = make_bearing("a", BearingStatus::Exploring, true, true);

        assert!(ready_empty.priority_key() < exploring_full.priority_key());
    }
}
