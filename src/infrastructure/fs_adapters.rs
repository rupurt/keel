//! Filesystem adapter implementations for repository and document ports.
//!
//! These adapters keep application-layer ports decoupled from direct `std::fs`
//! usage while preserving the existing `.keel` markdown storage model.

use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::application::ports::{
    AdrRepositoryPort, BearingRepositoryPort, BoardRepositoryPort, DocumentServicePort,
    EpicRepositoryPort, StoryRepositoryPort, VoyageRepositoryPort,
};
use crate::domain::model::{Adr, Bearing, Board, Epic, Story, Voyage};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::parser::parse_frontmatter;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FilesystemAdapter {
    board_dir: PathBuf,
}

#[allow(dead_code)]
impl FilesystemAdapter {
    pub fn new(board_dir: impl AsRef<Path>) -> Self {
        Self {
            board_dir: board_dir.as_ref().to_path_buf(),
        }
    }

    fn load_board_snapshot(&self) -> Result<Board> {
        load_board(&self.board_dir).with_context(|| {
            format!(
                "load board snapshot from {}",
                self.board_dir.as_path().display()
            )
        })
    }

    fn resolve_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.board_dir.join(path)
        }
    }

    fn persist_frontmatter<T: Serialize>(&self, entity_path: &Path, frontmatter: &T) -> Result<()> {
        let path = self.resolve_path(entity_path);
        let original = fs::read_to_string(&path)
            .with_context(|| format!("read entity markdown at {}", path.display()))?;
        let (_, body): (serde_yaml::Value, &str) = parse_frontmatter(&original)
            .with_context(|| format!("parse entity frontmatter at {}", path.display()))?;
        let serialized = serde_yaml::to_string(frontmatter)
            .with_context(|| format!("serialize frontmatter for {}", path.display()))?;
        let updated = format!("---\n{}---\n{}", serialized, body);
        fs::write(&path, updated)
            .with_context(|| format!("persist entity markdown at {}", path.display()))?;
        Ok(())
    }
}

impl BoardRepositoryPort for FilesystemAdapter {
    fn load_board(&self) -> Result<Board> {
        self.load_board_snapshot()
    }

    fn persist_board(&self, board: &Board) -> Result<()> {
        for story in board.stories.values() {
            self.persist_frontmatter(&story.path, &story.frontmatter)?;
        }
        for voyage in board.voyages.values() {
            self.persist_frontmatter(&voyage.path, &voyage.frontmatter)?;
        }
        for epic in board.epics.values() {
            self.persist_frontmatter(&epic.path, &epic.frontmatter)?;
        }
        for bearing in board.bearings.values() {
            self.persist_frontmatter(&bearing.path, &bearing.frontmatter)?;
        }
        for adr in board.adrs.values() {
            self.persist_frontmatter(&adr.path, &adr.frontmatter)?;
        }
        Ok(())
    }
}

impl StoryRepositoryPort for FilesystemAdapter {
    fn load_story(&self, id: &str) -> Result<Option<Story>> {
        let board = self.load_board_snapshot()?;
        Ok(board.stories.get(id).cloned())
    }

    fn list_stories(&self) -> Result<Vec<Story>> {
        let board = self.load_board_snapshot()?;
        let mut stories: Vec<_> = board.stories.values().cloned().collect();
        stories.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(stories)
    }

    fn persist_story(&self, story: &Story) -> Result<()> {
        self.persist_frontmatter(&story.path, &story.frontmatter)
    }
}

impl VoyageRepositoryPort for FilesystemAdapter {
    fn load_voyage(&self, id: &str) -> Result<Option<Voyage>> {
        let board = self.load_board_snapshot()?;
        Ok(board.voyages.get(id).cloned())
    }

    fn list_voyages(&self) -> Result<Vec<Voyage>> {
        let board = self.load_board_snapshot()?;
        let mut voyages: Vec<_> = board.voyages.values().cloned().collect();
        voyages.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(voyages)
    }

    fn persist_voyage(&self, voyage: &Voyage) -> Result<()> {
        self.persist_frontmatter(&voyage.path, &voyage.frontmatter)
    }
}

impl EpicRepositoryPort for FilesystemAdapter {
    fn load_epic(&self, id: &str) -> Result<Option<Epic>> {
        let board = self.load_board_snapshot()?;
        Ok(board.epics.get(id).cloned())
    }

    fn list_epics(&self) -> Result<Vec<Epic>> {
        let board = self.load_board_snapshot()?;
        let mut epics: Vec<_> = board.epics.values().cloned().collect();
        epics.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(epics)
    }

