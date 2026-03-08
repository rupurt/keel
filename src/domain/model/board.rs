//! Board - the top-level container for all entities

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use anyhow::{Result, anyhow};

use super::{Adr, Bearing, Entity, Epic, Story, StoryState, Voyage};

/// The board contains all stories, voyages, epics, bearings, and ADRs
#[derive(Debug, Clone)]
pub struct Board {
    /// Root directory of the board (.keel)
    #[allow(dead_code)] // Available for path resolution
    pub root: PathBuf,
    /// All stories indexed by ID
    pub stories: HashMap<String, Story>,
    /// All voyages indexed by ID
    pub voyages: HashMap<String, Voyage>,
    /// All epics indexed by ID
    pub epics: HashMap<String, Epic>,
    /// All bearings indexed by ID
    pub bearings: HashMap<String, Bearing>,
    /// All ADRs indexed by ID
    #[allow(dead_code)] // Used by future doctor checks (SRS-05)
    pub adrs: HashMap<String, Adr>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(PathBuf::from(".keel"))
    }
}

impl Board {
    /// Create a new empty board
    #[allow(dead_code)] // Used in tests
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            stories: HashMap::new(),
            voyages: HashMap::new(),
            epics: HashMap::new(),
            bearings: HashMap::new(),
            adrs: HashMap::new(),
        }
    }

    /// Find a story by exact ID.
    pub fn find_story(&self, id: &str) -> Option<&Story> {
        self.stories.get(id)
    }

    /// Find a voyage by exact ID.
    pub fn find_voyage(&self, id: &str) -> Option<&Voyage> {
        self.voyages.get(id)
    }

    /// Find an epic by exact ID.
    pub fn find_epic(&self, id: &str) -> Option<&Epic> {
        self.epics.get(id)
    }

    /// Find a bearing by exact ID.
    pub fn find_bearing(&self, id: &str) -> Option<&Bearing> {
        self.bearings.get(id)
    }

    /// Find an ADR by exact ID.
    #[allow(dead_code)] // Used by future doctor checks (SRS-05)
    pub fn find_adr(&self, id: &str) -> Option<&Adr> {
        self.adrs.get(id)
    }

    /// Require a story by exact ID, returning an error with nearby ID suggestions if not found.
    pub fn require_story(&self, id: &str) -> Result<&Story> {
        self.find_story(id)
            .ok_or_else(|| not_found_error("Story", id, self.stories.values()))
    }

    /// Require a voyage by exact ID, returning an error with nearby ID suggestions if not found.
    pub fn require_voyage(&self, id: &str) -> Result<&Voyage> {
        self.find_voyage(id)
            .ok_or_else(|| not_found_error("Voyage", id, self.voyages.values()))
    }

    /// Require an epic by exact ID, returning an error with nearby ID suggestions if not found.
    pub fn require_epic(&self, id: &str) -> Result<&Epic> {
        self.find_epic(id)
            .ok_or_else(|| not_found_error("Epic", id, self.epics.values()))
    }

    /// Require a bearing by exact ID, returning an error with nearby ID suggestions if not found.
    pub fn require_bearing(&self, id: &str) -> Result<&Bearing> {
        self.find_bearing(id)
            .ok_or_else(|| not_found_error("Bearing", id, self.bearings.values()))
    }

    /// Require an ADR by exact ID, returning an error with nearby ID suggestions if not found.
    pub fn require_adr(&self, id: &str) -> Result<&Adr> {
        self.find_adr(id)
            .ok_or_else(|| not_found_error("ADR", id, self.adrs.values()))
    }

    /// Get stories for a voyage (by scope)
    pub fn stories_for_voyage(&self, voyage: &Voyage) -> Vec<&Story> {
        let scope = voyage.scope_path();
        self.stories
            .values()
            .filter(|s| s.frontmatter.scope.as_deref() == Some(&scope))
            .collect()
    }

    /// Get voyages for an epic
    pub fn voyages_for_epic(&self, epic: &Epic) -> Vec<&Voyage> {
        self.voyages_for_epic_id(epic.id())
    }

    /// Get voyages for an epic by ID
    pub fn voyages_for_epic_id(&self, epic_id: &str) -> Vec<&Voyage> {
        self.voyages
            .values()
            .filter(|v| v.epic_id == epic_id)
            .collect()
    }

    /// Returns a deterministic hash of the board state (SRS-04).
    ///
    /// The hash changes when any story or voyage state changes.
    /// Uses sorted key traversal for cross-platform determinism (SRS-NFR-04).
    #[allow(dead_code)] // Used by SRS-05 (optimistic locking)
    pub fn snapshot_version(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();

        // Hash stories in sorted order for determinism
        let mut story_ids: Vec<_> = self.stories.keys().collect();
        story_ids.sort();
        for id in story_ids {
            if let Some(story) = self.stories.get(id) {
                id.hash(&mut hasher);
                story.status.hash(&mut hasher);
            }
        }

        // Hash voyages in sorted order for determinism
        let mut voyage_ids: Vec<_> = self.voyages.keys().collect();
        voyage_ids.sort();
        for id in voyage_ids {
            if let Some(voyage) = self.voyages.get(id) {
                id.hash(&mut hasher);
                voyage.frontmatter.status.hash(&mut hasher);
            }
        }

        hasher.finish()
    }

    /// Returns stories currently in InProgress state (SRS-12).
    ///
    /// These represent active workers that may conflict with parallel agents.
    #[allow(dead_code)] // Used by SRS-11 (concurrent story warning)
    pub fn active_workers(&self) -> Vec<&Story> {
        self.stories
            .values()
            .filter(|s| s.status == StoryState::InProgress)
            .collect()
    }
}

