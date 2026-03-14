//! Board loader - parallel file loading using rayon
//!
//! Loads all routines, stories, voyages, and epics from the board directory.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::domain::model::{
    Adr, AdrFrontmatter, Bearing, BearingFrontmatter, Board, Epic, EpicFrontmatter, Mission,
    MissionFrontmatter, Routine, RoutineFrontmatter, Story, StoryFrontmatter, Voyage,
    VoyageFrontmatter,
};
use crate::infrastructure::parser::parse_frontmatter;

/// Trait for entities that can be loaded from a file path
pub trait FromPath: Sized + Send {
    /// Construct the entity from a file path
    fn from_path(path: &Path) -> Result<Self>;
    /// Get the entity's unique identifier (for HashMap key)
    fn entity_id(&self) -> &str;
    /// Human-readable entity type name (for error messages)
    fn entity_name() -> &'static str;
}

/// Generic parallel entity loader
///
/// Given a list of file paths, parses each in parallel using `FromPath::from_path()`
/// and collects results into a HashMap keyed by entity ID.
pub fn load_entities<T: FromPath>(paths: &[PathBuf]) -> HashMap<String, T> {
    let entities: Vec<_> = paths
        .par_iter()
        .filter_map(|path| {
            T::from_path(path)
                .map_err(|e| {
                    eprintln!(
                        "Warning: Failed to load {} {}: {}",
                        T::entity_name(),
                        path.display(),
                        e
                    )
                })
                .ok()
        })
        .collect();

    let mut map = HashMap::new();
    for e in entities {
        map.insert(e.entity_id().to_string(), e);
    }
    map
}

/// Load the entire board from disk
pub fn load_board(board_dir: &Path) -> Result<Board> {
    let routines = load_routines(board_dir)?;
    let stories = load_stories(board_dir)?;
    let voyages = load_voyages(board_dir)?;
    let mut epics = load_epics(board_dir)?;
    derive_epic_statuses(&mut epics, &voyages);
    let bearings = load_bearings(board_dir)?;
    let adrs = load_adrs(board_dir)?;
    let missions = load_missions(board_dir)?;

    Ok(Board {
        root: board_dir.to_path_buf(),
        routines,
        stories,
        voyages,
        epics,
        bearings,
        adrs,
        missions,
    })
}

impl FromPath for Story {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read story file: {}", path.display()))?;
        let (frontmatter, _body): (StoryFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse story frontmatter: {}", path.display()))?;
        Ok(Story::new(frontmatter, path.to_path_buf()))
    }
    fn entity_id(&self) -> &str {
        self.id()
    }
    fn entity_name() -> &'static str {
        "story"
    }
}

/// Load all routines from routines/*/README.md
fn load_routines(board_dir: &Path) -> Result<HashMap<String, Routine>> {
    let routines_dir = board_dir.join("routines");
    if !routines_dir.exists() {
        return Ok(HashMap::new());
    }

    let paths: Vec<_> = fs::read_dir(&routines_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().join("README.md"))
        .filter(|p| p.exists())
        .collect();

    Ok(load_entities(&paths))
}

impl FromPath for Routine {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read routine file: {}", path.display()))?;
        let (mut frontmatter, body): (RoutineFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse routine frontmatter: {}", path.display()))?;

        let routine_id = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        frontmatter.id = routine_id;

        Ok(Routine::new(
            frontmatter,
            body.trim().to_string(),
            path.to_path_buf(),
        ))
    }

    fn entity_id(&self) -> &str {
        self.id()
    }

    fn entity_name() -> &'static str {
        "routine"
    }
}

/// Load all stories from stories/*/README.md
fn load_stories(board_dir: &Path) -> Result<HashMap<String, Story>> {
    let stories_dir = board_dir.join("stories");
    if !stories_dir.exists() {
        return Ok(HashMap::new());
    }

    // Find all story README.md files (direct children of story directories)
    let paths: Vec<_> = fs::read_dir(&stories_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().join("README.md"))
        .filter(|p| p.exists())
        .collect();

    Ok(load_entities(&paths))
}

/// Load all voyages from epics/*/voyages/*/README.md
fn load_voyages(board_dir: &Path) -> Result<HashMap<String, Voyage>> {
    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Ok(HashMap::new());
    }

    // Find all voyage README.md files
    let voyage_paths: Vec<_> = WalkDir::new(&epics_dir)
        .min_depth(3) // epics/*/voyages/*/
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "README.md")
        .filter(|e| {
            e.path()
                .parent()
                .and_then(|p| p.parent())
                .is_some_and(|p| p.file_name().is_some_and(|n| n == "voyages"))
        })
        .map(|e| e.into_path())
        .collect();

    Ok(load_entities(&voyage_paths))
}

