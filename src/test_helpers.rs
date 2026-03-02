#![allow(dead_code)]
//! Test helpers for creating test fixtures.
//!
//! This module provides a builder pattern for creating test boards,
//! eliminating duplicated test fixture code across command tests.

use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::domain::model::StoryState;

/// Builder for creating test boards with flexible configuration.
pub struct TestBoardBuilder {
    temp: TempDir,
    stories: Vec<TestStory>,
    epics: Vec<TestEpic>,
    voyages: Vec<TestVoyage>,
    adrs: Vec<TestAdr>,
    bearings: Vec<TestBearing>,
}

/// Configuration for a test story.
pub struct TestStory {
    pub id: String,
    pub title: String,
    pub story_type: String,
    pub stage: StoryState,
    pub scope: Option<String>,
    pub index: Option<u32>,
    pub body: String,
    pub role: Option<String>,
}

/// Configuration for a test epic.
pub struct TestEpic {
    pub id: String,
    pub title: String,
    pub status: String,
    pub index: Option<u32>,
}

/// Configuration for a test voyage.
pub struct TestVoyage {
    pub id: String,
    pub title: String,
    pub status: String,
    pub epic_id: String,
    pub srs_content: Option<String>,
    pub index: Option<u32>,
}

/// Configuration for a test ADR.
pub struct TestAdr {
    pub id: String,
    pub title: String,
    pub status: String,
    pub context: Option<String>,
    pub applies_to: Vec<String>,
}

/// Configuration for a test bearing.
pub struct TestBearing {
    pub id: String,
    pub title: String,
    pub status: String,
    pub has_survey: bool,
    pub has_assessment: bool,
    pub index: Option<u32>,
}

impl Default for TestStory {
    fn default() -> Self {
        Self {
            id: "TEST0001".to_string(),
            title: "Test Story".to_string(),
            story_type: "feat".to_string(),
            stage: StoryState::Backlog,
            scope: None,
            index: None,
            body: "Body content".to_string(),
            role: None,
        }
    }
}

impl TestStory {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn story_type(mut self, t: &str) -> Self {
        self.story_type = t.to_string();
        self
    }

    pub fn stage(mut self, stage: StoryState) -> Self {
        self.stage = stage;
        self
    }

    pub fn scope(mut self, scope: &str) -> Self {
        self.scope = Some(scope.to_string());
        self
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    /// Generate frontmatter for this story.
    fn frontmatter(&self) -> String {
        let mut fm = format!(
            "---\nid: {}\ntitle: {}\ntype: {}\nstatus: {}\n",
            self.id, self.title, self.story_type, self.stage
        );

        if let Some(ref scope) = self.scope {
            fm.push_str(&format!("scope: {}\n", scope));
        }

        if let Some(index) = self.index {
            fm.push_str(&format!("index: {}\n", index));
        }

        if let Some(ref role) = self.role {
            fm.push_str(&format!("role: \"{}\"\n", role));
        }

        fm.push_str("---\n");
        fm
    }

    /// Generate full content for this story.
    fn content(&self) -> String {
        format!("{}{}\n", self.frontmatter(), self.body)
    }
}

impl Default for TestEpic {
    fn default() -> Self {
        Self {
            id: "test-epic".to_string(),
            title: "Test Epic".to_string(),
            status: "strategic".to_string(),
            index: None,
        }
    }
}

impl TestEpic {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: format!("{} Epic", id),
            ..Default::default()
        }
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    #[allow(dead_code)] // Test helper for future use
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }
}

impl Default for TestVoyage {
    fn default() -> Self {
        Self {
            id: "01-voyage".to_string(),
            title: "Test Voyage".to_string(),
            status: "in-progress".to_string(),
            epic_id: "test-epic".to_string(),
            srs_content: None,
            index: None,
        }
    }
}

