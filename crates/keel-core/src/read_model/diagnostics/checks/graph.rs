use std::collections::BTreeSet;

use crate::domain::model::Board;
use crate::infrastructure::validation::{CheckId, GapCategory, Problem};
use crate::read_model::board_graph::{
    BoardEdgeKind, BoardGraph, BoardNodeId, BoardNodeKind, build_board_graph,
};

pub fn check_workflow_graph_integrity(board: &Board) -> Vec<Problem> {
    check_workflow_graph_integrity_with_builder(board, build_board_graph)
}

fn check_workflow_graph_integrity_with_builder<F>(board: &Board, build: F) -> Vec<Problem>
where
    F: FnOnce(&Board) -> BoardGraph,
{
    let graph = build(board);
    collect_graph_integrity_problems(board, &graph)
}

fn collect_graph_integrity_problems(board: &Board, graph: &BoardGraph) -> Vec<Problem> {
    let mut problems = Vec::new();
    problems.extend(check_orphaned_nodes(board, graph));
    problems.extend(check_containment_cycles(board, graph));
    problems.extend(check_terminal_parent_violations(board, graph));
    problems
}

fn check_orphaned_nodes(board: &Board, graph: &BoardGraph) -> Vec<Problem> {
    let mut problems = Vec::new();

    for node in graph.nodes() {
        if node.kind == BoardNodeKind::Board {
            continue;
        }

        let expected_parent = node.declared_parent.clone().unwrap_or(BoardNodeId::Board);
        let containment_parents = graph.incoming(&node.id, BoardEdgeKind::Contains);

        if containment_parents.len() > 1 {
            problems.push(
                Problem::error(
                    path_for_node(board, &node.id),
                    format!(
                        "graph integrity: {} has multiple containment parents [{}]. Containment must form a tree rooted at the board.",
                        describe_node_ref(&node.id),
                        containment_parents
                            .iter()
                            .map(describe_node_ref)
                            .collect::<Vec<_>>()
                            .join(", "),
                    ),
                )
                .with_check_id(CheckId::WorkflowGraphIntegrity)
                .with_category(GapCategory::Coherence)
                .with_scope(node.id.raw_id()),
            );
            continue;
        }

        match containment_parents.first() {
            Some(actual_parent) if *actual_parent == expected_parent => {}
            Some(actual_parent) => {
                problems.push(
                    Problem::error(
                        path_for_node(board, &node.id),
                        format!(
                            "graph integrity: {} expects containment parent {} but is attached to {}. Repair the lineage so the node sits under the declared parent.",
                            describe_node_ref(&node.id),
                            describe_node_ref(&expected_parent),
                            describe_node_ref(actual_parent),
                        ),
                    )
                    .with_check_id(CheckId::WorkflowGraphIntegrity)
                    .with_category(GapCategory::Coherence)
                    .with_scope(node.id.raw_id()),
                );
            }
            None => {
                problems.push(
                    Problem::error(
                        path_for_node(board, &node.id),
                        format!(
                            "graph integrity: {} is orphaned from the containment tree. Expected parent: {}.",
                            describe_node_ref(&node.id),
                            describe_node_ref(&expected_parent),
                        ),
                    )
                    .with_check_id(CheckId::WorkflowGraphIntegrity)
                    .with_category(GapCategory::Coherence)
                    .with_scope(node.id.raw_id()),
                );
            }
        }
    }

    for edge in graph
        .dangling_edges()
        .iter()
        .filter(|edge| edge.kind == BoardEdgeKind::Contains)
    {
        let orphaned_node = match graph.node(&edge.to) {
            Some(node) if node.kind != BoardNodeKind::Board => Some(&node.id),
            _ => None,
        };

        if let Some(node_id) = orphaned_node {
            let path = path_for_node(board, node_id);
            let already_reported = problems.iter().any(|problem| problem.path == path);
            if !already_reported {
                problems.push(
                    Problem::error(
                        path,
                        format!(
                            "graph integrity: {} references missing containment parent {}.",
                            describe_node_ref(node_id),
                            describe_node_ref(&edge.from),
                        ),
                    )
                    .with_check_id(CheckId::WorkflowGraphIntegrity)
                    .with_category(GapCategory::Coherence)
                    .with_scope(node_id.raw_id()),
                );
            }
        }
    }

    problems
}