impl FromPath for Voyage {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read voyage file: {}", path.display()))?;
        let (mut frontmatter, _body): (VoyageFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse voyage frontmatter: {}", path.display()))?;

        // Extract IDs from path: epics/{epic_id}/voyages/{voyage_id}/README.md
        let voyage_id = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let epic_id = path
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Override frontmatter id with directory name (source of truth)
        frontmatter.id = voyage_id;

        Ok(Voyage {
            frontmatter,
            path: path.to_path_buf(),
            epic_id,
        })
    }
    fn entity_id(&self) -> &str {
        self.id()
    }
    fn entity_name() -> &'static str {
        "voyage"
    }
}

/// Load all epics from epics/*/README.md
fn load_epics(board_dir: &Path) -> Result<HashMap<String, Epic>> {
    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Ok(HashMap::new());
    }

    // Find all epic README.md files (direct children of epic directories)
    let epic_paths: Vec<_> = fs::read_dir(&epics_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().join("README.md"))
        .filter(|p| p.exists())
        .collect();

    Ok(load_entities(&epic_paths))
}

impl FromPath for Epic {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read epic file: {}", path.display()))?;
        let (mut frontmatter, _body): (EpicFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse epic frontmatter: {}", path.display()))?;

        // Extract ID from path: epics/{epic_id}/README.md
        let epic_id = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Override frontmatter id with directory name (source of truth)
        frontmatter.id = epic_id;

        Ok(Epic {
            frontmatter,
            path: path.to_path_buf(),
            status: crate::domain::model::EpicState::Draft,
        })
    }
    fn entity_id(&self) -> &str {
        self.id()
    }
    fn entity_name() -> &'static str {
        "epic"
    }
}

fn derive_epic_statuses(epics: &mut HashMap<String, Epic>, voyages: &HashMap<String, Voyage>) {
    for epic in epics.values_mut() {
        let voyage_states: Vec<_> = voyages
            .values()
            .filter(|voyage| voyage.epic_id == epic.id())
            .map(|voyage| voyage.status())
            .collect();
        epic.set_status(crate::domain::model::Epic::derive_status(&voyage_states));
    }
}

/// Load all bearings from bearings/*/README.md
fn load_bearings(board_dir: &Path) -> Result<HashMap<String, Bearing>> {
    let bearings_dir = board_dir.join("bearings");
    if !bearings_dir.exists() {
        return Ok(HashMap::new());
    }

    // Find all bearing directories with README.md
    let paths: Vec<_> = fs::read_dir(&bearings_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().join("README.md"))
        .filter(|p| p.exists())
        .collect();

    Ok(load_entities(&paths))
}

impl FromPath for Bearing {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read bearing file: {}", path.display()))?;
        let (mut frontmatter, _body): (BearingFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse bearing frontmatter: {}", path.display()))?;

        // Extract ID from path: bearings/{bearing_id}/README.md
        let bearing_id = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Override frontmatter id with directory name (source of truth)
        frontmatter.id = bearing_id;

        let bearing_dir = path.parent().unwrap();
        let has_evidence = bearing_dir.join("EVIDENCE.md").exists();
        let has_assessment = bearing_dir.join("ASSESSMENT.md").exists();

        Ok(Bearing {
            frontmatter,
            path: path.to_path_buf(),
            has_evidence,
            has_assessment,
        })
    }
    fn entity_id(&self) -> &str {
        self.id()
    }
    fn entity_name() -> &'static str {
        "bearing"
    }
}

/// Load all ADRs from adrs/*.md
fn load_adrs(board_dir: &Path) -> Result<HashMap<String, Adr>> {
    let adrs_dir = board_dir.join("adrs");
    if !adrs_dir.exists() {
        return Ok(HashMap::new());
    }

    // Find all ADR markdown files
    let adr_paths: Vec<_> = fs::read_dir(&adrs_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .map(|e| e.path())
        .collect();

    Ok(load_entities(&adr_paths))
}

impl FromPath for Adr {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read ADR file: {}", path.display()))?;
        let (frontmatter, _body): (AdrFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse ADR frontmatter: {}", path.display()))?;
        Ok(Adr {
            frontmatter,
            path: path.to_path_buf(),
        })
    }
    fn entity_id(&self) -> &str {
        self.id()
    }
    fn entity_name() -> &'static str {
        "ADR"
    }
}

