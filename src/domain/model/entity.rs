//! Entity trait — common interface for all board entities

use std::path::Path;

/// Common interface for board entities (Story, Voyage, Epic, Bearing, Adr)
#[allow(dead_code)] // Trait used by impls; trait-bound usage in future stories
pub trait Entity {
    /// Unique identifier
    fn id(&self) -> &str;
    /// Human-readable title
    fn title(&self) -> &str;
    /// Path to the entity's file on disk
    fn path(&self) -> &Path;
}