fn check_containment_cycles(board: &Board, graph: &BoardGraph) -> Vec<Problem> {
    let cycles = find_containment_cycles(graph);
    let mut problems = Vec::new();

    for cycle in cycles {
        let Some(anchor) = cycle.first() else {
            continue;
        };
        problems.push(
            Problem::error(
                path_for_node(board, anchor),
                format!(
                    "graph integrity: containment cycle detected [{}]. Containment must remain acyclic and rooted at the board.",
                    render_cycle(&cycle),
                ),
            )
            .with_check_id(CheckId::WorkflowGraphIntegrity)
            .with_category(GapCategory::Coherence)
            .with_scope(anchor.raw_id()),
        );
    }

    problems
}

fn check_terminal_parent_violations(board: &Board, graph: &BoardGraph) -> Vec<Problem> {
    let mut problems = Vec::new();

    for node in graph.nodes() {
        if !node.terminal || !is_containment_parent(node.kind) {
            continue;
        }

        let mut non_terminal_descendants: Vec<_> = graph
            .descendants(&node.id)
            .into_iter()
            .filter_map(|descendant_id| {
                let descendant = graph.node(&descendant_id)?;
                if descendant.terminal {
                    return None;
                }
                Some(format!(
                    "{} ({})",
                    describe_node_ref(&descendant_id),
                    descendant.state
                ))
            })
            .collect();

        if non_terminal_descendants.is_empty() {
            continue;
        }

        non_terminal_descendants.sort();
        non_terminal_descendants.dedup();

        problems.push(
            Problem::error(
                path_for_node(board, &node.id),
                format!(
                    "graph integrity: terminal {} still contains non-terminal descendants [{}]. Terminal parents must not retain unfinished descendants.",
                    describe_node_ref(&node.id),
                    non_terminal_descendants.join(", "),
                ),
            )
            .with_check_id(CheckId::WorkflowGraphIntegrity)
            .with_category(GapCategory::Coherence)
            .with_scope(node.id.raw_id()),
        );
    }

    problems
}

fn find_containment_cycles(graph: &BoardGraph) -> Vec<Vec<BoardNodeId>> {
    let mut visited = BTreeSet::new();
    let mut stack = Vec::new();
    let mut cycles = BTreeSet::new();

    for node in graph.nodes() {
        if node.kind == BoardNodeKind::Board {
            continue;
        }

        find_cycles_from(graph, &node.id, &mut visited, &mut stack, &mut cycles);
    }

    cycles.into_iter().collect()
}

fn find_cycles_from(
    graph: &BoardGraph,
    node_id: &BoardNodeId,
    visited: &mut BTreeSet<BoardNodeId>,
    stack: &mut Vec<BoardNodeId>,
    cycles: &mut BTreeSet<Vec<BoardNodeId>>,
) {
    if let Some(position) = stack.iter().position(|node| node == node_id) {
        cycles.insert(canonicalize_cycle(stack[position..].to_vec()));
        return;
    }

    if !visited.insert(node_id.clone()) {
        return;
    }

    stack.push(node_id.clone());
    for child_id in graph.children(node_id) {
        find_cycles_from(graph, child_id, visited, stack, cycles);
    }
    stack.pop();
}

fn canonicalize_cycle(cycle: Vec<BoardNodeId>) -> Vec<BoardNodeId> {
    if cycle.is_empty() {
        return cycle;
    }

    let len = cycle.len();
    let best_start = (0..len)
        .min_by(|left, right| compare_node_ids(&cycle[*left], &cycle[*right]))
        .unwrap_or(0);

    (0..len)
        .map(|offset| cycle[(best_start + offset) % len].clone())
        .collect()
}

fn render_cycle(cycle: &[BoardNodeId]) -> String {
    let mut rendered: Vec<_> = cycle.iter().map(describe_node_ref).collect();
    if let Some(first) = cycle.first() {
        rendered.push(describe_node_ref(first));
    }
    rendered.join(" -> ")
}

fn is_containment_parent(kind: BoardNodeKind) -> bool {
    matches!(
        kind,
        BoardNodeKind::Mission | BoardNodeKind::Epic | BoardNodeKind::Voyage | BoardNodeKind::Watch
    )
}

