//! Filesystem store for canonical knowledge graph cache artifacts.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::infrastructure::utils::sha256_hex;
use crate::read_model::knowledge_graph::KnowledgeGraphProjection;

pub const KNOWLEDGE_GRAPH_CACHE_SCHEMA_VERSION: u32 = 1;
pub const DEFAULT_EMBEDDING_MODEL_ID: &str = "candle-local-placeholder";
pub const DEFAULT_EMBEDDING_CHUNKING_VERSION: &str = "v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeGraphEmbeddingCacheMeta {
    pub model_id: String,
    pub chunking_version: String,
    pub semantic_blob_hash: Option<String>,
    pub semantic_state: String,
}

impl Default for KnowledgeGraphEmbeddingCacheMeta {
    fn default() -> Self {
        Self {
            model_id: DEFAULT_EMBEDDING_MODEL_ID.to_string(),
            chunking_version: DEFAULT_EMBEDDING_CHUNKING_VERSION.to_string(),
            semantic_blob_hash: None,
            semantic_state: "missing".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeGraphCacheManifest {
    pub schema_version: u32,
    pub board_snapshot: u64,
    pub node_count: usize,
    pub edge_count: usize,
    pub input_files: BTreeMap<String, String>,
    pub projection_blob_hash: String,
    pub embeddings: KnowledgeGraphEmbeddingCacheMeta,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeGraphCacheSnapshot {
    pub cache_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub projection_blob_path: PathBuf,
    pub manifest: KnowledgeGraphCacheManifest,
    pub reused_manifest: bool,
    pub reused_projection_blob: bool,
}

pub fn save_knowledge_graph_cache(
    board_dir: &Path,
    board_snapshot: u64,
    projection: &KnowledgeGraphProjection,
    input_files: &BTreeMap<String, String>,
) -> Result<KnowledgeGraphCacheSnapshot> {
    let cache_dir = cache_dir(board_dir);
    let blobs_dir = blob_dir(board_dir);
    fs::create_dir_all(&blobs_dir)?;

    let projection_serialized = format!("{}\n", serde_json::to_string_pretty(projection)?);
    let projection_blob_hash = sha256_hex(projection_serialized.as_bytes());
    let projection_blob_path = blobs_dir.join(format!("{projection_blob_hash}.json"));
    let reused_projection_blob = write_if_changed(&projection_blob_path, &projection_serialized)?;

    let manifest = KnowledgeGraphCacheManifest {
        schema_version: KNOWLEDGE_GRAPH_CACHE_SCHEMA_VERSION,
        board_snapshot,
        node_count: projection.nodes.len(),
        edge_count: projection.edges.len(),
        input_files: input_files.clone(),
        projection_blob_hash: projection_blob_hash.clone(),
        embeddings: KnowledgeGraphEmbeddingCacheMeta::default(),
    };
    let serialized_manifest = format!("{}\n", serde_json::to_string_pretty(&manifest)?);
    let manifest_path = cache_dir.join("manifest.json");
    let reused_manifest = write_if_changed(&manifest_path, &serialized_manifest)?;

    Ok(KnowledgeGraphCacheSnapshot {
        cache_dir,
        manifest_path,
        projection_blob_path,
        manifest,
        reused_manifest,
        reused_projection_blob,
    })
}

pub fn cache_dir(board_dir: &Path) -> PathBuf {
    board_dir.join("cache").join("knowledge-graph")
}

fn blob_dir(board_dir: &Path) -> PathBuf {
    cache_dir(board_dir).join("blobs")
}

fn write_if_changed(path: &Path, content: &str) -> Result<bool> {
    let existing = fs::read_to_string(path).unwrap_or_default();
    if existing == content {
        return Ok(true);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_model::knowledge_graph::{
        KnowledgeGraphEdge, KnowledgeGraphEdgeKind, KnowledgeGraphNode, KnowledgeGraphNodeKind,
        KnowledgeGraphProjection, StructuralDriftInputs,
    };
    use tempfile::TempDir;

    fn sample_projection() -> KnowledgeGraphProjection {
        KnowledgeGraphProjection {
            schema_version: 1,
            nodes: vec![
                KnowledgeGraphNode {
                    id: "world:board".to_string(),
                    kind: KnowledgeGraphNodeKind::World,
                    title: "Keel Board".to_string(),
                    state: Some("live".to_string()),
                    path: None,
                    parent_id: None,
                },
                KnowledgeGraphNode {
                    id: "story:S1".to_string(),
                    kind: KnowledgeGraphNodeKind::Story,
                    title: "Story".to_string(),
                    state: Some("done".to_string()),
                    path: Some(".keel/stories/S1/README.md".to_string()),
                    parent_id: Some("world:board".to_string()),
                },
            ],
            edges: vec![KnowledgeGraphEdge {
                from: "world:board".to_string(),
                to: "story:S1".to_string(),
                kind: KnowledgeGraphEdgeKind::Contains,
            }],
            drift_inputs: StructuralDriftInputs::default(),
        }
    }

    #[test]
    fn save_knowledge_graph_cache_is_idempotent() {
        let temp = TempDir::new().unwrap();
        let input_files = BTreeMap::from([("README.md".to_string(), "abc".to_string())]);
        let projection = sample_projection();

        let first = save_knowledge_graph_cache(temp.path(), 7, &projection, &input_files).unwrap();
        let second = save_knowledge_graph_cache(temp.path(), 7, &projection, &input_files).unwrap();

        assert!(!first.reused_manifest);
        assert!(!first.reused_projection_blob);
        assert!(second.reused_manifest);
        assert!(second.reused_projection_blob);
        assert_eq!(first.manifest, second.manifest);
    }
}