impl TestVoyage {
    pub fn new(id: &str, epic_id: &str) -> Self {
        Self {
            id: id.to_string(),
            epic_id: epic_id.to_string(),
            title: format!("{} Voyage", id),
            ..Default::default()
        }
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    #[allow(dead_code)] // Test helper for future use
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn srs_content(mut self, content: &str) -> Self {
        self.srs_content = Some(content.to_string());
        self
    }
}

impl Default for TestAdr {
    fn default() -> Self {
        Self {
            id: "ADR-0001".to_string(),
            title: "Test ADR".to_string(),
            status: "proposed".to_string(),
            context: None,
            applies_to: vec![],
        }
    }
}

impl TestAdr {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn applies_to(mut self, applies: Vec<&str>) -> Self {
        self.applies_to = applies.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Generate filename for this ADR.
    fn filename(&self) -> String {
        let slug = self.title.to_lowercase().replace(' ', "-");
        format!("{}-{}.md", self.id, slug)
    }

    /// Generate full content for this ADR.
    fn content(&self) -> String {
        let applies_to_str = if self.applies_to.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}]", self.applies_to.join(", "))
        };
        let context_str = self.context.as_deref().unwrap_or("null");

        format!(
            r#"---
id: {}
index: 1
title: {}
status: {}
context: {}
applies-to: {}
supersedes: []
superseded-by: null
decided_at: 2026-01-01T00:00:00
---

# {}

## Status

**Proposed**

## Context

Test ADR content.
"#,
            self.id, self.title, self.status, context_str, applies_to_str, self.title
        )
    }
}

impl Default for TestBearing {
    fn default() -> Self {
        Self {
            id: "BRG-01".to_string(),
            title: "Test Bearing".to_string(),
            status: "exploring".to_string(),
            has_survey: false,
            has_assessment: false,
            index: None,
        }
    }
}

impl TestBearing {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn has_survey(mut self, has_survey: bool) -> Self {
        self.has_survey = has_survey;
        self
    }

    pub fn has_assessment(mut self, has_assessment: bool) -> Self {
        self.has_assessment = has_assessment;
        self
    }
}

impl TestBoardBuilder {
    /// Create a new test board builder.
    pub fn new() -> Self {
        Self {
            temp: TempDir::new().unwrap(),
            stories: vec![],
            epics: vec![],
            voyages: vec![],
            adrs: vec![],
            bearings: vec![],
        }
    }

    /// Add a story to the board.
    pub fn story(mut self, story: TestStory) -> Self {
        self.stories.push(story);
        self
    }

    /// Add an epic to the board.
    pub fn epic(mut self, epic: TestEpic) -> Self {
        self.epics.push(epic);
        self
    }

    /// Add a voyage to the board.
    pub fn voyage(mut self, voyage: TestVoyage) -> Self {
        self.voyages.push(voyage);
        self
    }

    /// Add an ADR to the board.
    pub fn adr(mut self, adr: TestAdr) -> Self {
        self.adrs.push(adr);
        self
    }

    /// Add a bearing to the board.
    pub fn bearing(mut self, bearing: TestBearing) -> Self {
        self.bearings.push(bearing);
        self
    }