fn path_for_node(board: &Board, node_id: &BoardNodeId) -> std::path::PathBuf {
    match node_id {
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

fn describe_node_ref(node_id: &BoardNodeId) -> String {
    match node_id {
        BoardNodeId::Board => "board root".to_string(),
        BoardNodeId::Mission(id) => format!("mission {id}"),
        BoardNodeId::Epic(id) => format!("epic {id}"),
        BoardNodeId::Bearing(id) => format!("bearing {id}"),
        BoardNodeId::Adr(id) => format!("adr {id}"),
        BoardNodeId::Voyage(id) => format!("voyage {id}"),
        BoardNodeId::Story(id) => format!("story {id}"),
        BoardNodeId::Routine(id) => format!("routine {id}"),
        BoardNodeId::Watch(id) => format!("watch {id}"),
        BoardNodeId::Heartbeat => "pacemaker".to_string(),
    }
}

fn compare_node_ids(left: &BoardNodeId, right: &BoardNodeId) -> std::cmp::Ordering {
    left.raw_id()
        .cmp(right.raw_id())
        .then_with(|| left.kind().cmp(&right.kind()))
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;
    use crate::read_model::board_graph::{BoardGraphEdge, BoardGraphNode};
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};

    #[test]
    fn doctor_graph_integrity_reports_orphans_and_cycles() {
        let orphan_board_temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .story(TestStory::new("S1").scope("E1/99-missing"))
            .build();
        let orphan_board = crate::infrastructure::loader::load_board(orphan_board_temp.path())
            .expect("load orphan board");
        let orphan_problems = check_workflow_graph_integrity(&orphan_board);

        assert!(
            orphan_problems
                .iter()
                .any(|problem| problem.message.contains("story S1 is orphaned"))
        );

        let cycle_board_temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();
        let cycle_board = crate::infrastructure::loader::load_board(cycle_board_temp.path())
            .expect("load cycle board");
        let cycle_graph = BoardGraph::new(
            vec![
                BoardGraphNode {
                    id: BoardNodeId::Board,
                    title: "Keel Board".to_string(),
                    kind: BoardNodeKind::Board,
                    state: "live".to_string(),
                    terminal: false,
                    order_index: Some(0),
                    declared_parent: None,
                },
                BoardGraphNode {
                    id: BoardNodeId::Mission("M1".to_string()),
                    title: "Mission".to_string(),
                    kind: BoardNodeKind::Mission,
                    state: "active".to_string(),
                    terminal: false,
                    order_index: None,
                    declared_parent: Some(BoardNodeId::Board),
                },
                BoardGraphNode {
                    id: BoardNodeId::Epic("E1".to_string()),
                    title: "Epic".to_string(),
                    kind: BoardNodeKind::Epic,
                    state: "active".to_string(),
                    terminal: false,
                    order_index: None,
                    declared_parent: Some(BoardNodeId::Mission("M1".to_string())),
                },
            ],
            vec![
                BoardGraphEdge {
                    from: BoardNodeId::Board,
                    to: BoardNodeId::Mission("M1".to_string()),
                    kind: BoardEdgeKind::Contains,
                },
                BoardGraphEdge {
                    from: BoardNodeId::Mission("M1".to_string()),
                    to: BoardNodeId::Epic("E1".to_string()),
                    kind: BoardEdgeKind::Contains,
                },
                BoardGraphEdge {
                    from: BoardNodeId::Epic("E1".to_string()),
                    to: BoardNodeId::Mission("M1".to_string()),
                    kind: BoardEdgeKind::Contains,
                },
            ],
            Vec::new(),
        );
        let cycle_problems = collect_graph_integrity_problems(&cycle_board, &cycle_graph);

        assert!(cycle_problems.iter().any(|problem| {
            problem
                .message
                .contains("containment cycle detected [epic E1 -> mission M1 -> epic E1]")
        }));
    }

    #[test]
    fn doctor_graph_integrity_reports_terminal_parent_violations() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("verified"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("01-graph", "E1").status("planned"))
            .story(TestStory::new("S1").scope("E1/01-graph"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).expect("load board");

        let problems = check_workflow_graph_integrity(&board);

        assert!(problems.iter().any(|problem| {
            problem.message.contains("terminal mission M1")
                && problem.message.contains("story S1 (backlog)")
        }));
    }

    #[test]
    fn doctor_graph_integrity_uses_single_graph_build() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).expect("load board");
        let build_count = Cell::new(0usize);

        let _ = check_workflow_graph_integrity_with_builder(&board, |board| {
            build_count.set(build_count.get() + 1);
            build_board_graph(board)
        });

        assert_eq!(build_count.get(), 1);
    }
}