    fn persist_epic(&self, epic: &Epic) -> Result<()> {
        self.persist_frontmatter(&epic.path, &epic.frontmatter)
    }
}

impl BearingRepositoryPort for FilesystemAdapter {
    fn load_bearing(&self, id: &str) -> Result<Option<Bearing>> {
        let board = self.load_board_snapshot()?;
        Ok(board.bearings.get(id).cloned())
    }

    fn list_bearings(&self) -> Result<Vec<Bearing>> {
        let board = self.load_board_snapshot()?;
        let mut bearings: Vec<_> = board.bearings.values().cloned().collect();
        bearings.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(bearings)
    }

    fn persist_bearing(&self, bearing: &Bearing) -> Result<()> {
        self.persist_frontmatter(&bearing.path, &bearing.frontmatter)
    }
}

impl AdrRepositoryPort for FilesystemAdapter {
    fn load_adr(&self, id: &str) -> Result<Option<Adr>> {
        let board = self.load_board_snapshot()?;
        Ok(board.adrs.get(id).cloned())
    }

    fn list_adrs(&self) -> Result<Vec<Adr>> {
        let board = self.load_board_snapshot()?;
        let mut adrs: Vec<_> = board.adrs.values().cloned().collect();
        adrs.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(adrs)
    }

    fn persist_adr(&self, adr: &Adr) -> Result<()> {
        self.persist_frontmatter(&adr.path, &adr.frontmatter)
    }
}

impl DocumentServicePort for FilesystemAdapter {
    fn read_document(&self, path: &Path) -> Result<String> {
        let resolved = self.resolve_path(path);
        fs::read_to_string(&resolved)
            .with_context(|| format!("read document at {}", resolved.display()))
    }

    fn write_document(&self, path: &Path, content: &str) -> Result<()> {
        let resolved = self.resolve_path(path);
        if let Some(parent) = resolved.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create document parent {}", parent.display()))?;
        }
        fs::write(&resolved, content)
            .with_context(|| format!("write document at {}", resolved.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::FilesystemAdapter;
    use crate::application::ports::{
        DocumentServicePort, EpicRepositoryPort, StoryRepositoryPort, VoyageRepositoryPort,
    };
    use crate::domain::model::{StoryState, VoyageState};
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use std::path::Path;

    #[test]
    fn filesystem_adapter_matches_loader_for_repository_reads() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("repo-epic"))
            .voyage(
                TestVoyage::new("01-repo", "repo-epic")
                    .status("in-progress")
                    .title("Repo Voyage"),
            )
            .story(
                TestStory::new("REPO01")
                    .title("Repo Story")
                    .status(StoryState::Backlog)
                    .scope("repo-epic/01-repo"),
            )
            .build();

        let adapter = FilesystemAdapter::new(temp.path());
        let stories = adapter.list_stories().unwrap();
        let voyages = adapter.list_voyages().unwrap();
        let epics = adapter.list_epics().unwrap();

        assert_eq!(stories.len(), 1);
        assert_eq!(voyages.len(), 1);
        assert_eq!(epics.len(), 1);
        assert_eq!(stories[0].id(), "REPO01");
        assert_eq!(voyages[0].status(), VoyageState::InProgress);
        assert_eq!(epics[0].id(), "repo-epic");
    }

    #[test]
    fn filesystem_adapter_persists_story_frontmatter_without_losing_body() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("PERSIST1")
                    .title("Original Title")
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] Keep body"),
            )
            .build();

        let adapter = FilesystemAdapter::new(temp.path());
        let mut story = adapter.load_story("PERSIST1").unwrap().unwrap();
        story.frontmatter.title = "Updated Title".to_string();
        story.set_status(StoryState::Done);
        adapter.persist_story(&story).unwrap();

        let reloaded = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let persisted = reloaded.require_story("PERSIST1").unwrap();
        assert_eq!(persisted.title(), "Updated Title");
        assert_eq!(persisted.status, StoryState::Done);

        let content = fs::read_to_string(temp.path().join("stories/PERSIST1/README.md")).unwrap();
        assert!(content.contains("status: done"));
        assert!(content.contains("- [ ] Keep body"));
    }

    #[test]
    fn filesystem_adapter_document_service_reads_and_writes_documents() {
        let temp = TestBoardBuilder::new().build();
        let adapter = FilesystemAdapter::new(temp.path());
        let doc_path = Path::new("notes/adapter-test.md");
        let expected = "# Adapter Note\n";

        adapter.write_document(doc_path, expected).unwrap();
        let content = adapter.read_document(doc_path).unwrap();
        assert_eq!(content, expected);
    }
}