    /// Build the test board and return the temp directory.
    pub fn build(self) -> TempDir {
        let root = self.temp.path();

        // Create stories directory
        fs::create_dir_all(root.join("stories")).unwrap();

        // Ensure at least one epic exists (required for valid board)
        let epics = if self.epics.is_empty() {
            vec![TestEpic::default()]
        } else {
            self.epics
        };

        // Create epics
        for epic in &epics {
            let epic_dir = root.join("epics").join(&epic.id);
            fs::create_dir_all(&epic_dir).unwrap();
            let index_line = epic
                .index
                .map(|i| format!("index: {}\n", i))
                .unwrap_or_default();
            fs::write(
                        epic_dir.join("README.md"),
                        format!(
                            "---\nid: {}\ntitle: {}\nstatus: {}\n{}---\n\n# {}\n\n## Voyages\n\n<!-- BEGIN GENERATED -->\n<!-- END GENERATED -->\n",
                            epic.id, epic.title, epic.status, index_line, epic.title
                        ),
                    )
                    .unwrap();

            // Create default PRD.md
            fs::write(
                        epic_dir.join("PRD.md"),
                        "# PRD\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n| FR-01 | Test |\n<!-- END FUNCTIONAL_REQUIREMENTS -->\n<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->\n| NFR-01 | Test |\n<!-- END NON_FUNCTIONAL_REQUIREMENTS -->\n<!-- BEGIN SUCCESS_CRITERIA -->\n- [ ] Success!\n<!-- END SUCCESS_CRITERIA -->\n"
                    ).unwrap();

            // Create default PRESS_RELEASE.md
            fs::write(
                epic_dir.join("PRESS_RELEASE.md"),
                format!(
                    "# PRESS RELEASE: {}\n\nKeel introduces {}.\n",
                    epic.title, epic.title
                ),
            )
            .unwrap();
        }

        // Create voyages
        for voyage in &self.voyages {
            let voyage_dir = root
                .join("epics")
                .join(&voyage.epic_id)
                .join("voyages")
                .join(&voyage.id);
            fs::create_dir_all(&voyage_dir).unwrap();
            let index_line = voyage
                .index
                .map(|i| format!("index: {}\n", i))
                .unwrap_or_default();
            fs::write(
                        voyage_dir.join("README.md"),
                        format!(
                            "---\nid: {}\ntitle: {}\nstatus: {}\nepic: {}\n{}---\n\n# {}\n\n## Stories\n\n<!-- BEGIN GENERATED -->\n<!-- END GENERATED -->\n",
                            voyage.id, voyage.title, voyage.status, voyage.epic_id, index_line, voyage.title
                        ),
                    )
                    .unwrap();

            // Write SRS.md if specified
            if let Some(ref srs) = voyage.srs_content {
                fs::write(voyage_dir.join("SRS.md"), srs).unwrap();
            } else {
                // Default minimal SRS for coverage checks
                fs::write(voyage_dir.join("SRS.md"), "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\nEND FUNCTIONAL_REQUIREMENTS").unwrap();
            }

            // Write SDD.md
            fs::write(
                voyage_dir.join("SDD.md"),
                "# SDD\n\n## Architecture\n[SRS link](SRS.md)",
            )
            .unwrap();
        }

        // Create story bundles
        for story in &self.stories {
            let story_bundle_dir = root.join("stories").join(&story.id);
            fs::create_dir_all(&story_bundle_dir).unwrap();
            let story_path = story_bundle_dir.join("README.md");
            fs::write(story_path, story.content()).unwrap();

            // Mandatory REFLECT.md with at least one knowledge unit for hardened gates
            fs::write(
                story_bundle_dir.join("REFLECT.md"),
                format!(
                    "# Reflection - {}\n\n### L-01: Test lesson\n\nSome insight.",
                    story.title
                ),
            )
            .unwrap();

            // Mandatory EVIDENCE directory
            fs::create_dir_all(story_bundle_dir.join("EVIDENCE")).unwrap();
        }

        // Create ADRs
        if !self.adrs.is_empty() {
            let adrs_dir = root.join("adrs");
            fs::create_dir_all(&adrs_dir).unwrap();
            for adr in &self.adrs {
                let adr_path = adrs_dir.join(adr.filename());
                fs::write(adr_path, adr.content()).unwrap();
            }
        }

        // Create bearings
        if !self.bearings.is_empty() {
            let bearings_dir = root.join("bearings");
            fs::create_dir_all(&bearings_dir).unwrap();
            for bearing in &self.bearings {
                let bearing_dir = bearings_dir.join(&bearing.id);
                fs::create_dir_all(&bearing_dir).unwrap();

                let index_line = bearing
                    .index
                    .map(|i| format!("index: {}\n", i))
                    .unwrap_or_default();

                // README.md (new source of truth)
                fs::write(
                                            bearing_dir.join("README.md"),
                                            format!(
                                                "---\nid: {}\ntitle: {}\nstatus: {}\ncreated_at: 2026-01-01T00:00:00\n{}---\n\n# {}\n\nSee [BRIEF.md](BRIEF.md) for details.\n",
                                                bearing.id, bearing.title, bearing.status, index_line, bearing.title
                                            )
                                        ).unwrap();

                // BRIEF.md
                fs::write(
                                            bearing_dir.join("BRIEF.md"),
                                            "# BRIEF\n\n## Hypothesis\nTest\n## Problem Space\nTest\n## Success Criteria\nTest\n## Open Questions\nTest\n"
                                        ).unwrap();

                // SURVEY.md
                if bearing.has_survey {
                    fs::write(bearing_dir.join("SURVEY.md"), "# SURVEY\n").unwrap();
                }
                // ASSESSMENT.md
                if bearing.has_assessment {
                    fs::write(bearing_dir.join("ASSESSMENT.md"), "# ASSESSMENT\n").unwrap();
                }
            }
        }

        self.temp
    }

