//! Storage Ports (traits) for Keel's hexagonal architecture.
//!
//! These traits decouple the domain and application logic from the
//! underlying persistence implementation (e.g., FileSystem, Database, Server).

use crate::domain::model::{Adr, Bearing, Board, Entity, Epic, Story, Voyage};
use anyhow::Result;
use std::path::Path;

/// Aggregate operations for the entire Board.
pub trait BoardStore: Send + Sync {
    /// Load the complete board state.
    fn load(&self) -> Result<Board>;

    /// Save the complete board state.
    fn save(&self, board: &Board) -> Result<()>;
}

/// CRUD operations for individual entities.
pub trait EntityStore<T: Entity>: Send + Sync {
    /// Retrieve a single entity by its unique identifier.
    fn get(&self, id: &str) -> Result<T>;

    /// Create or update an entity.
    fn put(&self, entity: &T) -> Result<()>;

    /// List all entities of this type.
    fn list(&self) -> Result<Vec<T>>;

    /// Remove an entity by its unique identifier.
    fn delete(&self, id: &str) -> Result<()>;
}

// Legacy application-layer ports moved here for consolidation during hexagonal migration.

pub trait BoardRepositoryPort {
    fn load_board(&self) -> Result<Board>;
    fn persist_board(&self, board: &Board) -> Result<()>;
}

pub trait StoryRepositoryPort {
    fn load_story(&self, id: &str) -> Result<Option<Story>>;
    fn list_stories(&self) -> Result<Vec<Story>>;
    fn persist_story(&self, story: &Story) -> Result<()>;
}

pub trait VoyageRepositoryPort {
    fn load_voyage(&self, id: &str) -> Result<Option<Voyage>>;
    fn list_voyages(&self) -> Result<Vec<Voyage>>;
    fn persist_voyage(&self, voyage: &Voyage) -> Result<()>;
}

pub trait EpicRepositoryPort {
    fn load_epic(&self, id: &str) -> Result<Option<Epic>>;
    fn list_epics(&self) -> Result<Vec<Epic>>;
    fn persist_epic(&self, epic: &Epic) -> Result<()>;
}

pub trait BearingRepositoryPort {
    fn load_bearing(&self, id: &str) -> Result<Option<Bearing>>;
    fn list_bearings(&self) -> Result<Vec<Bearing>>;
    fn persist_bearing(&self, bearing: &Bearing) -> Result<()>;
}

pub trait AdrRepositoryPort {
    fn load_adr(&self, id: &str) -> Result<Option<Adr>>;
    fn list_adrs(&self) -> Result<Vec<Adr>>;
    fn persist_adr(&self, adr: &Adr) -> Result<()>;
}

pub trait DocumentServicePort {
    fn read_document(&self, path: &Path) -> Result<String>;
    fn write_document(&self, path: &Path, content: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::Story;
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct MockBoardStore {
        board: Mutex<Board>,
    }

    impl BoardStore for MockBoardStore {
        fn load(&self) -> Result<Board> {
            Ok(self.board.lock().unwrap().clone())
        }
        fn save(&self, board: &Board) -> Result<()> {
            *self.board.lock().unwrap() = board.clone();
            Ok(())
        }
    }

    struct MockEntityStore<T: Entity + Clone> {
        entities: Mutex<HashMap<String, T>>,
    }

    impl<T: Entity + Clone + Send + Sync> EntityStore<T> for MockEntityStore<T> {
        fn get(&self, id: &str) -> Result<T> {
            self.entities
                .lock()
                .unwrap()
                .get(id)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Not found"))
        }
        fn put(&self, entity: &T) -> Result<()> {
            self.entities
                .lock()
                .unwrap()
                .insert(entity.id().to_string(), entity.clone());
            Ok(())
        }
        fn list(&self) -> Result<Vec<T>> {
            Ok(self.entities.lock().unwrap().values().cloned().collect())
        }
        fn delete(&self, id: &str) -> Result<()> {
            self.entities.lock().unwrap().remove(id);
            Ok(())
        }
    }

    #[test]
    fn board_store_mock_verified() {
        let board = Board::default();
        let store = MockBoardStore {
            board: Mutex::new(board.clone()),
        };
        let loaded = store.load().unwrap();
        assert_eq!(loaded.snapshot_version(), board.snapshot_version());
    }

    #[test]
    fn entity_store_mock_verified() {
        let store: MockEntityStore<Story> = MockEntityStore {
            entities: Mutex::new(HashMap::new()),
        };
        assert!(store.list().unwrap().is_empty());
    }
}
