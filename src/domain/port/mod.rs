//! Storage Ports (traits) for Keel's hexagonal architecture.
//!
//! These traits decouple the domain and application logic from the
//! underlying persistence implementation (e.g., FileSystem, Database, Server).

use anyhow::Result;
use crate::domain::model::{Board, Entity};

/// Aggregate operations for the entire Board.
pub trait BoardStore {
    /// Load the complete board state.
    fn load(&self) -> Result<Board>;
    
    /// Save the complete board state.
    fn save(&self, board: &Board) -> Result<()>;
}

/// CRUD operations for individual entities.
pub trait EntityStore<T: Entity> {
    /// Retrieve a single entity by its unique identifier.
    fn get(&self, id: &str) -> Result<T>;
    
    /// Create or update an entity.
    fn put(&self, entity: &T) -> Result<()>;
    
    /// List all entities of this type.
    fn list(&self) -> Result<Vec<T>>;
    
    /// Remove an entity by its unique identifier.
    fn delete(&self, id: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::collections::HashMap;
    use crate::domain::model::Story;

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

    impl<T: Entity + Clone> EntityStore<T> for MockEntityStore<T> {
        fn get(&self, id: &str) -> Result<T> {
            self.entities.lock().unwrap().get(id).cloned().ok_or_else(|| anyhow::anyhow!("Not found"))
        }
        fn put(&self, entity: &T) -> Result<()> {
            self.entities.lock().unwrap().insert(entity.id().to_string(), entity.clone());
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
        let store = MockBoardStore { board: Mutex::new(board.clone()) };
        let loaded = store.load().unwrap();
        assert_eq!(loaded.snapshot_version(), board.snapshot_version());
    }

    #[test]
    fn entity_store_mock_verified() {
        let store: MockEntityStore<Story> = MockEntityStore { entities: Mutex::new(HashMap::new()) };
        assert!(store.list().unwrap().is_empty());
    }
}