    /// Get the path to the board directory.
    pub fn path(&self) -> &Path {
        self.temp.path()
    }
}

impl Default for TestBoardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to get story path in a test board.
pub fn story_path(temp: &TempDir, id: &str) -> PathBuf {
    temp.path().join("stories").join(id).join("README.md")
}

// ============================================================================
// In-memory model factories for unit tests
// ============================================================================

use chrono::NaiveDateTime;

use crate::domain::model::{
    Bearing, BearingFrontmatter, BearingStatus, Story, StoryFrontmatter, StoryType,
};

/// Builder for creating in-memory Story structs for unit tests.
///
/// Unlike TestStory (which creates files on disk), this creates actual
/// Story model instances for testing functions that operate on Story structs.
#[derive(Clone)]
pub struct StoryFactory {
    id: String,
    title: String,
    stage: StoryState,
    scope: Option<String>,
    index: Option<u32>,
    submitted_at: Option<NaiveDateTime>,
    completed_at: Option<NaiveDateTime>,
    role: Option<String>,
}

impl Default for StoryFactory {
    fn default() -> Self {
        Self {
            id: "test".to_string(),
            title: "Test Story".to_string(),
            stage: StoryState::Backlog,
            scope: None,
            index: None,
            submitted_at: None,
            completed_at: None,
            role: None,
        }
    }
}

impl StoryFactory {
    /// Create a new factory with the given ID.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: format!("Story {}", id),
            ..Default::default()
        }
    }

    /// Set the story title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the story stage.
    pub fn stage(mut self, stage: StoryState) -> Self {
        self.stage = stage;
        self
    }

    /// Set the story scope.
    pub fn scope(mut self, scope: &str) -> Self {
        self.scope = Some(scope.to_string());
        self
    }

    /// Set the story index.
    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    /// Set the role taxonomy string.
    pub fn role(mut self, role: &str) -> Self {
        self.role = Some(role.to_string());
        self
    }

    /// Set the submitted_at datetime.
    pub fn submitted_at(mut self, datetime: NaiveDateTime) -> Self {
        self.submitted_at = Some(datetime);
        self
    }

    /// Set the completed_at datetime.
    pub fn completed_at(mut self, datetime: NaiveDateTime) -> Self {
        self.completed_at = Some(datetime);
        self
    }

    /// Build the Story struct.
    pub fn build(self) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: self.id.clone(),
                title: self.title,
                story_type: StoryType::Feat,
                status: self.stage,
                scope: self.scope,
                milestone: None,
                created_at: None,
                updated_at: None,
                completed_at: self.completed_at,
                submitted_at: self.submitted_at,
                index: self.index,
                governed_by: vec![],
                role: self.role,
            },
            path: PathBuf::from(format!("{}.md", self.id)),
            stage: self.stage,
        }
    }
}

/// Builder for creating in-memory Bearing structs for unit tests.
#[derive(Clone)]
pub struct BearingFactory {
    id: String,
    title: String,
    status: BearingStatus,
    has_survey: bool,
    has_assessment: bool,
}

impl Default for BearingFactory {
    fn default() -> Self {
        Self {
            id: "test".to_string(),
            title: "Test Bearing".to_string(),
            status: BearingStatus::Exploring,
            has_survey: false,
            has_assessment: false,
        }
    }
}

impl BearingFactory {
    /// Create a new factory with the given ID.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: format!("Bearing {}", id),
            ..Default::default()
        }
    }

    /// Set the bearing title.
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Set the bearing status.
    pub fn status(mut self, status: BearingStatus) -> Self {
        self.status = status;
        self
    }

    /// Set whether the bearing has a survey.
    pub fn has_survey(mut self, has_survey: bool) -> Self {
        self.has_survey = has_survey;
        self
    }

    /// Set whether the bearing has an assessment.
    pub fn has_assessment(mut self, has_assessment: bool) -> Self {
        self.has_assessment = has_assessment;
        self
    }

    /// Build the Bearing struct.
    pub fn build(self) -> Bearing {
        Bearing {
            frontmatter: BearingFrontmatter {
                id: self.id.clone(),
                title: self.title,
                status: self.status,
                index: None,
                created_at: None,
                decline_reason: None,
                laid_at: None,
            },
            path: PathBuf::from(format!("{}.md", self.id)),
            has_survey: self.has_survey,
            has_assessment: self.has_assessment,
        }
    }
}