/// Load all missions from missions/*/README.md
fn load_missions(board_dir: &Path) -> Result<HashMap<String, Mission>> {
    let missions_dir = board_dir.join("missions");
    if !missions_dir.exists() {
        return Ok(HashMap::new());
    }

    let paths: Vec<_> = fs::read_dir(&missions_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path().join("README.md"))
        .filter(|p| p.exists())
        .collect();

    Ok(load_entities(&paths))
}

impl FromPath for Mission {
    fn from_path(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read mission file: {}", path.display()))?;
        let (mut frontmatter, _body): (MissionFrontmatter, _) = parse_frontmatter(&content)
            .with_context(|| format!("Failed to parse mission frontmatter: {}", path.display()))?;

        // Extract ID from path: missions/{mission_id}/README.md
        let mission_id = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Override frontmatter id with directory name (source of truth)
        frontmatter.id = mission_id;

        let mission_dir = path.parent().unwrap();
        let charter_path = mission_dir.join("CHARTER.md");
        let has_charter = charter_path.exists();
        let has_log = mission_dir.join("LOG.md").exists();

        let archetype = if has_charter {
            if let Ok(content) = fs::read_to_string(&charter_path) {
                crate::infrastructure::validation::charter::parse_mission_archetype(&content)
            } else {
                crate::domain::model::MissionArchetype::default()
            }
        } else {
            crate::domain::model::MissionArchetype::default()
        };

        Ok(Mission::new(
            frontmatter,
            path,
            archetype,
            has_charter,
            has_log,
        ))
    }
    fn entity_id(&self) -> &str {
        self.id()
    }
    fn entity_name() -> &'static str {
        "mission"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use std::fs;
    use tempfile::TempDir;

