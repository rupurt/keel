//! Knowledge capture and management
//!
//! Scans and manages knowledge from stories, voyages, and ad-hoc files.

#![allow(dead_code)]
#![allow(unused_imports)]

mod model;
pub mod navigator;
pub mod scanner;

pub use model::{Knowledge, KnowledgeSourceType, KnowledgeSummary};
#[allow(unused_imports)]
pub use navigator::{
    BearingDraft, DetectionConfig, ReflectionSignal, RisingPattern, RolloutBatch, RolloutPhase,
    SeedBlocker, bearing_draft, detect_rising_patterns, rank_seed_candidates, rollout_batches,
    seed_blockers,
};
pub use scanner::{filter_by_category, filter_unapplied, parse_applies_to, scan_all_knowledge};