/// Builder for creating in-memory Voyage structs for unit tests.
#[derive(Clone)]
pub struct VoyageFactory {
    id: String,
    title: String,
    status: crate::domain::model::VoyageState,
    epic_id: String,
}

impl Default for VoyageFactory {
    fn default() -> Self {
        Self {
            id: "01-voyage".to_string(),
            title: "Test Voyage".to_string(),
            status: crate::domain::model::VoyageState::Draft,
            epic_id: "test-epic".to_string(),
        }
    }
}

impl VoyageFactory {
    pub fn new(id: &str, epic_id: &str) -> Self {
        Self {
            id: id.to_string(),
            epic_id: epic_id.to_string(),
            title: format!("Voyage {}", id),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn status(mut self, status: crate::domain::model::VoyageState) -> Self {
        self.status = status;
        self
    }

    pub fn build(self) -> crate::domain::model::Voyage {
        crate::domain::model::Voyage {
            frontmatter: crate::domain::model::VoyageFrontmatter {
                id: self.id.clone(),
                title: self.title,
                goal: None,
                status: self.status,
                epic: Some(self.epic_id.clone()),
                index: None,
                created_at: None,
                updated_at: None,
                completed_at: None,
            },
            path: PathBuf::from(format!("{}.md", self.id)),
            epic_id: self.epic_id,
        }
    }
}

/// Builder for creating in-memory Adr structs for unit tests.
#[derive(Clone)]
pub struct AdrFactory {
    id: String,
    title: String,
    status: crate::domain::model::AdrStatus,
}

impl Default for AdrFactory {
    fn default() -> Self {
        Self {
            id: "ADR-0001".to_string(),
            title: "Test ADR".to_string(),
            status: crate::domain::model::AdrStatus::Proposed,
        }
    }
}

impl AdrFactory {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            title: format!("ADR {}", id),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn status(mut self, status: crate::domain::model::AdrStatus) -> Self {
        self.status = status;
        self
    }

    pub fn build(self) -> crate::domain::model::Adr {
        crate::domain::model::Adr {
            frontmatter: crate::domain::model::AdrFrontmatter {
                id: self.id.clone(),
                title: self.title,
                status: self.status,
                context: None,
                applies_to: vec![],
                supersedes: vec![],
                superseded_by: None,
                decided_at: None,
                rejection_reason: None,
                deprecation_reason: None,
                index: Some(1),
            },
            path: PathBuf::from(format!("{}.md", self.id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_staged_board() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("TEST001").stage(StoryState::Backlog))
            .build();

        assert!(temp.path().join("stories/TEST001/README.md").exists());
        let content = fs::read_to_string(temp.path().join("epics/test-epic/README.md")).unwrap();
        assert!(content.contains("status: strategic"));
    }

    #[test]
    fn builder_creates_flat_board() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("TEST001").stage(StoryState::InProgress))
            .build();

        assert!(temp.path().join("stories/TEST001/README.md").exists());
    }

    #[test]
    fn builder_creates_epics_and_voyages() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("my-epic"))
            .voyage(TestVoyage::new("01-voyage", "my-epic"))
            .build();

        assert!(temp.path().join("epics/my-epic/README.md").exists());
        assert!(
            temp.path()
                .join("epics/my-epic/voyages/01-voyage/README.md")
                .exists()
        );
    }

    #[test]
    fn story_generates_correct_content() {
        let story = TestStory::new("ABC123")
            .title("My Test Story")
            .story_type("bug")
            .stage(StoryState::InProgress)
            .scope("my-epic/01-voyage");

        let content = story.content();

        assert!(content.contains("id: ABC123"));
        assert!(content.contains("title: My Test Story"));
        assert!(content.contains("type: bug"));
        assert!(content.contains("status: in-progress"));
        assert!(content.contains("scope: my-epic/01-voyage"));
    }
}
