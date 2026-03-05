//! Story data structure

use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Entity, FuzzyMatch, StoryState, StoryType, deserialize_strict_datetime};
use crate::domain::model::taxonomy::{self, ParseError, RoleTaxonomy};

/// Story frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryFrontmatter {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub story_type: StoryType,
    pub status: StoryState,
    #[serde(default)]
    pub scope: Option<String>,
    #[serde(default)]
    pub milestone: Option<String>,
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
    /// Datetime submitted for acceptance
    #[serde(default, deserialize_with = "deserialize_strict_datetime")]
    pub submitted_at: Option<NaiveDateTime>,
    /// Index number for ordering within scope (lower = earlier)
    #[serde(default)]
    pub index: Option<u32>,
    /// ADR IDs that govern this story's implementation
    #[serde(default, rename = "governed-by")]
    pub governed_by: Vec<String>,
    /// Story IDs that should not be run in parallel with this story
    #[serde(default)]
    pub blocked_by: Vec<String>,
    /// Role taxonomy string specifying required actor capabilities
    #[serde(default)]
    pub role: Option<String>,
}

/// A story with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Story {
    /// Parsed frontmatter
    pub frontmatter: StoryFrontmatter,
    /// Path to the story file
    pub path: PathBuf,
    /// Which stage directory the story is in
    pub stage: StoryState,
}

impl Story {
    /// Get the story ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the story title
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the story type
    pub fn story_type(&self) -> StoryType {
        self.frontmatter.story_type
    }

    /// Get the index number (for ordering within scope)
    pub fn index(&self) -> Option<u32> {
        self.frontmatter.index
    }

    /// Get the scope (epic/voyage path)
    pub fn scope(&self) -> Option<&str> {
        self.frontmatter.scope.as_deref()
    }

    /// Get the epic ID derived from scope.
    ///
    /// The scope format is `epic/voyage`, so this returns the first part.
    pub fn epic(&self) -> Option<&str> {
        self.frontmatter
            .scope
            .as_ref()
            .and_then(|s| s.split('/').next())
    }

    /// Get the voyage ID derived from scope.
    ///
    /// The scope format is `epic/voyage`, so this returns the second part.
    /// Returns None if scope only contains an epic (no voyage).
    #[allow(dead_code)] // Accessor for future voyage-based filtering
    pub fn voyage(&self) -> Option<&str> {
        self.frontmatter
            .scope
            .as_ref()
            .and_then(|s| s.split('/').nth(1))
    }

    /// Get the filename of this story (without parent path)
    ///
    /// Returns the filename from the path, or "unknown" if the path has no filename.
    pub fn filename(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".to_string())
    }

    /// Get the required role taxonomy, if specified.
    ///
    /// Returns `None` if no role is set, `Some(Ok(taxonomy))` if valid,
    /// or `Some(Err(ParseError))` if the role string is invalid.
    #[allow(dead_code)] // Used by tests now, production use in voyage 03 (actor-story matching)
    pub fn required_role(&self) -> Option<Result<RoleTaxonomy, ParseError>> {
        self.frontmatter.role.as_ref().map(|r| taxonomy::parse(r))
    }

    /// Check if story matches a pattern (fuzzy match)
    pub fn matches(&self, pattern: &str) -> bool {
        super::fuzzy_match(&self.frontmatter.id, &self.frontmatter.title, pattern)
    }
}

impl Entity for Story {
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

impl FuzzyMatch for Story {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }

    fn matches(&self, pattern: &str) -> bool {
        Story::matches(self, pattern)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_frontmatter_yaml() -> &'static str {
        r#"
id: FEAT0238
title: Create board crate
type: feat
status: backlog
scope: board-cli/01-core-infrastructure
created_at: 2025-01-23T10:00:00
updated_at: 2025-01-23T10:00:00
started_at: 2025-01-23T12:00:00
"#
    }

