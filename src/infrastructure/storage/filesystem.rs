//! Concrete FileSystem storage adapter for Keel entities.

use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::model::{Adr, Bearing, Board, Epic, Mission, Story, Voyage};
use crate::domain::port::{
    AdrRepositoryPort, BearingRepositoryPort, BoardRepositoryPort, DocumentServicePort,
    EpicRepositoryPort, StoryRepositoryPort, VoyageRepositoryPort,
};
use crate::domain::port::{BoardStore, EntityStore};
use crate::infrastructure::loader;
use crate::infrastructure::parser::parse_frontmatter;

/// Storage adapter that operates on a local directory structure.
pub struct FileSystemAdapter {
    /// Root directory of the board (usually .keel/)
    pub root: PathBuf,
}

impl FileSystemAdapter {
    /// Create a new adapter for the given root directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn resolve_path(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
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

impl BoardStore for FileSystemAdapter {
    fn load(&self) -> Result<Board> {
        loader::load_board(&self.root)
    }

    fn save(&self, board: &Board) -> Result<()> {
        // Implement aggregate save by delegating to individual entity persists
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

impl EntityStore<Story> for FileSystemAdapter {
    fn get(&self, id: &str) -> Result<Story> {
        let board = self.load()?;
        board.require_story(id).cloned()
    }
    fn put(&self, entity: &Story) -> Result<()> {
        self.persist_frontmatter(&entity.path, &entity.frontmatter)
    }
    fn list(&self) -> Result<Vec<Story>> {
        let board = self.load()?;
        Ok(board.stories.into_values().collect())
    }
    fn delete(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}

impl EntityStore<Voyage> for FileSystemAdapter {
    fn get(&self, id: &str) -> Result<Voyage> {
        let board = self.load()?;
        board.require_voyage(id).cloned()
    }
    fn put(&self, entity: &Voyage) -> Result<()> {
        self.persist_frontmatter(&entity.path, &entity.frontmatter)
    }
    fn list(&self) -> Result<Vec<Voyage>> {
        let board = self.load()?;
        Ok(board.voyages.into_values().collect())
    }
    fn delete(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}

impl EntityStore<Epic> for FileSystemAdapter {
    fn get(&self, id: &str) -> Result<Epic> {
        let board = self.load()?;
        board.require_epic(id).cloned()
    }
    fn put(&self, entity: &Epic) -> Result<()> {
        self.persist_frontmatter(&entity.path, &entity.frontmatter)
    }
    fn list(&self) -> Result<Vec<Epic>> {
        let board = self.load()?;
        Ok(board.epics.into_values().collect())
    }
    fn delete(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}

impl EntityStore<Bearing> for FileSystemAdapter {
    fn get(&self, id: &str) -> Result<Bearing> {
        let board = self.load()?;
        board.require_bearing(id).cloned()
    }
    fn put(&self, entity: &Bearing) -> Result<()> {
        self.persist_frontmatter(&entity.path, &entity.frontmatter)
    }
    fn list(&self) -> Result<Vec<Bearing>> {
        let board = self.load()?;
        Ok(board.bearings.into_values().collect())
    }
    fn delete(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}

impl EntityStore<Adr> for FileSystemAdapter {
    fn get(&self, id: &str) -> Result<Adr> {
        let board = self.load()?;
        board.require_adr(id).cloned()
    }
    fn put(&self, entity: &Adr) -> Result<()> {
        self.persist_frontmatter(&entity.path, &entity.frontmatter)
    }
    fn list(&self) -> Result<Vec<Adr>> {
        let board = self.load()?;
        Ok(board.adrs.into_values().collect())
    }
    fn delete(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}

impl EntityStore<Mission> for FileSystemAdapter {
    fn get(&self, id: &str) -> Result<Mission> {
        let board = self.load()?;
        board.require_mission(id).cloned()
    }
    fn put(&self, entity: &Mission) -> Result<()> {
        self.persist_frontmatter(&entity.path, &entity.frontmatter)
    }
    fn list(&self) -> Result<Vec<Mission>> {
        let board = self.load()?;
        Ok(board.missions.into_values().collect())
    }
    fn delete(&self, _id: &str) -> Result<()> {
        Ok(())
    }
}

// Implement legacy application-layer ports for backward compatibility during migration
impl BoardRepositoryPort for FileSystemAdapter {
    fn load_board(&self) -> Result<Board> {
        BoardStore::load(self)
    }
    fn persist_board(&self, board: &Board) -> Result<()> {
        BoardStore::save(self, board)
    }
}

impl StoryRepositoryPort for FileSystemAdapter {
    fn load_story(&self, id: &str) -> Result<Option<Story>> {
        Ok(EntityStore::<Story>::get(self, id).ok())
    }
    fn list_stories(&self) -> Result<Vec<Story>> {
        EntityStore::<Story>::list(self)
    }
    fn persist_story(&self, story: &Story) -> Result<()> {
        EntityStore::<Story>::put(self, story)
    }
}

impl VoyageRepositoryPort for FileSystemAdapter {
    fn load_voyage(&self, id: &str) -> Result<Option<Voyage>> {
        Ok(EntityStore::<Voyage>::get(self, id).ok())
    }
    fn list_voyages(&self) -> Result<Vec<Voyage>> {
        EntityStore::<Voyage>::list(self)
    }
    fn persist_voyage(&self, voyage: &Voyage) -> Result<()> {
        EntityStore::<Voyage>::put(self, voyage)
    }
}

impl EpicRepositoryPort for FileSystemAdapter {
    fn load_epic(&self, id: &str) -> Result<Option<Epic>> {
        Ok(EntityStore::<Epic>::get(self, id).ok())
    }
    fn list_epics(&self) -> Result<Vec<Epic>> {
        EntityStore::<Epic>::list(self)
    }
    fn persist_epic(&self, epic: &Epic) -> Result<()> {
        EntityStore::<Epic>::put(self, epic)
    }
}

impl BearingRepositoryPort for FileSystemAdapter {
    fn load_bearing(&self, id: &str) -> Result<Option<Bearing>> {
        Ok(EntityStore::<Bearing>::get(self, id).ok())
    }
    fn list_bearings(&self) -> Result<Vec<Bearing>> {
        EntityStore::<Bearing>::list(self)
    }
    fn persist_bearing(&self, bearing: &Bearing) -> Result<()> {
        EntityStore::<Bearing>::put(self, bearing)
    }
}

impl AdrRepositoryPort for FileSystemAdapter {
    fn load_adr(&self, id: &str) -> Result<Option<Adr>> {
        Ok(EntityStore::<Adr>::get(self, id).ok())
    }
    fn list_adrs(&self) -> Result<Vec<Adr>> {
        EntityStore::<Adr>::list(self)
    }
    fn persist_adr(&self, adr: &Adr) -> Result<()> {
        EntityStore::<Adr>::put(self, adr)
    }
}

impl DocumentServicePort for FileSystemAdapter {
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
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn filesystem_board_store() {
        let temp = TestBoardBuilder::new().build();
        let adapter = FileSystemAdapter::new(temp.path());
        let board = adapter.load().unwrap();
        assert_eq!(board.root, temp.path());
    }

    #[test]
    fn filesystem_entity_store() {
        let temp = TestBoardBuilder::new()
            .story(crate::test_helpers::TestStory::new("S1"))
            .build();
        let adapter = FileSystemAdapter::new(temp.path());

        let story: Story = adapter.get("S1").unwrap();
        assert_eq!(story.id(), "S1");

        let stories: Vec<Story> = adapter.list().unwrap();
        assert_eq!(stories.len(), 1);
    }
}