    fn write_routine(root: &Path, directory_id: &str, frontmatter_id: &str) {
        let routine_dir = root.join("routines").join(directory_id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {frontmatter_id}
title: Weekly Review
cadence:
  cron: 0 9 * * 1
  timezone: America/Los_Angeles
target-scope: VDakm8eVW/VDcFd11nc
created_at: 2026-03-11T09:00:00
updated_at: 2026-03-11T09:30:00
---

# Blueprint

- Review the open backlog.
"#
            ),
        )
        .unwrap();
    }

    fn create_test_board() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create stories directory
        let story_dir = root.join("stories/FEAT0001");
        fs::create_dir_all(&story_dir).unwrap();

        // Create a story README
        fs::write(
            story_dir.join("README.md"),
            r#"---
id: FEAT0001
title: Test Story
type: feat
status: backlog
---
Body content
"#,
        )
        .unwrap();

        // Create epic directory structure
        fs::create_dir_all(root.join("epics/test-epic/voyages/01-first")).unwrap();

        // Create epic README
        fs::write(
            root.join("epics/test-epic/README.md"),
            r#"---
id: test-epic
title: Test Epic
---
Epic description
"#,
        )
        .unwrap();

        // Create voyage README
        fs::write(
            root.join("epics/test-epic/voyages/01-first/README.md"),
            r#"---
id: 01-first
title: First Voyage
status: in-progress
epic: test-epic
---
Voyage description
"#,
        )
        .unwrap();

        temp
    }

    #[test]
    fn load_board_finds_stories() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();

        assert_eq!(board.stories.len(), 1);
        assert!(board.stories.contains_key("FEAT0001"));
    }

    #[test]
    fn load_board_finds_epics() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();

        assert_eq!(board.epics.len(), 1);
        assert!(board.epics.contains_key("test-epic"));
    }

    #[test]
    fn load_board_finds_voyages() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();

        assert_eq!(board.voyages.len(), 1);
        assert!(board.voyages.contains_key("01-first"));
    }

    #[test]
    fn load_board_sets_stage_from_directory() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();

        let story = board.stories.get("FEAT0001").unwrap();
        assert_eq!(story.status, StoryState::Backlog);
    }

    #[test]
    fn load_board_extracts_epic_id_for_voyage() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();

        let voyage = board.voyages.get("01-first").unwrap();
        assert_eq!(voyage.epic_id, "test-epic");
    }

    #[test]
    fn load_board_handles_missing_directories() {
        let temp = TempDir::new().unwrap();
        // Don't create any directories

        let result = load_board(temp.path());
        // Should succeed with empty board
        assert!(result.is_ok());
        let board = result.unwrap();
        assert!(board.stories.is_empty());
        assert!(board.epics.is_empty());
        assert!(board.voyages.is_empty());
        assert!(board.routines.is_empty());
    }

    #[test]
    fn load_board_finds_routines() {
        let temp = TempDir::new().unwrap();
        write_routine(
            temp.path(),
            "routine-weekly-review",
            "routine-weekly-review",
        );

        let board = load_board(temp.path()).unwrap();

        assert_eq!(board.routines.len(), 1);
        let routine = board.routines.get("routine-weekly-review").unwrap();
        assert_eq!(routine.id(), "routine-weekly-review");
        assert_eq!(routine.title(), "Weekly Review");
        assert_eq!(routine.target_scope(), "VDakm8eVW/VDcFd11nc");
        assert!(routine.blueprint_markdown().contains("# Blueprint"));
    }

    #[test]
    fn load_board_derives_routine_id_from_directory_name() {
        let temp = TempDir::new().unwrap();
        write_routine(temp.path(), "routine-weekly-review", "wrong-id");

        let board = load_board(temp.path()).unwrap();

        assert!(board.routines.contains_key("routine-weekly-review"));
        let routine = board.routines.get("routine-weekly-review").unwrap();
        assert_eq!(routine.id(), "routine-weekly-review");
    }

    #[test]
    fn load_board_skips_malformed_files() {
        let temp = create_test_board();

        // Add a malformed story directory
        let bad_dir = temp.path().join("stories/0002");
        fs::create_dir_all(&bad_dir).unwrap();
        fs::write(bad_dir.join("README.md"), "This has no frontmatter").unwrap();

        let board = load_board(temp.path()).unwrap();

        // Should still load the valid story
        assert_eq!(board.stories.len(), 1);
        assert!(board.stories.contains_key("FEAT0001"));
    }

    #[test]
    fn load_board_derives_voyage_id_from_directory_name() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create stage directories
        fs::create_dir_all(root.join("stories")).unwrap();

        // Create epic and voyage with MISMATCHED frontmatter id
        fs::create_dir_all(root.join("epics/test-epic/voyages/02-full-name")).unwrap();
        fs::write(
            root.join("epics/test-epic/README.md"),
            "---\nid: test-epic\ntitle: Test Epic\n---\n",
        )
        .unwrap();
        fs::write(
            root.join("epics/test-epic/voyages/02-full-name/README.md"),
            // Frontmatter has wrong/incomplete id "02" but directory is "02-full-name"
            "---\nid: \"02\"\ntitle: Full Name Voyage\nstatus: planned\n---\n",
        )
        .unwrap();

        let board = load_board(root).unwrap();

        // Should use directory name "02-full-name", not frontmatter "02"
        assert!(
            board.voyages.contains_key("02-full-name"),
            "Expected voyage keyed by directory name '02-full-name', got keys: {:?}",
            board.voyages.keys().collect::<Vec<_>>()
        );
        let voyage = board.voyages.get("02-full-name").unwrap();
        assert_eq!(voyage.id(), "02-full-name");
    }

    #[test]
    fn load_real_board_performance() {
        use std::time::Instant;

        let board_dir = Path::new(".keel");
        if !board_dir.exists() {
            println!("Skipping: .keel not found");
            return;
        }

        let start = Instant::now();
        let board = load_board(board_dir).unwrap();
        let elapsed = start.elapsed();

        println!(
            "Loaded {} routines, {} stories, {} voyages, {} epics in {:?}",
            board.routines.len(),
            board.stories.len(),
            board.voyages.len(),
            board.epics.len(),
            elapsed
        );

        // Should complete in under 500ms
        assert!(
            elapsed.as_millis() < 500,
            "Loading took too long: {:?}",
            elapsed
        );
    }

    // ========== Flat directory structure tests ==========

    fn create_flat_test_board() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create story directories
        let s1_dir = root.join("stories/1vkqAAA01");
        let s2_dir = root.join("stories/1vkqAAA02");
        fs::create_dir_all(&s1_dir).unwrap();
        fs::create_dir_all(&s2_dir).unwrap();

        // Create stories
        fs::write(
            s1_dir.join("README.md"),
            r#"---
id: 1vkqAAA01
title: test-feature
type: feat
status: backlog
---
Body content
"#,
        )
        .unwrap();

        fs::write(
            s2_dir.join("README.md"),
            r#"---
id: 1vkqAAA02
title: fix-crash
type: bug
status: in-progress
---
Body content
"#,
        )
        .unwrap();

        // Create epic directory structure
        fs::create_dir_all(root.join("epics/test-epic/voyages/01-first")).unwrap();
        fs::write(
            root.join("epics/test-epic/README.md"),
            r#"---
id: test-epic
title: Test Epic
---
Epic description
"#,
        )
        .unwrap();
        fs::write(
            root.join("epics/test-epic/voyages/01-first/README.md"),
            r#"---
id: 01-first
title: First Voyage
status: in-progress
---
Voyage description
"#,
        )
        .unwrap();

        temp
    }

    #[test]
    fn load_board_uses_flat_structure_when_present() {
        let temp = create_flat_test_board();
        let board = load_board(temp.path()).unwrap();

        // Should find both stories from flat directory
        assert_eq!(board.stories.len(), 2);
        assert!(board.stories.contains_key("1vkqAAA01"));
        assert!(board.stories.contains_key("1vkqAAA02"));
    }

    #[test]
    fn load_board_flat_uses_frontmatter_status() {
        let temp = create_flat_test_board();
        let board = load_board(temp.path()).unwrap();

        // Story status should come from frontmatter, not directory
        let backlog_story = board.stories.get("1vkqAAA01").unwrap();
        assert_eq!(backlog_story.status, StoryState::Backlog);

        let in_progress_story = board.stories.get("1vkqAAA02").unwrap();
        assert_eq!(in_progress_story.status, StoryState::InProgress);
    }

    #[test]
    fn load_board_loads_flat_stories() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create story directory
        let story_dir = root.join("stories/1vkqAAA01");
        fs::create_dir_all(&story_dir).unwrap();

        // Story README
        fs::write(
            story_dir.join("README.md"),
            r#"---
id: 1vkqAAA01
title: Test Story
type: feat
status: backlog
---
"#,
        )
        .unwrap();

        let board = load_board(root).unwrap();

        // Should load from stories/ directory
        assert_eq!(board.stories.len(), 1);
        assert!(board.stories.contains_key("1vkqAAA01"));
    }

    #[test]
    fn load_board_finds_bearings() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create bearings directory with a bearing
        fs::create_dir_all(root.join("bearings/test-research")).unwrap();
        fs::write(
            root.join("bearings/test-research/README.md"),
            r#"---
id: test-research
title: Test Research
status: exploring
created_at: 2026-01-29T12:00:00
---
# Test Research
"#,
        )
        .unwrap();

        let board = load_board(root).unwrap();

        assert_eq!(board.bearings.len(), 1);
        assert!(board.bearings.contains_key("test-research"));

        let bearing = board.bearings.get("test-research").unwrap();
        assert_eq!(bearing.frontmatter.title, "Test Research");
        assert!(!bearing.has_evidence);
        assert!(!bearing.has_assessment);
    }

    #[test]
    fn load_board_detects_bearing_documents() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create bearings directory with a fully documented bearing
        fs::create_dir_all(root.join("bearings/documented-research")).unwrap();
        fs::write(
            root.join("bearings/documented-research/README.md"),
            r#"---
id: documented-research
title: Documented Research
status: evaluating
---
# Documented Research
"#,
        )
        .unwrap();
        fs::write(
            root.join("bearings/documented-research/EVIDENCE.md"),
            "# Evidence\nResearch notes...",
        )
        .unwrap();
        fs::write(
            root.join("bearings/documented-research/ASSESSMENT.md"),
            "# Assessment\nEvaluation...",
        )
        .unwrap();

        let board = load_board(root).unwrap();

        let bearing = board.bearings.get("documented-research").unwrap();
        assert!(bearing.has_evidence);
        assert!(bearing.has_assessment);
    }

    // ========== Mission loader tests (SRS-06, SRS-07) ==========

    #[test]
    fn load_board_has_missions_field() {
        // SRS-06/AC-01: Board struct has missions: HashMap<String, Mission> field
        let temp = TempDir::new().unwrap();
        let board = load_board(temp.path()).unwrap();
        assert!(board.missions.is_empty());
    }

    #[test]
    fn load_board_finds_missions() {
        // SRS-07/AC-01: load_missions() discovers all .keel/missions/*/README.md files
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("missions/test-mission")).unwrap();
        fs::write(
            root.join("missions/test-mission/README.md"),
            r#"---
id: test-mission
title: Test Mission
status: defining
created_at: 2026-03-01T10:00:00
updated_at: 2026-03-01T10:00:00
---
# Test Mission
"#,
        )
        .unwrap();

        let board = load_board(root).unwrap();

        assert_eq!(board.missions.len(), 1);
        assert!(board.missions.contains_key("test-mission"));

        let mission = board.missions.get("test-mission").unwrap();
        assert_eq!(mission.title(), "Test Mission");
        assert_eq!(
            mission.status(),
            crate::domain::model::MissionStatus::Defining
        );
    }

    #[test]
    fn load_board_populates_missions() {
        // SRS-07/AC-02: load_board() calls load_missions() and populates Board.missions
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create two missions
        for (id, title, status) in [
            ("mission-a", "Alpha Mission", "defining"),
            ("mission-b", "Beta Mission", "active"),
        ] {
            fs::create_dir_all(root.join(format!("missions/{id}"))).unwrap();
            fs::write(
                root.join(format!("missions/{id}/README.md")),
                format!(
                    "---\nid: {id}\ntitle: {title}\nstatus: {status}\ncreated_at: 2026-03-01T10:00:00\nupdated_at: 2026-03-01T10:00:00\n---\n"
                ),
            )
            .unwrap();
        }

        let board = load_board(root).unwrap();
        assert_eq!(board.missions.len(), 2);
        assert!(board.missions.contains_key("mission-a"));
        assert!(board.missions.contains_key("mission-b"));
    }

    #[test]
    fn load_board_skips_malformed_missions() {
        // SRS-07/AC-03: Malformed mission files are skipped with warning, not fatal
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Valid mission
        fs::create_dir_all(root.join("missions/valid-mission")).unwrap();
        fs::write(
            root.join("missions/valid-mission/README.md"),
            "---\nid: valid-mission\ntitle: Valid\nstatus: defining\ncreated_at: 2026-03-01T10:00:00\nupdated_at: 2026-03-01T10:00:00\n---\n",
        )
        .unwrap();

        // Malformed mission (no frontmatter)
        fs::create_dir_all(root.join("missions/bad-mission")).unwrap();
        fs::write(
            root.join("missions/bad-mission/README.md"),
            "This has no frontmatter at all",
        )
        .unwrap();

        let board = load_board(root).unwrap();

        // Should load the valid mission and skip the bad one
        assert_eq!(board.missions.len(), 1);
        assert!(board.missions.contains_key("valid-mission"));
    }

    #[test]
    fn load_board_detects_mission_documents() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("missions/full-mission")).unwrap();
        fs::write(
            root.join("missions/full-mission/README.md"),
            "---\nid: full-mission\ntitle: Full Mission\nstatus: active\ncreated_at: 2026-03-01T10:00:00\nupdated_at: 2026-03-01T10:00:00\n---\n",
        )
        .unwrap();
        fs::write(root.join("missions/full-mission/CHARTER.md"), "# Charter\n").unwrap();
        fs::write(root.join("missions/full-mission/LOG.md"), "# Log\n").unwrap();

        let board = load_board(root).unwrap();
        let mission = board.missions.get("full-mission").unwrap();
        assert!(mission.has_charter);
        assert!(mission.has_log);
    }

    #[test]
    fn load_board_detects_missing_mission_documents() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("missions/bare-mission")).unwrap();
        fs::write(
            root.join("missions/bare-mission/README.md"),
            "---\nid: bare-mission\ntitle: Bare Mission\nstatus: defining\ncreated_at: 2026-03-01T10:00:00\nupdated_at: 2026-03-01T10:00:00\n---\n",
        )
        .unwrap();

        let board = load_board(root).unwrap();
        let mission = board.missions.get("bare-mission").unwrap();
        assert!(!mission.has_charter);
        assert!(!mission.has_log);
    }

    #[test]
    fn load_board_derives_mission_id_from_directory() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        fs::create_dir_all(root.join("missions/dir-name")).unwrap();
        fs::write(
            root.join("missions/dir-name/README.md"),
            "---\nid: wrong-id\ntitle: Directory ID Test\nstatus: defining\ncreated_at: 2026-03-01T10:00:00\nupdated_at: 2026-03-01T10:00:00\n---\n",
        )
        .unwrap();

        let board = load_board(root).unwrap();
        // Should use directory name, not frontmatter id
        assert!(board.missions.contains_key("dir-name"));
        assert_eq!(board.missions.get("dir-name").unwrap().id(), "dir-name");
    }
}
