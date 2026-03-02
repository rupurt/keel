//! Repository ports for board aggregate persistence.
//!
//! These traits define application-facing contracts for loading and persisting
//! board aggregates without binding use cases to filesystem details.

use anyhow::Result;
use std::path::Path;

use crate::domain::model::{Adr, Bearing, Board, Epic, Story, Voyage};

/// Board-level repository port for whole-board snapshots.
#[allow(dead_code)]
pub trait BoardRepositoryPort {
    fn load_board(&self) -> Result<Board>;
    fn persist_board(&self, board: &Board) -> Result<()>;
}

/// Story aggregate repository port.
#[allow(dead_code)]
pub trait StoryRepositoryPort {
    fn load_story(&self, id: &str) -> Result<Option<Story>>;
    fn list_stories(&self) -> Result<Vec<Story>>;
    fn persist_story(&self, story: &Story) -> Result<()>;
}

/// Voyage aggregate repository port.
#[allow(dead_code)]
pub trait VoyageRepositoryPort {
    fn load_voyage(&self, id: &str) -> Result<Option<Voyage>>;
    fn list_voyages(&self) -> Result<Vec<Voyage>>;
    fn persist_voyage(&self, voyage: &Voyage) -> Result<()>;
}

/// Epic aggregate repository port.
#[allow(dead_code)]
pub trait EpicRepositoryPort {
    fn load_epic(&self, id: &str) -> Result<Option<Epic>>;
    fn list_epics(&self) -> Result<Vec<Epic>>;
    fn persist_epic(&self, epic: &Epic) -> Result<()>;
}

/// Bearing aggregate repository port.
#[allow(dead_code)]
pub trait BearingRepositoryPort {
    fn load_bearing(&self, id: &str) -> Result<Option<Bearing>>;
    fn list_bearings(&self) -> Result<Vec<Bearing>>;
    fn persist_bearing(&self, bearing: &Bearing) -> Result<()>;
}

/// ADR aggregate repository port.
#[allow(dead_code)]
pub trait AdrRepositoryPort {
    fn load_adr(&self, id: &str) -> Result<Option<Adr>>;
    fn list_adrs(&self) -> Result<Vec<Adr>>;
    fn persist_adr(&self, adr: &Adr) -> Result<()>;
}

/// Document service port for reading and writing board documents.
#[allow(dead_code)]
pub trait DocumentServicePort {
    fn read_document(&self, path: &Path) -> Result<String>;
    fn write_document(&self, path: &Path, content: &str) -> Result<()>;
}
