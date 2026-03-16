//! Watch data structure
//!
//! Watches are constraints on time. They provide a 12hr analog face
//! metaphor for tracking mission duration or windows.

use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use super::Entity;

/// Watch frontmatter from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WatchFrontmatter {
    /// Unique identifier
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Time limit in hours (analog cycle)
    pub limit_hours: u32,
}

/// A watch with its frontmatter and file location
#[derive(Debug, Clone)]
pub struct Watch {
    /// Parsed frontmatter
    pub frontmatter: WatchFrontmatter,
    /// Path to the watch file
    pub path: PathBuf,
}

impl Watch {
    /// Construct a watch from parsed frontmatter and its path.
    pub fn new(frontmatter: WatchFrontmatter, path: impl Into<PathBuf>) -> Self {
        Self {
            frontmatter,
            path: path.into(),
        }
    }

    /// Get the watch ID
    pub fn id(&self) -> &str {
        &self.frontmatter.id
    }

    /// Get the watch title
    pub fn title(&self) -> &str {
        &self.frontmatter.title
    }

    /// Get the time limit in hours
    pub fn limit_hours(&self) -> u32 {
        self.frontmatter.limit_hours
    }
}

impl Entity for Watch {
    fn id(&self) -> &str {
        &self.frontmatter.id
    }
    fn title(&self) -> &str {
        &self.frontmatter.title
    }
    fn path(&self) -> &Path {
        &self.path
    }
}
