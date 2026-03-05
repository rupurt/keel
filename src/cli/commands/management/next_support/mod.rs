//! Pull-system decision selection and formatting

pub mod algorithm;
pub mod format;
pub mod parallel_features;
pub mod parallel_scoring;
pub mod staleness;

pub use algorithm::{
    AcceptDecision, AdrDecision, BlockedDecision, DecomposeDecision, EmptyDecision, NextDecision,
    ResearchDecision, StoryDecision, calculate_next,
};
pub use format::format_decision;
// pub use staleness::staleness_score;