    #[test]
    fn story_frontmatter_deserializes() {
        let fm: StoryFrontmatter = serde_yaml::from_str(sample_frontmatter_yaml()).unwrap();

        assert_eq!(fm.id, "FEAT0238");
        assert_eq!(fm.title, "Create board crate");
        assert_eq!(fm.story_type, StoryType::Feat);
        assert_eq!(fm.status, StoryState::Backlog);
        assert_eq!(
            fm.scope,
            Some("board-cli/01-core-infrastructure".to_string())
        );
    }

    #[test]
    fn story_frontmatter_handles_missing_optional_fields() {
        let yaml = r#"
id: BUG0001
title: Fix crash
type: bug
status: in-progress
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.id, "BUG0001");
        assert!(fm.scope.is_none());
    }

    #[test]
    fn story_frontmatter_accepts_datetime_fields() {
        // New format (created_at, updated_at, completed_at, submitted_at) with datetime
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: done
created_at: 2026-01-01T09:00:00
updated_at: 2026-01-15T14:30:00
started_at: 2026-01-10T08:00:00
submitted_at: 2026-01-20T16:45:00
completed_at: 2026-01-29T11:00:00
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(
            fm.created_at,
            NaiveDateTime::parse_from_str("2026-01-01T09:00:00", "%Y-%m-%dT%H:%M:%S").ok()
        );
        assert_eq!(
            fm.updated_at,
            NaiveDateTime::parse_from_str("2026-01-15T14:30:00", "%Y-%m-%dT%H:%M:%S").ok()
        );
        assert_eq!(
            fm.started_at,
            NaiveDateTime::parse_from_str("2026-01-10T08:00:00", "%Y-%m-%dT%H:%M:%S").ok()
        );
        assert_eq!(
            fm.submitted_at,
            NaiveDateTime::parse_from_str("2026-01-20T16:45:00", "%Y-%m-%dT%H:%M:%S").ok()
        );
        assert_eq!(
            fm.completed_at,
            NaiveDateTime::parse_from_str("2026-01-29T11:00:00", "%Y-%m-%dT%H:%M:%S").ok()
        );
    }

    #[test]
    fn story_matches_by_id() {
        let story = Story {
            frontmatter: serde_yaml::from_str(sample_frontmatter_yaml()).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };

        assert!(story.matches("FEAT0238"));
        assert!(story.matches("feat0238")); // case insensitive
        assert!(story.matches("0238")); // partial
        assert!(story.matches("Create")); // title match
        assert!(!story.matches("BUG"));
    }

    #[test]
    fn story_frontmatter_accepts_index_field() {
        let yaml = r#"
id: FEAT0001
title: First story
type: feat
status: backlog
scope: auth/02-social-login
index: 1
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.index, Some(1));
    }

    #[test]
    fn story_frontmatter_index_defaults_to_none() {
        let yaml = r#"
id: FEAT0001
title: First story
type: feat
status: backlog
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.index, None);
    }

    #[test]
    fn filename_extracts_from_path() {
        let story = Story {
            frontmatter: serde_yaml::from_str(sample_frontmatter_yaml()).unwrap(),
            path: PathBuf::from("/board/stories/[FEAT][0238]-create-board-crate.md"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.filename(), "[FEAT][0238]-create-board-crate.md");
    }

    #[test]
    fn filename_handles_missing_filename() {
        let story = Story {
            frontmatter: serde_yaml::from_str(sample_frontmatter_yaml()).unwrap(),
            path: PathBuf::from("/"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.filename(), "unknown");
    }

    #[test]
    fn epic_derives_from_scope() {
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
scope: core/01-session
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.epic(), Some("core"));
    }

    #[test]
    fn epic_returns_none_when_no_scope() {
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.epic(), None);
    }

    #[test]
    fn voyage_derives_from_scope() {
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
scope: core/01-session
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.voyage(), Some("01-session"));
    }

    #[test]
    fn voyage_returns_none_for_epic_only_scope() {
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
scope: core
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.voyage(), None);
    }

    #[test]
    fn voyage_returns_none_when_no_scope() {
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        assert_eq!(story.voyage(), None);
    }

    // === governed-by field tests (SRS-08) ===

    #[test]
    fn story_frontmatter_supports_governed_by_field() {
        // [SRS-08/AC-01] StoryFrontmatter struct supports optional governed-by: Vec<String>
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
governed-by: [ADR-0001, ADR-0003]
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.governed_by, vec!["ADR-0001", "ADR-0003"]);
    }

    #[test]
    fn story_frontmatter_without_governed_by_parses_successfully() {
        // [SRS-08/AC-02] Stories without governed-by parse successfully (backward compatible)
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert!(fm.governed_by.is_empty());
    }

    #[test]
    fn story_frontmatter_governed_by_serializes_correctly() {
        // [SRS-08/AC-03] Stories with governed-by field serialize/deserialize correctly
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
governed-by: [ADR-0001]
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        // Verify deserialization
        assert_eq!(fm.governed_by, vec!["ADR-0001"]);

        // Verify serialization roundtrip
        let serialized = serde_yaml::to_string(&fm).unwrap();
        let fm2: StoryFrontmatter = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(fm2.governed_by, vec!["ADR-0001"]);
    }

    // === role field tests (SRS-01, SRS-04) ===

    // === required_role() method tests (SRS-02, SRS-03, SRS-05) ===

    #[test]
    fn required_role_returns_none_when_no_role() {
        // [SRS-05/AC-02] When `role` is None, method returns `None`
        let story = Story {
            frontmatter: serde_yaml::from_str(sample_frontmatter_yaml()).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        assert!(story.required_role().is_none());
    }

    #[test]
    fn required_role_returns_parsed_taxonomy_for_valid_role() {
        // [SRS-05/AC-01] returns `Option<Result<RoleTaxonomy, ParseError>>`
        // [SRS-02/AC-01] When `role` is valid taxonomy string, returns `Some(Ok(taxonomy))`
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
role: "engineer/software"
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        let result = story.required_role();
        assert!(result.is_some());
        let taxonomy = result.unwrap().unwrap();
        assert_eq!(taxonomy.role, "engineer");
        assert_eq!(taxonomy.specialization, Some("software".to_string()));
    }

    #[test]
    fn required_role_returns_error_for_invalid_syntax() {
        // [SRS-03/AC-01] When `role` is invalid syntax, returns `Some(Err(ParseError))`
        let yaml = r#"
id: FEAT0001
title: Test
type: feat
status: backlog
role: ""
"#;
        let story = Story {
            frontmatter: serde_yaml::from_str(yaml).unwrap(),
            path: PathBuf::from("test.md"),
            stage: StoryState::Backlog,
        };
        let result = story.required_role();
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    // === role field tests (SRS-01, SRS-04) ===

    #[test]
    fn story_frontmatter_supports_role_field() {
        // [SRS-01/AC-01] StoryFrontmatter has `pub role: Option<String>` field
        // [SRS-01/AC-02] Stories with `role: "engineer"` deserialize with `role: Some("engineer")`
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
role: "engineer"
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(fm.role, Some("engineer".to_string()));
    }

    #[test]
    fn story_frontmatter_without_role_parses_successfully() {
        // [SRS-04/AC-01] Field is decorated with `#[serde(default)]` for backward compatibility
        // [SRS-04/AC-02] Stories without `role` field in YAML deserialize with `role: None`
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(fm.role, None);
    }

    #[test]
    fn story_frontmatter_governed_by_handles_empty_list() {
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
governed-by: []
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert!(fm.governed_by.is_empty());
    }

    #[test]
    fn next_parallel_blocked_by_frontmatter_parses() {
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
blocked_by: [FEAT0002, FEAT0003]
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(fm.blocked_by, vec!["FEAT0002", "FEAT0003"]);
    }

    #[test]
    fn story_frontmatter_without_blocked_by_defaults_empty() {
        let yaml = r#"
id: FEAT0001
title: Test story
type: feat
status: backlog
"#;
        let fm: StoryFrontmatter = serde_yaml::from_str(yaml).unwrap();

        assert!(fm.blocked_by.is_empty());
    }
}
