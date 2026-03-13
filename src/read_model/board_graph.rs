//! Canonical board relationship graph projection.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

use crate::domain::model::{Board, Epic, Routine, Story, Voyage, VoyageState};
use crate::infrastructure::utils::cmp_optional_index_then_id;
use crate::infrastructure::verification::parse_ac_references;

const BOARD_TITLE: &str = "Keel Board";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoardNodeKind {
    Board,
    Mission,
    Epic,
    Bearing,
    Adr,
    Voyage,
    Story,
    Routine,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoardNodeId {
    Board,
    Mission(String),
    Epic(String),
    Bearing(String),
    Adr(String),
    Voyage(String),
    Story(String),
    Routine(String),
}

impl BoardNodeId {
    pub fn kind(&self) -> BoardNodeKind {
        match self {
            Self::Board => BoardNodeKind::Board,
            Self::Mission(_) => BoardNodeKind::Mission,
            Self::Epic(_) => BoardNodeKind::Epic,
            Self::Bearing(_) => BoardNodeKind::Bearing,
            Self::Adr(_) => BoardNodeKind::Adr,
            Self::Voyage(_) => BoardNodeKind::Voyage,
            Self::Story(_) => BoardNodeKind::Story,
            Self::Routine(_) => BoardNodeKind::Routine,
        }
    }

    pub fn raw_id(&self) -> &str {
        match self {
            Self::Board => "__board__",
            Self::Mission(id)
            | Self::Epic(id)
            | Self::Bearing(id)
            | Self::Adr(id)
            | Self::Voyage(id)
            | Self::Story(id)
            | Self::Routine(id) => id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoardEdgeKind {
    Contains,
    DependsOn,
    GovernedBy,
    LaidInto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardGraphNode {
    pub id: BoardNodeId,
    pub title: String,
    pub kind: BoardNodeKind,
    pub state: String,
    pub terminal: bool,
    pub order_index: Option<u32>,
    pub declared_parent: Option<BoardNodeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardGraphEdge {
    pub from: BoardNodeId,
    pub to: BoardNodeId,
    pub kind: BoardEdgeKind,
}

impl BoardGraphEdge {
    fn new(from: BoardNodeId, to: BoardNodeId, kind: BoardEdgeKind) -> Self {
        Self { from, to, kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardGraphDanglingEdge {
    pub from: BoardNodeId,
    pub to: BoardNodeId,
    pub kind: BoardEdgeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardGraph {
    nodes: Vec<BoardGraphNode>,
    edges: Vec<BoardGraphEdge>,
    dangling_edges: Vec<BoardGraphDanglingEdge>,
    node_positions: BTreeMap<BoardNodeId, usize>,
    outgoing: BTreeMap<BoardNodeId, BTreeMap<BoardEdgeKind, Vec<BoardNodeId>>>,
    incoming: BTreeMap<BoardNodeId, BTreeMap<BoardEdgeKind, Vec<BoardNodeId>>>,
}

pub fn build_board_graph(board: &Board) -> BoardGraph {
    let mut nodes = vec![BoardGraphNode {
        id: BoardNodeId::Board,
        title: BOARD_TITLE.to_string(),
        kind: BoardNodeKind::Board,
        state: "live".to_string(),
        terminal: false,
        order_index: Some(0),
        declared_parent: None,
    }];

    for mission in sorted_missions(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Mission(mission.id().to_string()),
            title: mission.title().to_string(),
            kind: BoardNodeKind::Mission,
            state: mission.status().to_string(),
            terminal: mission.status().is_terminal(),
            order_index: None,
            declared_parent: Some(BoardNodeId::Board),
        });
    }

    for epic in sorted_epics(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Epic(epic.id().to_string()),
            title: epic.title().to_string(),
            kind: BoardNodeKind::Epic,
            state: epic.status().to_string(),
            terminal: epic.status().to_string() == "done",
            order_index: epic.index(),
            declared_parent: epic
                .frontmatter
                .mission
                .as_ref()
                .map(|mission_id| BoardNodeId::Mission(mission_id.clone())),
        });
    }

    for bearing in sorted_bearings(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Bearing(bearing.id().to_string()),
            title: bearing.title().to_string(),
            kind: BoardNodeKind::Bearing,
            state: bearing.status().to_string(),
            terminal: bearing.is_complete(),
            order_index: bearing.frontmatter.index,
            declared_parent: bearing
                .frontmatter
                .mission
                .as_ref()
                .map(|mission_id| BoardNodeId::Mission(mission_id.clone())),
        });
    }

    for adr in sorted_adrs(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Adr(adr.id().to_string()),
            title: adr.title().to_string(),
            kind: BoardNodeKind::Adr,
            state: adr.status().to_string(),
            terminal: adr.status().is_terminal(),
            order_index: adr.index(),
            declared_parent: adr
                .frontmatter
                .mission
                .as_ref()
                .map(|mission_id| BoardNodeId::Mission(mission_id.clone())),
        });
    }

    for voyage in sorted_voyages(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Voyage(voyage.id().to_string()),
            title: voyage.title().to_string(),
            kind: BoardNodeKind::Voyage,
            state: voyage.status().to_string(),
            terminal: voyage.status() == VoyageState::Done,
            order_index: voyage.index(),
            declared_parent: Some(BoardNodeId::Epic(voyage.epic_id.clone())),
        });
    }

    for story in sorted_stories(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Story(story.id().to_string()),
            title: story.title().to_string(),
            kind: BoardNodeKind::Story,
            state: story.status().to_string(),
            terminal: story.status().is_terminal(),
            order_index: story.index(),
            declared_parent: story_declared_parent(story),
        });
    }

    for routine in sorted_routines(board) {
        nodes.push(BoardGraphNode {
            id: BoardNodeId::Routine(routine.id().to_string()),
            title: routine.title().to_string(),
            kind: BoardNodeKind::Routine,
            state: "scheduled".to_string(),
            terminal: false,
            order_index: None,
            declared_parent: routine_declared_parent(routine),
        });
    }

    let known_ids: BTreeSet<_> = nodes.iter().map(|node| node.id.clone()).collect();
    let mut edges = BTreeSet::new();
    let mut dangling_edges = BTreeSet::new();

    for mission in sorted_missions(board) {
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            BoardNodeId::Board,
            BoardNodeId::Mission(mission.id().to_string()),
            BoardEdgeKind::Contains,
        );
    }

    for epic in sorted_epics(board) {
        let parent = epic
            .frontmatter
            .mission
            .as_ref()
            .map(|mission_id| BoardNodeId::Mission(mission_id.clone()))
            .unwrap_or(BoardNodeId::Board);
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            parent,
            BoardNodeId::Epic(epic.id().to_string()),
            BoardEdgeKind::Contains,
        );

        if let Some(bearing_id) = &epic.frontmatter.bearing {
            add_edge(
                &known_ids,
                &mut edges,
                &mut dangling_edges,
                BoardNodeId::Bearing(bearing_id.clone()),
                BoardNodeId::Epic(epic.id().to_string()),
                BoardEdgeKind::LaidInto,
            );
        }
    }

    for bearing in sorted_bearings(board) {
        let parent = bearing
            .frontmatter
            .mission
            .as_ref()
            .map(|mission_id| BoardNodeId::Mission(mission_id.clone()))
            .unwrap_or(BoardNodeId::Board);
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            parent,
            BoardNodeId::Bearing(bearing.id().to_string()),
            BoardEdgeKind::Contains,
        );

        if let Some(epic_id) = &bearing.frontmatter.epic {
            add_edge(
                &known_ids,
                &mut edges,
                &mut dangling_edges,
                BoardNodeId::Bearing(bearing.id().to_string()),
                BoardNodeId::Epic(epic_id.clone()),
                BoardEdgeKind::LaidInto,
            );
        }
    }

    for adr in sorted_adrs(board) {
        let parent = adr
            .frontmatter
            .mission
            .as_ref()
            .map(|mission_id| BoardNodeId::Mission(mission_id.clone()))
            .unwrap_or(BoardNodeId::Board);
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            parent,
            BoardNodeId::Adr(adr.id().to_string()),
            BoardEdgeKind::Contains,
        );
    }

    for voyage in sorted_voyages(board) {
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            BoardNodeId::Epic(voyage.epic_id.clone()),
            BoardNodeId::Voyage(voyage.id().to_string()),
            BoardEdgeKind::Contains,
        );
    }

    for story in sorted_stories(board) {
        let story_id = BoardNodeId::Story(story.id().to_string());
        let parent = story_declared_parent(story).unwrap_or(BoardNodeId::Board);
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            parent,
            story_id.clone(),
            BoardEdgeKind::Contains,
        );

        for adr_id in &story.frontmatter.governed_by {
            add_edge(
                &known_ids,
                &mut edges,
                &mut dangling_edges,
                story_id.clone(),
                BoardNodeId::Adr(adr_id.clone()),
                BoardEdgeKind::GovernedBy,
            );
        }
    }

    for routine in sorted_routines(board) {
        let routine_id = BoardNodeId::Routine(routine.id().to_string());
        let parent = routine_declared_parent(routine).unwrap_or(BoardNodeId::Board);
        add_edge(
            &known_ids,
            &mut edges,
            &mut dangling_edges,
            parent,
            routine_id,
            BoardEdgeKind::Contains,
        );
    }

    for (story_id, dependency_ids) in derive_story_dependencies(board) {
        let from = BoardNodeId::Story(story_id);
        for dependency_id in dependency_ids {
            add_edge(
                &known_ids,
                &mut edges,
                &mut dangling_edges,
                from.clone(),
                BoardNodeId::Story(dependency_id),
                BoardEdgeKind::DependsOn,
            );
        }
    }

    BoardGraph::new(
        nodes,
        edges.into_iter().collect(),
        dangling_edges.into_iter().collect(),
    )
}

