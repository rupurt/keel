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
    KnowledgeCatalog, KnowledgeSimilarityConflict, KnowledgeSort,
    NEAR_DUPLICATE_KNOWLEDGE_THRESHOLD, RankedKnowledge, detect_similarity_conflicts,
    filter_by_category, filter_unapplied, is_canonical_knowledge_id, knowledge_file_path,
    load_reflection_knowledge, materialize_reflection_knowledge, parse_applies_to,
    parse_reflection_candidates, project_knowledge_catalog, prune_knowledge_catalog,
    rank_relevant_knowledge, scan_all_knowledge,
};
