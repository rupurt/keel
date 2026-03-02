//! Story verification manifest

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// High-fidelity verification manifest for a story
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Manifest {
    pub id: String,
    pub git_sha: String,
    pub evidence: BTreeMap<String, String>, // relative_path -> sha256
}
