//! Canonical knowledge-graph projection for entities, artifacts, documents, and source files.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::domain::model::{Board, Story};
use crate::infrastructure::utils::{cmp_optional_index_then_id, sha256_hex};
use crate::infrastructure::verification::parse_ac_references;
use crate::read_model::board_graph::{BoardEdgeKind, BoardNodeId, build_board_graph};
use crate::read_model::knowledge::{self, parse_applies_to};

pub const KNOWLEDGE_GRAPH_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KnowledgeGraphNodeKind {
    World,
    Mission,
    Epic,
    Bearing,
    Adr,
    Voyage,
    Story,
    Routine,
    Watch,
    Heartbeat,
    Artifact,
    Knowledge,
    ProjectDoc,
    SourceFile,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KnowledgeGraphEdgeKind {
    Contains,
    DependsOn,
    GovernedBy,
    LaidInto,
    Energizes,
    Documents,
    Provenance,
    Traceability,
    Attachment,
    AppliesTo,
    KnowledgeLink,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeGraphNode {
    pub id: String,
    pub kind: KnowledgeGraphNodeKind,
    pub title: String,
    pub state: Option<String>,
    pub path: Option<String>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct KnowledgeGraphEdge {
    pub from: String,
    pub to: String,
    pub kind: KnowledgeGraphEdgeKind,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuralDriftInputs {
    pub total_entities: usize,
    pub entities_with_artifacts: usize,
    pub entities_without_artifacts: usize,
    pub total_knowledge_units: usize,
    pub knowledge_with_source_attachments: usize,
    pub knowledge_without_source_attachments: usize,
    pub total_source_files: usize,
    pub source_files_with_attachments: usize,
    pub source_files_without_attachments: usize,
    pub total_project_docs: usize,
    pub linked_project_docs: usize,
    pub unlinked_project_docs: usize,
}

impl StructuralDriftInputs {
    pub fn structural_drift_coefficient(&self) -> f64 {
        let segments = [
            coverage_gap(self.entities_with_artifacts, self.total_entities),
            coverage_gap(
                self.knowledge_with_source_attachments,
                self.total_knowledge_units,
            ),
            coverage_gap(self.source_files_with_attachments, self.total_source_files),
            coverage_gap(self.linked_project_docs, self.total_project_docs),
        ];
        let active: Vec<_> = segments
            .into_iter()
            .filter(|value| !value.is_nan())
            .collect();
        if active.is_empty() {
            0.0
        } else {
            let average = active.iter().sum::<f64>() / active.len() as f64;
            (average * 100.0).round() / 100.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DriftFacetKind {
    EntityArtifacts,
    KnowledgeProvenance,
    SourceAttachments,
    ProjectDocs,
}

impl DriftFacetKind {
    pub fn short_label(self) -> &'static str {
        match self {
            Self::EntityArtifacts => "entities",
            Self::KnowledgeProvenance => "knowledge",
            Self::SourceAttachments => "source",
            Self::ProjectDocs => "docs",
        }
    }

    pub fn missing_label(self, missing: usize) -> String {
        match self {
            Self::EntityArtifacts => pluralized_gap(
                missing,
                "entity lacks an authored artifact",
                "entities lack authored artifacts",
            ),
            Self::KnowledgeProvenance => pluralized_gap(
                missing,
                "knowledge unit lacks source provenance",
                "knowledge units lack source provenance",
            ),
            Self::SourceAttachments => pluralized_gap(
                missing,
                "source file lacks graph attachment",
                "source files lack graph attachments",
            ),
            Self::ProjectDocs => pluralized_gap(
                missing,
                "project doc is unlinked",
                "project docs are unlinked",
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftFacetSummary {
    pub kind: DriftFacetKind,
    pub covered: usize,
    pub total: usize,
}

impl DriftFacetSummary {
    pub fn missing(&self) -> usize {
        self.total.saturating_sub(self.covered)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriftSurfaceSummary {
    pub coefficient: f64,
    pub facets: Vec<DriftFacetSummary>,
}

impl DriftSurfaceSummary {
    pub fn severity_label(&self) -> &'static str {
        match self.coefficient {
            value if value < 0.15 => "aligned",
            value if value < 0.35 => "watch",
            value if value < 0.60 => "elevated",
            _ => "severe",
        }
    }

    pub fn hotspot_messages(&self, limit: usize) -> Vec<String> {
        let mut hotspots = self
            .facets
            .iter()
            .filter(|facet| facet.missing() > 0)
            .map(|facet| {
                (
                    facet.missing(),
                    facet.kind,
                    facet.kind.missing_label(facet.missing()),
                )
            })
            .collect::<Vec<_>>();
        hotspots.sort_by(|left, right| {
            right
                .0
                .cmp(&left.0)
                .then_with(|| left.1.cmp(&right.1))
                .then_with(|| left.2.cmp(&right.2))
        });
        hotspots
            .into_iter()
            .take(limit)
            .map(|(_, _, message)| message)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeGraphProjection {
    pub schema_version: u32,
    pub nodes: Vec<KnowledgeGraphNode>,
    pub edges: Vec<KnowledgeGraphEdge>,
    pub drift_inputs: StructuralDriftInputs,
}

pub fn build_knowledge_graph_projection(board: &Board) -> Result<KnowledgeGraphProjection> {
    let board_graph = build_board_graph(board);
    let mut nodes = BTreeMap::<String, KnowledgeGraphNode>::new();
    let mut edges = BTreeSet::<KnowledgeGraphEdge>::new();

    for node in board_graph.nodes() {
        let graph_node = graph_node_from_board_node(board, node);
        nodes.insert(graph_node.id.clone(), graph_node);
    }

    for edge in board_graph.edges() {
        edges.insert(KnowledgeGraphEdge {
            from: graph_node_id(&edge.from),
            to: graph_node_id(&edge.to),
            kind: edge_kind_from_board_edge(edge.kind),
        });
    }

    attach_artifacts(board, &mut nodes, &mut edges);
    attach_project_docs(board, &mut nodes, &mut edges);
    attach_source_files(board, &mut nodes, &mut edges);
    attach_traceability(board, &mut edges);
    attach_knowledge(board, &mut nodes, &mut edges)?;

    let nodes = nodes.into_values().collect::<Vec<_>>();
    let edges = edges.into_iter().collect::<Vec<_>>();
    let drift_inputs = build_structural_drift_inputs(&nodes, &edges);

    Ok(KnowledgeGraphProjection {
        schema_version: KNOWLEDGE_GRAPH_SCHEMA_VERSION,
        nodes,
        edges,
        drift_inputs,
    })
}

pub fn build_structural_drift_summary(
    projection: &KnowledgeGraphProjection,
) -> DriftSurfaceSummary {
    build_structural_drift_summary_from_inputs(&projection.drift_inputs)
}

pub fn project_structural_drift_summary(board: &Board) -> Result<DriftSurfaceSummary> {
    let projection = build_knowledge_graph_projection(board)?;
    Ok(build_structural_drift_summary(&projection))
}

pub fn projection_input_hashes(
    board: &Board,
    projection: &KnowledgeGraphProjection,
) -> Result<BTreeMap<String, String>> {
    let mut paths = BTreeSet::new();
    for node in &projection.nodes {
        if let Some(path) = &node.path {
            paths.insert(path.clone());
        }
    }

    let project_root = project_root(board);
    let mut hashes = BTreeMap::new();
    for relative in paths {
        let absolute = project_root.join(&relative);
        if !absolute.exists() || !absolute.is_file() {
            continue;
        }
        let bytes = fs::read(&absolute)?;
        hashes.insert(relative, sha256_hex(&bytes));
    }
    Ok(hashes)
}

fn attach_artifacts(
    board: &Board,
    nodes: &mut BTreeMap<String, KnowledgeGraphNode>,
    edges: &mut BTreeSet<KnowledgeGraphEdge>,
) {
    let project_root = project_root(board);
    for path in artifact_paths(board) {
        let Some(relative) = relative_to(&project_root, &path) else {
            continue;
        };
        let artifact_id = format!("artifact:{relative}");
        nodes
            .entry(artifact_id.clone())
            .or_insert_with(|| KnowledgeGraphNode {
                id: artifact_id.clone(),
                kind: KnowledgeGraphNodeKind::Artifact,
                title: relative.clone(),
                state: None,
                path: Some(relative.clone()),
                parent_id: artifact_owner_id(board, &path),
            });

        let owner_id = artifact_owner_id(board, &path).unwrap_or_else(|| "world:board".to_string());
        edges.insert(KnowledgeGraphEdge {
            from: owner_id,
            to: artifact_id,
            kind: KnowledgeGraphEdgeKind::Documents,
        });
    }
}

fn attach_project_docs(
    board: &Board,
    nodes: &mut BTreeMap<String, KnowledgeGraphNode>,
    edges: &mut BTreeSet<KnowledgeGraphEdge>,
) {
    let project_root = project_root(board);
    for path in project_doc_paths(board) {
        let Some(relative) = relative_to(&project_root, &path) else {
            continue;
        };
        let doc_id = format!("doc:{relative}");
        nodes
            .entry(doc_id.clone())
            .or_insert_with(|| KnowledgeGraphNode {
                id: doc_id.clone(),
                kind: KnowledgeGraphNodeKind::ProjectDoc,
                title: relative.clone(),
                state: None,
                path: Some(relative.clone()),
                parent_id: Some("world:board".to_string()),
            });
        edges.insert(KnowledgeGraphEdge {
            from: "world:board".to_string(),
            to: doc_id,
            kind: KnowledgeGraphEdgeKind::Contains,
        });
    }
}

fn attach_source_files(
    board: &Board,
    nodes: &mut BTreeMap<String, KnowledgeGraphNode>,
    edges: &mut BTreeSet<KnowledgeGraphEdge>,
) {
    let project_root = project_root(board);
    for path in source_file_paths(board) {
        let Some(relative) = relative_to(&project_root, &path) else {
            continue;
        };
        let source_id = format!("source:{relative}");
        nodes
            .entry(source_id.clone())
            .or_insert_with(|| KnowledgeGraphNode {
                id: source_id.clone(),
                kind: KnowledgeGraphNodeKind::SourceFile,
                title: relative.clone(),
                state: None,
                path: Some(relative.clone()),
                parent_id: Some("world:board".to_string()),
            });
        edges.insert(KnowledgeGraphEdge {
            from: "world:board".to_string(),
            to: source_id,
            kind: KnowledgeGraphEdgeKind::Contains,
        });
    }
}

fn attach_traceability(board: &Board, edges: &mut BTreeSet<KnowledgeGraphEdge>) {
    let project_root = project_root(board);

    for voyage in sorted_voyages(board) {
        let prd_relative = relative_to(
            &project_root,
            &voyage
                .path
                .parent()
                .and_then(|voyage_dir| voyage_dir.parent())
                .and_then(|voyages_dir| voyages_dir.parent())
                .map(|epic_dir| epic_dir.join("PRD.md"))
                .unwrap_or_else(|| project_root.join(".keel/missing/PRD.md")),
        );
        if let Some(prd_relative) = prd_relative
            && voyage
                .path
                .parent()
                .and_then(|voyage_dir| voyage_dir.parent())
                .and_then(|voyages_dir| voyages_dir.parent())
                .map(|epic_dir| epic_dir.join("PRD.md"))
                .is_some_and(|path| path.exists())
        {
            edges.insert(KnowledgeGraphEdge {
                from: format!("voyage:{}", voyage.id()),
                to: format!("artifact:{prd_relative}"),
                kind: KnowledgeGraphEdgeKind::Traceability,
            });
        }
    }

    for story in sorted_stories(board) {
        let content = match fs::read_to_string(&story.path) {
            Ok(content) => content,
            Err(_) => continue,
        };
        if parse_ac_references(&content).is_empty() {
            continue;
        }
        let Some(voyage_id) = story.voyage() else {
            continue;
        };
        let Some(voyage) = board.voyages.get(voyage_id) else {
            continue;
        };
        let srs_path = voyage.path.parent().unwrap().join("SRS.md");
        if !srs_path.exists() {
            continue;
        }
        let Some(relative) = relative_to(&project_root, &srs_path) else {
            continue;
        };
        edges.insert(KnowledgeGraphEdge {
            from: format!("story:{}", story.id()),
            to: format!("artifact:{relative}"),
            kind: KnowledgeGraphEdgeKind::Traceability,
        });
    }
}

fn attach_knowledge(
    board: &Board,
    nodes: &mut BTreeMap<String, KnowledgeGraphNode>,
    edges: &mut BTreeSet<KnowledgeGraphEdge>,
) -> Result<()> {
    let project_root = project_root(board);
    let source_lookup = nodes
        .iter()
        .filter_map(|(id, node)| {
            node.path
                .as_ref()
                .map(|path| (normalize_attachment_target(path), id.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    for unit in knowledge::scan_all_knowledge(&board.root)? {
        let knowledge_id = format!("knowledge:{}", unit.id);
        nodes
            .entry(knowledge_id.clone())
            .or_insert_with(|| KnowledgeGraphNode {
                id: knowledge_id.clone(),
                kind: KnowledgeGraphNodeKind::Knowledge,
                title: unit.title.clone(),
                state: Some(if unit.is_pending() {
                    "pending".to_string()
                } else {
                    "applied".to_string()
                }),
                path: relative_to(&project_root, &unit.source),
                parent_id: Some("world:board".to_string()),
            });

        if let Some(source_relative) = relative_to(&project_root, &unit.source) {
            let source_node_id = if source_relative.starts_with(".keel/") {
                format!("artifact:{source_relative}")
            } else {
                format!("doc:{source_relative}")
            };
            if nodes.contains_key(&source_node_id) {
                edges.insert(KnowledgeGraphEdge {
                    from: source_node_id,
                    to: knowledge_id.clone(),
                    kind: KnowledgeGraphEdgeKind::Provenance,
                });
            }
        } else if let Some(story_id) = unit.source_story_id.as_deref() {
            edges.insert(KnowledgeGraphEdge {
                from: format!("story:{story_id}"),
                to: knowledge_id.clone(),
                kind: KnowledgeGraphEdgeKind::Provenance,
            });
        }

        for linked_id in &unit.linked_ids {
            let target = format!("knowledge:{linked_id}");
            edges.insert(KnowledgeGraphEdge {
                from: knowledge_id.clone(),
                to: target,
                kind: KnowledgeGraphEdgeKind::KnowledgeLink,
            });
        }

        let mut owner_attachment_targets = BTreeSet::new();
        for target in parse_applies_to(&unit.applies_to) {
            let normalized = normalize_attachment_target(&target);
            let Some(source_node_id) = source_lookup.get(&normalized) else {
                continue;
            };
            edges.insert(KnowledgeGraphEdge {
                from: knowledge_id.clone(),
                to: source_node_id.clone(),
                kind: KnowledgeGraphEdgeKind::AppliesTo,
            });
            owner_attachment_targets.insert(source_node_id.clone());
        }

        if owner_attachment_targets.is_empty() {
            continue;
        }
        if let Some(owner) = knowledge_owner_node_id(board, &unit) {
            for target in owner_attachment_targets {
                edges.insert(KnowledgeGraphEdge {
                    from: owner.clone(),
                    to: target,
                    kind: KnowledgeGraphEdgeKind::Attachment,
                });
            }
        }
    }

    Ok(())
}

fn graph_node_from_board_node(
    board: &Board,
    node: &crate::read_model::board_graph::BoardGraphNode,
) -> KnowledgeGraphNode {
    KnowledgeGraphNode {
        id: graph_node_id(&node.id),
        kind: graph_node_kind(node.kind),
        title: node.title.clone(),
        state: Some(node.state.clone()),
        path: relative_to(&project_root(board), &path_for_board_node(board, &node.id)),
        parent_id: node.declared_parent.as_ref().map(graph_node_id),
    }
}

fn graph_node_kind(kind: crate::read_model::board_graph::BoardNodeKind) -> KnowledgeGraphNodeKind {
    match kind {
        crate::read_model::board_graph::BoardNodeKind::Board => KnowledgeGraphNodeKind::World,
        crate::read_model::board_graph::BoardNodeKind::Mission => KnowledgeGraphNodeKind::Mission,
        crate::read_model::board_graph::BoardNodeKind::Epic => KnowledgeGraphNodeKind::Epic,
        crate::read_model::board_graph::BoardNodeKind::Bearing => KnowledgeGraphNodeKind::Bearing,
        crate::read_model::board_graph::BoardNodeKind::Adr => KnowledgeGraphNodeKind::Adr,
        crate::read_model::board_graph::BoardNodeKind::Voyage => KnowledgeGraphNodeKind::Voyage,
        crate::read_model::board_graph::BoardNodeKind::Story => KnowledgeGraphNodeKind::Story,
        crate::read_model::board_graph::BoardNodeKind::Routine => KnowledgeGraphNodeKind::Routine,
        crate::read_model::board_graph::BoardNodeKind::Watch => KnowledgeGraphNodeKind::Watch,
        crate::read_model::board_graph::BoardNodeKind::Heartbeat => {
            KnowledgeGraphNodeKind::Heartbeat
        }
    }
}

fn graph_node_id(id: &BoardNodeId) -> String {
    match id {
        BoardNodeId::Board => "world:board".to_string(),
        BoardNodeId::Mission(id) => format!("mission:{id}"),
        BoardNodeId::Epic(id) => format!("epic:{id}"),
        BoardNodeId::Bearing(id) => format!("bearing:{id}"),
        BoardNodeId::Adr(id) => format!("adr:{id}"),
        BoardNodeId::Voyage(id) => format!("voyage:{id}"),
        BoardNodeId::Story(id) => format!("story:{id}"),
        BoardNodeId::Routine(id) => format!("routine:{id}"),
        BoardNodeId::Watch(id) => format!("watch:{id}"),
        BoardNodeId::Heartbeat => "system:heartbeat".to_string(),
    }
}

fn edge_kind_from_board_edge(kind: BoardEdgeKind) -> KnowledgeGraphEdgeKind {
    match kind {
        BoardEdgeKind::Contains => KnowledgeGraphEdgeKind::Contains,
        BoardEdgeKind::DependsOn => KnowledgeGraphEdgeKind::DependsOn,
        BoardEdgeKind::GovernedBy => KnowledgeGraphEdgeKind::GovernedBy,
        BoardEdgeKind::LaidInto => KnowledgeGraphEdgeKind::LaidInto,
        BoardEdgeKind::Energizes => KnowledgeGraphEdgeKind::Energizes,
    }
}

fn path_for_board_node(board: &Board, id: &BoardNodeId) -> PathBuf {
    match id {
        BoardNodeId::Board => board.root.join("README.md"),
        BoardNodeId::Mission(id) => board
            .missions
            .get(id)
            .map(|mission| mission.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Epic(id) => board
            .epics
            .get(id)
            .map(|epic| epic.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Bearing(id) => board
            .bearings
            .get(id)
            .map(|bearing| bearing.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Adr(id) => board
            .adrs
            .get(id)
            .map(|adr| adr.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Voyage(id) => board
            .voyages
            .get(id)
            .map(|voyage| voyage.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Story(id) => board
            .stories
            .get(id)
            .map(|story| story.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Routine(id) => board
            .routines
            .get(id)
            .map(|routine| routine.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Watch(id) => board
            .watches
            .get(id)
            .map(|watch| watch.path.clone())
            .unwrap_or_else(|| board.root.join("README.md")),
        BoardNodeId::Heartbeat => board.root.join("heartbeat"),
    }
}

fn artifact_paths(board: &Board) -> Vec<PathBuf> {
    let knowledge_catalog_dir = board.root.join("knowledge");
    let mut paths = WalkDir::new(&board.root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
        .map(|entry| entry.into_path())
        .filter(|path| !path.starts_with(&knowledge_catalog_dir))
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn project_doc_paths(board: &Board) -> Vec<PathBuf> {
    let project_root = project_root(board);
    let mut paths = Vec::new();

    if let Ok(entries) = fs::read_dir(&project_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                paths.push(path);
            }
        }
    }

    for extra in ["docs", "knowledge"] {
        let base = project_root.join(extra);
        if !base.exists() {
            continue;
        }
        paths.extend(
            WalkDir::new(base)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
                .map(|entry| entry.into_path()),
        );
    }

    paths.sort();
    paths.dedup();
    paths
}

fn source_file_paths(board: &Board) -> Vec<PathBuf> {
    let src_dir = project_root(board).join("src");
    if !src_dir.exists() {
        return Vec::new();
    }

    let mut paths = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn artifact_owner_id(board: &Board, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(&board.root).ok()?;
    let components = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>();

    match components.as_slice() {
        ["README.md"] => Some("world:board".to_string()),
        ["missions", mission_id, ..] => Some(format!("mission:{mission_id}")),
        ["epics", _epic_id, "voyages", voyage_id, ..] => Some(format!("voyage:{voyage_id}")),
        ["epics", epic_id, ..] => Some(format!("epic:{epic_id}")),
        ["stories", story_id, ..] => Some(format!("story:{story_id}")),
        ["routines", routine_id, ..] => Some(format!("routine:{routine_id}")),
        ["bearings", bearing_id, ..] => Some(format!("bearing:{bearing_id}")),
        ["adrs", filename] => Some(format!("adr:{}", filename.trim_end_matches(".md"))),
        _ => None,
    }
}

fn knowledge_owner_node_id(board: &Board, unit: &knowledge::Knowledge) -> Option<String> {
    if let Some(story_id) = unit.source_story_id.as_deref() {
        return Some(format!("story:{story_id}"));
    }

    let project_root = project_root(board);
    let relative = relative_to(&project_root, &unit.source)?;
    if relative.starts_with(".keel/epics/") && relative.contains("/voyages/") {
        let parts = relative.split('/').collect::<Vec<_>>();
        let voyage_id = parts.get(4)?;
        return Some(format!("voyage:{voyage_id}"));
    }
    if relative.starts_with(".keel/") {
        return artifact_owner_id(board, &project_root.join(relative));
    }
    Some("world:board".to_string())
}

fn build_structural_drift_inputs(
    nodes: &[KnowledgeGraphNode],
    edges: &[KnowledgeGraphEdge],
) -> StructuralDriftInputs {
    let entity_ids = nodes
        .iter()
        .filter(|node| {
            matches!(
                node.kind,
                KnowledgeGraphNodeKind::Mission
                    | KnowledgeGraphNodeKind::Epic
                    | KnowledgeGraphNodeKind::Bearing
                    | KnowledgeGraphNodeKind::Adr
                    | KnowledgeGraphNodeKind::Voyage
                    | KnowledgeGraphNodeKind::Story
                    | KnowledgeGraphNodeKind::Routine
            )
        })
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let artifact_attached_entities = edges
        .iter()
        .filter(|edge| edge.kind == KnowledgeGraphEdgeKind::Documents)
        .map(|edge| edge.from.clone())
        .collect::<BTreeSet<_>>();
    let knowledge_ids = nodes
        .iter()
        .filter(|node| node.kind == KnowledgeGraphNodeKind::Knowledge)
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let knowledge_with_source = edges
        .iter()
        .filter(|edge| edge.kind == KnowledgeGraphEdgeKind::AppliesTo)
        .map(|edge| edge.from.clone())
        .collect::<BTreeSet<_>>();
    let source_ids = nodes
        .iter()
        .filter(|node| node.kind == KnowledgeGraphNodeKind::SourceFile)
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let source_with_attachment = edges
        .iter()
        .filter(|edge| {
            matches!(
                edge.kind,
                KnowledgeGraphEdgeKind::AppliesTo | KnowledgeGraphEdgeKind::Attachment
            )
        })
        .map(|edge| edge.to.clone())
        .collect::<BTreeSet<_>>();
    let project_doc_ids = nodes
        .iter()
        .filter(|node| node.kind == KnowledgeGraphNodeKind::ProjectDoc)
        .map(|node| node.id.clone())
        .collect::<BTreeSet<_>>();
    let linked_project_docs = edges
        .iter()
        .filter(|edge| {
            edge.kind == KnowledgeGraphEdgeKind::Provenance && project_doc_ids.contains(&edge.from)
        })
        .map(|edge| edge.from.clone())
        .collect::<BTreeSet<_>>();

    StructuralDriftInputs {
        total_entities: entity_ids.len(),
        entities_with_artifacts: artifact_attached_entities.intersection(&entity_ids).count(),
        entities_without_artifacts: entity_ids.difference(&artifact_attached_entities).count(),
        total_knowledge_units: knowledge_ids.len(),
        knowledge_with_source_attachments: knowledge_with_source
            .intersection(&knowledge_ids)
            .count(),
        knowledge_without_source_attachments: knowledge_ids
            .difference(&knowledge_with_source)
            .count(),
        total_source_files: source_ids.len(),
        source_files_with_attachments: source_with_attachment.intersection(&source_ids).count(),
        source_files_without_attachments: source_ids.difference(&source_with_attachment).count(),
        total_project_docs: project_doc_ids.len(),
        linked_project_docs: linked_project_docs.len(),
        unlinked_project_docs: project_doc_ids.difference(&linked_project_docs).count(),
    }
}

fn coverage_gap(covered: usize, total: usize) -> f64 {
    if total == 0 {
        f64::NAN
    } else {
        1.0 - (covered as f64 / total as f64)
    }
}

fn build_structural_drift_summary_from_inputs(
    inputs: &StructuralDriftInputs,
) -> DriftSurfaceSummary {
    DriftSurfaceSummary {
        coefficient: inputs.structural_drift_coefficient(),
        facets: vec![
            DriftFacetSummary {
                kind: DriftFacetKind::EntityArtifacts,
                covered: inputs.entities_with_artifacts,
                total: inputs.total_entities,
            },
            DriftFacetSummary {
                kind: DriftFacetKind::KnowledgeProvenance,
                covered: inputs.knowledge_with_source_attachments,
                total: inputs.total_knowledge_units,
            },
            DriftFacetSummary {
                kind: DriftFacetKind::SourceAttachments,
                covered: inputs.source_files_with_attachments,
                total: inputs.total_source_files,
            },
            DriftFacetSummary {
                kind: DriftFacetKind::ProjectDocs,
                covered: inputs.linked_project_docs,
                total: inputs.total_project_docs,
            },
        ],
    }
}

fn pluralized_gap(count: usize, singular: &str, plural: &str) -> String {
    match count {
        1 => format!("1 {singular}"),
        _ => format!("{count} {plural}"),
    }
}

fn relative_to(root: &Path, path: &Path) -> Option<String> {
    path.strip_prefix(root)
        .ok()
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
}

fn project_root(board: &Board) -> PathBuf {
    if board.root.file_name().is_some_and(|name| name == ".keel") {
        board.root.parent().unwrap_or(&board.root).to_path_buf()
    } else {
        board.root.clone()
    }
}

fn normalize_attachment_target(target: &str) -> String {
    target
        .trim()
        .trim_matches('`')
        .trim_start_matches("./")
        .replace('\\', "/")
}

fn sorted_voyages(board: &Board) -> Vec<&crate::domain::model::Voyage> {
    let mut voyages = board.voyages.values().collect::<Vec<_>>();
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    voyages
}

fn sorted_stories(board: &Board) -> Vec<&Story> {
    let mut stories = board.stories.values().collect::<Vec<_>>();
    stories.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    stories
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::knowledge_graph_store::save_knowledge_graph_cache;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};
    use tempfile::TempDir;

    fn build_fixture(epic_order: &[(&str, u32)], story_order: &[(&str, u32)]) -> TempDir {
        let srs = r#"# SRS

## Scope
### In Scope
- [SCOPE-01] Graph

### Out of Scope
- [SCOPE-02] Other

## Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Graph projection | SCOPE-01 | FR-01 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let mut builder = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E0").mission("M1").index(0))
            .voyage(
                TestVoyage::new("V0", "E0")
                    .status("in-progress")
                    .index(0)
                    .srs_content(srs),
            );

        for (epic_id, index) in epic_order {
            builder = builder.epic(TestEpic::new(epic_id).mission("M1").index(*index));
        }

        for (story_id, index) in story_order {
            builder = builder.story(
                TestStory::new(story_id)
                    .title(&format!("Story {story_id}"))
                    .scope("E0/V0")
                    .status(crate::domain::model::StoryState::Done)
                    .index(*index)
                    .body(
                        "# Summary\n\nGraph slice.\n\n## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] graph <!-- verify: manual, SRS-01:start:end -->\n",
                    ),
            );
        }

        let temp = builder.build();
        fs::create_dir_all(temp.path().join("knowledge")).unwrap();
        fs::write(
            temp.path().join("knowledge/semantic.md"),
            r#"---
source_type: Adhoc
source: knowledge/semantic.md
---

### 1AbCdE241: Knowledge Graph Uses Deterministic Roots

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Graph kernel |
| **Insight** | Use canonical nodes before rendering |
| **Suggested Action** | Reuse BoardGraph |
| **Applies To** | src/read_model/knowledge_graph.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-12T00:00:00Z |
| **Score** | 0.90 |
| **Confidence** | 0.95 |
| **Applied** |  |
"#,
        )
        .unwrap();
        fs::write(temp.path().join("README.md"), "# Project README\n").unwrap();
        fs::write(temp.path().join("ARCHITECTURE.md"), "# Architecture\n").unwrap();
        fs::create_dir_all(temp.path().join("src/read_model")).unwrap();
        fs::write(temp.path().join("src/lib.rs"), "pub mod read_model;\n").unwrap();
        fs::write(
            temp.path().join("src/read_model/knowledge_graph.rs"),
            "pub fn kernel() {}\n",
        )
        .unwrap();

        temp
    }

    #[test]
    fn knowledge_graph_projection_builds_entity_document_and_code_nodes() {
        let temp = build_fixture(&[("E2", 2), ("E1", 1)], &[("S2", 2), ("S1", 1)]);
        let board = load_board(temp.path()).unwrap();

        let projection = build_knowledge_graph_projection(&board).unwrap();
        let node_ids = projection
            .nodes
            .iter()
            .map(|node| node.id.as_str())
            .collect::<BTreeSet<_>>();

        assert!(node_ids.contains("world:board"));
        assert!(node_ids.contains("mission:M1"));
        assert!(node_ids.contains("epic:E0"));
        assert!(node_ids.contains("voyage:V0"));
        assert!(node_ids.contains("story:S1"));
        assert!(node_ids.contains("artifact:epics/E0/PRD.md"));
        assert!(node_ids.contains("doc:README.md"));
        assert!(node_ids.contains("source:src/read_model/knowledge_graph.rs"));
        assert!(node_ids.contains("knowledge:1AbCdE241"));
    }

    #[test]
    fn knowledge_graph_projection_builds_structural_edges() {
        let temp = build_fixture(&[], &[("S1", 1)]);
        let board = load_board(temp.path()).unwrap();

        let projection = build_knowledge_graph_projection(&board).unwrap();
        let edges = projection
            .edges
            .iter()
            .map(|edge| (edge.from.clone(), edge.to.clone(), edge.kind.clone()))
            .collect::<BTreeSet<_>>();

        assert!(edges.contains(&(
            "world:board".to_string(),
            "mission:M1".to_string(),
            KnowledgeGraphEdgeKind::Contains
        )));
        assert!(edges.contains(&(
            "mission:M1".to_string(),
            "epic:E0".to_string(),
            KnowledgeGraphEdgeKind::Contains
        )));
        assert!(edges.contains(&(
            "epic:E0".to_string(),
            "voyage:V0".to_string(),
            KnowledgeGraphEdgeKind::Contains
        )));
        assert!(edges.contains(&(
            "story:S1".to_string(),
            "artifact:epics/E0/voyages/V0/SRS.md".to_string(),
            KnowledgeGraphEdgeKind::Traceability
        )));
        assert!(edges.contains(&(
            "doc:knowledge/semantic.md".to_string(),
            "knowledge:1AbCdE241".to_string(),
            KnowledgeGraphEdgeKind::Provenance
        )));
        assert!(edges.contains(&(
            "knowledge:1AbCdE241".to_string(),
            "source:src/read_model/knowledge_graph.rs".to_string(),
            KnowledgeGraphEdgeKind::AppliesTo
        )));
    }

    #[test]
    fn knowledge_graph_cache_manifest_is_deterministic() {
        let temp_a = build_fixture(&[("E2", 2), ("E1", 1)], &[("S2", 2), ("S1", 1)]);
        let temp_b = build_fixture(&[("E1", 1), ("E2", 2)], &[("S1", 1), ("S2", 2)]);
        let board_a = load_board(temp_a.path()).unwrap();
        let board_b = load_board(temp_b.path()).unwrap();

        let projection_a = build_knowledge_graph_projection(&board_a).unwrap();
        let projection_b = build_knowledge_graph_projection(&board_b).unwrap();
        let input_hashes_a = projection_input_hashes(&board_a, &projection_a).unwrap();
        let input_hashes_b = projection_input_hashes(&board_b, &projection_b).unwrap();
        let cache_a = save_knowledge_graph_cache(
            temp_a.path(),
            board_a.snapshot_version(),
            &projection_a,
            &input_hashes_a,
        )
        .unwrap();
        let cache_a_repeat = save_knowledge_graph_cache(
            temp_a.path(),
            board_a.snapshot_version(),
            &projection_a,
            &input_hashes_a,
        )
        .unwrap();
        let cache_b = save_knowledge_graph_cache(
            temp_b.path(),
            board_b.snapshot_version(),
            &projection_b,
            &input_hashes_b,
        )
        .unwrap();

        assert!(cache_a_repeat.reused_manifest);
        assert!(cache_a_repeat.reused_projection_blob);
        assert_eq!(projection_a, projection_b);
        assert_eq!(cache_a.manifest, cache_b.manifest);
        assert_eq!(
            fs::read_to_string(cache_a.manifest_path).unwrap(),
            fs::read_to_string(cache_b.manifest_path).unwrap()
        );
    }

    #[test]
    fn knowledge_graph_projection_exposes_structural_drift_inputs() {
        let temp = build_fixture(&[], &[("S1", 1)]);
        fs::write(temp.path().join("src/orphan.rs"), "pub fn orphan() {}\n").unwrap();
        let board = load_board(temp.path()).unwrap();

        let projection = build_knowledge_graph_projection(&board).unwrap();
        let drift = projection.drift_inputs.clone();

        assert!(drift.total_entities >= 4);
        assert_eq!(drift.total_knowledge_units, 1);
        assert_eq!(drift.knowledge_with_source_attachments, 1);
        assert!(drift.total_source_files >= 2);
        assert!(drift.source_files_without_attachments >= 1);
        assert!(drift.structural_drift_coefficient() > 0.0);
    }

    #[test]
    fn graph_drift_surfaces_remain_offline_without_semantic_cache() {
        let temp = build_fixture(&[], &[("S1", 1)]);
        fs::write(temp.path().join("src/orphan.rs"), "pub fn orphan() {}\n").unwrap();
        let cache_dir = temp.path().join("cache/knowledge-graph");
        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir).unwrap();
        }
        let board = load_board(temp.path()).unwrap();

        let first = project_structural_drift_summary(&board).unwrap();
        let second = project_structural_drift_summary(&board).unwrap();

        assert_eq!(first, second);
        assert!(first.coefficient > 0.0);
        assert!(!cache_dir.exists());
    }
}
