//! Logic for verifying requirements via automated or manual checks

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod comparator;
pub mod executor;
pub mod judge_bundle;
pub mod parser;
pub mod reporter;
#[cfg(test)]
mod tests;

pub use executor::{verify_all, verify_story};
pub use judge_bundle::{JudgeBundle, JudgeBundleCriterion, JudgeBundleEvidence, JudgeBundleStory};
pub use parser::{
    AcReference, Comparison, RequirementPhase, RequirementRef, VerifyAnnotation,
    parse_ac_references, parse_verify_annotations,
};
pub use reporter::{
    VerificationReport, VerificationResult, VerificationSummary, format_terminal,
    print_terminal_report,
};
