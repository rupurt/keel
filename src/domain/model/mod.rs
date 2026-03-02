//! Board data model
//!
//! Structs for stories, voyages, epics, bearings, ADRs, and the board itself.

mod adr;
mod bearing;
mod board;
mod entity;
mod epic;
mod fuzzy;
mod manifest;
mod priority;
mod story;
mod story_type;
pub mod taxonomy;
mod voyage;

pub use crate::domain::state_machine::story::StoryState;
pub use crate::domain::state_machine::voyage::VoyageState;
#[allow(unused_imports)] // Used by future doctor checks (SRS-05)
pub use adr::{Adr, AdrFrontmatter, AdrStatus};
pub use bearing::{Bearing, BearingFrontmatter, BearingStatus};
pub use board::Board;
pub use entity::Entity;
pub use epic::{Epic, EpicFrontmatter, EpicState};
pub use fuzzy::{FuzzyMatch, find_in, fuzzy_match};
pub use manifest::Manifest;
#[allow(unused_imports)] // Priority may be used for voyages in future
pub use priority::Priority;
pub use story::{Story, StoryFrontmatter};
pub use story_type::StoryType;
pub use voyage::{Voyage, VoyageFrontmatter};

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

/// Custom deserializer that only accepts datetime (YYYY-MM-DDTHH:MM:SS)
pub(crate) fn deserialize_strict_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) => {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S") {
                return Ok(Some(dt));
            }
            Err(serde::de::Error::custom(format!(
                "invalid datetime format (expected YYYY-MM-DDTHH:MM:SS): {}",
                s
            )))
        }
    }
}
