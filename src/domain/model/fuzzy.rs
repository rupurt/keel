//! FuzzyMatch trait for consistent entity lookup across the board.
//!
//! This trait provides a unified pattern for fuzzy matching entities by pattern.
//! All board entities (Story, Voyage, Epic, Bearing) implement this trait,
//! enabling the generic `find_in` helper to search any collection.
//!
//! # Adding New Entity Types
//!
//! When adding a new entity type to the board:
//!
//! 1. Implement `FuzzyMatch` for your entity
//! 2. Add a `find_<entity>` method to Board that calls `find_in`
//!
//! Example:
//! ```ignore
//! impl FuzzyMatch for NewEntity {
//!     fn id(&self) -> &str {
//!         &self.frontmatter.id
//!     }
//!
//!     fn matches(&self, pattern: &str) -> bool {
//!         let pattern_lower = pattern.to_lowercase();
//!         let id_lower = self.id().to_lowercase();
//!         id_lower == pattern_lower
//!             || id_lower.contains(&pattern_lower)
//!             || self.frontmatter.title.to_lowercase().contains(&pattern_lower)
//!     }
//! }
//! ```

use std::collections::HashMap;

/// Trait for entities that support fuzzy matching by pattern.
pub trait FuzzyMatch {
    /// Returns the entity's unique identifier.
    #[allow(dead_code)] // Part of trait interface, used by implementors
    fn id(&self) -> &str;

    /// Returns true if this entity matches the given pattern.
    ///
    /// Implementations should support:
    /// - Exact ID match (case-insensitive)
    /// - ID contains pattern (case-insensitive)
    /// - Title contains pattern (case-insensitive) if applicable
    fn matches(&self, pattern: &str) -> bool;
}

/// Perform fuzzy matching on an ID and title against a pattern.
///
/// This is the shared implementation used by all entity types.
/// Matches if:
/// - ID equals pattern (case-insensitive)
/// - ID contains pattern (case-insensitive)
/// - Title contains pattern (case-insensitive)
pub fn fuzzy_match(id: &str, title: &str, pattern: &str) -> bool {
    let pattern_lower = pattern.to_lowercase();
    let id_lower = id.to_lowercase();

    // Exact ID match
    if id_lower == pattern_lower {
        return true;
    }

    // ID contains pattern
    if id_lower.contains(&pattern_lower) {
        return true;
    }

    // Title contains pattern
    if title.to_lowercase().contains(&pattern_lower) {
        return true;
    }

    false
}

/// Find an entity in a HashMap by pattern.
///
/// First tries exact match by key, then falls back to fuzzy matching.
/// This is the generic helper used by all `find_*` methods on Board.
pub fn find_in<'a, T: FuzzyMatch>(
    collection: &'a HashMap<String, T>,
    pattern: &str,
) -> Option<&'a T> {
    // Try exact match first (fast path)
    if let Some(entity) = collection.get(pattern) {
        return Some(entity);
    }

    // Fall back to fuzzy match
    collection.values().find(|e| e.matches(pattern))
}