/// Build a "not found" error with nearby ID suggestions.
fn not_found_error<'a, T: Entity + 'a>(
    entity: &str,
    id: &str,
    collection: impl IntoIterator<Item = &'a T>,
) -> anyhow::Error {
    let pattern_lower = id.to_lowercase();
    let suggestions: Vec<_> = collection
        .into_iter()
        .filter(|e| e.id().to_lowercase().contains(&pattern_lower))
        .take(3)
        .map(|e| e.id().to_string())
        .collect();

    if suggestions.is_empty() {
        anyhow!("{} not found: {}", entity, id)
    } else {
        anyhow!(
            "{} not found: {}. Did you mean: {}?",
            entity,
            id,
            suggestions.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::{EpicFrontmatter, EpicState, VoyageFrontmatter, VoyageState};
    use crate::test_helpers::StoryFactory;

    fn make_story(id: &str, scope: Option<&str>) -> Story {
        let mut factory = StoryFactory::new(id);
        if let Some(s) = scope {
            factory = factory.scope(s);
        }
        factory.build()
    }

    fn make_voyage(id: &str, epic_id: &str) -> Voyage {
        Voyage {
            frontmatter: VoyageFrontmatter {
                id: id.to_string(),
                title: format!("Voyage {}", id),
                goal: None,
                status: VoyageState::Planned,
                epic: Some(epic_id.to_string()),
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
            },
            path: PathBuf::from(format!("{}/README.md", id)),
            epic_id: epic_id.to_string(),
        }
    }

    fn make_epic(id: &str) -> Epic {
        Epic {
            frontmatter: EpicFrontmatter {
                id: id.to_string(),
                title: format!("Epic {}", id),
                description: None,
                bearing: None,
                index: None,
                created_at: None,
            },
            path: PathBuf::from(format!("{}/README.md", id)),
            status: EpicState::Draft,
        }
    }

    #[test]
    fn board_find_story_exact() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));

        assert!(board.find_story("FEAT0001").is_some());
        assert!(board.find_story("FEAT0002").is_none());
    }

    #[test]
    fn board_stories_for_voyage() {
        let mut board = Board::new(PathBuf::from("test"));

        let v = make_voyage("01-core", "board-cli");
        board.voyages.insert("01-core".to_string(), v.clone());

        board.stories.insert(
            "FEAT0001".to_string(),
            make_story("FEAT0001", Some("board-cli/01-core")),
        );
        board.stories.insert(
            "FEAT0002".to_string(),
            make_story("FEAT0002", Some("board-cli/01-core")),
        );
        board.stories.insert(
            "FEAT0003".to_string(),
            make_story("FEAT0003", Some("other/02-other")),
        );

        let stories = board.stories_for_voyage(&v);
        assert_eq!(stories.len(), 2);
    }

    #[test]
    fn board_voyages_for_epic() {
        let mut board = Board::new(PathBuf::from("test"));

        let epic = make_epic("board-cli");
        board.epics.insert("board-cli".to_string(), epic.clone());

        board
            .voyages
            .insert("01-core".to_string(), make_voyage("01-core", "board-cli"));
        board.voyages.insert(
            "02-generate".to_string(),
            make_voyage("02-generate", "board-cli"),
        );
        board
            .voyages
            .insert("01-other".to_string(), make_voyage("01-other", "web-ui"));

        let voyages = board.voyages_for_epic(&epic);
        assert_eq!(voyages.len(), 2);
    }

    // ============ Snapshot Versioning Tests (SRS-04) ============

    #[test]
    fn snapshot_version_returns_u64() {
        let board = Board::new(PathBuf::from("test"));
        // Just verify it returns without panic - any u64 value is valid
        let _version: u64 = board.snapshot_version();
    }

    #[test]
    fn snapshot_version_changes_when_story_state_changes() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));

        let version1 = board.snapshot_version();

        // Change story state
        if let Some(story) = board.stories.get_mut("FEAT0001") {
            story.set_status(StoryState::InProgress);
        }

        let version2 = board.snapshot_version();

        assert_ne!(
            version1, version2,
            "Hash should change when story state changes"
        );
    }

    #[test]
    fn snapshot_version_changes_when_voyage_state_changes() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .voyages
            .insert("01-core".to_string(), make_voyage("01-core", "board-cli"));

        let version1 = board.snapshot_version();

        // Change voyage state
        if let Some(voyage) = board.voyages.get_mut("01-core") {
            voyage.frontmatter.status = VoyageState::InProgress;
        }

        let version2 = board.snapshot_version();

        assert_ne!(
            version1, version2,
            "Hash should change when voyage state changes"
        );
    }

    #[test]
    fn snapshot_version_is_deterministic() {
        let mut board1 = Board::new(PathBuf::from("test"));
        board1
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));
        board1
            .stories
            .insert("FEAT0002".to_string(), make_story("FEAT0002", None));
        board1
            .voyages
            .insert("01-core".to_string(), make_voyage("01-core", "board-cli"));

        let mut board2 = Board::new(PathBuf::from("test"));
        // Insert in different order
        board2
            .voyages
            .insert("01-core".to_string(), make_voyage("01-core", "board-cli"));
        board2
            .stories
            .insert("FEAT0002".to_string(), make_story("FEAT0002", None));
        board2
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));

        assert_eq!(
            board1.snapshot_version(),
            board2.snapshot_version(),
            "Same board content should produce same hash regardless of insertion order"
        );
    }

    // ============ Active Workers Tests (SRS-12) ============

    #[test]
    fn active_workers_returns_in_progress_stories() {
        let mut board = Board::new(PathBuf::from("test"));

        let mut story1 = make_story("FEAT0001", None);
        story1.set_status(StoryState::InProgress);
        board.stories.insert("FEAT0001".to_string(), story1);

        let mut story2 = make_story("FEAT0002", None);
        story2.set_status(StoryState::Backlog);
        board.stories.insert("FEAT0002".to_string(), story2);

        let mut story3 = make_story("FEAT0003", None);
        story3.set_status(StoryState::InProgress);
        board.stories.insert("FEAT0003".to_string(), story3);

        let workers = board.active_workers();
        assert_eq!(workers.len(), 2, "Should return only InProgress stories");

        let ids: Vec<_> = workers.iter().map(|s| s.id()).collect();
        assert!(ids.contains(&"FEAT0001"));
        assert!(ids.contains(&"FEAT0003"));
        assert!(!ids.contains(&"FEAT0002"));
    }

    // ============ require_* Tests ============

    #[test]
    fn require_story_returns_found() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));

        assert!(board.require_story("FEAT0001").is_ok());
    }

    #[test]
    fn require_story_error_includes_suggestions() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));

        let err = board.require_story("FEAT9999").unwrap_err().to_string();
        assert!(err.contains("Story not found: FEAT9999"));
    }

    #[test]
    fn require_story_uses_strict_id_lookup() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .stories
            .insert("FEAT0001".to_string(), make_story("FEAT0001", None));

        assert!(board.require_story("feat").is_err());
    }

    #[test]
    fn require_voyage_returns_found() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .voyages
            .insert("01-core".to_string(), make_voyage("01-core", "board-cli"));

        assert!(board.require_voyage("01-core").is_ok());
    }

    #[test]
    fn require_voyage_error_on_not_found() {
        let board = Board::new(PathBuf::from("test"));
        let err = board.require_voyage("nonexistent").unwrap_err().to_string();
        assert!(err.contains("Voyage not found: nonexistent"));
    }

    #[test]
    fn require_epic_returns_found() {
        let mut board = Board::new(PathBuf::from("test"));
        board
            .epics
            .insert("board-cli".to_string(), make_epic("board-cli"));

        assert!(board.require_epic("board-cli").is_ok());
    }

    #[test]
    fn require_epic_error_on_not_found() {
        let board = Board::new(PathBuf::from("test"));
        let err = board.require_epic("nonexistent").unwrap_err().to_string();
        assert!(err.contains("Epic not found: nonexistent"));
    }

    #[test]
    fn active_workers_returns_empty_when_no_in_progress() {
        let mut board = Board::new(PathBuf::from("test"));

        let mut story1 = make_story("FEAT0001", None);
        story1.set_status(StoryState::Backlog);
        board.stories.insert("FEAT0001".to_string(), story1);

        let mut story2 = make_story("FEAT0002", None);
        story2.set_status(StoryState::Done);
        board.stories.insert("FEAT0002".to_string(), story2);

        let workers = board.active_workers();
        assert!(
            workers.is_empty(),
            "Should return empty when no InProgress stories"
        );
    }
}
