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
pub use scanner::{
    KnowledgeManifest, KnowledgeMigrationReport, KnowledgeSort, RankedKnowledge,
    filter_by_category, filter_unapplied, is_canonical_knowledge_id, load_knowledge_manifest,
    migrate_legacy_knowledge_ids, parse_applies_to, rank_relevant_knowledge, scan_all_knowledge,
    sync_knowledge_manifest,
};
