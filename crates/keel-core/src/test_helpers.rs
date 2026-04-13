#![allow(dead_code)]
//! Test helpers for creating test fixtures.
//!
//! This module provides a builder pattern for creating test boards,
//! eliminating duplicated test fixture code across command tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::domain::model::StoryState;
use crate::infrastructure::frontmatter_mutation::Mutation;
use crate::infrastructure::{template_rendering, templates};

const FIXED_DATETIME: &str = "2026-01-01T00:00:00";

/// Builder for creating test boards with flexible configuration.
pub struct TestBoardBuilder {
    temp: TempDir,
    stories: Vec<TestStory>,
    epics: Vec<TestEpic>,
    voyages: Vec<TestVoyage>,
    adrs: Vec<TestAdr>,
    bearings: Vec<TestBearing>,
    missions: Vec<TestMission>,
}

pub fn write_stack_manifest(board_dir: &Path, id: &str, manifest: &str) {
    let stack_dir = board_dir.join("stacks").join(id);
    fs::create_dir_all(&stack_dir).unwrap();
    fs::write(stack_dir.join("manifest.yaml"), manifest.trim_start()).unwrap();
}

pub fn init_git_repo(repo_root: &Path) {
    git(repo_root, &["init"]);
    git(repo_root, &["config", "user.name", "Keel Test"]);
    git(repo_root, &["config", "user.email", "keel@example.com"]);
    git(repo_root, &["config", "commit.gpgsign", "false"]);
    fs::write(repo_root.join("README.md"), "# Test Repo\n").unwrap();
    git(repo_root, &["add", "."]);
    git(repo_root, &["commit", "-m", "initial"]);
}

pub fn git(repo_root: &Path, args: &[&str]) {
    let mut command = Command::new("git");
    command.args(args).current_dir(repo_root);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr).trim()
    );
}

/// Configuration for a test mission.
pub struct TestMission {
    pub id: String,
    pub title: String,
    pub status: String,
    pub watch: Option<String>,
    pub activated_at: Option<chrono::NaiveDateTime>,
}

/// Configuration for a test story.
pub struct TestStory {
    pub id: String,
    pub title: String,
    pub story_type: String,
    pub status: StoryState,
    pub scope: Option<String>,
    pub index: Option<u32>,
    pub body: String,
    pub role: Option<String>,
    pub blocked_by: Vec<String>,
}

/// Configuration for a test epic.
pub struct TestEpic {
    pub id: String,
    pub title: String,
    pub index: Option<u32>,
    pub mission: Option<String>,
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
    pub mission: Option<String>,
}

/// Configuration for a test bearing.
pub struct TestBearing {
    pub id: String,
    pub title: String,
    pub status: String,
    pub has_evidence: bool,
    pub has_assessment: bool,
    pub index: Option<u32>,
    pub epic: Option<String>,
    pub goals: Option<Vec<String>>,
    pub mission: Option<String>,
    pub depends_on: Option<Vec<String>>,
}

impl Default for TestMission {
    fn default() -> Self {
        Self {
            id: "M1".to_string(),
            title: "Test Mission".to_string(),
            status: "defining".to_string(),
            watch: None,
            activated_at: None,
        }
    }
}

impl TestMission {
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

    pub fn watch(mut self, watch: &str) -> Self {
        self.watch = Some(watch.to_string());
        self
    }

    pub fn activated_at(mut self, dt: chrono::NaiveDateTime) -> Self {
        self.activated_at = Some(dt);
        self
    }
}

