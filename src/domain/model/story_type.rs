#![allow(dead_code)]
//! Story type classification

use serde::{Deserialize, Serialize};

/// Type of story (feature, bug fix, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StoryType {
    Feat,
    Bug,
    Chore,
    Refactor,
    Fix,
    Docs,
}

impl StoryType {
    /// Parse from string, case-insensitive
    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "feat" | "feature" => Some(StoryType::Feat),
            "bug" => Some(StoryType::Bug),
            "chore" => Some(StoryType::Chore),
            "refactor" => Some(StoryType::Refactor),
            "fix" => Some(StoryType::Fix),
            "docs" | "doc" => Some(StoryType::Docs),
            _ => None,
        }
    }
}

impl std::fmt::Display for StoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoryType::Feat => write!(f, "feat"),
            StoryType::Bug => write!(f, "bug"),
            StoryType::Chore => write!(f, "chore"),
            StoryType::Refactor => write!(f, "refactor"),
            StoryType::Fix => write!(f, "fix"),
            StoryType::Docs => write!(f, "docs"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn story_type_serializes_lowercase() {
        assert_eq!(
            serde_yaml::to_string(&StoryType::Feat).unwrap().trim(),
            "feat"
        );
        assert_eq!(
            serde_yaml::to_string(&StoryType::Refactor).unwrap().trim(),
            "refactor"
        );
    }

    #[test]
    fn story_type_deserializes_lowercase() {
        let st: StoryType = serde_yaml::from_str("feat").unwrap();
        assert_eq!(st, StoryType::Feat);
    }

    #[test]
    fn story_type_from_str_loose_handles_variants() {
        assert_eq!(StoryType::from_str_loose("FEAT"), Some(StoryType::Feat));
        assert_eq!(StoryType::from_str_loose("feature"), Some(StoryType::Feat));
        assert_eq!(StoryType::from_str_loose("Bug"), Some(StoryType::Bug));
        assert_eq!(StoryType::from_str_loose("doc"), Some(StoryType::Docs));
        assert_eq!(StoryType::from_str_loose("unknown"), None);
    }
}