impl BoardGraph {
    pub(crate) fn new(
        mut nodes: Vec<BoardGraphNode>,
        mut edges: Vec<BoardGraphEdge>,
        mut dangling_edges: Vec<BoardGraphDanglingEdge>,
    ) -> Self {
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        edges.sort();
        dangling_edges.sort();

        let node_positions = nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| (node.id.clone(), idx))
            .collect();

        let mut outgoing =
            BTreeMap::<BoardNodeId, BTreeMap<BoardEdgeKind, Vec<BoardNodeId>>>::new();
        let mut incoming =
            BTreeMap::<BoardNodeId, BTreeMap<BoardEdgeKind, Vec<BoardNodeId>>>::new();

        for edge in &edges {
            outgoing
                .entry(edge.from.clone())
                .or_default()
                .entry(edge.kind)
                .or_default()
                .push(edge.to.clone());
            incoming
                .entry(edge.to.clone())
                .or_default()
                .entry(edge.kind)
                .or_default()
                .push(edge.from.clone());
        }

        normalize_index(&mut outgoing);
        normalize_index(&mut incoming);

        Self {
            nodes,
            edges,
            dangling_edges,
            node_positions,
            outgoing,
            incoming,
        }
    }

    pub fn nodes(&self) -> &[BoardGraphNode] {
        &self.nodes
    }

    pub fn edges(&self) -> &[BoardGraphEdge] {
        &self.edges
    }

    pub fn dangling_edges(&self) -> &[BoardGraphDanglingEdge] {
        &self.dangling_edges
    }

    pub fn node(&self, id: &BoardNodeId) -> Option<&BoardGraphNode> {
        self.node_positions.get(id).map(|idx| &self.nodes[*idx])
    }

    pub fn outgoing(&self, id: &BoardNodeId, kind: BoardEdgeKind) -> &[BoardNodeId] {
        self.outgoing
            .get(id)
            .and_then(|edges| edges.get(&kind))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn incoming(&self, id: &BoardNodeId, kind: BoardEdgeKind) -> &[BoardNodeId] {
        self.incoming
            .get(id)
            .and_then(|edges| edges.get(&kind))
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn parent(&self, id: &BoardNodeId) -> Option<&BoardNodeId> {
        self.incoming(id, BoardEdgeKind::Contains).first()
    }

    pub fn children(&self, id: &BoardNodeId) -> &[BoardNodeId] {
        self.outgoing(id, BoardEdgeKind::Contains)
    }

    pub fn dependencies(&self, id: &BoardNodeId) -> &[BoardNodeId] {
        self.outgoing(id, BoardEdgeKind::DependsOn)
    }

    pub fn ancestors(&self, id: &BoardNodeId) -> Vec<BoardNodeId> {
        traverse(self, id, BoardEdgeKind::Contains, Direction::Incoming)
    }

    pub fn descendants(&self, id: &BoardNodeId) -> Vec<BoardNodeId> {
        traverse(self, id, BoardEdgeKind::Contains, Direction::Outgoing)
    }

    pub fn story_dependencies(&self) -> HashMap<String, Vec<String>> {
        let mut dependencies = HashMap::new();

        for node in &self.nodes {
            let BoardNodeId::Story(story_id) = &node.id else {
                continue;
            };

            let story_dependencies: Vec<_> = self
                .dependencies(&node.id)
                .iter()
                .filter_map(|dependency_id| match dependency_id {
                    BoardNodeId::Story(id) => Some(id.clone()),
                    _ => None,
                })
                .collect();

            if !story_dependencies.is_empty() {
                dependencies.insert(story_id.clone(), story_dependencies);
            }
        }

        dependencies
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Outgoing,
    Incoming,
}

fn traverse(
    graph: &BoardGraph,
    origin: &BoardNodeId,
    kind: BoardEdgeKind,
    direction: Direction,
) -> Vec<BoardNodeId> {
    let mut visited = BTreeSet::new();
    let mut result = Vec::new();
    let mut stack: Vec<_> = match direction {
        Direction::Outgoing => graph.outgoing(origin, kind).iter().rev().cloned().collect(),
        Direction::Incoming => graph.incoming(origin, kind).iter().rev().cloned().collect(),
    };

    while let Some(next) = stack.pop() {
        if !visited.insert(next.clone()) {
            continue;
        }

        result.push(next.clone());

        let neighbors = match direction {
            Direction::Outgoing => graph.outgoing(&next, kind),
            Direction::Incoming => graph.incoming(&next, kind),
        };
        for neighbor in neighbors.iter().rev() {
            stack.push(neighbor.clone());
        }
    }

    result
}

fn normalize_index(index: &mut BTreeMap<BoardNodeId, BTreeMap<BoardEdgeKind, Vec<BoardNodeId>>>) {
    for edges_by_kind in index.values_mut() {
        for ids in edges_by_kind.values_mut() {
            ids.sort_by(compare_node_id);
            ids.dedup();
        }
    }
}

fn compare_node_id(left: &BoardNodeId, right: &BoardNodeId) -> std::cmp::Ordering {
    left.raw_id()
        .cmp(right.raw_id())
        .then_with(|| left.kind().cmp(&right.kind()))
}

fn add_edge(
    known_ids: &BTreeSet<BoardNodeId>,
    edges: &mut BTreeSet<BoardGraphEdge>,
    dangling_edges: &mut BTreeSet<BoardGraphDanglingEdge>,
    from: BoardNodeId,
    to: BoardNodeId,
    kind: BoardEdgeKind,
) {
    if known_ids.contains(&from) && known_ids.contains(&to) {
        edges.insert(BoardGraphEdge::new(from, to, kind));
    } else if known_ids.contains(&from) || known_ids.contains(&to) {
        dangling_edges.insert(BoardGraphDanglingEdge { from, to, kind });
    }
}

fn story_declared_parent(story: &Story) -> Option<BoardNodeId> {
    match (story.epic(), story.voyage()) {
        (Some(_), Some(voyage_id)) => Some(BoardNodeId::Voyage(voyage_id.to_string())),
        (Some(epic_id), None) => Some(BoardNodeId::Epic(epic_id.to_string())),
        (None, _) => None,
    }
}

fn routine_declared_parent(routine: &Routine) -> Option<BoardNodeId> {
    let target_scope = routine.target_scope().trim();
    if target_scope.is_empty() {
        return None;
    }

    if let Some((_, voyage_id)) = target_scope.split_once('/') {
        return Some(BoardNodeId::Voyage(voyage_id.to_string()));
    }

    Some(BoardNodeId::Epic(target_scope.to_string()))
}

fn derive_story_dependencies(board: &Board) -> BTreeMap<String, Vec<String>> {
    let mut dependencies = BTreeMap::<String, Vec<String>>::new();
    let mut scope_to_requirements: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for story in sorted_stories(board) {
        let Some(scope) = story.scope() else {
            continue;
        };

        let content = match fs::read_to_string(&story.path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        for requirement_ref in parse_ac_references(&content) {
            scope_to_requirements
                .entry(scope.to_string())
                .or_default()
                .entry(requirement_ref.srs_id)
                .or_default()
                .push(story.id().to_string());
        }
    }

    for requirements in scope_to_requirements.values_mut() {
        for story_ids in requirements.values_mut() {
            story_ids.sort();
            story_ids.dedup();
        }
    }

    for voyage in sorted_voyages(board) {
        let srs_path = match voyage.path.parent() {
            Some(parent) => parent.join("SRS.md"),
            None => continue,
        };
        if !srs_path.exists() {
            continue;
        }

        let requirement_order =
            crate::domain::state_machine::invariants::parse_requirements(&srs_path);
        let scope = voyage.scope_path();
        let Some(requirement_to_stories) = scope_to_requirements.get(&scope) else {
            continue;
        };

        let mut previous_stories = Vec::<String>::new();
        for requirement_id in requirement_order {
            if requirement_id.contains("NFR") {
                continue;
            }

            if let Some(story_ids) = requirement_to_stories.get(&requirement_id) {
                for story_id in story_ids {
                    let entry = dependencies.entry(story_id.clone()).or_default();
                    entry.extend(
                        previous_stories
                            .iter()
                            .filter(|dependency_id| dependency_id.as_str() != story_id.as_str())
                            .cloned(),
                    );
                }

                previous_stories.extend(story_ids.iter().cloned());
                previous_stories.sort();
                previous_stories.dedup();
            }
        }
    }

    for story in sorted_stories(board) {
        if story.frontmatter.blocked_by.is_empty() {
            continue;
        }

        let entry = dependencies.entry(story.id().to_string()).or_default();
        entry.extend(story.frontmatter.blocked_by.iter().cloned());
    }

    for story_ids in dependencies.values_mut() {
        story_ids.sort();
        story_ids.dedup();
    }

    dependencies.retain(|_, story_ids| !story_ids.is_empty());
    dependencies
}

fn sorted_missions(board: &Board) -> Vec<&crate::domain::model::Mission> {
    let mut missions: Vec<_> = board.missions.values().collect();
    missions.sort_by(|left, right| left.id().cmp(right.id()));
    missions
}

fn sorted_epics(board: &Board) -> Vec<&Epic> {
    let mut epics: Vec<_> = board.epics.values().collect();
    epics.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    epics
}

fn sorted_bearings(board: &Board) -> Vec<&crate::domain::model::Bearing> {
    let mut bearings: Vec<_> = board.bearings.values().collect();
    bearings.sort_by(|left, right| {
        cmp_optional_index_then_id(
            left.frontmatter.index,
            left.id(),
            right.frontmatter.index,
            right.id(),
        )
    });
    bearings
}

fn sorted_adrs(board: &Board) -> Vec<&crate::domain::model::Adr> {
    let mut adrs: Vec<_> = board.adrs.values().collect();
    adrs.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    adrs
}

fn sorted_voyages(board: &Board) -> Vec<&Voyage> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    voyages
}

fn sorted_stories(board: &Board) -> Vec<&Story> {
    let mut stories: Vec<_> = board.stories.values().collect();
    stories.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    stories
}

fn sorted_routines(board: &Board) -> Vec<&Routine> {
    let mut routines: Vec<_> = board.routines.values().collect();
    routines.sort_by(|left, right| left.id().cmp(right.id()));
    routines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{
        TestAdr, TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };
    use std::fs;

    fn write_routine(root: &std::path::Path, id: &str, target_scope: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: Weekly Review
cadence:
  cron: 0 9 * * 1
  timezone: America/Los_Angeles
target-scope: {target_scope}
created_at: 2026-03-12T09:00:00
updated_at: 2026-03-12T09:00:00
---

# Blueprint
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn board_graph_builds_typed_relationships() {
        let srs = r#"# SRS

## Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | First requirement | SCOPE-01 | FR-01 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("epic-1").mission("M1").index(2))
            .bearing(
                TestBearing::new("BRG-01")
                    .mission("M1")
                    .status("laid")
                    .epic("epic-1"),
            )
            .adr(TestAdr::new("ADR-0001").mission("M1").status("accepted"))
            .voyage(
                TestVoyage::new("01-graph", "epic-1")
                    .status("planned")
                    .index(1)
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY-01")
                    .scope("epic-1/01-graph")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] build graph"),
            )
            .build();
        write_routine(temp.path(), "routine-weekly-review", "epic-1/01-graph");

        let board = load_board(temp.path()).unwrap();
        let graph = build_board_graph(&board);

        assert!(graph.node(&BoardNodeId::Board).is_some());
        assert!(
            graph
                .node(&BoardNodeId::Mission("M1".to_string()))
                .is_some()
        );
        assert!(
            graph
                .node(&BoardNodeId::Epic("epic-1".to_string()))
                .is_some()
        );
        assert!(
            graph
                .node(&BoardNodeId::Bearing("BRG-01".to_string()))
                .is_some()
        );
        assert!(
            graph
                .node(&BoardNodeId::Adr("ADR-0001".to_string()))
                .is_some()
        );
        assert!(
            graph
                .node(&BoardNodeId::Voyage("01-graph".to_string()))
                .is_some()
        );
        assert!(
            graph
                .node(&BoardNodeId::Story("STORY-01".to_string()))
                .is_some()
        );
        assert!(
            graph
                .node(&BoardNodeId::Routine("routine-weekly-review".to_string()))
                .is_some()
        );

        assert_eq!(
            graph.children(&BoardNodeId::Mission("M1".to_string())),
            &[
                BoardNodeId::Adr("ADR-0001".to_string()),
                BoardNodeId::Bearing("BRG-01".to_string()),
                BoardNodeId::Epic("epic-1".to_string()),
            ]
        );
        assert_eq!(
            graph.children(&BoardNodeId::Epic("epic-1".to_string())),
            &[BoardNodeId::Voyage("01-graph".to_string())]
        );
        assert_eq!(
            graph.children(&BoardNodeId::Voyage("01-graph".to_string())),
            &[
                BoardNodeId::Story("STORY-01".to_string()),
                BoardNodeId::Routine("routine-weekly-review".to_string()),
            ]
        );
        assert_eq!(
            graph.outgoing(
                &BoardNodeId::Bearing("BRG-01".to_string()),
                BoardEdgeKind::LaidInto,
            ),
            &[BoardNodeId::Epic("epic-1".to_string())]
        );
    }

    #[test]
    fn board_graph_queries_traverse_relationships() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("epic-1").mission("M1"))
            .voyage(TestVoyage::new("01-graph", "epic-1").status("planned"))
            .story(TestStory::new("STORY-01").scope("epic-1/01-graph"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let graph = build_board_graph(&board);
        let story_id = BoardNodeId::Story("STORY-01".to_string());

        assert_eq!(
            graph.parent(&story_id),
            Some(&BoardNodeId::Voyage("01-graph".to_string()))
        );
        assert_eq!(
            graph.ancestors(&story_id),
            vec![
                BoardNodeId::Voyage("01-graph".to_string()),
                BoardNodeId::Epic("epic-1".to_string()),
                BoardNodeId::Mission("M1".to_string()),
                BoardNodeId::Board,
            ]
        );
        assert_eq!(
            graph.descendants(&BoardNodeId::Mission("M1".to_string())),
            vec![
                BoardNodeId::Epic("epic-1".to_string()),
                BoardNodeId::Voyage("01-graph".to_string()),
                BoardNodeId::Story("STORY-01".to_string()),
            ]
        );
    }

    #[test]
    fn board_graph_merges_story_dependency_sources() {
        let srs = r#"# SRS

## Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | First requirement | SCOPE-01 | FR-01 | test |
| SRS-02 | Second requirement | SCOPE-01 | FR-02 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .voyage(
                TestVoyage::new("01-graph", "epic-1")
                    .status("planned")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY-01")
                    .scope("epic-1/01-graph")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] first"),
            )
            .story(
                TestStory::new("STORY-02")
                    .scope("epic-1/01-graph")
                    .blocked_by(&["STORY-03"])
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] second"),
            )
            .story(
                TestStory::new("STORY-03")
                    .scope("epic-1/01-graph")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-02/AC-02] explicit"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let graph = build_board_graph(&board);

        assert_eq!(
            graph.dependencies(&BoardNodeId::Story("STORY-02".to_string())),
            &[
                BoardNodeId::Story("STORY-01".to_string()),
                BoardNodeId::Story("STORY-03".to_string()),
            ]
        );
        assert_eq!(
            graph.story_dependencies().get("STORY-02"),
            Some(&vec!["STORY-01".to_string(), "STORY-03".to_string()])
        );
    }

    #[test]
    fn board_graph_is_deterministic_across_equivalent_boards() {
        let srs = r#"# SRS

## Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | First requirement | SCOPE-01 | FR-01 | test |
| SRS-02 | Second requirement | SCOPE-01 | FR-02 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        let temp_a = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("epic-1").mission("M1").index(2))
            .epic(TestEpic::new("epic-2").index(1))
            .voyage(
                TestVoyage::new("01-a", "epic-1")
                    .status("planned")
                    .index(2)
                    .srs_content(srs),
            )
            .voyage(
                TestVoyage::new("01-b", "epic-2")
                    .status("planned")
                    .index(1)
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY-02")
                    .scope("epic-1/01-a")
                    .blocked_by(&["STORY-01"])
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] second"),
            )
            .story(
                TestStory::new("STORY-01")
                    .scope("epic-1/01-a")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] first"),
            )
            .build();
        let temp_b = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-2").index(1))
            .mission(TestMission::new("M1").status("active"))
            .voyage(
                TestVoyage::new("01-b", "epic-2")
                    .status("planned")
                    .index(1)
                    .srs_content(srs),
            )
            .epic(TestEpic::new("epic-1").mission("M1").index(2))
            .story(
                TestStory::new("STORY-01")
                    .scope("epic-1/01-a")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] first"),
            )
            .voyage(
                TestVoyage::new("01-a", "epic-1")
                    .status("planned")
                    .index(2)
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY-02")
                    .scope("epic-1/01-a")
                    .blocked_by(&["STORY-01"])
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] second"),
            )
            .build();

        let board_a = load_board(temp_a.path()).unwrap();
        let board_b = load_board(temp_b.path()).unwrap();
        let graph_a = build_board_graph(&board_a);
        let graph_b = build_board_graph(&board_b);

        assert_eq!(graph_a.nodes(), graph_b.nodes());
        assert_eq!(graph_a.edges(), graph_b.edges());
        assert_eq!(graph_a.dangling_edges(), graph_b.dangling_edges());
        assert_eq!(graph_a.story_dependencies(), graph_b.story_dependencies());
    }
}