impl Default for TestStory {
    fn default() -> Self {
        Self {
            id: "TEST0001".to_string(),
            title: "Test Story".to_string(),
            story_type: "feat".to_string(),
            status: StoryState::Backlog,
            scope: None,
            index: None,
            body: "Body content".to_string(),
            role: None,
            blocked_by: vec![],
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

    pub fn status(mut self, status: StoryState) -> Self {
        self.status = status;
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

    pub fn blocked_by(mut self, story_ids: &[&str]) -> Self {
        self.blocked_by = story_ids.iter().map(|id| (*id).to_string()).collect();
        self
    }

    /// Generate frontmatter for this story.
    fn frontmatter(&self) -> String {
        let mut fm = format!(
            "---\nid: {}\ntitle: {}\ntype: {}\nstatus: {}\n",
            self.id, self.title, self.story_type, self.status
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

        if !self.blocked_by.is_empty() {
            fm.push_str("blocked_by:\n");
            for blocked_story_id in &self.blocked_by {
                fm.push_str(&format!("  - {}\n", blocked_story_id));
            }
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
            index: None,
            mission: None,
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

    pub fn mission(mut self, mission: &str) -> Self {
        self.mission = Some(mission.to_string());
        self
    }

    #[allow(dead_code)] // Test helper for future use
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
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
            mission: None,
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

    pub fn mission(mut self, mission: &str) -> Self {
        self.mission = Some(mission.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn applies_to(mut self, applies: Vec<&str>) -> Self {
        self.applies_to = applies.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Generate filename for this ADR.
    fn filename(&self) -> String {
        format!("{}/README.md", self.id)
    }

    /// Generate full content for this ADR.
    fn content(&self) -> String {
        let applies_to_str = if self.applies_to.is_empty() {
            "[]".to_string()
        } else {
            format!("[{}]", self.applies_to.join(", "))
        };
        let context_str = self.context.as_deref().unwrap_or("null");
        let mission_line = self
            .mission
            .as_ref()
            .map(|mission| format!("mission: {mission}\n"))
            .unwrap_or_default();

        format!(
            r#"---
id: {}
index: 1
title: {}
status: {}
context: {}
applies-to: {}
{}supersedes: []
superseded-by: null
decided_at: 2026-01-01T00:00:00
---

# {}

## Status

**Proposed**

## Context

Test ADR content.
"#,
            self.id, self.title, self.status, context_str, applies_to_str, mission_line, self.title
        )
    }
}

impl Default for TestBearing {
    fn default() -> Self {
        Self {
            id: "BRG-01".to_string(),
            title: "Test Bearing".to_string(),
            status: "exploring".to_string(),
            has_evidence: false,
            has_assessment: false,
            index: None,
            epic: None,
            goals: None,
            mission: None,
            depends_on: None,
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

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = Some(index);
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }

    pub fn has_evidence(mut self, has_evidence: bool) -> Self {
        self.has_evidence = has_evidence;
        self
    }

    pub fn has_assessment(mut self, has_assessment: bool) -> Self {
        self.has_assessment = has_assessment;
        self
    }

    pub fn epic(mut self, epic: &str) -> Self {
        self.epic = Some(epic.to_string());
        self
    }

    pub fn goals(mut self, goals: Vec<&str>) -> Self {
        self.goals = Some(goals.into_iter().map(|g| g.to_string()).collect());
        self
    }

    pub fn mission(mut self, mission: &str) -> Self {
        self.mission = Some(mission.to_string());
        self
    }

    pub fn depends_on(mut self, deps: Vec<&str>) -> Self {
        self.depends_on = Some(deps.into_iter().map(|d| d.to_string()).collect());
        self
    }
}

fn render_fixture_epic_readme(epic: &TestEpic) -> String {
    let mut mutations = Vec::new();
    if let Some(index) = epic.index {
        mutations.push(Mutation::set("index", index.to_string()));
    }
    if let Some(ref mission) = epic.mission {
        mutations.push(Mutation::set("mission", mission.clone()));
    }

    template_rendering::render_with_mutations(
        templates::epic::README,
        &[
            ("id", &epic.id),
            ("title", &epic.title),
            ("created_at", FIXED_DATETIME),
            ("problem", "Test harness epic problem statement."),
        ],
        &mutations,
    )
}

fn render_fixture_voyage_readme(voyage: &TestVoyage) -> String {
    let mut mutations = vec![Mutation::set("status", voyage.status.clone())];
    if let Some(index) = voyage.index {
        mutations.push(Mutation::set("index", index.to_string()));
    }
    let goal = format!("Deliver {}", voyage.title);

    template_rendering::render_with_mutations(
        templates::voyage::README,
        &[
            ("id", &voyage.id),
            ("title", &voyage.title),
            ("created_at", FIXED_DATETIME),
            ("epic", &voyage.epic_id),
            ("goal", &goal),
        ],
        &mutations,
    )
}

fn render_fixture_bearing_readme(bearing: &TestBearing) -> String {
    let mut mutations = Vec::new();
    if let Some(index) = bearing.index {
        mutations.push(Mutation::set("index", index.to_string()));
    }
    if let Some(epic) = &bearing.epic {
        mutations.push(Mutation::set("epic", epic.clone()));
    }
    if let Some(ref mission) = bearing.mission {
        mutations.push(Mutation::set("mission", mission.clone()));
    }
    if let Some(goals) = &bearing.goals
        && !goals.is_empty()
    {
        mutations.push(Mutation::set("goals", format!("[{}]", goals.join(", "))));
    }
    if let Some(deps) = &bearing.depends_on
        && !deps.is_empty()
    {
        mutations.push(Mutation::set(
            "depends_on",
            format!("[{}]", deps.join(", ")),
        ));
    }

    template_rendering::render_with_mutations(
        templates::bearing::README,
        &[
            ("id", &bearing.id),
            ("title", &bearing.title),
            ("status", &bearing.status),
            ("created_at", FIXED_DATETIME),
        ],
        &mutations,
    )
}

fn render_fixture_bearing_brief(bearing: &TestBearing) -> String {
    format!(
        r#"# {} — Brief

## Hypothesis

{} is worth exploring because it addresses a repeated strategic pain point.

## Problem Space

The team needs a clear problem statement for {} before the bearing can advance.

## Success Criteria

- [ ] The brief explains the bet clearly enough to guide research.

## Open Questions

- What evidence would materially change the recommendation?
"#,
        bearing.title, bearing.title, bearing.title
    )
}

fn render_fixture_bearing_evidence(bearing: &TestBearing) -> String {
    format!(
        r#"---
id: {}
---

# {} — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | manual:test-fixture | https://example.test/{}/source-1 | 2026-01-01 | 2026-01-02 | medium | medium | Fixture source one supports the bearing summary. |

## Technical Research

## Feasibility
Fixture evidence indicates the direction is feasible enough for tests.

## Key Findings

1. Fixture finding supported by [SRC-01]

## Unknowns

- Fixture unknown to preserve the canonical section shape
"#,
        bearing.id, bearing.title, bearing.id
    )
}

fn render_fixture_bearing_assessment(bearing: &TestBearing) -> String {
    format!(
        r#"# {} — Assessment

## Scoring Factors

| Factor | Score | Notes |
|--------|-------|-------|
| Feasibility | 4 | Fixture scoring factor |

## Findings

Fixture findings for test bearing {}.

## Opportunity Cost

Fixture opportunity cost analysis.

## Dependencies

- No external dependencies for fixture.

## Alternatives Considered

- Status quo (baseline).

## Recommendation

- [x] Proceed
- [ ] Park
- [ ] Decline
"#,
        bearing.title, bearing.id
    )
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
            missions: vec![],
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

    /// Add a mission to the board.
    pub fn mission(mut self, mission: TestMission) -> Self {
        self.missions.push(mission);
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
            fs::write(epic_dir.join("README.md"), render_fixture_epic_readme(epic)).unwrap();

            // Create default PRD.md
            fs::write(
                epic_dir.join("PRD.md"),
                r#"# PRD

## Problem Statement
Test harness epic problem statement.

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Ship test harness behavior | Passing validation checks | 100% |

## Users
| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Test Author | Maintains fixtures | Deterministic board artifacts |

## Scope
### In Scope
- [SCOPE-01] Provide valid baseline planning artifacts for tests.

### Out of Scope
- [SCOPE-02] Product-level behavior design.

## Requirements
### Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keep default fixture docs structurally valid. | GOAL-01 | must | Prevents unrelated test failures. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Keep fixtures deterministic. | GOAL-01 | must | Stabilizes test outputs. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy
- Run structural, quality, and unit checks in CI.

## Assumptions
| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Fixture defaults are sufficient for most tests | Test friction | Update defaults alongside new invariants |

## Open Questions & Risks
| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Are new template contracts reflected here quickly enough? | Maintainer | Open |

## Success Criteria
<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Default test board passes structural epic PRD validation.
<!-- END SUCCESS_CRITERIA -->
"#,
            )
            .unwrap();

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

        // Create missions
        for mission in &self.missions {
            let mission_dir = root.join("missions").join(&mission.id);
            fs::create_dir_all(&mission_dir).unwrap();

            let mut mutations = Vec::new();
            if let Some(ref watch) = mission.watch {
                mutations.push(Mutation::set("watch", watch.clone()));
            }
            if let Some(activated_at) = mission.activated_at {
                mutations.push(Mutation::set(
                    "activated_at",
                    activated_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
                ));
            }

            let readme = template_rendering::render_with_mutations(
                templates::mission::README,
                &[
                    ("id", &mission.id),
                    ("title", &mission.title),
                    ("created_at", FIXED_DATETIME),
                    ("updated_at", FIXED_DATETIME),
                    ("status", &mission.status),
                ],
                &mutations,
            );
            fs::write(mission_dir.join("README.md"), readme).unwrap();

            let charter = template_rendering::render(
                templates::mission::CHARTER,
                &[("id", &mission.id), ("title", &mission.title)],
            );
            fs::write(mission_dir.join("CHARTER.md"), charter).unwrap();

            let log = template_rendering::render(
                templates::mission::LOG,
                &[("id", &mission.id), ("title", &mission.title)],
            );
            fs::write(mission_dir.join("LOG.md"), log).unwrap();
        }

        // Create voyages
        for voyage in &self.voyages {
            let voyage_dir = root
                .join("epics")
                .join(&voyage.epic_id)
                .join("voyages")
                .join(&voyage.id);
            fs::create_dir_all(&voyage_dir).unwrap();
            fs::write(
                voyage_dir.join("README.md"),
                render_fixture_voyage_readme(voyage),
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

            // Mandatory EVIDENCE directory
            fs::create_dir_all(story_bundle_dir.join("EVIDENCE")).unwrap();
        }

        // Create ADRs
        if !self.adrs.is_empty() {
            let adrs_dir = root.join("adrs");
            fs::create_dir_all(&adrs_dir).unwrap();
            for adr in &self.adrs {
                let adr_path = adrs_dir.join(adr.filename());
                if let Some(parent) = adr_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
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

                fs::write(
                    bearing_dir.join("README.md"),
                    render_fixture_bearing_readme(bearing),
                )
                .unwrap();

                // BRIEF.md
                fs::write(
                    bearing_dir.join("BRIEF.md"),
                    render_fixture_bearing_brief(bearing),
                )
                .unwrap();

                // EVIDENCE.md
                if bearing.has_evidence {
                    fs::write(
                        bearing_dir.join("EVIDENCE.md"),
                        render_fixture_bearing_evidence(bearing),
                    )
                    .unwrap();
                }
                // ASSESSMENT.md
                if bearing.has_assessment {
                    fs::write(
                        bearing_dir.join("ASSESSMENT.md"),
                        render_fixture_bearing_assessment(bearing),
                    )
                    .unwrap();
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
    status: StoryState,
    scope: Option<String>,
    index: Option<u32>,
    submitted_at: Option<NaiveDateTime>,
    completed_at: Option<NaiveDateTime>,
    role: Option<String>,
    blocked_by: Vec<String>,
    materialization_key: Option<String>,
}

impl Default for StoryFactory {
    fn default() -> Self {
        Self {
            id: "test".to_string(),
            title: "Test Story".to_string(),
            status: StoryState::Backlog,
            scope: None,
            index: None,
            submitted_at: None,
            completed_at: None,
            role: None,
            blocked_by: vec![],
            materialization_key: None,
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

    /// Set the story status.
    pub fn status(mut self, status: StoryState) -> Self {
        self.status = status;
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

    /// Set blocked_by story IDs.
    pub fn blocked_by(mut self, story_ids: &[&str]) -> Self {
        self.blocked_by = story_ids.iter().map(|id| (*id).to_string()).collect();
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

    /// Set the materialization key.
    pub fn materialization_key(mut self, key: &str) -> Self {
        self.materialization_key = Some(key.to_string());
        self
    }

    /// Build the Story struct.
    pub fn build(self) -> Story {
        Story::new(
            StoryFrontmatter {
                id: self.id.clone(),
                title: self.title,
                story_type: StoryType::Feat,
                status: self.status,
                scope: self.scope,
                milestone: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: self.completed_at,
                submitted_at: self.submitted_at,
                index: self.index,
                governed_by: vec![],
                blocked_by: self.blocked_by,
                role: self.role,
                operator_signal: None,
            },
            PathBuf::from(format!("{}.md", self.id)),
            self.materialization_key,
        )
    }
}

/// Builder for creating in-memory Bearing structs for unit tests.
#[derive(Clone)]
pub struct BearingFactory {
    id: String,
    title: String,
    status: BearingStatus,
    has_evidence: bool,
    has_assessment: bool,
    epic: Option<String>,
    goals: Option<Vec<String>>,
}

impl Default for BearingFactory {
    fn default() -> Self {
        Self {
            id: "test".to_string(),
            title: "Test Bearing".to_string(),
            status: BearingStatus::Exploring,
            has_evidence: false,
            has_assessment: false,
            epic: None,
            goals: None,
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

    /// Set whether the bearing has evidence.
    pub fn has_evidence(mut self, has_evidence: bool) -> Self {
        self.has_evidence = has_evidence;
        self
    }

    /// Set whether the bearing has an assessment.
    pub fn has_assessment(mut self, has_assessment: bool) -> Self {
        self.has_assessment = has_assessment;
        self
    }

    /// Set the epic lineage token.
    pub fn epic(mut self, epic: &str) -> Self {
        self.epic = Some(epic.to_string());
        self
    }

    /// Set the goal lineage references.
    pub fn goals(mut self, goals: Vec<&str>) -> Self {
        self.goals = Some(goals.into_iter().map(ToString::to_string).collect());
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
                epic: self.epic,
                mission: None,
                goals: self.goals,
                depends_on: None,
            },
            path: PathBuf::from(format!("{}.md", self.id)),
            has_evidence: self.has_evidence,
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
                started_at: None,
                completed_at: None,
                operator_signal: None,
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
                mission: None,
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
            .story(TestStory::new("TEST001").status(StoryState::Backlog))
            .build();

        assert!(temp.path().join("stories/TEST001/README.md").exists());
        let content = fs::read_to_string(temp.path().join("epics/test-epic/README.md")).unwrap();
        assert!(!content.contains("\nstatus:"));
    }

    #[test]
    fn builder_creates_flat_board() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("TEST001").status(StoryState::InProgress))
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
            .status(StoryState::InProgress)
            .scope("my-epic/01-voyage");

        let content = story.content();

        assert!(content.contains("id: ABC123"));
        assert!(content.contains("title: My Test Story"));
        assert!(content.contains("type: bug"));
        assert!(content.contains("status: in-progress"));
        assert!(content.contains("scope: my-epic/01-voyage"));
    }

    #[test]
    fn bearing_fixture_boards_use_evidence_contract() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("BRG-01")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();

        let bearing_dir = temp.path().join("bearings/BRG-01");
        let readme = fs::read_to_string(bearing_dir.join("README.md")).unwrap();

        assert!(readme.contains("[EVIDENCE.md](EVIDENCE.md)"));
        assert!(!readme.contains("[SURVEY.md](SURVEY.md)"));
        assert!(bearing_dir.join("EVIDENCE.md").exists());
        assert!(!bearing_dir.join("SURVEY.md").exists());
    }
}
